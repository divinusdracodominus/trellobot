use crate::error::BotError;
use std::net::IpAddr;

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum EncryptionState {
    Encrypted,
    Unencrypted,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct Host {
    pub name: String,
    pub ip: Option<IpAddr>,
    pub domain: Option<String>,
}

impl Default for Host {
    fn default() -> Self {
        Host {
            name: String::from("local"),
            ip: None,
            domain: None,
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GenericBot {
    name: String,
    api_key: Option<String>,
    token: Option<String>,
    host: Option<Host>,
}

impl GenericBot {
    pub fn new(
        name: String,
        api_key: Option<String>,
        token: Option<String>,
        host: Option<Host>,
    ) -> Self {
        Self {
            name,
            api_key,
            token,
            host,
        }
    }
    pub fn get_key(&self) -> Result<&str, BotError> {
        match self.api_key.as_ref() {
            Some(key) => Ok(key),
            None => Err(BotError::MissingKey),
        }
    }
    pub fn get_token(&self) -> Result<&str, BotError> {
        match self.token.as_ref() {
            Some(token) => Ok(token),
            None => Err(BotError::MissingKey),
        }
    }
}
