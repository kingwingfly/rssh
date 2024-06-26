use crate::error::{Error, Result};
use serde::{Deserialize, Serialize};
use std::{ffi::OsStr, path::PathBuf};

#[derive(Serialize, Deserialize, Clone)]
#[cfg_attr(test, derive(Debug))]
pub(crate) struct Client(PathBuf);

impl Default for Client {
    // Returns a new instance of `Client` with the default path `ssh`.
    fn default() -> Self {
        Client::new_unchecked(PathBuf::from("ssh"))
    }
}

impl Client {
    pub(crate) fn new(path: PathBuf) -> Result<Self> {
        if !path.exists() {
            return Err(Error::SshClientNotFound { path });
        }
        Ok(Client(path))
    }

    pub(crate) fn new_unchecked(path: PathBuf) -> Self {
        Client(path)
    }
}

impl AsRef<OsStr> for Client {
    fn as_ref(&self) -> &OsStr {
        self.0.as_os_str()
    }
}
