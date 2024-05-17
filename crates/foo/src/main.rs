use subxt::{Error, OnlineClient, SubstrateConfig};
use subxt_signer::sr25519;

#[subxt::subxt(runtime_metadata_path = "../../chain.scale")]
pub mod chain {}

use chain::{contracts::events::CodeStored, runtime_types::{pallet_contracts::wasm::Determinism, sp_weights::weight_v2::Weight}};

use tokio::fs::File;
use tokio::io::AsyncReadExt;

const PROOF_SIZE: u64 = u64::MAX / 2;

#[tokio::main]
async fn main() {
    // look into using chain::tx().instantiate_with_code() instead of doing upload_code
    let client = OnlineClient::<SubstrateConfig>::new().await.unwrap();

    let contract_wasm = read_wasm().await;
    let alice = sr25519::dev::alice();

    // let upload_tx = chain::tx()
    //     .contracts()
    //     .upload_code(contract_wasm, None, Determinism::Enforced);

    let instantiate_tx = chain::tx().contracts().instantiate_with_code(
        100_000_000_000_000_000, // endowment
        Weight {
            ref_time: 500_000_000_000,
            proof_size: PROOF_SIZE,
        }, // gas_limit
        None,
        contract_wasm,
        vec![], // data
        vec![], // salt
    );

    let signed_extrinsic = client
        .tx()
        .create_signed(&instantiate_tx, &alice, Default::default())
        .await
        .unwrap();

    let upload_result = signed_extrinsic
        .submit_and_watch()
        .await
        .unwrap()
        .wait_for_finalized()
        .await
        .unwrap();

    println!("{:?}", upload_result);

    // let code_stored = events
    //     .find_first::<CodeStored>()
    //     .unwrap()
    //     .ok_or_else(|| Error::Other("Failed to find a CodeStored event".into()))
    //     .unwrap();

    // println!("Code stored {:?}", code_stored);

    // let events = client.events().at_latest().await.unwrap();

    // for event in events.iter() {
    //     let event = event.unwrap();

    //     let pallet = event.pallet_name();
    //     let variant = event.variant_name();
    //     let field_values = event.field_values().unwrap();

    //     println!("{pallet}::{variant}: {field_values}");
    // }
}

async fn read_wasm() -> Vec<u8> {
    let mut file = File::open("./target/ink/contract/contract.wasm")
        .await
        .unwrap();

    let mut contents = vec![];
    file.read_to_end(&mut contents).await.unwrap();

    contents
}
