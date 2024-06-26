use crate::rssh_core::Entry;
use encrypt_config::{Config, PersistSource};
use serde::{Deserialize, Serialize};

pub(crate) fn config() -> Config {
    let mut config = Config::new();
    config.load_source::<RsshConfig>();
    config
}

#[derive(Default, Serialize, Deserialize, PersistSource)]
#[source(name = "rssh.json")]
pub(crate) struct RsshConfig {
    entries: Vec<Entry>,
}

impl RsshConfig {
    pub(crate) fn insert(&mut self, entry: Entry) {
        self.entries.push(entry);
    }

    pub(crate) fn entries(&self) -> &Vec<Entry> {
        &self.entries
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[ignore = "This test needs human read to ensure the correctness"]
    fn test_config() {
        let config = config();
        let rssh_config = config.get::<RsshConfig>().unwrap();
        dbg!(rssh_config.entries());
    }
}
