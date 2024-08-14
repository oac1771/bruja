#[subxt::subxt(runtime_metadata_path = "../../chain.scale")]
pub mod chain {}

use catalog::catalog::WorkerSet;
use chain::contracts::events::ContractEmitted;
use codec::Decode;
use contract_extrinsics::{
    CallCommandBuilder, CallExec, ExtrinsicOptsBuilder, InstantiateCommandBuilder, InstantiateExec,
};
use ink_env::DefaultEnvironment;
use sp_core::Bytes;
use subxt::{utils::AccountId32, SubstrateConfig};
use subxt_signer::sr25519::{dev::alice, Keypair};

const FILE_PATH: &str = "./target/ink/catalog/catalog.contract";

#[tokio::main]
async fn main() {
    let signer = alice();

    let address = deploy_contract(signer.clone()).await;
    get_worker(address.clone(), signer.clone()).await;
    set_worker(address.clone(), signer.clone()).await;
    get_worker(address.clone(), signer.clone()).await;
}

async fn deploy_contract(signer: Keypair) -> AccountId32 {
    let extrinsic_opts = ExtrinsicOptsBuilder::new(signer)
        .file(Some(FILE_PATH))
        .done();

    let bytes: [u8; 8] = rand::random();
    let salt: Bytes = bytes.to_vec().into();

    let instantiate_exec: InstantiateExec<SubstrateConfig, DefaultEnvironment, Keypair> =
        InstantiateCommandBuilder::new(extrinsic_opts)
            .constructor("new")
            .salt(Some(salt))
            .done()
            .await
            .unwrap();

    let address = instantiate_exec
        .instantiate(None)
        .await
        .unwrap()
        .contract_address;

    return address;
}

async fn get_worker(address: AccountId32, signer: Keypair) {
    let message = "get_worker";
    let extrinsic_opts = ExtrinsicOptsBuilder::new(signer)
        .file(Some(FILE_PATH))
        .done();

    let call_exec: CallExec<SubstrateConfig, DefaultEnvironment, Keypair> =
        CallCommandBuilder::new(address, &message, extrinsic_opts)
            .done()
            .await
            .unwrap();

    let call_result = call_exec.call_dry_run().await.unwrap().result.unwrap().data;
    let value = call_exec
        .transcoder()
        .decode_message_return(call_exec.message(), &mut &call_result[..])
        .unwrap();

    println!("Get worker result: {}", value);
}

async fn set_worker(address: AccountId32, signer: Keypair) {
    let message = "set_worker";
    let val = "10";
    let extrinsic_opts = ExtrinsicOptsBuilder::new(signer)
        .file(Some(FILE_PATH))
        .done();

    let call_exec: CallExec<SubstrateConfig, DefaultEnvironment, Keypair> =
        CallCommandBuilder::new(address, &message, extrinsic_opts)
            .args(vec![val])
            .done()
            .await
            .unwrap();

    let events = call_exec.call(None).await.unwrap();
    let contract_emitted = events.find_first::<ContractEmitted>().unwrap().unwrap();
    let worker_set_event =
        <WorkerSet as Decode>::decode(&mut contract_emitted.data.as_slice()).unwrap();
    println!("Worker Set Event {:?}", worker_set_event);
}
