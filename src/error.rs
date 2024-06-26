//! Error and Result types for the library.
use snafu::Snafu;
use std::path::PathBuf;

pub type Result<T, E = Error> = ::core::result::Result<T, E>;

#[derive(Debug, Snafu)]
#[snafu(context(suffix(false)))]
pub enum Error {
    #[snafu(display("SSH client not found at {:?}", path))]
    SshClientNotFound { path: PathBuf },
    #[snafu(display("Invalid user: {}", user))]
    InvalidUser { user: String },
    #[snafu(display("Invalid hostname: {}", hostname))]
    InvalidHostname { hostname: String },
    #[snafu(display("Invalid port: {}", port))]
    InvalidPort { port: u16 },
    #[snafu(display("Interative input error"))]
    InteractiveInput,
    #[snafu(display("Configuration error: {}", source), context(false))]
    Config {
        source: encrypt_config::error::ConfigError,
    },
    #[snafu(display("No entry to choose, create with `rssh new`"))]
    NoEntry,
}
