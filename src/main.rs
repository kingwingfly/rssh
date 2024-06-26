use rssh::Cli;
use std::io;
use tracing::Level;
use tracing_subscriber::{filter, prelude::*};

fn main() {
    let filter = filter::Targets::new()
        // Enable the `INFO` level for anything in `rssh`
        .with_target("rssh", Level::INFO);
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::fmt::layer()
                .with_writer(io::stderr)
                .without_time()
                .with_target(false),
        )
        .with(filter)
        .init();

    if let Err(e) = Cli::run() {
        tracing::error!("{e}");
    }
}
