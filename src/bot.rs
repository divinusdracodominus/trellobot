use crate::trellobot::TrelloError;
use std::net::IpAddr;
use std::fmt;
/*#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum EncryptionState {
    Encrypted,
    Unencrypted,
}*/
/// tracks what host this instance of trellobot is running on
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

/// the purpose of this structure is to initialize data held by trellobot
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
    /// returns an error if the key is a None value
    pub fn get_key(&self) -> Result<&str, TrelloError> {
        match self.api_key.as_ref() {
            Some(key) => Ok(key),
            None => Err(TrelloError::MissingKey),
        }
    }
    /// returns an error of the token is a None value
    pub fn get_token(&self) -> Result<&str, TrelloError> {
        match self.token.as_ref() {
            Some(token) => Ok(token),
            None => Err(TrelloError::MissingKey),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Date{
    day: u8,
    month: u8,
    year: u16,
}

impl fmt::Display for Date {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}/{}/{}", self.day, self.month, self.year)
    }
}