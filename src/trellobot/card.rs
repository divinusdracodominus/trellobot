use std::collections::HashMap;

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
        Self::null
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
    pub id: Option<String>,
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
    pub idMembersVoted: Option<Vec<String>>,
    pub idShort: u32,
    pub idAttachmentCover: Option<String>,
    pub labels: Vec<String>,

    pub locationName: Option<String>,
    pub manualCoverAttachment: Option<bool>,
    pub name: String,
    pub pos: f32,
    pub shortLink: String,
    pub shortUrl: String,
    pub subscribed: bool,
    pub url: String,
    pub cover: cover,
}
impl Card {
    pub fn as_hashmap(&self) -> HashMap<&str, String> {
        let mut map = HashMap::new();
        match &self.id {
            Some(id) => {
                map.insert("id", id.to_owned());
            }
            None => (),
        }
        match &self.address {
            Some(address) => {
                map.insert("address", address.to_owned());
            }
            None => (),
        }
        map.insert(
            "checkitemStates",
            serde_json::to_string(&self.checkitemStates).unwrap(),
        );
        map.insert("closed", format!("{}", self.closed));
        match &self.coordinates {
            Some(coordinates) => {
                map.insert("coordinates", coordinates.to_owned());
            }
            None => (),
        }
        match &self.creationMethod {
            Some(creationMethod) => {
                map.insert("creationMethod", creationMethod.to_owned());
            }
            None => (),
        }

        map
    }
}

pub type Cards = Vec<Card>;

pub trait CardList {
    fn build_id_map(&self) -> HashMap<String, String>;
}

impl CardList for Cards {
    fn build_id_map(&self) -> HashMap<String, String> {
        let mut map = HashMap::new();
        for card in self.iter() {
            map.insert(card.name.clone(), card.shortLink.clone());
        }
        return map;
    }
}
