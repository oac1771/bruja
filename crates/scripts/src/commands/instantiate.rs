use clap::Parser;
use ink::env::DefaultEnvironment;
use std::str::FromStr;
use subxt::SubstrateConfig;
use subxt_signer::{sr25519::Keypair, SecretUri};
use utils::contract_client::ContractClient;

#[derive(Debug, Parser)]
pub struct InstantiateCmd {
    #[arg(long)]
    suri: String,

    #[arg(long)]
    file: String,
}

impl InstantiateCmd {
    pub async fn handle(&self) {
        let signer = Keypair::from_uri(&SecretUri::from_str(&self.suri).unwrap()).unwrap();

        let client: ContractClient<SubstrateConfig, DefaultEnvironment, Keypair> =
            ContractClient::new(&self.file, &signer).await.unwrap();

        let contract_address = client.instantiate("new").await.unwrap();

        println!("{}", contract_address);
    }
}
