use crate::{
    error::{Error, Result},
    rssh_core::Entry,
};
use cliclack::{Input, Select};

pub(crate) fn new_entry_iteractive() -> Result<Entry> {
    let client: String = Input::new("Client")
        .default_input("ssh")
        .interact()
        .map_err(|_| Error::InteractiveInput)?;
    let user: String = Input::new("User")
        .required(true)
        .interact()
        .map_err(|_| Error::InteractiveInput)?;
    let hostname: String = Input::new("Hostname")
        .required(true)
        .interact()
        .map_err(|_| Error::InteractiveInput)?;
    let port: u16 = Input::new("Port")
        .default_input("22")
        .interact()
        .map_err(|_| Error::InteractiveInput)?;
    Entry::new(client, user, hostname, port)
}

pub(crate) fn choose_entry(entries: &[Entry]) -> Result<&Entry> {
    if entries.is_empty() {
        return Err(Error::NoEntry);
    }
    let chosen = Select::new("Choose")
        .items(
            &entries
                .iter()
                .enumerate()
                .map(|(i, e)| (i, e.hostname(), e.user()))
                .collect::<Vec<_>>(),
        )
        .interact()
        .map_err(|_| Error::InteractiveInput)?;
    Ok(&entries[chosen])
}
