mod cli;
mod commands;
mod config;
mod error;

use cli::run;

#[tokio::main]
async fn main() {
    run().await;
}
