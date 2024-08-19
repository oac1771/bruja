use crate::{config::Config, error::Error};
use clap::Parser;
use ink_env::DefaultEnvironment;
use subxt::SubstrateConfig;
use subxt_signer::sr25519::Keypair;
use utils::client::Client;

#[derive(Debug, Parser)]
pub struct RegisterCmd;

impl RegisterCmd {
    pub async fn handle(&self, config: Config) -> Result<(), Error> {
        let client: Client<SubstrateConfig, DefaultEnvironment, Keypair> =
            Client::new(&config.artifact_file_path, config.signer);

        let _events = client
            .mutable_call("set_worker", config.contract_address, vec!["10"])
            .await?;

        Ok(())
    }
}
