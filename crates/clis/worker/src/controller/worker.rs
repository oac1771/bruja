use catalog::catalog::JobRequestSubmitted;
use clis::Gossip;
use codec::Encode;
use std::fmt::Display;
use subxt::{ext::futures::StreamExt, Config};
use tokio::{pin, select, signal::ctrl_c, task::JoinHandle};
use tracing::{error, info};
use utils::{
    chain::contracts::events::ContractEmitted,
    services::{
        contract_client::{ContractClient, ContractClientError},
        p2p::{NetworkClient, NetworkClientError},
    },
};

pub struct WorkerController<C: Config, CC, NC> {
    contract_client: CC,
    contract_address: <C as Config>::AccountId,
    network_client: NC,
}

impl<C, CC, NC> WorkerController<C, CC, NC>
where
    C: Config,
    <C as Config>::AccountId: Display,
    CC: ContractClient<C = C>,
    NC: NetworkClient,
{
    pub fn new(
        contract_client: CC,
        contract_address: <C as Config>::AccountId,
        network_client: NC,
    ) -> Self {
        Self {
            contract_address,
            contract_client,
            network_client,
        }
    }

    pub async fn start(&self, node_handle: JoinHandle<Result<(), NetworkClientError>>) {
        info!("Starting Worker Controller");

        select! {
            _ = node_handle => {},
            result = self.listen() => {
                match result {
                    Err(err) => error!("Error: {}", err),
                    Ok(()) => info!("Shutting down...")
                };
            }
            _ = ctrl_c() => {
                info!("Shutting down...")
            }
        };
    }

    async fn listen(&self) -> Result<(), WorkerControllerError> {
        let ev_stream = self
            .contract_client
            .contract_event_sub(self.contract_address.clone())
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
        let res = if let Ok(job_request) = self
            .contract_client
            .decode_event::<JobRequestSubmitted>(&ev)
        {
            self.accept_job(job_request).await
        } else {
            Err(WorkerControllerError::DecodeContractEvent { data: ev.data })
        };

        if let Err(e) = res {
            error!("Error while handling event: {}", e);
        }
    }

    async fn accept_job(
        &self,
        job_request: JobRequestSubmitted,
    ) -> Result<(), WorkerControllerError> {
        let job_id = job_request.id.to_vec();
        let msg = Gossip::JobAcceptance { job_id };
        let topic = self.contract_address.to_string();

        self.network_client
            .publish_message(&topic, msg.encode())
            .await?;

        Ok(())
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
    NetworkClient {
        #[from]
        source: NetworkClientError,
    },

    #[error("Unable to decode Contract Emitted event: {data:?}")]
    DecodeContractEvent { data: Vec<u8> },
}
