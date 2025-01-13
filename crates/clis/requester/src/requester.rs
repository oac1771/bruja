use catalog::catalog::{HashId, JobRequest, JobRequestSubmitted, PaidWorker};
use clis::{Gossip, Request, Response};
use codec::Encode;
use ink_env::Environment;
use subxt::{ext::futures::StreamExt, Config};
use tracing::{error, info};
use utils::services::{
    contract_client::{ContractClient, ContractClientError},
    job::{
        job_handler::{JobHandlerService, JobHandlerServiceError},
        JobT, RawResultsT,
    },
    p2p::{GossipMessageT, NetworkClient, NetworkClientError, NetworkIdT, RequestT, ResponseT},
};

pub struct RequesterController<C: Config, E: Environment, CC, JH, NC> {
    contract_address: <C as Config>::AccountId,
    value: <E as Environment>::Balance,
    contract_client: CC,
    job_handler_service: JH,
    network_client: NC,
}

impl<C, E, CC, JH, NC> RequesterController<C, E, CC, JH, NC>
where
    C: Config,
    E: Environment,
    E::Balance: From<u128>,
    CC: ContractClient<C = C, E = E>,
    JH: JobHandlerService,
    NC: NetworkClient,
    RequesterControllerError: From<<NC as NetworkClient>::Err>
        + From<<CC as ContractClient>::Err>
        + From<<JH as JobHandlerService>::Err>,
{
    pub fn new(
        contract_address: <C as Config>::AccountId,
        value: <E as Environment>::Balance,
        contract_client: CC,
        job_handler_service: JH,
        network_client: NC,
    ) -> Self {
        Self {
            contract_address,
            value,
            contract_client,
            job_handler_service,
            network_client,
        }
    }

    pub async fn run(&self) -> Result<(), RequesterControllerError> {
        let job = self.job_handler_service.build_job().await?;
        let job_request = JobRequest::new(job.code_ref(), job.params_ref());
        let job_id = job_request.id();

        self.submit_job(&job_request).await?;
        let msg = self
            .wait_for_job_acceptance(&job_request)
            .await
            .ok_or_else(|| RequesterControllerError::JobNeverAccepted)?;

        self.send_job(msg.network_id(), job).await?;
        self.wait_for_job_acknowledgement(job_id).await;

        let (req_id, results, worker) = self
            .wait_for_job_results(job_id)
            .await
            .ok_or_else(|| RequesterControllerError::ResultsNeverReceived)?;
        self.send_result_acknowledgement(req_id, job_id).await?;
        self.pay_worker(worker, job_id).await?;

        self.display_results(results).await?;

        Ok(())
    }

    async fn submit_job(&self, job_request: &JobRequest) -> Result<(), RequesterControllerError> {
        self.contract_client
            .write::<JobRequestSubmitted, JobRequest>(
                self.contract_address.clone(),
                "submit_job_request",
                job_request,
                self.value,
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

        info!("Job sent to peer: {}", network_id);

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
    ) -> Option<(
        <NC as NetworkClient>::Id,
        <JH as JobHandlerService>::RawResults,
        [u8; 32],
    )> {
        let req_stream = self.network_client.req_stream().await;
        tokio::pin!(req_stream);
        while let Some(req) = req_stream.next().await {
            if let Ok(Request::Result {
                result,
                job_id,
                worker,
            }) = Request::decode(req.body_ref())
            {
                if job_id == id {
                    info!("Received results");
                    let result = <JH as JobHandlerService>::RawResults::from_vec(result);
                    return Some((req.id(), result, worker));
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

    async fn display_results(
        &self,
        results: <JH as JobHandlerService>::RawResults,
    ) -> Result<(), RequesterControllerError> {
        let parsed_results = self.job_handler_service.unpack_results(results).await?;

        let results = format!("Results: {}", parsed_results);
        let stars = vec!["*"; results.len() + 4].join("");

        info!("\n{}\n  {}  \n{}", stars.clone(), results, stars.clone());

        Ok(())
    }

    async fn pay_worker(
        &self,
        worker: [u8; 32],
        job_id: HashId,
    ) -> Result<(), RequesterControllerError> {
        let args = (worker, job_id);
        let value: <E as Environment>::Balance = 0_u128.into();

        self.contract_client
            .write::<PaidWorker, ([u8; 32], HashId)>(
                self.contract_address.clone(),
                "pay_worker",
                &args,
                value,
            )
            .await?;

        info!("Paid Worker!");

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
    JobHandlerService {
        #[from]
        source: JobHandlerServiceError,
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
