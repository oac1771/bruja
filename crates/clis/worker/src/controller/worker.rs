use catalog::catalog::JobRequestSubmitted;
use subxt::{ext::futures::StreamExt, Config};
use tokio::{pin, select, signal::ctrl_c};
use tracing::{error, info};
use utils::{
    chain::contracts::events::ContractEmitted,
    services::contract_client::{ContractClient, ContractClientError},
};

// might need to not pass contract address except to start maybe?
pub struct WorkerController<C: Config, CC> {
    contract_client: CC,
    contract_address: <C as Config>::AccountId,
}

impl<C, CC> WorkerController<C, CC>
where
    C: Config,
    CC: ContractClient<C = C>,
{
    pub fn new(contract_client: CC, contract_address: <C as Config>::AccountId) -> Self {
        Self {
            contract_address,
            contract_client,
        }
    }

    pub async fn start(&self) {
        info!("Starting Controller");

        select! {
            result = self.listen() => {
                if let Err(e) = result {
                    error!("Error: {}", e);
                }
            }
            _ = ctrl_c() => {
                info!("Shutting down...")
            }
        };
    }

    async fn listen(&self) -> Result<(), WorkerControllerError> {
        let ev_stream = self
            .contract_client
            .contract_event_sub(&self.contract_address)
            .await?;
        pin!(ev_stream);

        while let Some(stream_result) = ev_stream.next().await {
            match stream_result {
                Ok(ev) => self.handle_event(ev).await,
                Err(err) => error!("Error retreiving event: {}", err),
            }
        }

        Ok(())
    }

    async fn handle_event(&self, ev: ContractEmitted) {
        if let Ok(_ev) = self
            .contract_client
            .decode_event::<JobRequestSubmitted>(&ev)
        {
            //
        } else {
            error!(
                "Unable to decode Contract Emitted event: {:?}",
                ev.data.to_vec()
            );
        }
    }
}

#[derive(Debug, thiserror::Error)]
pub enum WorkerControllerError {
    #[error("{source}")]
    ContractClient {
        #[from]
        source: ContractClientError,
    },
}
