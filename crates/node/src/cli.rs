use super::commands::start::StartCmd;
use clap::Parser;

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

#[derive(Debug, thiserror::Error)]
pub enum Error {}

pub fn run() -> Result<()> {
    let cli = Cli::parse();

    match &cli.command {
        Some(Command::Start(cmd)) => cmd.run(),
        None => {}
    }
    Ok(())
}
