use crate::{
    action::{choose_entry, manage_entries, new_entry_iteractive},
    config::{config, RsshConfig},
    error::Result,
};
use clap::{Parser, Subcommand};

const VERSION: &str = const_format::formatcp!(
    "{}\nRUSTC: {} {} {}",
    match option_env!("VERGEN_GIT_DESCRIBE") {
        Some(var) => var,
        _ => concat!(env!("CARGO_PKG_VERSION"), "(CARGO_PKG_VERSION)"),
    },
    env!("VERGEN_RUSTC_HOST_TRIPLE"),
    env!("VERGEN_RUSTC_CHANNEL"),
    env!("VERGEN_RUSTC_SEMVER")
);

/// The main CLI entry point.
#[derive(Parser)]
#[command(author, version = VERSION, about)]
pub struct Cli {
    #[clap(subcommand)]
    subcmd: Option<SubCmd>,
}

#[derive(Subcommand)]
enum SubCmd {
    /// New a ssh entry and connect.
    New,
    /// Manage ssh entries.
    Manage,
}

impl Cli {
    /// Run the CLI.
    pub fn run() -> Result<()> {
        let args = Cli::parse();
        let config = config();

        match args.subcmd {
            Some(SubCmd::New) => {
                let entry = new_entry_iteractive()?;
                let mut rssh_config = config.get_mut::<RsshConfig>();
                rssh_config.insert(entry.clone());
                entry.connect();
            }
            Some(SubCmd::Manage) => {
                let mut rssh_config = config.get_mut::<RsshConfig>();
                manage_entries(rssh_config.entries_mut())?;
            }
            None => {
                let rssh_config = config.get::<RsshConfig>();
                let entry = choose_entry(rssh_config.entries())?;
                entry.connect();
            }
        }
        Ok(())
    }
}
