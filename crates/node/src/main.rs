// use runtime::WASM_BINARY;
// println!("Binary: {:?}", WASM_BINARY);

mod cli;
mod command;

fn main() -> cli::Result<()> {
    cli::run()
}
