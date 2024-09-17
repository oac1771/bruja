pub mod client;
mod ink_project;

#[subxt::subxt(runtime_metadata_path = "../../chain.scale")]
pub mod chain {}
