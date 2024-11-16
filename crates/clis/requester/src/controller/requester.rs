use catalog::catalog::{JobRequest, JobRequestSubmitted};
use subxt::{ext::futures::StreamExt, Config};
use tokio::{select, signal::ctrl_c, task::JoinHandle};
use tracing::{error, info};
use utils::services::{
    contract_client::{ContractClient, ContractClientError},
    job::{JobService, JobServiceError},
    p2p::NetworkClient,
};

pub struct RequesterController<C: Config, CC, JS, NC> {
    contract_client: CC,
    contract_address: <C as Config>::AccountId,
    job_service: JS,
    network_client: NC,
}

impl<C, CC, JS, NC> RequesterController<C, CC, JS, NC>
where
    C: Config,
    CC: ContractClient<C = C>,
    JS: JobService,
    NC: NetworkClient,
{
    pub fn new(
        contract_client: CC,
        contract_address: <C as Config>::AccountId,
        job_service: JS,
        network_client: NC,
    ) -> Self {
        Self {
            contract_client,
            contract_address,
            job_service,
            network_client,
        }
    }

    pub async fn start(
        &self,
        handle: JoinHandle<Result<(), <NC as NetworkClient>::Err>>,
    ) -> Result<(), SubmitJobControllerError> {
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

    async fn run(&self) -> Result<(), SubmitJobControllerError> {
        let job_request = self.job_service.build_job_request().await?;
        self.submit_job(&job_request).await?;
        self.wait_for_job_acceptance(&job_request).await?;
        Ok(())
    }

    async fn submit_job(&self, job_request: &JobRequest) -> Result<(), SubmitJobControllerError> {
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
        _job_request: &JobRequest,
    ) -> Result<(), SubmitJobControllerError> {
        let gossip_stream = self.network_client.gossip_msg_stream().await;
        tokio::pin!(gossip_stream);
        while let Some(_msg) = gossip_stream.next().await {
            info!("Gossip Message received");
        }
        Ok(())
    }
}

#[derive(Debug, thiserror::Error)]
pub enum SubmitJobControllerError {
    #[error("{source}")]
    ContractClient {
        #[from]
        source: ContractClientError,
    },

    #[error("{source}")]
    JobService {
        #[from]
        source: JobServiceError,
    },
}
