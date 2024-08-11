mod primitives;

use contract_transcode::{ink_metadata::InkProject, ContractMessageTranscoder};
use primitives::ContractExecResult;
use rand::Rng;
use scale::{Decode, Encode};
use subxt::{
    backend::{legacy::LegacyRpcMethods, rpc::RpcClient},
    utils::{AccountId32, MultiAddress},
    Error, OnlineClient, SubstrateConfig,
};
use subxt_signer::sr25519::{self, Keypair};

#[subxt::subxt(runtime_metadata_path = "../../chain.scale")]
pub mod chain {}
use chain::{contracts::events::Instantiated, runtime_types::sp_weights::weight_v2::Weight};

const PROOF_SIZE: u64 = u64::MAX / 2;

#[derive(Encode)]
struct CallRequest<AccountId, Balance> {
    origin: AccountId,
    dest: AccountId,
    value: Balance,
    gas_limit: Option<Weight>,
    storage_deposit_limit: Option<Balance>,
    input_data: Vec<u8>,
}

#[tokio::main]
async fn main() {
    let transcoder = load_abi();
    let client = OnlineClient::<SubstrateConfig>::new().await.unwrap();
    let signer = sr25519::dev::alice();

    let address = deploy_contract(transcoder.metadata(), &client, &signer).await;
    get_worker(&transcoder, &address, &signer).await;
    // set_worker(&contract, &address, &client);
}

fn load_abi() -> ContractMessageTranscoder {
    let metadata_file = std::fs::File::open("./target/ink/catalog/catalog.json").unwrap();
    let abi: InkProject = serde_json::from_reader(metadata_file).unwrap();

    let transcoder = ContractMessageTranscoder::new(abi);

    // let json = serde_json::to_string_pretty(&abi).unwrap();
    // println!("{}", json);

    transcoder
}

fn read_wasm() -> Vec<u8> {
    let path = "./target/ink/catalog/catalog.wasm";
    let file = std::fs::read(path).unwrap();

    file
}

async fn deploy_contract(
    contract: &InkProject,
    client: &OnlineClient<SubstrateConfig>,
    signer: &Keypair,
) -> AccountId32 {
    let code = read_wasm();
    let label = "new";
    let selector_bytes = contract
        .spec()
        .constructors()
        .iter()
        .find(|val| val.label() == label)
        .unwrap()
        .selector()
        .to_bytes()
        .to_vec();

    let salt: u8 = rand::thread_rng().gen();

    let instantiate_tx = chain::tx().contracts().instantiate_with_code(
        0,
        Weight {
            ref_time: 500_000_000_000,
            proof_size: PROOF_SIZE,
        },
        None,
        code,
        selector_bytes,
        vec![salt],
    );

    let signed_extrinsic = client
        .tx()
        .create_signed(&instantiate_tx, signer, Default::default())
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

    instantiated.contract
}

async fn get_worker(
    transcoder: &ContractMessageTranscoder,
    address: &AccountId32,
    signer: &Keypair,
) {
    let function = "ContractsApi_call";
    let label = "get_worker";
    let rpc: LegacyRpcMethods<SubstrateConfig> =
        LegacyRpcMethods::new(RpcClient::from_url("ws://127.0.0.1:9944").await.unwrap());

    let call_data = transcoder
        .metadata()
        .spec()
        .messages()
        .iter()
        .find(|msg| msg.label() == label)
        .unwrap()
        .selector()
        .to_bytes()
        .to_vec();

    let value: u128 = 0;

    let call_request = CallRequest {
        origin: signer.public_key().to_account_id(),
        dest: address.clone(),
        value: value,
        gas_limit: None,
        storage_deposit_limit: None,
        input_data: call_data,
    };

    let args = call_request.encode();

    let response = rpc.state_call(function, Some(&args), None).await.unwrap();
    let result: ContractExecResult<u128> =
        ContractExecResult::decode(&mut response.as_slice()).unwrap();

    if let Ok(val) = result.result {
        let foo = transcoder
            .decode_message_return(label, &mut val.data.as_slice())
            .unwrap();
        println!("$$$ {:?}", foo);
        println!(">>> {:?}", format!("{}", foo));
    }
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
