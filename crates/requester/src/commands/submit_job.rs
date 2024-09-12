use crate::{config::Config, error::Error};
use catalog::catalog::JobSubmitted;
use clap::Parser;
use codec::Decode;
use ink_env::DefaultEnvironment;
use std::str::FromStr;
use subxt::{utils::AccountId32, SubstrateConfig};
use subxt_signer::sr25519::Keypair;
use utils::{chain::contracts::events::ContractEmitted, client::Client};

#[derive(Debug, Parser)]
pub struct SubmitJobCmd {
    #[arg(long)]
    address: String,
}

impl SubmitJobCmd {
    pub async fn handle(&self, config: &Config) -> Result<(), Error>{
        let contract_address =
            AccountId32::from_str(&self.address).map_err(|err| Error::Other(err.to_string()))?;

        let client: Client<SubstrateConfig, DefaultEnvironment, Keypair> =
            Client::new(&config.artifact_file_path, &config.signer);

        let args: Vec<&str> = vec!["[1,2,3,4]"];

        let events = client.mutable_call("submit_job", contract_address, args).await?;

        match events.find_first::<ContractEmitted>()? {
            Some(event) => {
                let job_submitted_event = <JobSubmitted>::decode(&mut event.data.as_slice())?;

                if job_submitted_event.who.as_ref() == config.signer.public_key().0 {
                    println!("Successfully submitted job!");
                    Ok(())
                } else {
                    Err(Error::Other(String::from(
                        "Worker Set Event did not contain expected value",
                    )))
                }
            }
            None => Err(Error::Other(String::from("contract did not emit event"))),
        }?;

        Ok(())
    }
}