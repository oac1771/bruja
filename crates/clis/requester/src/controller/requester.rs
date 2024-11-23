use catalog::catalog::{HashId, JobRequest, JobRequestSubmitted};
use clis::{Gossip, Request, Response};
use codec::Encode;
use subxt::{ext::futures::StreamExt, Config};
use tracing::{error, info};
use utils::services::{
    contract_client::{ContractClient, ContractClientError},
    job::{
        job_builder::{JobBuilderService, JobBuilderServiceError},
        JobT,
    },
    p2p::{GossipMessageT, NetworkClient, NetworkClientError, NetworkIdT, RequestT, ResponseT},
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

    pub async fn run(&self) -> Result<Vec<Vec<u8>>, RequesterControllerError> {
        let job = self.job_builder_service.build_job().await?;
        let job_request = JobRequest::new(job.code_ref(), job.params_ref());

        self.submit_job(&job_request).await?;
        let msg = self
            .wait_for_job_acceptance(&job_request)
            .await
            .ok_or_else(|| RequesterControllerError::JobNeverAccepted)?;
        self.send_job(msg.network_id(), job).await?;
        self.wait_for_job_acknowledgement(job_request.id()).await;
        let (req_id, results) = self
            .wait_for_job_results(job_request.id())
            .await
            .ok_or_else(|| RequesterControllerError::ResultsNeverReceived)?;
        self.send_result_acknowledgement(req_id, job_request.id())
            .await?;

        Ok(results)
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

    async fn wait_for_job_acceptance(
        &self,
        job_request: &JobRequest,
    ) -> Option<<NC as NetworkClient>::GossipMessage> {
        let gossip_stream = self.network_client.gossip_msg_stream().await;
        tokio::pin!(gossip_stream);

        while let Some(gsp_msg) = gossip_stream.next().await {
            let network_id = gsp_msg.network_id();
            if let Ok(Gossip::JobAcceptance { job_id }) = Gossip::decode(gsp_msg.message_ref()) {
                if job_id == job_request.id() {
                    info!("Job acceptance received from peer: {}", network_id);
                    return Some(gsp_msg);
                }
            } else {
                error!(
                    "Unable to decode gossip message from peer {}: {:?}",
                    network_id,
                    gsp_msg.message_ref()
                );
            }
        }

        None
    }

    async fn send_job(
        &self,
        network_id: <NC as NetworkClient>::NetworkId,
        job: impl JobT,
    ) -> Result<(), RequesterControllerError> {
        let who = self.network_client.get_local_network_id().await?;
        let req = Request::build_job_req(job.into_parts(), who.to_vec());

        self.network_client
            .send_request(network_id, req.encode())
            .await?;

        info!("Job sent to peer: {:?}", network_id.to_vec());

        Ok(())
    }

    async fn wait_for_job_acknowledgement(&self, id: HashId) {
        let resp_stream = self.network_client.resp_stream().await;
        tokio::pin!(resp_stream);

        while let Some(resp) = resp_stream.next().await {
            if let Ok(Response::AcknowledgeJob { job_id }) = Response::decode(resp.body_ref()) {
                if job_id == id {
                    info!("Job has been accepted by a worker");
                    break;
                }
            } else {
                error!("Unable to decode response");
            }
        }
    }

    async fn wait_for_job_results(
        &self,
        id: HashId,
    ) -> Option<(<NC as NetworkClient>::Id, Vec<Vec<u8>>)> {
        let req_stream = self.network_client.req_stream().await;
        tokio::pin!(req_stream);
        while let Some(req) = req_stream.next().await {
            if let Ok(Request::Result { result, job_id }) = Request::decode(req.body_ref()) {
                if job_id == id {
                    info!("Received results");
                    return Some((req.id(), result));
                }
            } else {
                error!("Unable to decode request");
            }
        }

        None
    }

    async fn send_result_acknowledgement(
        &self,
        req_id: <NC as NetworkClient>::Id,
        job_id: HashId,
    ) -> Result<(), RequesterControllerError> {
        let resp = Response::AcknowledgeResult { job_id };

        self.network_client
            .send_response(req_id, resp.encode())
            .await?;

        info!("Result acknowledgement sent");

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
    ResultsNeverReceived,
}
