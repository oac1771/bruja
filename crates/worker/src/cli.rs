use clap::{Parser, Subcommand};
use crate::commands::register::RegisterCmd;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct Cli {

    #[command(subcommand)]
    command: Command,

}

#[derive(Subcommand, Debug)]
enum Command {
    Register(RegisterCmd)
}

pub fn run() {
    let args = Cli::parse();

    match args.command {
        Command::Register(cmd) => {
            println!("{:?}", cmd);
        }
    }
}