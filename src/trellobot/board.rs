use crate::trellobot::SimpleCard;

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

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ScaledImage {
    width: u32,
    height: u32,
    url: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct perBoard {
    status: String,
    disabledAt: u32,
    warnAt: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct attachments {
    perBoard: perBoard,
}

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct limits {
    attachments: attachments,
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

/// for anything marked with a comment about enums, I don't know all possible fields
/// so havn't made them into enums yet.
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

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord)]
pub struct SimpleBoard {
    /// same as board shortUrl used to fetch info on boards.
    pub id: String,
    pub name: String,
    pub desc: String,
    pub cards: Vec<SimpleCard>,
}

/// tempted to for the sake of efficiency make it a heap allocated type
/// using box might help
/// same with cards
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Board {
    pub id: String,
    pub desc: Option<String>,
    pub descData: Option<String>,
    pub closed: Option<bool>,
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
    pub shortLink: Option<String>,
    pub subscribed: Option<bool>,
    pub powerups: Option<String>,
    pub dateLastActive: Option<String>,
    pub dateLastView: Option<String>,
    pub idTags: Option<String>,
    pub datePluginDisable: Option<String>,
    pub creationMethod: Option<String>,
    pub ixUpdate: Option<i32>,
    pub templateGallery: Option<String>,
    pub enterpriseOwned: Option<String>,
}
