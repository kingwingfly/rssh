use crate::rssh_core::Entry;
use encrypt_config::{Config, SecretSource};
use serde::{Deserialize, Serialize};
use std::collections::HashSet;

pub(crate) fn config() -> Config {
    Config::default()
}

#[derive(Default, Serialize, Deserialize, SecretSource)]
#[source(name = "rssh/rssh.json", keyring_entry = "rssh")]
pub(crate) struct RsshConfig {
    entries: HashSet<Entry>,
}

impl RsshConfig {
    pub(crate) fn insert(&mut self, entry: Entry) {
        self.entries.insert(entry);
    }

    pub(crate) fn entries(&self) -> &HashSet<Entry> {
        &self.entries
    }

    pub(crate) fn entries_mut(&mut self) -> &mut HashSet<Entry> {
        &mut self.entries
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    // #[ignore = "This test needs human read to ensure the correctness"]
    fn test_config() {
        let config = config();
        let rssh_config = config.get::<RsshConfig>();
        dbg!(RsshConfig::path());
        dbg!(rssh_config.entries());
    }
}
