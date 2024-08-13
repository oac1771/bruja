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
    get_worker(address, signer).await;
    // set_worker(&contract, &address, &client);
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

// async fn _set_worker(
//     contract: &InkProject,
//     address: &AccountId32,
//     client: &OnlineClient<SubstrateConfig>,
//     signer: &Keypair,
// ) {
//     let mut data = contract
//         .spec
//         .messages
//         .iter()
//         .filter(|msg| &msg.label() == "set_worker")
//         .collect::<Vec<&Message>>()[0]
//         .get_selector_bytes()
//         .unwrap();

//     let id: u32 = 10;
//     id.encode_to(&mut data);

//     let call_tx = chain::tx().contracts().call(
//         MultiAddress::Id(address.clone()),
//         0,
//         Weight {
//             ref_time: 500_000_000,
//             proof_size: PROOF_SIZE,
//         },
//         None,
//         data,
//     );

//     let _result = client
//         .tx()
//         .sign_and_submit_then_watch_default(&call_tx, signer)
//         .await
//         .unwrap();
// }
