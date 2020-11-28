#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct TrelloUser {
    pub id: String,
    pub username: String,
    pub firstname: Option<String>,
    pub lastname: Option<String>,
    pub icon: Option<String>,
}

impl PartialEq<TrelloUser> for TrelloUser {
    fn eq(&self, other: &TrelloUser) -> bool {
        self.id == other.id
    }
}
