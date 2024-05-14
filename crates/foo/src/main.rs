use subxt::{Error, OnlineClient, SubstrateConfig};
use subxt_signer::sr25519;

#[subxt::subxt(runtime_metadata_path = "../../chain.scale")]
pub mod chain {}

use chain::{contracts::events::CodeStored, runtime_types::pallet_contracts::wasm::Determinism};

use tokio::fs::File;
use tokio::io::AsyncReadExt; //

#[tokio::main]
async fn main() {
    let contract_wasm = read_wasm().await;
    let client = OnlineClient::<SubstrateConfig>::new().await.unwrap();
    let alice = sr25519::dev::alice();

    let upload_tx = chain::tx()
        .contracts()
        .upload_code(contract_wasm, None, Determinism::Enforced);
    let signed_extrinsic = client
        .tx()
        .create_signed(&upload_tx, &alice, Default::default())
        .await
        .unwrap();

    let events = signed_extrinsic
        .submit_and_watch()
        .await
        .unwrap()
        .wait_for_finalized_success()
        .await
        .unwrap();

    let code_stored = events
        .find_first::<CodeStored>()
        .unwrap()
        .ok_or_else(|| Error::Other("Failed to find a CodeStored event".into()))
        .unwrap();

    println!("Code stored {:?}", code_stored);
}


async fn read_wasm() -> Vec<u8> {
    let mut file = File::open("./target/ink/contract/contract.wasm").await.unwrap();

    let mut contents = vec![];
    file.read_to_end(&mut contents).await.unwrap();

    contents
}
