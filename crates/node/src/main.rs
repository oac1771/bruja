mod cli;
mod commands;

fn main() -> cli::Result<()> {
    cli::run()
}
