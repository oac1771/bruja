pub mod contract_client;
mod ink_project;
pub mod p2p;

#[subxt::subxt(runtime_metadata_path = "../../chain.scale")]
pub mod chain {}
