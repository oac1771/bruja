use rand::Rng;
use subxt::{Error, OnlineClient, SubstrateConfig};
use subxt_signer::sr25519;

#[subxt::subxt(runtime_metadata_path = "../../chain.scale")]
pub mod chain {}

use chain::{
    contracts::events::Instantiated,
    runtime_types::sp_weights::weight_v2::Weight,
};

const PROOF_SIZE: u64 = u64::MAX / 2;

const _CONTRACT: &str = r#"
(module
    (import "env" "memory" (memory 1 1))
    (func (export "deploy"))
    (func (export "call"))
)
"#;

#[tokio::main]
async fn main() {
    deploy_conract().await
}

fn read_wasm() -> Vec<u8> {
    let path = "./target/ink/contract/contract.wasm";
    let file = std::fs::read(path).unwrap();

    file
}

async fn deploy_conract() {
    let code = read_wasm();

    let client = OnlineClient::<SubstrateConfig>::new().await.unwrap();
    let salt: u8 = rand::thread_rng().gen();
    let alice = sr25519::dev::alice();

    let instantiate_tx = chain::tx().contracts().instantiate_with_code(
        0,
        Weight {
            ref_time: 500_000_000_000,
            proof_size: PROOF_SIZE,
        },
        None,
        code,
        vec![155, 174, 157, 94],
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
