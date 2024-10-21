use requester::cli::run;
use tracing_subscriber::{filter::LevelFilter, fmt::layer, prelude::*};

#[tokio::main]
async fn main() {
    let info_layer = layer().with_filter(LevelFilter::INFO);
    tracing_subscriber::registry().with(info_layer).init();
    run().await;
}
