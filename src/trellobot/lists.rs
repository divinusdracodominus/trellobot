#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct List {
    pub id: String,
    pub name: String,
    pub closed: bool,
    pub pos: u32,
    pub softLimit: Option<String>,
    pub idBoard: String,
    pub subscribed: bool,
}

pub type Lists = Vec<List>;
