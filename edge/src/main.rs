use tracing::info;
use tracing_subscriber::prelude::*;
use tracing_subscriber::{EnvFilter, fmt};

#[tokio::main]
async fn main() {
    let fmt_layer = fmt::layer().with_level(true).with_target(true).compact();

    // The default logging level is `info`.
    // You can override this by setting the `RUST_LOG` environment variable.
    let filter_layer = EnvFilter::try_from_default_env()
        .or_else(|_| EnvFilter::try_new("info"))
        .unwrap();

    tracing_subscriber::registry()
        .with(filter_layer)
        .with(fmt_layer)
        .init();

    info!("Hello, world!");
}
