use crate::{config::Config, error::Error};
use catalog::catalog::WorkerRegistered;
use clap::Parser;
use ink_env::DefaultEnvironment;
use std::str::FromStr;
use subxt::{utils::AccountId32, SubstrateConfig};
use subxt_signer::sr25519::Keypair;
use utils::client::Client;

#[derive(Debug, Parser)]
pub struct RegisterCmd {
    #[arg(long)]
    val: String,

    #[arg(long)]
    address: String,
}

impl RegisterCmd {
    pub async fn handle(&self, config: Config) -> Result<(), Error> {
        let contract_address =
            AccountId32::from_str(&self.address).map_err(|err| Error::Other(err.to_string()))?;

        let client: Client<SubstrateConfig, DefaultEnvironment, Keypair> =
            Client::new(&config.artifact_file_path, &config.signer).await?;
        let args = self.args();

        let result = match client
            .mutable_call::<WorkerRegistered>("register_worker", contract_address, args)
            .await
        {
            Ok(event) => {
                if event.who.as_ref() == config.signer.public_key().0 {
                    println!("Successfully registered worker!");
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
        };

        result
    }

    fn args(&self) -> Vec<&str> {
        vec![&self.val]
    }
}
