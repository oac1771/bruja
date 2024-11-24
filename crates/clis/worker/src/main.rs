use tracing_subscriber::{fmt, prelude::*, EnvFilter};
use worker::cli::run;

#[tokio::main]
pub async fn main() {
    tracing_subscriber::registry()
        .with(fmt::layer())
        .with(EnvFilter::from_default_env().add_directive("worker=info".parse().unwrap()))
        .init();
    run().await;
}
