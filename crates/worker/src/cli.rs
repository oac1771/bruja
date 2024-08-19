use crate::{commands::register::RegisterCmd, config::Config};
use clap::{Parser, Subcommand};

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    command: Command,
}

#[derive(Subcommand, Debug)]
enum Command {
    Register(RegisterCmd),
}

pub fn run() {
    let args = Cli::parse();
    let config = Config;

    match args.command {
        Command::Register(cmd) => {
            cmd.handle(config);
        }
    }
}
