use crate::{
    action::{choose_entry, new_entry_iteractive},
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
    New,
}

impl Cli {
    /// Run the CLI.
    pub fn run() -> Result<()> {
        let args = Cli::parse();
        let config = config();

        match args.subcmd {
            Some(SubCmd::New) => {
                let entry = new_entry_iteractive()?;
                let mut rssh_config = config.get_mut::<RsshConfig>().unwrap();
                rssh_config.insert(entry.clone());
                entry.connect();
            }
            None => {
                let rssh_config = config.get::<RsshConfig>().unwrap();
                let entry = choose_entry(rssh_config.entries())?;
                entry.connect();
            }
        }
        config.save::<RsshConfig>()?;
        Ok(())
    }
}
