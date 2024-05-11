use crate::commands::start::{StartCmd, StartCmdError};
use clap::Parser;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error(transparent)]
	StartCmdError(#[from] StartCmdError),
}

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, clap::Parser)]
#[command(version, about, long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Option<Command>,
}

#[derive(Debug, clap::Subcommand)]
pub enum Command {
    Start(StartCmd),
}

pub fn run() -> Result<()> {
    let cli = Cli::parse();

    let result = match &cli.command {
        Some(Command::Start(cmd)) => cmd.run()?,
        None => {},
    };

    Ok(result)
}
