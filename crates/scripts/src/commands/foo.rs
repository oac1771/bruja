use catalog::catalog::JobSubmitted;
use clap::Parser;
use ink::env::DefaultEnvironment;
use std::str::FromStr;
use subxt::SubstrateConfig;
use subxt_signer::{sr25519::Keypair, SecretUri};
use utils::client::{Client, Args};

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

        let address = contract_client.instantiate_v2("new").await;
        println!("{}", address);
        let job_submitted = contract_client
            .mutable_call_v2::<JobSubmitted>(address, "submit_job", vec![Args::Vec(vec![1,2,3,4])])
            .await;

        // let jobs = contract_client.get_storage::<Vec<Keccak256HashOutput>>(contract_address).await.unwrap();

        println!("{:?}", job_submitted);
    }
}
