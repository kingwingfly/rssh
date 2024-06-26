use crate::rssh_core::Entry;
use encrypt_config::{Config, PersistSource};
use serde::{Deserialize, Serialize};
use std::collections::HashSet;

pub(crate) fn config() -> Config {
    let mut config = Config::new();
    config.load_source::<RsshConfig>();
    config
}

#[derive(Default, Serialize, Deserialize, PersistSource)]
#[source(name = "rssh/rssh.json")]
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
        let rssh_config = config.get::<RsshConfig>().unwrap();
        dbg!(RsshConfig::path());
        dbg!(rssh_config.entries());
    }
}
