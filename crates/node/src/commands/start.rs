use runtime::{RuntimeGenesisConfig, WASM_BINARY};
use sc_service::GenericChainSpec;

#[derive(Debug, clap::Args)]
pub struct StartCmd {}

impl StartCmd {
    pub fn run(&self) {
        let code = WASM_BINARY
            .ok_or_else(|| "wasm binary not available".to_string())
            .unwrap();
        let chain_spec: GenericChainSpec<RuntimeGenesisConfig> =
            GenericChainSpec::builder(code, None).build();

        println!("name: {}", chain_spec.name());
    }
}
