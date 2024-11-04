use crate::{config::Config, error::Error};
use catalog::catalog::WorkerRegistered;
use clap::Parser;
use ink_env::DefaultEnvironment;
use std::str::FromStr;
use subxt::{utils::AccountId32, SubstrateConfig};
use subxt_signer::sr25519::Keypair;
use tracing::{info, instrument};
use utils::client::Client;

#[derive(Debug, Parser)]
pub struct RegisterCmd {
    #[arg(long)]
    pub val: u32,

    #[arg(long)]
    pub address: String,
}

impl RegisterCmd {
    #[instrument(skip_all)]
    pub async fn handle(&self, config: Config) -> Result<(), Error> {
        let contract_address =
            AccountId32::from_str(&self.address).map_err(|err| Error::Other(err.to_string()))?;

        let client: Client<SubstrateConfig, DefaultEnvironment, Keypair> =
            Client::new(&config.artifact_file_path, &config.signer).await?;

        let result = match client
            .write::<WorkerRegistered, u32>(contract_address, "register_worker", &self.val)
            .await
        {
            Ok(event) => {
                if event.who.as_ref() == config.signer.public_key().0 {
                    info!("Successfully registered worker!");
                    Ok(())
                } else {
                    Err(Error::Other(String::from(
                        "WorkerRegistered Event did not contain expected value",
                    )))
                }
            }
            Err(err) => Err(Error::Other(format!(
                "Error during registration: {:?}",
                err
            ))),
        }?;

        return Ok(result);
    }
}
