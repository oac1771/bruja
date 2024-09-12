use crate::{commands::submit_job::SubmitJobCmd, config::Config};
use clap::{Parser, Subcommand};

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    command: Command,
}

#[derive(Subcommand, Debug)]
enum Command {
    SubmitJob(SubmitJobCmd),
}

pub async fn run() {
    let args = Cli::parse();
    let config = Config::new();

    let result = match args.command {
        Command::SubmitJob(cmd) => cmd.handle(&config).await,
    };

    if let Err(err) = result {
        println!("Error {:?}", err);
    }
}
