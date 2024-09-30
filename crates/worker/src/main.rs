mod cli;
mod commands;
mod config;
mod error;
mod services;

use cli::run;

#[tokio::main]
async fn main() {
    run().await;
}
