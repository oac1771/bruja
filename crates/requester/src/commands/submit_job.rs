use crate::{config::Config, error::Error};
use catalog::catalog::{JobRequest, JobRequestSubmitted};
use clap::Parser;
use codec::Encode;
use ink_env::DefaultEnvironment;
use std::{fs::File, io::Read, path::Path, str::FromStr};
use subxt::{utils::AccountId32, SubstrateConfig};
use subxt_signer::sr25519::Keypair;
use tokio::{select, signal};
use tracing::{info, instrument};
use utils::{client::Client, p2p::NodeBuilder};
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

impl SubmitJobCmd {
    #[instrument(skip_all)]
    pub async fn handle(&self, config: Config) -> Result<(), Error> {
        let contract_address = AccountId32::from_str(&self.address).map_err(|err| {
            Error::Other(format!(
                "Failed to parse provided contract address: {}",
                err.to_string()
            ))
        })?;

        let client: Client<SubstrateConfig, DefaultEnvironment, Keypair> =
            Client::new(&config.artifact_file_path, &config.signer).await?;

        let job_request = self.build_job_request()?;

        client
            .write::<JobRequestSubmitted, JobRequest>(
                contract_address,
                "submit_job_request",
                job_request,
            )
            .await?;

        info!("Job Request Submitted!");

        let node = NodeBuilder::build()?;
        let (handle, node_client) = node.start()?;
        node_client.subscribe(&self.address).await?;

        select! {
            _ = handle => {},
            _ = signal::ctrl_c() => {
                info!("Shutting down...")
            }
        };

        Ok(())
    }

    fn build_job_request(&self) -> Result<JobRequest, Error> {
        let code = self.read_file()?;
        let engine = Engine::default();
        let module = Module::from_file(&engine, &self.path)?;

        let _resources = module.resources_required();

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

        if path.exists() {
            let mut file = File::open(path)?;
            let mut code = Vec::new();
            file.read_to_end(&mut code)?;

            return Ok(code);
        } else {
            return Err(Error::Other(format!(
                "Path: {:?} does not exist",
                self.path
            )));
        }
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
}
