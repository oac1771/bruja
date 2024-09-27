use catalog::catalog::{JobSubmitted, Keccak256HashOutput, WorkerRegistered};
use clap::Parser;
use ink::env::DefaultEnvironment;
use std::str::FromStr;
use subxt::SubstrateConfig;
use subxt_signer::{sr25519::Keypair, SecretUri};
use utils::client::Client;

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
            .write::<WorkerRegistered, u32>(address.clone(), "register_worker", 10)
            .await
            .unwrap();

        println!("{:?}", worker_registerd);

        let worker = contract_client
            .read::<u32, Vec<()>>(address.clone(), "get_worker", vec![])
            .await
            .unwrap();

        println!("{}", worker);

        let job_submitted = contract_client
            .write::<JobSubmitted, Vec<u8>>(address.clone(), "submit_job", vec![1, 2, 3, 4, 5])
            .await
            .unwrap();

        println!("job submitted {:?}", job_submitted);

        let job_ids: Vec<Keccak256HashOutput> = contract_client
            .read_storage(address.clone(), "jobs", &signer.public_key().0)
            .await
            .unwrap();

        println!("job_ids: {:?}", job_ids);
    }
}
