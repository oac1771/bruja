mod cli;
mod commands;

use cli::run;

#[tokio::main]
async fn main() {
    run().await;
}