use subxt::{ext::futures::StreamExt, Config};
use tokio::{select, signal::ctrl_c};
use tracing::{error, info};
use utils::{
    chain::contracts::events::ContractEmitted,
    contract_client::{ContractClient, ContractClientError},
};

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
            _ = self.listen() => {}
            _ = ctrl_c() => {
                info!("Shutting down...")
            }
        };
    }

    async fn listen(&self) {
        let ev_stream = self
            .contract_client
            .contract_event_sub(&self.contract_address)
            .await
            .unwrap();
        tokio::pin!(ev_stream);

        while let Some(stream_result) = ev_stream.next().await {
            match stream_result {
                Ok(ev) => self.handle_event(ev).await,
                Err(err) => error!("Error retreiving event: {}", err),
            }
        }
    }

    async fn handle_event(&self, ev: ContractEmitted) {}
}

#[derive(Debug, thiserror::Error)]
pub enum WorkerControllerError {
    #[error("{source}")]
    ContractClient {
        #[from]
        source: ContractClientError,
    },
}
