mod chain_spec;
mod cli;
mod commands;
mod rpc;
mod service;

fn main() -> cli::Result<()> {
    cli::run()
}
