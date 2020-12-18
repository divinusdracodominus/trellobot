use crate::{Cards, Member, Lists, SimpleCard, TrelloBot, TrelloError};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum PermLevel {
    public,
    private,
}

impl Default for PermLevel {
    fn default() -> Self {
        Self::private
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct CheckList {
    name: String,
    id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ScaledImage {
    width: u32,
    height: u32,
    url: String,
}

#[derive(Serialize, Deserialize, Debug, Clone, Default, PartialEq, Eq)]
pub struct limits {
    status: Option<String>,
    disableAt: Option<i32>,
    warnAt: Option<i32>,
}

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct labelNames {
    pub yellow: String,
    pub purple: String,
    pub blue: String,
    pub red: String,
    pub green: String,
    pub orange: String,
    pub black: String,
    pub sky: String,
    pub pink: String,
    pub lime: String,
}

/// this is essentially the same as BoardConfig, but is in this form for serde purposes
/// kind of inconsistent on the part of trello, as most fields are the same
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct prefs {
    pub permissionLevel: Option<PermLevel>,
    pub hideVotes: Option<bool>,
    /// should make this an enum
    pub voting: Option<String>,
    pub comments: Option<String>,
    /// also should be enum
    pub invitations: Option<String>,
    pub selfJoin: Option<bool>,
    pub cardCovers: Option<bool>,
    pub isTemplate: Option<bool>,
    /// another enum
    pub cardAging: Option<String>,
    pub calenderFeedEnabled: Option<bool>,
    pub background: Option<String>,
    pub backgroundImage: Option<String>,
    pub backgroundImageScaled: Vec<ScaledImage>,
    pub backgroundTile: bool,
    /// enum
    pub backgroundBrightness: String,
    /// custom deserialize should be created, as they are represented as #ffffff
    pub backgroundBottomColor: String,
    pub backgroundTopColor: String,
    pub canBePublic: bool,
    pub canBeEnterprise: bool,
    pub canBeOrg: bool,
    pub canBePrivate: bool,
    pub canInvite: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct SimpleBoard {
    /// same as board shortUrl used to fetch info on boards.
    pub id: String,
    pub name: String,
    pub desc: String,
    pub lists: Vec<CheckList>,
    pub cards: Vec<SimpleCard>,
    pub members: Vec<String>,
}

/// tempted to for the sake of efficiency make it a heap allocated type
/// using box might help
/// same with cards
/// when creating cards, the only field that is required is name
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Board {
    pub id: String,
    pub desc: Option<String>,
    pub descData: Option<String>,
    pub closed: Option<bool>,
    /// this is tracked when creating a card based on the key passed into the bot
    pub idMemberCreator: Option<String>,
    pub idOrganization: Option<String>,
    pub pinned: Option<bool>,
    pub url: Option<String>,
    pub shortUrl: Option<String>,
    pub prefs: prefs,
    pub labelNames: labelNames,
    pub limits: Option<limits>,
    pub starred: Option<bool>,
    pub memberships: Option<String>,
    /// this is the id used as a parameter to get requests that interact with the trello servers
    /// problem is that trello doesn't always return in when getting a board, I guess because it is needed to 
    /// submit the get request
    pub shortLink: Option<String>,
    pub subscribed: Option<bool>,
    pub powerUps: Option<String>,
    pub dateLastActive: Option<String>,
    pub dateLastView: Option<String>,
    pub idTags: Option<String>,
    pub datePluginDisable: Option<String>,
    pub creationMethod: Option<String>,
    pub ixUpdate: Option<i32>,
    pub templateGallery: Option<String>,
    pub enterpriseOwned: Option<String>,
}

impl Board {
    pub fn get_cards(&self, bot: &mut TrelloBot) -> Result<Cards, TrelloError> {
        let root = format!("https://api.trello.com/1/boards/{}/cards", self.shortLink.as_ref().unwrap());
        Ok(serde_json::from_str(&bot.get_item(&root)?)?)
    }
    pub fn get_lists(&self, bot: &mut TrelloBot) -> Result<Lists, TrelloError> {
        let root = format!("https://api.trello.com/1/boards/{}/lists", self.shortLink.as_ref().unwrap());
        Ok(serde_json::from_str(&bot.get_item(&root)?)?)
    }
    pub fn create(config: BoardConfig, bot: &mut TrelloBot) -> Result<String, TrelloError> {
        bot.create_object("https://api.trello.com/1/boards", &config.into_hashmap())
    }
    pub fn delete(&self, bot: &mut TrelloBot) -> Result<String, TrelloError> {
        bot.delete_item(&format!("https://api.trello.com/1/boards/{}", self.id))
    }
    pub fn get_members(&self, bot: &mut TrelloBot) -> Result<String, TrelloError>{
        let root = format!("https://api.trello.com/1/boards/{}/cards", self.shortLink.as_ref().unwrap());
        bot.get_item(&root)
    }
}

/// used to create a board, since the paramters to create are different then what you get
/// from an existing board, (note that a lot of these doc comments are taken from the trello docs)
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct BoardConfig {
    pub name: String,
    pub defaultLabels: bool,
    pub defaultLists: bool,
    pub desc: Option<String>,
    pub idOrganization: Option<String>,
    /// used to copy boards
    pub idBoardSource: Option<String>,
    /// if cards should be kept pass in cards (is what the trello docs say)
    /// valid values cards, none, default: none
    pub keepFromSource: String,
    /// The Power-Ups that should be enabled on the new board. One of: all, calendar, cardAging, recap, voting.
    pub powerUps: String,
    /// values are org, public, private, default is private
    pub prefs_permissionLevel: String,
    /// who can vote, values are disabled, members, observers, org, public, default disabled
    pub prefs_voting: String,
    /// who can comment on the board values: disabled, members, observers, org, public, default: members
    pub prefs_comments: String,
    /// who can invite new people to the board, values: admin, members, default: members
    pub prefs_invitations: String,
    /// can people join themselves default: true
    pub prefs_selfjoin: bool,
    /// default: true,
    pub prefs_cardCovers: bool,
    /// id of custom background or one of: blue, orange, green, red, purple, pink, lime, sky, grey. default: bool
    pub prefs_background: String,
    /// type of card aging, values: pirate, regular. default: regular.
    pub prefs_cardAging: String,
}

impl BoardConfig {
    pub fn into_hashmap(self) -> HashMap<&'static str, String> {
        let mut map = HashMap::new();
        map.insert("name", self.name);
        map.insert("defaultLabels", format!("{}", self.defaultLabels));
        map.insert("defaultLists", format!("{}", self.defaultLists));
        if let Some(desc) = self.desc {
            map.insert("desc", desc);
        }
        if let Some(org) = self.idOrganization {
            map.insert("idOrganization", org);
        }
        if let Some(source) = self.idBoardSource {
            map.insert("idBoardSource", source);
        }
        map.insert("keepFromSource", self.keepFromSource);
        map.insert("powerUps", self.powerUps);
        map.insert("prefs_permissionLevel", self.prefs_permissionLevel);
        map.insert("pref_voting", self.prefs_voting);
        map.insert("prefs_comments", self.prefs_comments);
        map.insert("prefs_invitations", self.prefs_invitations);
        map.insert("prefs_selfjoin", format!("{}", self.prefs_selfjoin));
        map.insert("prefs_cardCovers", format!("{}", self.prefs_cardCovers));
        map.insert("prefs_background", self.prefs_background);
        map.insert("prefs_cardAging", self.prefs_cardAging);
        map
    }
    pub fn name(mut self, name: &str) -> Self {
        self.name = name.to_string();
        self
    }
}

impl Default for BoardConfig {
    fn default() -> BoardConfig {
        Self {
            name: String::from("should be changed"),
            defaultLabels: true,
            defaultLists: true,
            desc: None,
            idOrganization: None,
            idBoardSource: None,
            keepFromSource: String::from("none"),
            powerUps: String::from("all"),
            prefs_permissionLevel: String::from("private"),
            prefs_voting: String::from("disabled"),
            prefs_comments: String::from("members"),
            prefs_invitations: String::from("members"),
            prefs_selfjoin: true,
            prefs_cardCovers: true,
            prefs_background: String::from("blue"),
            prefs_cardAging: String::from("regular"),
        }
    }
}
