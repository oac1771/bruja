use catalog::catalog::{JobRequest, JobRequestSubmitted};
use subxt::Config;
use tokio::{select, signal::ctrl_c};
use tracing::{error, info};
use utils::services::{
    contract_client::{ContractClient, ContractClientError},
    job::{JobService, JobServiceError},
};

pub struct RequesterController<C: Config, CC, JS> {
    contract_client: CC,
    contract_address: <C as Config>::AccountId,
    job_service: JS,
}

impl<C, CC, JS> RequesterController<C, CC, JS>
where
    C: Config,
    CC: ContractClient<C = C>,
    JS: JobService,
{
    pub fn new(
        contract_client: CC,
        contract_address: <C as Config>::AccountId,
        job_service: JS,
    ) -> Self {
        Self {
            contract_client,
            contract_address,
            job_service,
        }
    }

    pub async fn start(&self) -> Result<(), SubmitJobControllerError> {
        select! {
            result = self.submit_job() => {
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
