use std::time::Duration;

use crate::processor::process_config;

mod err;
mod parser;
mod processor;
mod reader;

#[tokio::main]
async fn main() {
    subscribe_tracing();

    loop {
        match process_config().await {
            Err(err) => {
                tracing::error!("{}", err);
            }
            Ok(result) => {
                tracing::info!("result {}", result);
            }
        }

        tokio::time::sleep(Duration::from_secs(4)).await;
    }
}

fn subscribe_tracing() {
    use tracing::Level;
    use tracing_subscriber::{
        EnvFilter, Layer, filter, fmt, layer::SubscriberExt, util::SubscriberInitExt,
    };

    let bins = ["err_trace", "http_server"];

    let out_layer = fmt::layer()
        .with_writer(std::io::stdout)
        .with_filter(
            bins.iter()
                .fold(EnvFilter::from_default_env(), |filter, bin| {
                    filter.add_directive(bin.parse().unwrap())
                }),
        )
        .with_filter(filter::filter_fn(|metadata| {
            *metadata.level() != Level::ERROR
        }));

    let err_layer = fmt::layer()
        .with_writer(std::io::stderr)
        .with_filter(filter::LevelFilter::ERROR);

    tracing_subscriber::registry()
        .with(out_layer)
        .with(err_layer)
        .with(tracing_error::ErrorLayer::default())
        .init();
}
