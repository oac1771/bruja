use catalog::catalog::{JobRequest, JobRequestSubmitted};
use subxt::{ext::futures::StreamExt, Config};
use tokio::{select, signal::ctrl_c, task::JoinHandle};
use tracing::{error, info};
use utils::services::{
    contract_client::{ContractClient, ContractClientError},
    job::{JobService, JobServiceError},
    p2p::{NetworkClient, NetworkClientError},
};

pub struct RequesterController<C: Config, CC, JS, NS> {
    contract_client: CC,
    contract_address: <C as Config>::AccountId,
    job_service: JS,
    network_client: NS,
}

impl<C, CC, JS, NS> RequesterController<C, CC, JS, NS>
where
    C: Config,
    CC: ContractClient<C = C>,
    JS: JobService,
    NS: NetworkClient,
{
    pub fn new(
        contract_client: CC,
        contract_address: <C as Config>::AccountId,
        job_service: JS,
        network_client: NS,
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
        handle: JoinHandle<Result<(), NetworkClientError>>,
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
        self.submit_job().await?;
        self.wait_for_job_acceptance().await?;
        Ok(())
    }

    async fn submit_job(&self) -> Result<(), SubmitJobControllerError> {
        let job_request = self.job_service.build_job_request().await?;
        self.contract_client
            .write::<JobRequestSubmitted, JobRequest>(
                self.contract_address.clone(),
                "submit_job_request",
                &job_request,
            )
            .await?;

        info!("Job Request Submitted!");
        Ok(())
    }

    async fn wait_for_job_acceptance(&self) -> Result<(), SubmitJobControllerError> {
        let gossip_stream = self.network_client.gossip_msg_stream().await;
        tokio::pin!(gossip_stream);
        while let Some(_) = gossip_stream.next().await {
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
