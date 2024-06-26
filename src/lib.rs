//! A library for rssh (remember ssh).

#![deny(missing_docs)]

mod action;
mod cli;
mod config;
mod error;
mod rssh_core;

pub use cli::Cli;
