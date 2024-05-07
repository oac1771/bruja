use runtime::{RuntimeGenesisConfig, WASM_BINARY};
use sc_service::GenericChainSpec;

#[derive(Debug, clap::Args)]
pub struct StartCmd {}

impl StartCmd {
    pub fn run(&self) {
        let _ = load_spec();
    }
}

fn load_spec() {
    let code = WASM_BINARY
        .ok_or_else(|| "wasm binary not available".to_string())
        .unwrap();
    let _foo: GenericChainSpec<RuntimeGenesisConfig> = GenericChainSpec::builder(code, None).build();
}
