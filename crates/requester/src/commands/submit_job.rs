use crate::{config::Config, error::Error};
use catalog::catalog::JobSubmitted;
use clap::Parser;
use ink_env::DefaultEnvironment;
use std::str::FromStr;
use subxt::{utils::AccountId32, SubstrateConfig};
use subxt_signer::sr25519::Keypair;
use utils::client::Client;

#[derive(Debug, Parser)]
pub struct SubmitJobCmd {
    #[arg(long)]
    address: String,
}

impl SubmitJobCmd {
    pub async fn handle(&self, config: &Config) -> Result<(), Error> {
        let contract_address =
            AccountId32::from_str(&self.address).map_err(|err| Error::Other(err.to_string()))?;

        let client: Client<SubstrateConfig, DefaultEnvironment, Keypair> =
            Client::new(&config.artifact_file_path, &config.signer);

        let args: Vec<&str> = vec!["[1,2,3,5]"];

        match client
            .mutable_call::<JobSubmitted>("submit_job", contract_address, args)
            .await
        {
            Ok(_) => {
                println!("Job Submitted!");
            }
            Err(err) => {
                println!("Job Submission unsuccessful {:?}", err);
            }
        }

        Ok(())
    }
}