pub mod contract_client;
mod ink_project;
pub mod p2p;
pub mod substrate_client;

#[subxt::subxt(runtime_metadata_path = "../../chain.scale")]
pub mod chain {}
