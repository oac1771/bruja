use contract_extrinsics::{ExtrinsicOptsBuilder, InstantiateCommandBuilder, InstantiateExec};
// use contract_transcode::{ink_metadata::InkProject, ContractMessageTranscoder};
use sp_core::Bytes;
// use contract_extrinsics::pallet_contracts_primitives::ContractExecResult;
use ink_env::DefaultEnvironment;
use subxt::{utils::AccountId32, SubstrateConfig};
use subxt_signer::sr25519::{dev::alice, Keypair};

const FILE_PATH: &str = "./target/ink/catalog/catalog.contract";

#[tokio::main]
async fn main() {
    // let transcoder = load_transcoder();
    let signer = alice();

    deploy_contract(signer).await;
    // get_worker(&transcoder, &address, &signer).await;
    // set_worker(&contract, &address, &client);
}

// fn load_transcoder() -> ContractMessageTranscoder {
//     let metadata_file = std::fs::File::open("./target/ink/catalog/catalog.json").unwrap();
//     let abi: InkProject = serde_json::from_reader(metadata_file).unwrap();
//     let transcoder = ContractMessageTranscoder::new(abi);

//     transcoder
// }

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
