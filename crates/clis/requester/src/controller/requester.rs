use catalog::catalog::{JobRequest, JobRequestSubmitted};
use clis::{Gossip, Request};
use codec::Encode;
use libp2p::PeerId;
use std::any::Any;
use subxt::{ext::futures::StreamExt, Config};
use tokio::{select, signal::ctrl_c, task::JoinHandle};
use tracing::{error, info};
use utils::services::{
    contract_client::{ContractClient, ContractClientError},
    job::{
        job_builder::{JobBuilderService, JobBuilderServiceError},
        Job, JobT,
    },
    p2p::{GossipMessage, NetworkClient, NetworkClientError},
};

pub struct RequesterController<C: Config, CC, JB, NC> {
    contract_client: CC,
    contract_address: <C as Config>::AccountId,
    job_builder_service: JB,
    network_client: NC,
}

impl<C, CC, JB, NC> RequesterController<C, CC, JB, NC>
where
    C: Config,
    CC: ContractClient<C = C>,
    JB: JobBuilderService,
    NC: NetworkClient,
    RequesterControllerError: From<<NC as NetworkClient>::Err>
        + From<<CC as ContractClient>::Err>
        + From<<JB as JobBuilderService>::Err>,
{
    pub fn new(
        contract_client: CC,
        contract_address: <C as Config>::AccountId,
        job_builder_service: JB,
        network_client: NC,
    ) -> Self {
        Self {
            contract_client,
            contract_address,
            job_builder_service,
            network_client,
        }
    }

    pub async fn start(
        &self,
        handle: JoinHandle<Result<(), <NC as NetworkClient>::Err>>,
    ) -> Result<(), RequesterControllerError> {
        select! {
            _ = handle => {},
            result = self.run() => {
                match result {
                    Err(err) => error!("Encountered error: {}", err),
                    Ok(()) => info!("Successfully submitted Job")
                };
            },
            _ = ctrl_c() => {
                info!("Shutting down...")
            }
        };

        Ok(())
    }

    async fn run(&self) -> Result<(), RequesterControllerError> {
        let job = self.job_builder_service.build_job().await?;
        let job_request = JobRequest::new(job.code_ref(), job.params_ref());

        self.submit_job(&job_request).await?;
        let msg = self
            .wait_for_job_acceptance(&job_request)
            .await
            .ok_or_else(|| RequesterControllerError::JobNeverAccepted)?;
        self.send_job(msg.peer_id(), job).await?;

        Ok(())
    }

    async fn submit_job(&self, job_request: &JobRequest) -> Result<(), RequesterControllerError> {
        self.contract_client
            .write::<JobRequestSubmitted, JobRequest>(
                self.contract_address.clone(),
                "submit_job_request",
                job_request,
            )
            .await?;

        info!("Job Request Submitted!");
        Ok(())
    }

    async fn wait_for_job_acceptance(&self, job_request: &JobRequest) -> Option<GossipMessage> {
        let gossip_stream = self.network_client.gossip_msg_stream().await;
        tokio::pin!(gossip_stream);

        while let Some(gsp_msg) = gossip_stream.next().await {
            if let Ok(Gossip::JobAcceptance { job_id }) = Gossip::decode(gsp_msg.message()) {
                if job_id == job_request.id() {
                    info!("Job acceptance received from peer: {}", gsp_msg.peer_id());
                    return Some(gsp_msg);
                }
            } else {
                error!(
                    "Unable to decode gossip message from peer {}: {:?}",
                    gsp_msg.peer_id(),
                    gsp_msg.message()
                );
            }
        }

        None
    }

    async fn send_job(
        &self,
        peer_id: PeerId,
        job: impl JobT,
    ) -> Result<(), RequesterControllerError> {
        let boxed_job: Box<dyn Any> = Box::new(job);
        let job = boxed_job
            .downcast::<Job>()
            .map_err(|_| RequesterControllerError::DownCastError)?;
        let req = Request::Job(*job);

        self.network_client
            .send_request(peer_id, req.encode())
            .await?;
        info!("Job sent to peer: {}", peer_id);

        Ok(())
    }
}

#[derive(Debug, thiserror::Error)]
pub enum RequesterControllerError {
    #[error("{source}")]
    ContractClient {
        #[from]
        source: ContractClientError,
    },

    #[error("{source}")]
    JobBuilderService {
        #[from]
        source: JobBuilderServiceError,
    },

    #[error("{source}")]
    NetworkClient {
        #[from]
        source: NetworkClientError,
    },

    #[error("")]
    JobNeverAccepted,

    #[error("")]
    DownCastError,
}
