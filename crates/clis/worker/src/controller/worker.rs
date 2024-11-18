use catalog::catalog::{HashId, JobRequest, JobRequestSubmitted};
use clis::{Gossip, Job, Request};
use codec::Encode;
use libp2p::request_response::InboundRequestId;
use std::fmt::Display;
use subxt::{ext::futures::StreamExt, Config};
use tokio::{
    pin, select,
    signal::ctrl_c,
    task::JoinHandle,
    time::{sleep, Duration},
};
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
    WorkerControllerError: From<<NC as NetworkClient>::Err> + From<<CC as ContractClient>::Err>,
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

    pub async fn start(&self, node_handle: JoinHandle<Result<(), <NC as NetworkClient>::Err>>) {
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
            self.handle_job_request(job_request).await
        } else {
            Err(WorkerControllerError::DecodeContractEvent { data: ev.data })
        };

        if let Err(e) = res {
            error!("Error while handling event: {}", e);
        }
    }

    async fn handle_job_request(
        &self,
        job_request: JobRequestSubmitted,
    ) -> Result<(), WorkerControllerError> {
        self.accept_job_request(&job_request).await?;
        let (_id, _job) = self
            .wait_for_job(job_request.id())
            .await
            .ok_or_else(|| WorkerControllerError::JobNeverSent)?;

        Ok(())
    }

    async fn accept_job_request(
        &self,
        job_request: &JobRequestSubmitted,
    ) -> Result<(), WorkerControllerError> {
        let job_id = job_request.id.to_vec();
        let msg = Gossip::JobAcceptance { job_id };
        let topic = self.contract_address.to_string();

        self.wait_for_gossip_peers().await?;
        self.network_client
            .publish_message(&topic, msg.encode())
            .await?;

        info!("Published job acceptance");

        Ok(())
    }

    async fn wait_for_gossip_peers(&self) -> Result<(), WorkerControllerError> {
        let mut gossip_nodes = Vec::new();
        let address = self.contract_address.to_string();

        while gossip_nodes.is_empty() {
            info!("Waiting for gossip peers");
            gossip_nodes = self.network_client.get_gossip_nodes(&address).await?;
            sleep(Duration::from_millis(500)).await;
        }
        info!("Connected to gossip peers");
        Ok(())
    }

    async fn wait_for_job(&self, id: HashId) -> Option<(InboundRequestId, Job)> {
        let req_stream = self.network_client.req_stream().await;
        tokio::pin!(req_stream);

        while let Some((req_id, req)) = req_stream.next().await {
            if let Ok(Request::Job(job)) = Request::decode(&req.0) {
                if JobRequest::new(job.code(), job.params()).id() == id {
                    info!("Job received!");
                    return Some((req_id, job));
                }
            } else {
                error!("Unable to decode request: {:?}", req.0);
            }
        }

        None
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

    #[error("")]
    JobNeverSent,
}
