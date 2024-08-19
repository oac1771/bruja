use clap::Parser;

#[derive(Debug, Parser)]
pub struct RegisterCmd {
    pub foo: String,
    pub bar: String,
}