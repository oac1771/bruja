use clap::Parser;
use ink::env::DefaultEnvironment;
use std::str::FromStr;
use subxt::SubstrateConfig;
use subxt_signer::{sr25519::Keypair, SecretUri};
use utils::client::Client;

#[derive(Debug, Parser)]
pub struct Foo {
    #[arg(long)]
    address: String,
}

impl Foo {
    pub async fn handle(&self) {
        let signer = Keypair::from_uri(&SecretUri::from_str("//Alice").unwrap()).unwrap();
        let artifact_file = "./target/ink/catalog/catalog.contract";
        let contract_client: Client<SubstrateConfig, DefaultEnvironment, Keypair> =
            Client::new(&artifact_file, &signer);

        contract_client.instantiate_v2("new").await;

        // let jobs = contract_client.get_storage::<Vec<Keccak256HashOutput>>(contract_address).await.unwrap();

        // println!("{:?}", jobs);
    }
}
