mod card;
pub use card::*;
mod users;
use users::*;
mod board;
pub use board::*;
pub mod lists;
pub use lists::*;

pub use crate::trellobot::Cards;
use reqwest::blocking::{Client, Request};
use reqwest::{header::HeaderValue, Method, Url};
use std::collections::HashMap;
use std::default::Default;
use std::error::Error;

use crate::bot::GenericBot;
pub mod error;
pub use error::*;

pub trait TrelloItem {
    type Error: std::error::Error;
    fn get(id: &str, bot: &mut TrelloBot) -> Result<Self, Self::Error>
    where Self: Sized;
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Rest<'a> {
    root: &'a str,
    table: HashMap<&'a str, &'a str>,
}

impl<'a> Rest<'a> {
    pub fn from_hashmap(root: &'a str, table: HashMap<&'a str, &'a str>) -> Self {
        Self { root, table }
    }
    pub fn build_request(&self) -> Request {
        let mut request_uri = String::new();
        request_uri.push_str(&self.root);
        request_uri.push('?');
        for (k, v) in self.table.iter() {
            request_uri.push_str(&format!("{}={}&", k, v));
        }
        request_uri.pop();
        let mut request = Request::new(Method::GET, Url::parse(&request_uri).unwrap());
        request.headers_mut().insert(
            "accept",
            HeaderValue::from_bytes(&String::from("application/json").into_bytes()).unwrap(),
        );
        return request;
    }
    pub fn build_post(&self) -> Request {
        let mut request_uri = String::new();
        request_uri.push_str(&self.root);
        request_uri.push('?');
        for (k, v) in self.table.iter() {
            request_uri.push_str(&format!("{}={}&", k, v));
        }
        request_uri.pop();
        let mut request = Request::new(Method::POST, Url::parse(&request_uri).unwrap());
        request.headers_mut().insert(
            "accept",
            HeaderValue::from_bytes(&String::from("application/json").into_bytes()).unwrap(),
        );
        return request;
    }
    pub fn new(root: &'a str) -> Self {
        Self {
            root,
            table: HashMap::new(),
        }
    }
    pub fn insert(&mut self, key: &'a str, value: &'a str) {
        self.table.insert(key, value);
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
    /*pub fn get_card(&mut self, id: &str) -> Result<Card, Box<dyn Error>> {
        let root = format!("https://api.trello.com/1/cards/{}/", id);
        Ok(serde_json::from_str(&self.get_item(&root)?)?)
    }
    pub fn get_board(&mut self, id: &str) -> Result<String, Box<dyn Error>> {
        let root = format!("https://api.trello.com/1/boards/{}", id);
        Ok(self.get_item(&root)?)
    }

    /// # Arguments
    /// id: the id of the board to search
    pub fn get_cards(&mut self, id: &str) -> Result<Cards, Box<dyn Error>> {
        let root = format!("https://api.trello.com/1/boards/{}/cards", id);
        Ok(serde_json::from_str(&self.get_item(&root)?)?)
    }
    /// for now this creates a virtually empty card
    pub fn create_card(&mut self, list: &str) -> Result<String, Box<dyn Error>> {
        let root = format!("https://api.trello.com/1/cards");
        let mut rest = Rest::new(&root);
        rest.insert("key", self.data.get_key()?);
        rest.insert("token", self.data.get_token()?);
        rest.insert("idList", list);
        let request = rest.build_post();
        Ok(self.client.execute(request)?.text()?)
    }*/
    fn get_item(&mut self, root: &str) -> Result<String, TrelloError> {
        let mut rest = Rest::new(&root);
        rest.insert("key", self.data.get_key()?);
        rest.insert("token", self.data.get_token()?);
        let request = rest.build_request();
        Ok(self.client.execute(request)?.text()?)
    }
    fn exec(&mut self, request: Request) -> Result<String, TrelloError>{
        Ok(self.client.execute(request)?.text()?)
    }
}

impl From<GenericBot> for TrelloBot {
    fn from(bot: GenericBot) -> Self {
        Self::from_data(bot)
    }
}
