use catalog::catalog::WorkerRegistered;
use clap::Parser;
use ink::env::DefaultEnvironment;
use std::str::FromStr;
use subxt::SubstrateConfig;
use subxt_signer::{sr25519::Keypair, SecretUri};
use utils::client::{Client, Args};

#[derive(Debug, Parser)]
pub struct Foo {}

impl Foo {
    pub async fn handle(&self) {
        let signer = Keypair::from_uri(&SecretUri::from_str("//Alice").unwrap()).unwrap();
        let artifact_file = "./target/ink/catalog/catalog.contract";
        let contract_client: Client<SubstrateConfig, DefaultEnvironment, Keypair> =
            Client::new(&artifact_file, &signer).await.unwrap();

        let address = contract_client.instantiate("new").await.unwrap();
        println!("{}", address);

        let worker_registerd = contract_client
            .write::<WorkerRegistered>(
                address.clone(),
                "register_worker",
                vec![Args::U32(10)],
            )
            .await;

        println!("{:?}", worker_registerd);

        let foo = contract_client
            .read::<u32>(address, "get_worker", vec![])
            .await;

        println!("{}", foo.unwrap());

    }
}
