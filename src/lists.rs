use crate::{Cards, TrelloBot, TrelloError};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct List {
    pub id: String,
    pub name: String,
    pub closed: bool,
    pub pos: String,
    pub softLimit: Option<String>,
    pub idBoard: String,
    pub subscribed: bool,
}
impl List {
    pub fn create(config: ListConfig, bot: &mut TrelloBot) -> Result<String, TrelloError> {
        bot.create_object("https://api.trello.com/1/lists", &config.into_hashmap())
    }
    /// id don't know if this will work docs doesn't include it as a feature
    pub fn delete(&self, bot: &mut TrelloBot) -> Result<String, TrelloError> {
        bot.delete_item(&format!("https://api.trello.com/1/lists/{}", self.id))
    }
    /// # Arguments
    /// id: shortLink of the list
    pub fn get_cards(id: &str, bot: &mut TrelloBot) -> Result<Cards, TrelloError> {
        Ok(serde_json::from_str(&bot.get_item(&format!(
            "https://api.trello.com/1/lists/{}/cards",
            id
        ))?)?)
    }
}

pub type Lists = Vec<List>;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct ListConfig {
    pub name: String,
    pub idBoard: String,
    pub pos: Option<String>,
    pub idListSource: Option<String>,
}

impl ListConfig {
    pub fn into_hashmap(self) -> HashMap<&'static str, String> {
        let mut map = HashMap::new();
        map.insert("name", self.name);
        map.insert("idBoard", self.idBoard);
        if let Some(pos) = self.pos {
            map.insert("pos", pos);
        }
        if let Some(source) = self.idListSource {
            map.insert("idListSource", source);
        }
        map
    }
}
