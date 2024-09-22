use catalog::catalog::WorkerRegistered;
use clap::Parser;
use ink::env::DefaultEnvironment;
use std::str::FromStr;
use subxt::SubstrateConfig;
use subxt_signer::{sr25519::Keypair, SecretUri};
use utils::client::{Args, Client};

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

        // let worker_registerd = contract_client
        //     .mutable_call::<WorkerRegistered>(
        //         address.clone(),
        //         "register_worker",
        //         vec![Args::U32(10)],
        //     )
        //     .await;

        // println!("{:?}", worker_registerd);

        // // let foo = contract_client
        // //     .immutable_call::<u32>("get_worker", address, vec![])
        // //     .await;

        // let foo = contract_client
        //     .immutable_call::<u32>(address, "get_worker", vec![])
        //     .await;

        // println!("{:?}", foo);

        // let job_submitted = contract_client
        //     .mutable_call_v2::<JobSubmitted>(
        //         address,
        //         "submit_job",
        //         vec![Args::Vec(vec![1, 2, 3, 4])],
        //     )
        //     .await;

        // println!("{:?}", job_submitted);
    }
}