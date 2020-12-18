/*!
the reason that I didn't use trello-rs, is because it is quite limited in configurability
the same for the trello crate
 ```
use botnet::bot::{GenericBot};
use botnet::trellobot::{Card, CardConfig, TrelloBot};
use std::net::{IpAddr, Ipv4Addr};

fn main() {
    let bot: GenericBot = GenericBot::new(
        String::from("testbot"),
        Some(String::from("81e0add419e06edf04e46fac10f8e63d")),
        Some(String::from(
            "76b1f642507d11b398ef6d5187abf8f879e15df6a3465393cffffc123f954994",
        )),
        None,
    );

    let mut trellobot = TrelloBot::from(bot);
    let result = Card::create(
        CardConfig::basic("basic card", "this card is to test my trellobot api", "10/10/2021", "5bee387b3bdaed643b3a69f8", None),
        &mut trellobot,
    )
    .unwrap();
    println!("result: {}", result);
}

 ```
*/
#![allow(incomplete_features)]
#![feature(let_chains)]
#![allow(non_snake_case)]
#![allow(non_camel_case_types)]

#[macro_use]
extern crate err_derive;
#[macro_use]
extern crate serde_derive;
/// this module is used for parsing command line arguments that are passed to the bot from discord
/// will add support for running this application from the command line as well.
pub mod card;
pub use card::*;
pub mod board;
pub use board::*;
pub mod lists;
pub use lists::*;
pub mod error;
pub use error::*;
pub mod members;
pub use members::*;

use reqwest::blocking::{Client, Request};
use reqwest::{header::HeaderValue, Method, Url};
use std::collections::HashMap;

use crate::TrelloError;
use std::fmt;
use std::net::IpAddr;
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
pub struct Date {
    day: u8,
    month: u8,
    year: u16,
}

impl fmt::Display for Date {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}/{}/{}", self.day, self.month, self.year)
    }
}

pub trait TrelloItem {
    type Error: std::error::Error;
    //type Config;
    fn get(id: &str, bot: &mut TrelloBot) -> Result<Self, Self::Error>
    where
        Self: Sized;
    /*fn create(config: Self::Config, bot: &mut TrelloBot) -> Result<String, Self::Error>;*/
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Rest<'a> {
    url: &'a str,
    table: HashMap<&'a str, &'a str>,
}

impl<'a> Rest<'a> {
    pub fn from_hashmap(url: &'a str, table: &'a HashMap<&'a str, String>) -> Self {
        let mut map = HashMap::new();
        for (k, v) in table.iter() {
            map.insert(*k, v.as_str());
        }
        Self {url, table: map}
    }
    pub fn build_request(&self, method: Method) -> Request {
        let mut request_uri = String::new();
        request_uri.push_str(&self.url);
        request_uri.push('?');
        for (k, v) in self.table.iter() {
            request_uri.push_str(&format!("{}={}&", k, v));
        }
        request_uri.pop();
        let mut request = Request::new(method, Url::parse(&request_uri).unwrap());
        request.headers_mut().insert(
            "accept",
            HeaderValue::from_bytes(&String::from("application/json").into_bytes()).unwrap(),
        );
        request
    }
    pub fn new(url: &'a str) -> Self {
        Self {
            url,
            table: HashMap::new(),
        }
    }
    pub fn insert(&mut self, key: &'a str, value: &'a str) {
        self.table.insert(key, value);
    }
    pub fn param(&mut self, key: &'a str, value: &'a str) -> &mut Self {
        self.table.insert(key, value);
        self
    }
}

#[test]
fn request_test() {
    let mut rest: Rest = Rest::new("https://example.com");
    rest.insert("greeting", "hello");
    rest.insert("sender", "cardinal");
    println!("{:?}", rest.build_request());
}

#[derive(Debug, Clone)]
pub struct TrelloBot {
    data: GenericBot,
    client: Client,
}

impl TrelloBot {
    pub(crate) fn get_bot(&self) -> &GenericBot {
        &self.data
    }
    pub fn from_data(data: GenericBot) -> Self {
        let client = Client::new();
        Self { data, client }
    }
    /// this is its own function because I don't want to mandate that other
    /// parts of the crate neeed to use reqwest in case I want to change the library
    pub(crate) fn get_item(&mut self, url: &str) -> Result<String, TrelloError> {
        Ok(self
            .client
            .execute(
                Rest::new(&url)
                    .param("key", self.data.get_key()?)
                    .param("token", self.data.get_token()?)
                    .build_request(Method::GET),
            )?
            .text()?)
    }
    pub(crate) fn delete_item(&mut self, url: &str) -> Result<String, TrelloError>{
        Ok(self
            .client
            .execute(
                Rest::new(&url)
                    .param("key", self.data.get_key()?)
                    .param("token", self.data.get_token()?)
                    .build_request(Method::DELETE),
            )?
            .text()?)
    }
    fn exec(&mut self, request: Request) -> Result<String, TrelloError> {
        Ok(self.client.execute(request)?.text()?)
    }
    pub(crate) fn create_object(
        &mut self,
        url: &str,
        config: &HashMap<&'static str, String>,
    ) -> Result<String, TrelloError> {
        Ok(self
            .client
            .execute(
                Rest::from_hashmap(&url, config)
                    .param("key", self.data.get_key()?)
                    .param("token", self.data.get_token()?)
                    .build_request(Method::POST),
            )?
            .text()?)
    }
    pub(crate) fn update_item(&mut self, url: &str, config: &HashMap<&'static str, String>) -> Result<String, TrelloError> {
        Ok(self
            .client
            .execute(
                Rest::from_hashmap(&url, config)
                    .param("key", self.data.get_key()?)
                    .param("token", self.data.get_token()?)
                    .build_request(Method::PUT),
            )?
            .text()?)
    }
}

impl From<GenericBot> for TrelloBot {
    fn from(bot: GenericBot) -> Self {
        Self::from_data(bot)
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, Copy, PartialEq, Eq)]
pub enum color {
    yellow,
    purple,
    blue,
    red,
    green,
    orange,
    black,
    sky,
    pink,
    lime,
}
