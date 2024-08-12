use contract_transcode::{ink_metadata::InkProject, ContractMessageTranscoder};
// use contract_extrinsics::pallet_contracts_primitives::ContractExecResult;
// use rand::Rng;

use subxt_signer::sr25519::{self, Keypair};

// const PROOF_SIZE: u64 = u64::MAX / 2;

#[tokio::main]
async fn main() {
    let transcoder = load_transcoder();
    // let client = OnlineClient::<SubstrateConfig>::new().await.unwrap();
    let signer = sr25519::dev::alice();

    deploy_contract(transcoder.metadata(), &signer).await;
    // get_worker(&transcoder, &address, &signer).await;
    // set_worker(&contract, &address, &client);
}

fn load_transcoder() -> ContractMessageTranscoder {
    let metadata_file = std::fs::File::open("./target/ink/catalog/catalog.json").unwrap();
    let abi: InkProject = serde_json::from_reader(metadata_file).unwrap();

    let transcoder = ContractMessageTranscoder::new(abi);

    // let json = serde_json::to_string_pretty(&abi).unwrap();
    // println!("{}", json);

    transcoder
}

// fn read_wasm() -> Vec<u8> {
//     let path = "./target/ink/catalog/catalog.wasm";
//     let file = std::fs::read(path).unwrap();

//     file
// }

async fn deploy_contract(_contract: &InkProject, _signer: &Keypair) {}

// async fn get_worker(
//     transcoder: &ContractMessageTranscoder,
//     address: &AccountId32,
//     signer: &Keypair,
// ) {
//     let function = "ContractsApi_call";
//     let label = "get_worker";
//     let rpc: LegacyRpcMethods<SubstrateConfig> =
//         LegacyRpcMethods::new(RpcClient::from_url("ws://127.0.0.1:9944").await.unwrap());

//     let call_data = transcoder
//         .metadata()
//         .spec()
//         .messages()
//         .iter()
//         .find(|msg| msg.label() == label)
//         .unwrap()
//         .selector()
//         .to_bytes()
//         .to_vec();

//     let value: u128 = 0;

//     let call_request = CallRequest {
//         origin: signer.public_key().to_account_id(),
//         dest: address.clone(),
//         value: value,
//         gas_limit: None,
//         storage_deposit_limit: None,
//         input_data: call_data,
//     };

//     let args = call_request.encode();

//     let response = rpc.state_call(function, Some(&args), None).await.unwrap();
//     let result: ContractExecResult<u128> =
//         ContractExecResult::decode(&mut response.as_slice()).unwrap();

//     if let Ok(val) = result.result {
//         let foo = transcoder
//             .decode_message_return(label, &mut val.data.as_slice())
//             .unwrap();
//         println!("$$$ {:?}", foo);
//         println!(">>> {:?}", format!("{}", foo));
//     }
// }

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
