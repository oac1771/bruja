mod ink_project;
pub mod services;

#[subxt::subxt(runtime_metadata_path = "../../chain.scale")]
pub mod chain {}
