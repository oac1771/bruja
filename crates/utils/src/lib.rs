pub mod client;
mod ink_project;
pub mod networking;

#[subxt::subxt(runtime_metadata_path = "../../chain.scale")]
pub mod chain {}
