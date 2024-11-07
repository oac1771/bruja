use crate::commands::{instantiate::InstantiateCmd, wasm_time::WasmTime};
use clap::{Parser, Subcommand};

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    command: Command,
}

#[derive(Subcommand, Debug)]
enum Command {
    Instantiate(InstantiateCmd),
    WasmTime(WasmTime),
}

pub async fn run() {
    let args = Cli::parse();

    match args.command {
        Command::Instantiate(cmd) => cmd.handle().await,
        Command::WasmTime(cmd) => cmd.handle().await,
    };
}
