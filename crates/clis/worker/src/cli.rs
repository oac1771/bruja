use crate::{commands::start::StartCmd, config::Config};
use clap::{Parser, Subcommand};

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    command: Command,
}

#[derive(Subcommand, Debug)]
enum Command {
    Start(StartCmd),
}

pub async fn run() {
    let args = Cli::parse();
    let config = Config::default();

    let result = match args.command {
        Command::Start(cmd) => cmd.handle(config).await,
    };

    if let Err(err) = result {
        panic!("Error: {:?}", err);
    }
}
