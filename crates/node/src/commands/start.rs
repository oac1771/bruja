// use runtime::WASM_BINARY;
// println!("Binary: {:?}", WASM_BINARY);

#[derive(Debug, clap::Args)]
pub struct StartCmd {}

impl StartCmd {
    pub fn run(&self) {
        println!("inside start commmand");
    }
}
