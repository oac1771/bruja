use clap::Parser;
use ink::env::{
        hash::{HashOutput, Keccak256},
        DefaultEnvironment,
    }
;
use std::str::FromStr;
use subxt::{
    utils::AccountId32,
    SubstrateConfig,
};
use subxt_signer::{sr25519::Keypair, SecretUri};
use utils::client::Client;

#[derive(Debug, Parser)]
pub struct Foo {
    #[arg(long)]
    address: String,
}

type Keccak256HashOutput = <Keccak256 as HashOutput>::Type;

impl Foo {
    pub async fn handle(&self) {
        let contract_address = AccountId32::from_str(&self.address).unwrap();

        let signer = Keypair::from_uri(&SecretUri::from_str("//Alice").unwrap()).unwrap();
        let artifact_file = "./target/ink/catalog/catalog.contract";
        let contract_client: Client<SubstrateConfig, DefaultEnvironment, Keypair> =
            Client::new(&artifact_file, &signer);

        let jobs = contract_client.get_storage::<Vec<Keccak256HashOutput>>(contract_address).await.unwrap();

        println!("{:?}", jobs);
    }
}
