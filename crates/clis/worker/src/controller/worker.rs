use subxt::{
    blocks::{Block, ExtrinsicDetails},
    error::Error as SubxtError,
    ext::futures::StreamExt,
    Config, OnlineClient,
};
use tokio::{select, signal::ctrl_c};
use tracing::{error, info};
use utils::contract_client::{ContractClient, ContractClientError};

pub struct WorkerController<C: Config, CC> {
    contract_client: CC,
    contract_address: <C as Config>::AccountId,
}

// refactor this so that contract client simply returns stream of contract events
// should not care about block stuff and down

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
        let ev_stream = self.contract_client.foo().await.unwrap();
        tokio::pin!(ev_stream);
        while let Some(ev) = ev_stream.next().await {
            info!("event: {:?}", ev);
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

    #[error("{source}")]
    Subxt {
        #[from]
        source: SubxtError,
    },
}
