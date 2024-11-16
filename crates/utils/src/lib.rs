mod ink_project;
pub mod p2p;
pub mod substrate_client;

pub mod services;

#[subxt::subxt(runtime_metadata_path = "../../chain.scale")]
pub mod chain {}
