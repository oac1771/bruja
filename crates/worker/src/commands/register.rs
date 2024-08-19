use crate::config::Config;
use clap::Parser;

#[derive(Debug, Parser)]
pub struct RegisterCmd {
    #[arg(long)]
    pub foo: String,
}

impl RegisterCmd {
    pub fn handle(&self, _config: Config) {}
}
