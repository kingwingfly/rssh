use crate::{
    error::{Error, Result},
    rssh_core::Entry,
};
use cliclack::{outro, Input, Select};
use std::collections::HashSet;

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
    outro("New entry saved!").map_err(|_| Error::InteractiveInput)?;
    Entry::new(client, user, hostname, port)
}

pub(crate) fn choose_entry(entries: &HashSet<Entry>) -> Result<&Entry> {
    if entries.is_empty() {
        return Err(Error::NoEntry);
    }
    let chosen = Select::new("Choose")
        .items(
            &entries
                .iter()
                .map(|e| (e, e.hostname(), e.user()))
                .collect::<Vec<_>>(),
        )
        .interact()
        .map_err(|_| Error::InteractiveInput)?;
    Ok(chosen)
}

pub(crate) fn take_entry(entries: &mut HashSet<Entry>) -> Result<Entry> {
    if entries.is_empty() {
        return Err(Error::NoEntry);
    }
    let chosen = Select::new("Choose")
        .items(
            &entries
                .iter()
                .map(|e| (e, e.hostname(), e.user()))
                .collect::<Vec<_>>(),
        )
        .interact()
        .map_err(|_| Error::InteractiveInput)?
        .clone();
    entries.remove(&chosen);
    Ok(chosen)
}

fn edit_entry(entry: &mut Entry) -> Result<()> {
    let client: String = Input::new("Client")
        .default_input(entry.client())
        .interact()
        .map_err(|_| Error::InteractiveInput)?;
    let user: String = Input::new("User")
        .default_input(entry.user())
        .interact()
        .map_err(|_| Error::InteractiveInput)?;
    let hostname: String = Input::new("Hostname")
        .default_input(entry.hostname())
        .interact()
        .map_err(|_| Error::InteractiveInput)?;
    let port: u16 = Input::new("Port")
        .default_input("22")
        .interact()
        .map_err(|_| Error::InteractiveInput)?;
    *entry = Entry::new(client, user, hostname, port)?;
    Ok(())
}

pub(crate) fn manage_entries(entries: &mut HashSet<Entry>) -> Result<()> {
    loop {
        let mut entry = match take_entry(entries) {
            Ok(entry) => entry,
            Err(Error::NoEntry) => {
                outro("No entry found; Change saved").map_err(|_| Error::InteractiveInput)?;
                return Ok(());
            }
            Err(_) => {
                outro("Change saved").map_err(|_| Error::InteractiveInput)?;
                return Ok(());
            }
        };
        let items = &["Edit", "Remove", "Connect", "Back"];
        let chosen = Select::new("Manage")
            .items(
                &items
                    .iter()
                    .enumerate()
                    .map(|(i, op)| (i, op, ""))
                    .collect::<Vec<_>>(),
            )
            .interact()
            .map_err(|_| Error::InteractiveInput)?;
        match chosen {
            0 => match edit_entry(&mut entry) {
                Ok(_) => {
                    entries.insert(entry);
                    outro("Entry edited!").map_err(|_| Error::InteractiveInput)?;
                }
                Err(_) => {
                    entries.insert(entry);
                    outro("Edit cancelled!").map_err(|_| Error::InteractiveInput)?;
                }
            },
            1 => {}
            2 => {
                entries.insert(entry.clone());
                outro("Entry connecting!").map_err(|_| Error::InteractiveInput)?;
                entry.connect();
                return Ok(());
            }
            3 => {
                entries.insert(entry);
            }
            _ => {}
        }
    }
}
