use clap::Parser;
use contract_extrinsics::pallet_contracts_primitives::ContractAccessError;
use ink::{
    env::{
        hash::{HashOutput, Keccak256},
        hash_bytes, DefaultEnvironment,
    },
    metadata::layout::Layout,
    scale::{Encode, Decode},
};
use std::str::FromStr;
use subxt::{
    backend::{legacy::LegacyBackend, rpc::RpcClient, Backend},
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

fn hash(data: &[u8]) -> Keccak256HashOutput {
    let mut output = Keccak256HashOutput::default();
    hash_bytes::<Keccak256>(data, &mut output);
    output
}

impl Foo {
    pub async fn handle(&self) {
        let contract_address = AccountId32::from_str(&self.address).unwrap();

        let signer = Keypair::from_uri(&SecretUri::from_str("//Alice").unwrap()).unwrap();
        let artifact_file = "./target/ink/catalog/catalog.contract";
        let contract_client: Client<SubstrateConfig, DefaultEnvironment, Keypair> =
            Client::new(&artifact_file, &signer);

        let contract_message_transcoder = contract_client
            .extrinsic_opts_builder()
            .done()
            .contract_artifacts()
            .unwrap()
            .contract_transcoder()
            .unwrap();

        let mut jobs_key: u32 = 0;

        if let Layout::Root(root) = contract_message_transcoder.metadata().layout() {
            if let Layout::Struct(struct_layout) = root.layout() {
                for field in struct_layout.fields() {
                    if field.name() == "jobs" {
                        if let Layout::Root(root) = field.layout() {
                            jobs_key = *root.root_key().key();
                        }
                    }
                }
            }
        }

        let storage_key = (jobs_key, signer.public_key().0).encode();
        let args = (contract_address, storage_key.clone()).encode();

        let client = RpcClient::from_insecure_url("ws://127.0.0.1:9944")
            .await
            .unwrap();
        let backend: LegacyBackend<SubstrateConfig> = LegacyBackend::builder().build(client);

        let latest_block = backend.latest_finalized_block_ref().await.unwrap();

        let storage_data = backend
            .call("ContractsApi_get_storage", Some(&args), latest_block.hash())
            .await
            .unwrap();

        println!("Hash {:?}", hash(vec![1, 2, 3, 5].as_slice()));
        println!("Storage Data {:?}", storage_data);

        let foo: Result<Option<Vec<u8>>, ContractAccessError> = Decode::decode(&mut storage_data.as_slice()).unwrap();
        let hi = foo.unwrap().unwrap();
        let bar: Vec<Keccak256HashOutput> = Decode::decode(&mut hi.as_slice()).unwrap();

        println!("{:?}", bar);

    }
}
