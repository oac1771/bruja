use runtime::WASM_BINARY;

use subxt::{OnlineClient, SubstrateConfig, Error};
use subxt_signer::sr25519;

#[subxt::subxt(runtime_metadata_path = "../../chain.scale")]
pub mod chain {}

#[tokio::main]
async fn main() {
    let client = OnlineClient::<SubstrateConfig>::new().await.unwrap();
    let code = WASM_BINARY.unwrap().to_vec();
    let alice = sr25519::dev::alice();

    let upload_tx = chain::tx().contracts().upload_code(
        code,
        None,
        chain::runtime_types::pallet_contracts::wasm::Determinism::Enforced,
    );
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
        .find_first::<chain::contracts::events::CodeStored>().unwrap()
        .ok_or_else(|| Error::Other("Failed to find a CodeStored event".into())).unwrap();

    println!("Hello, world!");
    println!("Code stored {:?}", code_stored);
}
