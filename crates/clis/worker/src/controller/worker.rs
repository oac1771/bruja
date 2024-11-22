use catalog::catalog::{HashId, JobRequest, JobRequestSubmitted};
use clis::{Gossip, Request, Response};
use codec::Encode;
use std::fmt::Display;
use subxt::{ext::futures::StreamExt, Config};
use tokio::{
    pin, select,
    signal::ctrl_c,
    task::JoinHandle,
    time::{sleep, Duration},
};
use tracing::{error, info};
use utils::services::{
    contract_client::{ContractClient, ContractClientError, ContractEmittedT},
    job::{
        job_runner::{WasmJobRunnerService, WasmJobRunnerServiceError},
        JobT,
    },
    p2p::{NetworkClient, NetworkClientError, NetworkIdT, RequestT, ResponseT},
};

pub struct WorkerController<C: Config, CC, NC, JR> {
    contract_client: CC,
    contract_address: <C as Config>::AccountId,
    network_client: NC,
    job_runner: JR,
}

impl<C, CC, NC, JR> WorkerController<C, CC, NC, JR>
where
    C: Config,
    <C as Config>::AccountId: Display,
    CC: ContractClient<C = C>,
    NC: NetworkClient,
    JR: WasmJobRunnerService,
    WorkerControllerError: From<<NC as NetworkClient>::Err>
        + From<<CC as ContractClient>::Err>
        + From<<JR as WasmJobRunnerService>::Err>,
{
    pub fn new(
        contract_client: CC,
        contract_address: <C as Config>::AccountId,
        network_client: NC,
        job_runner: JR,
    ) -> Self {
        Self {
            contract_address,
            contract_client,
            network_client,
            job_runner,
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

    async fn handle_event(&self, ev: <CC as ContractClient>::ContractEmitted) {
        let res = if let Ok(job_request) = self
            .contract_client
            .decode_event::<JobRequestSubmitted>(ev.data_ref())
        {
            self.handle_job_request(job_request).await
        } else {
            Err(WorkerControllerError::DecodeContractEvent { data: ev.data() })
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
        let (id, job, who) = self
            .wait_for_job(job_request.id())
            .await
            .ok_or_else(|| WorkerControllerError::JobNeverSent)?;
        self.acknowledge_job_acceptance(id, job_request.id())
            .await?;

        let result = self.start_job(job).await?;
        self.send_result(result, job_request.id(), who).await?;
        self.wait_for_result_acknowledgement(job_request.id()).await;

        Ok(())
    }

    async fn accept_job_request(
        &self,
        job_request: &JobRequestSubmitted,
    ) -> Result<(), WorkerControllerError> {
        let job_id = job_request.id();
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
        let address = self.contract_address.to_string();

        while let None = self.network_client.get_gossip_nodes(&address).await?.next() {
            info!("Waiting for gossip peers");
            sleep(Duration::from_millis(500)).await;
        }

        info!("Connected to gossip peers");
        Ok(())
    }

    async fn wait_for_job(
        &self,
        id: HashId,
    ) -> Option<(
        <NC as NetworkClient>::Id,
        <JR as WasmJobRunnerService>::Job,
        <NC as NetworkClient>::NetworkId,
    )> {
        let req_stream = self.network_client.req_stream().await;
        tokio::pin!(req_stream);

        while let Some(req) = req_stream.next().await {
            if let Ok(Request::Job {
                code,
                params,
                func_name,
                who,
            }) = Request::decode(req.body_ref())
            {
                if JobRequest::hash(&code, &params) == id {
                    info!("Job received!");

                    let job: <JR as WasmJobRunnerService>::Job =
                        JobT::from_parts(code, params, func_name);
                    let id = req.id();
                    let who = <NC as NetworkClient>::NetworkId::from_bytes(&who);
                    return Some((id, job, who));
                }
            } else {
                error!("Unable to decode request: {:?}", req.body_ref());
            }
        }

        None
    }

    async fn acknowledge_job_acceptance(
        &self,
        id: <NC as NetworkClient>::Id,
        job_id: HashId,
    ) -> Result<(), WorkerControllerError> {
        let request = Response::AcknowledgeJob { job_id };
        self.network_client
            .send_response(id, request.encode())
            .await?;
        info!("Job acknowledgement sent");

        Ok(())
    }

    async fn start_job(
        &self,
        job: <JR as WasmJobRunnerService>::Job,
    ) -> Result<Vec<Vec<u8>>, WorkerControllerError> {
        let result = self.job_runner.start_job(job)?;

        Ok(result)
    }

    async fn send_result(
        &self,
        result: Vec<Vec<u8>>,
        job_id: HashId,
        who: <NC as NetworkClient>::NetworkId,
    ) -> Result<(), WorkerControllerError> {
        let req = Request::Result { result, job_id };

        self.network_client.send_request(who, req.encode()).await?;
        info!("Results sent");

        Ok(())
    }

    async fn wait_for_result_acknowledgement(&self, id: HashId) {
        let resp_stream = self.network_client.resp_stream().await;
        tokio::pin!(resp_stream);

        while let Some(resp) = resp_stream.next().await {
            if let Ok(Response::AcknowledgeResult { job_id }) = Response::decode(&resp.body_ref()) {
                if id == job_id {
                    info!("Result acknowledged by requester")
                }
            }
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
    NetworkClient {
        #[from]
        source: NetworkClientError,
    },

    #[error("{source}")]
    JobRunner {
        #[from]
        source: WasmJobRunnerServiceError,
    },

    #[error("Unable to decode Contract Emitted event: {data:?}")]
    DecodeContractEvent { data: Vec<u8> },

    #[error("")]
    JobNeverSent,
}
