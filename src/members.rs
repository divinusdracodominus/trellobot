use crate::board::limits;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub struct Optin {
    pub optedin: bool,
    pub date: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct messagesDismissed {
    pub name: String,
    pub count: String,
    /// format: date
    pub lastDismissed: String,
    pub _id: String,
}
/// this type is currently in a failing state
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct Member {
    pub id: String,
    pub activityBlocked: Option<bool>,
    pub avatarHash: Option<String>,
    pub avatarUrl: Option<String>,
    pub bio: Option<String>,
    pub bioData: Option<String>,
    pub unconfirmed: bool,
    pub fullName: Option<String>,
    pub idEnterprise: Option<String>,
    pub idEnterprisesDeactivated: Option<Vec<String>>,
    pub idMemberReferrer: Option<String>,
    pub idPremOrgsAdmin: Option<Vec<String>>,
    pub initials: Option<String>,
    pub memberType: Option<String>,
    pub nonPublic: Option<String>,
    pub nonPublicAvailable: Option<bool>,
    pub products: Option<Vec<u32>>,
    pub url: Option<String>,
    pub username: Option<String>,
    /// valid values: disconnected
    pub status: Option<String>,
    pub aaEmail: Option<String>,
    pub aaEnroledDate: Option<String>,
    pub aaid: Option<String>,
    /// valid values: gravatar, upload
    pub avatarSource: Option<String>,
    pub email: Option<String>,
    pub gravatarHash: Option<String>,
    /// list of boards they are on
    pub idBoards: Option<Vec<String>>,
    pub idOrganizations: Option<Vec<String>>,
    pub idEnterpriseAdmin: Option<Vec<String>>,
    pub limits: Option<limits>,
    /// valid values: password, saml
    pub loginTypes: Option<Vec<String>>,
    pub marketingOptin: Option<Optin>,
    pub messagesDismissed: Option<messagesDismissed>,
    pub oneTimeMessagesDismissed: Option<Vec<String>>,
    pub prefs: Option<String>,
    pub trophies: Option<Vec<String>>,
    pub uploadedAvatarHash: Option<String>,
    pub uploadedAvatarUrl: Option<String>,
    pub premiumFeatures: Option<Vec<String>>,
    pub isAaMastered: Option<bool>,
    pub ixUpdate: Option<i32>,
    pub idBoardsPinned: Option<Vec<String>>
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct TokenPermission {
    pub idModel: String,
    /// one of board, member, organization, enterprise
    pub modelType: String,
    read: bool,
    write: bool,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
pub struct Token {
    pub id: String,
    pub identifier: String,
    pub idMember: String,
    /// fromat: Date
    pub dateCreated: String,
    /// format: Date
    pub dateExpires: String,
    pub permissions: Vec<TokenPermission>,
}