use rand::Rng;
use subxt::{utils::MultiAddress, Error, OnlineClient, SubstrateConfig};
use subxt_signer::sr25519;

#[subxt::subxt(runtime_metadata_path = "../../chain.scale")]
pub mod chain {}

use chain::{
    contracts::events::Instantiated,
    runtime_types::sp_weights::weight_v2::Weight,
};

const PROOF_SIZE: u64 = u64::MAX / 2;

const CONTRACT: &str = r#"
(module
    (import "env" "memory" (memory 1 1))
    (func (export "deploy"))
    (func (export "call"))
)
"#;

#[tokio::main]
async fn main() {
    let alice = sr25519::dev::alice();
    let salt: u8 = rand::thread_rng().gen();

    // let code = read_wasm();
    let code = wabt::wat2wasm(CONTRACT).expect("invalid wabt");

    let client = OnlineClient::<SubstrateConfig>::new().await.unwrap();

    let instantiate_tx = chain::tx().contracts().instantiate_with_code(
        0,
        Weight {
            ref_time: 500_000_000_000,
            proof_size: PROOF_SIZE,
        },
        None,
        code,
        vec![],
        vec![salt],
    );

    let signed_extrinsic = client
        .tx()
        .create_signed(&instantiate_tx, &alice, Default::default())
        .await
        .unwrap();

    let events = signed_extrinsic
        .submit_and_watch()
        .await
        .unwrap()
        .wait_for_finalized_success()
        .await
        .unwrap();

    let instantiated = events
        .find_first::<Instantiated>()
        .unwrap()
        .ok_or_else(|| Error::Other("Failed to find a Instantiated event".into()))
        .unwrap();

    println!("AccoundId: {:?}", instantiated.contract);
    println!("Deployer: {:?}", instantiated.deployer);

    // let call_tx = chain::tx().contracts().call(
    //     MultiAddress::Id(instantiated.contract),
    //     0,
    //     Weight {
    //         ref_time: 500_000_000,
    //         proof_size: PROOF_SIZE,
    //     },
    //     None,
    //     vec![],
    // );

    // let _result = client
    //     .tx()
    //     .sign_and_submit_then_watch_default(&call_tx, &alice)
    //     .await
    //     .unwrap();
}

// fn read_wasm() -> Vec<u8> {
//     let path = "./target/ink/contract/contract.wasm";
//     let file = std::fs::read(path).unwrap();

//     file
// }
