use crate::{config::Config, error::Error};
use catalog::catalog::{Job, JobSubmitted};
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
    address: String,

    #[arg(long)]
    path: String,

    #[arg(long)]
    func_name: String,

    /// A comma seperated list of paramameters to pass to your function
    #[arg(long)]
    params: Option<String>,
}

impl SubmitJobCmd {
    #[instrument(skip_all)]
    pub async fn handle(&self, config: &Config) -> Result<(), Error> {
        let contract_address = AccountId32::from_str(&self.address).map_err(|err| {
            Error::Other(format!(
                "Parsing provided contract address {}",
                err.to_string()
            ))
        })?;

        let client: Client<SubstrateConfig, DefaultEnvironment, Keypair> =
            Client::new(&config.artifact_file_path, &config.signer).await?;

        let code = self.read_file()?;

        let params: Vec<Vec<u8>> = if let Some(params) = &self.params {
            let p = params.split(",").collect::<Vec<&str>>();
            let engine = Engine::default();
            let module = Module::from_file(&engine, &self.path)?;
            self.build_params(&p, &module)?
        } else {
            vec![]
        };

        let job = Job::new(code, params);

        match client
            .write::<JobSubmitted, Job>(contract_address, "submit_job", job)
            .await
        {
            Ok(_) => {
                info!("Job Submitted!");
            }
            Err(err) => {
                info!("Job Submission unsuccessful {:?}", err);
            }
        }

        let node = NodeBuilder::build()?;
        let (handle, node_client) = node.run().await?;
        node_client.subscribe("foo").await?;

        select! {
            _ = handle => {},
            _ = signal::ctrl_c() => {
                info!("Shutting down...")
            }
        };

        Ok(())
    }

    fn read_file(&self) -> Result<Vec<u8>, Error> {
        let path = Path::new(&self.path);

        if path.exists() {
            let mut file = File::open(path)?;
            let mut code: Vec<u8> = Vec::new();
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
            .ok_or_else(|| Error::Other("".to_string()))?;

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
