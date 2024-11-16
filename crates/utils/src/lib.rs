mod ink_project;
pub mod services;
pub mod substrate_client;

#[subxt::subxt(runtime_metadata_path = "../../chain.scale")]
pub mod chain {}
