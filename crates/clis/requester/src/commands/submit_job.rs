use crate::{config::Config, controller::submit_job::SubmitJobController, error::Error};
use clap::Parser;
use ink_env::DefaultEnvironment;
use std::str::FromStr;
use subxt::{utils::AccountId32, SubstrateConfig};
use subxt_signer::sr25519::Keypair;
use tracing::instrument;
use utils::services::{contract_client::Client, job::JobHandler};

#[derive(Debug, Parser)]
pub struct SubmitJobCmd {
    #[arg(long)]
    pub address: String,

    #[arg(long)]
    pub code_path: String,

    #[arg(long)]
    pub function_name: String,

    /// A comma seperated list of paramameters to pass to your function
    #[arg(long)]
    pub parameters: Option<String>,
}

impl SubmitJobCmd {
    #[instrument(skip_all)]
    pub async fn handle(&self, config: Config) -> Result<(), Error> {
        let contract_address =
            AccountId32::from_str(&self.address).map_err(|_| Error::ParsingContractAddress)?;

        let contract_client = Client::<SubstrateConfig, DefaultEnvironment, Keypair>::new(
            &config.artifact_file_path,
            &config.signer,
        )
        .await?;

        let job_service = JobHandler::new(
            &self.code_path,
            self.parameters.clone(),
            &self.function_name,
        )
        .await?;

        let submit_job_controller =
            SubmitJobController::new(contract_client, contract_address, job_service);

        let result = submit_job_controller.start().await?;

        Ok(result)
    }

    // async fn join_network(&self) -> Result<(JoinHandle<Result<(), P2pError>>, NodeClient), Error> {
    //     let node = NodeBuilder::build()?;
    //     let (node_handle, node_client) = node.start()?;
    //     node_client.subscribe(&self.address).await?;

    //     Ok((node_handle, node_client))
    // }
}

// impl Requester {
//     fn new(
//         config: Config,
//         address: String,
//         node_client: NodeClient,
//         path: String,
//         func_name: String,
//         params: Option<String>,
//     ) -> Result<Self, Error> {
//         let contract_address =
//             AccountId32::from_str(&address).map_err(|err| Error::Other(err.to_string()))?;

//         Ok(Self {
//             node_client,
//             config,
//             contract_address,
//             path,
//             func_name,
//             params,
//         })
//     }

//     async fn start(&mut self, node_handle: JoinHandle<Result<(), P2pError>>) -> Result<(), Error> {
//         select! {
//             _ = node_handle => {},
//             result = self.submit_job() => {
//                 match result {
//                     Err(err) => error!("Encountered Error: {}", err),
//                     Ok(()) => info!("Successfully submitted Job")
//                 };
//             },
//             _ = signal::ctrl_c() => {
//                 info!("Shutting down...")
//             }
//         };

//         Ok(())
//     }

//     async fn submit_job(&mut self) -> Result<(), Error> {
//         let job_request = self.build_job_request()?;
//         self.submit_job_request_extrinsic(&job_request).await?;
//         info!("Job Request Submitted!");

//         self.wait_for_job_acceptance(&job_request).await?;

//         tokio::time::sleep(tokio::time::Duration::from_secs(10000)).await;

//         Ok(())
//     }

//     async fn submit_job_request_extrinsic(&self, job_request: &JobRequest) -> Result<(), Error> {
//         let client: Client<SubstrateConfig, DefaultEnvironment, Keypair> =
//             Client::new(&self.config.artifact_file_path, &self.config.signer).await?;

//         client
//             .write::<JobRequestSubmitted, JobRequest>(
//                 self.contract_address.0,
//                 "submit_job_request",
//                 job_request,
//             )
//             .await?;

//         Ok(())
//     }

//     fn build_job_request(&self) -> Result<JobRequest, Error> {
//         let code = self.read_file()?;
//         let engine = Engine::default();
//         let module = Module::from_file(&engine, &self.path)?;

//         let params = if let Some(params) = &self.params {
//             let p = params.split(',').collect::<Vec<&str>>();
//             self.build_params(&p, &module)?
//         } else {
//             vec![]
//         };

//         let job_request = JobRequest::new(code, params, vec![]);

//         Ok(job_request)
//     }

// fn read_file(&self) -> Result<Vec<u8>, Error> {
//     let path = std::path::Path::new(&self.path);
//     let mut file = std::fs::File::open(path)?;
//     let mut code = Vec::new();
//     file.read_to_end(&mut code)?;

//     Ok(code)
// }

//     fn build_params(&self, p: &Vec<&str>, module: &Module) -> Result<Vec<Vec<u8>>, Error> {
//         let extern_type = module
//             .get_export(&self.func_name)
//             .ok_or_else(|| Error::Other("Func Not Found".to_string()))?;
//         let f = extern_type
//             .func()
//             .ok_or_else(|| Error::Other("Extern type func not found".to_string()))?;

//         let p = f
//             .params()
//             .zip(p)
//             .map(|(ty, param)| match ty {
//                 ValType::I32 => match param.parse::<i32>() {
//                     Ok(val) => Ok(val.encode()),
//                     Err(err) => Err(Error::ParseInt { source: err }),
//                 },
//                 _ => Ok(vec![]),
//             })
//             .collect::<Result<Vec<Vec<u8>>, Error>>()?;

//         Ok(p)
//     }

//     async fn wait_for_job_acceptance(&mut self, _job_request: &JobRequest) -> Result<(), Error> {
//         while let Some(msg) = self.node_client.recv_gossip_msg().await {
//             info!("Gossip Message received");
//             self.node_client
//                 .send_request(msg.peer_id(), msg.message())
//                 .await?;
//             info!("Job sent");
//         }

//         Ok(())
//     }
// }
