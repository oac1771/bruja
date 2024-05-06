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
    Start {
        #[arg(short, long)]
        foo: String,
    },
}

#[derive(Debug, thiserror::Error)]
pub enum Error {}

pub fn run() -> Result<()> {
    let cli = Cli::parse();

    match &cli.command {
        Some(Command::Start { foo }) => {
            println!("start val: {}", foo);
        }
        None => {}
    }
    Ok(())
}