use crate::{config::Config, error::Error};
use catalog::catalog::WorkerSet;
use clap::Parser;
use codec::Decode;
use ink_env::DefaultEnvironment;
use subxt::SubstrateConfig;
use subxt_signer::sr25519::Keypair;
use utils::{chain::contracts::events::ContractEmitted, client::Client};

#[derive(Debug, Parser)]
pub struct RegisterCmd {
    #[arg(long)]
    val: String,
}

impl RegisterCmd {
    pub async fn handle(&self, config: Config) -> Result<(), Error> {
        let client: Client<SubstrateConfig, DefaultEnvironment, Keypair> =
            Client::new(&config.artifact_file_path, &config.signer);
        let args = self.args();

        let events = client
            .mutable_call("set_worker", config.contract_address, args)
            .await?;

        match events.find_first::<ContractEmitted>()? {
            Some(event) => {
                let worker_set_event = <WorkerSet>::decode(&mut event.data.as_slice())?;

                if worker_set_event.who.as_ref() == config.signer.public_key().0 {
                    Ok(())
                } else {
                    Err(Error::Other(String::from(
                        "Worker Set Event did not contain expected value",
                    )))
                }
            }
            None => Err(Error::Other(String::from("contract did not emit event"))),
        }
    }

    fn args(&self) -> Vec<&str> {
        vec![&self.val]
    }
}
