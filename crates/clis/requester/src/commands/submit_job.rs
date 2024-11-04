use crate::{config::Config, error::Error};
use catalog::catalog::{JobRequest, JobRequestSubmitted};
use clap::Parser;
use codec::Encode;
use ink_env::DefaultEnvironment;
use std::{fs::File, io::Read, path::Path, str::FromStr};
use subxt::{utils::AccountId32, SubstrateConfig};
use subxt_signer::sr25519::Keypair;
use tokio::{select, signal, task::JoinHandle};
use tracing::{error, info, instrument};
use utils::{
    client::Client,
    p2p::{Error as P2pError, NodeBuilder, NodeClient},
};
use wasmtime::{Engine, Module, ValType};

#[derive(Debug, Parser)]
pub struct SubmitJobCmd {
    #[arg(long)]
    pub address: String,

    #[arg(long)]
    pub path: String,

    #[arg(long)]
    pub func_name: String,

    /// A comma seperated list of paramameters to pass to your function
    #[arg(long)]
    pub params: Option<String>,
}

struct Requester {
    _node_client: NodeClient,
    config: Config,
    contract_address: AccountId32,
    path: String,
    func_name: String,
    params: Option<String>,
}

impl SubmitJobCmd {
    #[instrument(skip_all)]
    pub async fn handle(&self, config: Config) -> Result<(), Error> {
        let (node_handle, node_client) = self.join_network().await?;

        let mut requester = Requester::new(
            config,
            self.address.clone(),
            node_client,
            self.path.clone(),
            self.func_name.clone(),
            self.params.clone(),
        )?;

        requester.start(node_handle).await?;

        Ok(())
    }

    async fn join_network(&self) -> Result<(JoinHandle<Result<(), P2pError>>, NodeClient), Error> {
        let node = NodeBuilder::build()?;
        let (node_handle, node_client) = node.start()?;
        node_client.subscribe(&self.address).await?;

        Ok((node_handle, node_client))
    }
}

impl Requester {
    fn new(
        config: Config,
        address: String,
        node_client: NodeClient,
        path: String,
        func_name: String,
        params: Option<String>,
    ) -> Result<Self, Error> {
        let contract_address =
            AccountId32::from_str(&address).map_err(|err| Error::Other(err.to_string()))?;

        Ok(Self {
            _node_client: node_client,
            config,
            contract_address,
            path,
            func_name,
            params,
        })
    }

    async fn start(&mut self, node_handle: JoinHandle<Result<(), P2pError>>) -> Result<(), Error> {
        select! {
            _ = node_handle => {},
            result = self.submit_job() => {
                match result {
                    Err(err) => error!("Encountered Error: {}", err),
                    Ok(()) => info!("Successfully submitted Job")
                };
            },
            _ = signal::ctrl_c() => {
                info!("Shutting down...")
            }
        };

        Ok(())
    }

    async fn submit_job(&mut self) -> Result<(), Error> {
        let job_request = self.build_job_request()?;
        self.submit_job_request_extrinsic(&job_request).await?;
        info!("Job Request Submitted!");

        // self.wait_for_job_acceptance(&job_request).await;

        tokio::time::sleep(tokio::time::Duration::from_secs(10000)).await;

        Ok(())
    }

    async fn submit_job_request_extrinsic(&self, job_request: &JobRequest) -> Result<(), Error> {
        let client: Client<SubstrateConfig, DefaultEnvironment, Keypair> =
            Client::new(&self.config.artifact_file_path, &self.config.signer).await?;

        client
            .write::<JobRequestSubmitted, JobRequest>(
                self.contract_address.0,
                "submit_job_request",
                job_request,
            )
            .await?;

        Ok(())
    }

    fn build_job_request(&self) -> Result<JobRequest, Error> {
        let code = self.read_file()?;
        let engine = Engine::default();
        let module = Module::from_file(&engine, &self.path)?;

        let params = if let Some(params) = &self.params {
            let p = params.split(",").collect::<Vec<&str>>();
            self.build_params(&p, &module)?
        } else {
            vec![]
        };

        let job_request = JobRequest::new(code, params, vec![]);

        Ok(job_request)
    }

    fn read_file(&self) -> Result<Vec<u8>, Error> {
        let path = Path::new(&self.path);
        let mut file = File::open(path)?;
        let mut code = Vec::new();
        file.read_to_end(&mut code)?;

        return Ok(code);
    }

    fn build_params(&self, p: &Vec<&str>, module: &Module) -> Result<Vec<Vec<u8>>, Error> {
        let extern_type = module
            .get_export(&self.func_name)
            .ok_or_else(|| Error::Other("Func Not Found".to_string()))?;
        let f = extern_type
            .func()
            .ok_or_else(|| Error::Other("Extern type func not found".to_string()))?;

        let p = f
            .params()
            .zip(p)
            .map(|(ty, param)| match ty {
                ValType::I32 => match param.parse::<i32>() {
                    Ok(val) => Ok(val.encode()),
                    Err(err) => Err(Error::ParseIntError { source: err }),
                },
                _ => Ok(vec![]),
            })
            .collect::<Result<Vec<Vec<u8>>, Error>>()?;

        Ok(p)
    }

    // async fn wait_for_job_acceptance(&mut self, job_request: &JobRequest) {
    //     let foo = self.node_client.read_gossip_messages().await;
    // }
}
