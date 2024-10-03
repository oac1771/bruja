use crate::commands::{foo::Foo, instantiate::InstantiateCmd, p2p::P2P, wasm_time::WasmTime};
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
    Foo(Foo),
    WasmTime(WasmTime),
    P2P(P2P),
}

pub async fn run() {
    let args = Cli::parse();

    match args.command {
        Command::Instantiate(cmd) => cmd.handle().await,
        Command::Foo(cmd) => cmd.handle().await,
        Command::WasmTime(cmd) => cmd.handle().await,
        Command::P2P(cmd) => cmd.handle().await,
    };
}
