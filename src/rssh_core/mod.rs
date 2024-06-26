mod args;
mod client;
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

use crate::error::Result;
use args::Args;
use client::Client;

#[derive(Serialize, Deserialize, Clone)]
#[cfg_attr(test, derive(Debug))]
pub struct Entry {
    client: Client,
    args: Args,
    other_args: Vec<String>,
}

impl Entry {
    pub fn new(
        client: impl AsRef<str>,
        user: impl AsRef<str>,
        hostname: impl AsRef<str>,
        port: u16,
    ) -> Result<Self> {
        let client = Client::new(PathBuf::from(client.as_ref())).unwrap_or_default();
        let args = Args::new(user, hostname, port)?;
        Ok(Entry {
            client,
            args,
            other_args: Vec::new(),
        })
    }

    pub fn connect(&self) {
        std::process::Command::new(&self.client)
            .args(self.args.args())
            .stdin(std::process::Stdio::inherit())
            .stdout(std::process::Stdio::inherit())
            .status()
            .expect("Failed to execute command");
    }

    pub(crate) fn hostname(&self) -> &str {
        self.args.hostname()
    }

    pub(crate) fn user(&self) -> &str {
        self.args.user()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[ignore = "This test requires a valid ssh configuration"]
    fn test_new() {
        let entry = Entry::new("ssh", "louis", "arch", 22);
        assert!(entry.is_ok());
    }
}
