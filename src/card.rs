use crate::{color, TrelloBot, TrelloError, TrelloItem};
use std::collections::HashMap;
/// used to create a label
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct Label {
    /// optional because creating a board doesn't require an ID
    pub id: Option<String>,
    pub name: String,
    pub color: color,
    pub idBoard: String,
}
impl Label {
    pub fn create(self, bot: &mut TrelloBot) -> Result<String, TrelloError> {
        bot.create_object("https://api.trello.com/1/labels", &self.into_hashmap())
    }
    pub fn delete(&self, bot: &mut TrelloBot) -> Result<String, TrelloError> {
        bot.delete_item(&format!("https://api.trello.com/1/labels/{}", self.id.clone().unwrap()))
    }
    pub fn into_hashmap(self) -> HashMap<&'static str, String> {
        let mut map = HashMap::new();
        if let Some(id) = self.id {
            map.insert("id", id);
        }
        map.insert("name", self.name);
        map.insert("color", format!("{:#?}", self.color));
        map.insert("idBoard", self.idBoard);
        map
    }
    /// # Arguments
    /// idBoard is the id of the board to create the label on, but it isn't the shortLink
    /// field that is used to operate on boards, rather it is the actual id field
    pub fn new(name: &str, color: color, idBoard: &str) -> Self {
        Self {
            name: name.to_string(),
            color,
            idBoard: idBoard.to_string(),
            id: None,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord)]
pub struct SimpleCard {
    /// same as card's shortUrl
    pub id: String,
    pub name: String,
    pub due: Option<String>,
    pub desc: Option<String>,
}
impl SimpleCard {
    pub fn ascii_fmt(&self) -> String {
        if let Some(desc) = &self.desc {
            if let Some(due) = &self.due {
                if !desc.is_empty() {
                    return format!(
                        "   + ----------------------
                     + id: {}  
                     + name: {} 
                     + description: {} 
                     + due: {} \n",
                        self.id, self.name, desc, due
                    );
                    return format!(
                        "   + ----------------------
                     + id: {}  
                     + name: {}
                     + due: {} \n",
                        self.id, self.name, due
                    );
                }
                if !desc.is_empty() {
                    return format!(
                        "   + ----------------------
                    + id: {}  
                    + name: {} 
                    + description: {} \n",
                        self.id, self.name, desc
                    );
                }
            }
        }
        format!(
            "+ ---------------------- 
             + id: {} 
             + name: {} \n",
            self.id, self.name
        )
    }
}

impl From<&Card> for SimpleCard {
    fn from(card: &Card) -> SimpleCard {
        SimpleCard {
            id: card.shortLink.clone(),
            name: card.name.clone(),
            desc: card.desc.clone(),
            due: card.due.clone(),
        }
    }
}

/*#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum idAttachmentCover {
    null,
    value(String),
}*/

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct emoji {}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum object {
    emoji(Option<emoji>),
    //idAttachmentCover(idAttachmentCover),
}
impl Default for object {
    fn default() -> Self {
        Self::emoji(None)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct cover {
    idAttachment: Option<String>,
    color: Option<color>,
    idUploadedBackground: Option<bool>,
    size: Option<String>,
    brightness: Option<String>,
    isTemplate: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct badges {
    location: Option<bool>,
    votes: Option<i32>,
    viewingMemberVotes: Option<bool>,
    subscription: Option<bool>,
    fogbugz: Option<String>,
    checkItems: Option<i32>,
    checkItemsChecked: Option<i32>,
    checkItemsEarliestDue: Option<String>,
    comments: Option<i32>,
    attachments: Option<i32>,
    description: Option<bool>,
    due: Option<String>,
    dueComplete: Option<bool>,
    start: Option<String>,
}

/// some of the object types may cause parsing to fail
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct Card {
    /// made optional for the purpose of creating cards
    pub id: String,
    pub address: Option<String>,
    pub checkitemStates: Option<Vec<String>>,
    pub closed: bool,
    pub coordinates: Option<String>,
    pub creationMethod: Option<String>,
    pub dateLastActive: Option<String>,
    pub desc: Option<String>,
    pub descData: Option<object>,

    pub due: Option<String>,
    pub dueReminder: Option<String>,
    pub email: Option<String>,
    pub idBoard: Option<String>,
    /// this might fail, the formatting is weird
    pub idCheckLists: Option<Vec<String>>,
    pub badges: Option<badges>,
    pub idLabels: Option<Vec<String>>,
    pub idList: String,
    pub idMembers: Option<Vec<String>>,
    /// used when creating a card to choose where to copy from
    pub idCardSource: Option<String>,
    pub idMembersVoted: Option<Vec<String>>,
    pub idShort: u32,
    pub idAttachmentCover: Option<String>,
    pub labels: Vec<Label>,
    pub keepFromSource: Option<String>,
    pub fileSource: Option<String>,
    pub dueComplete: bool,
    pub urlSource: Option<String>,
    pub locationName: Option<String>,
    pub manualCoverAttachment: Option<bool>,
    pub name: String,
    pub pos: Option<u32>,
    pub shortLink: String,
    pub shortUrl: String,
    pub subscribed: Option<bool>,
    pub url: String,
    pub cover: Option<cover>,
}

impl TrelloItem for Card {
    type Error = TrelloError;
    fn get(id: &str, bot: &mut TrelloBot) -> Result<Self, Self::Error> {
        let root = format!("https://api.trello.com/1/cards/{}/", id);
        Ok(serde_json::from_str(&bot.get_item(&root)?)?)
    }
}

impl Card {
    pub fn create(self, bot: &mut TrelloBot) -> Result<Self, TrelloError> {
        Ok(serde_json::from_str(&bot.create_object(
            "https://api.trello.com/1/cards",
            &self.into_hashmap(),
        )?)?)
    }
    pub fn basic(
        name: &str,
        desc: &str,
        due: &str,
        list_id: &str,
        members: Option<Vec<String>>,
    ) -> Self {
        Self {
            name: name.to_string(),
            desc: Some(desc.to_string()),
            pos: None,
            due: Some(due.to_string()),
            dueComplete: false,
            idList: list_id.to_string(),
            idMembers: members,
            ..Default::default()
        }
    }
    /// trello docs doesn't indicate wether or not idAttachmentCover, cover, or subscribed can be applied when
    /// creating a card, so have fun with that
    pub fn into_hashmap(self) -> HashMap<&'static str, String>{
        let mut map = HashMap::new();
        map.insert("name", self.name);
        if let Some(desc) = self.desc {
            map.insert("desc", desc);
        }
        if let Some(pos) = self.pos {
            map.insert("pos", pos.to_string());
        }
        if let Some(due) = self.due {
            map.insert("due", due);
        }
        map.insert("dueComplete", format!("{}", self.dueComplete));
        map.insert("idList", self.idList);
        if let Some(members) = self.idMembers {
            if !members.is_empty() { map.insert("idMembers", serde_json::to_string(&members).unwrap()); }
        }
        if let Some(labels) = self.idLabels.as_ref() {
            if !labels.is_empty()  { map.insert("idLabels", serde_json::to_string(labels).unwrap()); }
        }
        if let Some(urlSource) = self.urlSource {
            map.insert("urlSource", urlSource);
        }
        if let Some(fileSource) = self.fileSource {
            map.insert("fileSource", fileSource);
        }
        if let Some(idCardSource) = self.idCardSource {
            map.insert("idCardSource", idCardSource);
        }
        if let Some(keep) = self.keepFromSource {
            map.insert("keepFromSource", keep);
        }
        if let Some(address) = self.address {
            map.insert("address", address);
        }
        if let Some(local) = self.locationName {
            map.insert("locationName", local);
        }
        if let Some(coordinates) = self.coordinates {
            map.insert("coordinates", coordinates);
        }
        if let Some(idAttachmentCover) = self.idAttachmentCover {
            map.insert("idAttachmentCover", idAttachmentCover);
        }
        if let Some(subscribed) = self.subscribed { 
            map.insert("subscribed", format!("{}", subscribed));
        }
        /*if let Some(cover) = self.cover {
            map.insert("cover", serde_json::to_string(&cover).unwrap());
        }*/
        map
    }
    pub fn update(self, bot: &mut TrelloBot) -> Result<String, TrelloError>{
        Ok(bot.create_object(
            &format!("https://api.trello.com/1/cards/{}", self.shortLink),
            &self.into_hashmap(),
        )?)
    }
    pub fn delete(&self, bot: &mut TrelloBot) -> Result<String, TrelloError> {
        bot.delete_item(&format!("https://api.trello.com/1/cards/{}", self.id))
    }
}

pub type Cards = Vec<Card>;

pub trait CardList {
    fn build_id_map(&self) -> HashMap<String, String>;
    fn simplify(&self) -> Vec<SimpleCard>;
}

impl CardList for Cards {
    fn build_id_map(&self) -> HashMap<String, String> {
        let mut map = HashMap::new();
        for card in self.iter() {
            map.insert(card.name.clone(), card.shortLink.clone());
        }
        map
    }
    fn simplify(&self) -> Vec<SimpleCard> {
        let mut outvec = Vec::with_capacity(self.capacity());
        for card in self.iter() {
            outvec.push(SimpleCard::from(card));
        }
        outvec
    }
}