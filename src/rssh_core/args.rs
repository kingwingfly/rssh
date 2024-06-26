use crate::error::{Error, Result};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone)]
#[cfg_attr(test, derive(Debug))]
pub(crate) struct Args {
    user: User,
    hostname: HostName,
    port: Port,
}

impl Args {
    pub(crate) fn new(user: impl AsRef<str>, hostname: impl AsRef<str>, port: u16) -> Result<Self> {
        Ok(Args {
            user: User::new(user)?,
            hostname: HostName::new(hostname)?,
            port: Port::new(port)?,
        })
    }

    pub(crate) fn args(&self) -> impl IntoIterator<Item = &str> {
        vec![
            "-l",
            self.user.as_ref(),
            "-p",
            self.port.as_ref(),
            self.hostname.as_ref(),
        ]
    }

    pub(crate) fn hostname(&self) -> &str {
        self.hostname.as_ref()
    }

    pub(crate) fn user(&self) -> &str {
        self.user.as_ref()
    }
}

#[derive(Serialize, Deserialize, Clone)]
#[cfg_attr(test, derive(Debug))]
pub(crate) struct User(String);

impl User {
    pub(crate) fn new(user: impl AsRef<str>) -> Result<Self> {
        if user.as_ref().is_empty() {
            return Err(Error::InvalidUser {
                user: "<Empty>".to_string(),
            });
        }
        Ok(User(user.as_ref().to_string()))
    }
}

impl AsRef<str> for User {
    fn as_ref(&self) -> &str {
        self.0.as_ref()
    }
}

#[derive(Serialize, Deserialize, Clone)]
#[cfg_attr(test, derive(Debug))]
pub(crate) struct HostName(String);

impl HostName {
    pub(crate) fn new(hostname: impl AsRef<str>) -> Result<Self> {
        if hostname.as_ref().is_empty() {
            return Err(Error::InvalidHostname {
                hostname: "<Empty>".to_string(),
            });
        }
        Ok(HostName(hostname.as_ref().to_string()))
    }
}

impl AsRef<str> for HostName {
    fn as_ref(&self) -> &str {
        self.0.as_ref()
    }
}

#[derive(Serialize, Deserialize, Clone)]
#[cfg_attr(test, derive(Debug))]
pub(crate) struct Port(String);

impl Default for Port {
    fn default() -> Self {
        Port::new(22).unwrap()
    }
}

impl AsRef<str> for Port {
    fn as_ref(&self) -> &str {
        self.0.as_ref()
    }
}

impl Port {
    pub(crate) fn new(port: u16) -> Result<Self> {
        if port == 0 {
            return Err(Error::InvalidPort { port });
        }
        Ok(Port(port.to_string()))
    }
}
