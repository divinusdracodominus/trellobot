#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Error)]
#[error(display = "bot error occured")]
pub enum BotError {
    #[error(display = "null data")]
    VoidData,
    #[error(display = "bot token isn't set")]
    MissingToken,
    #[error(display = "bot key isn't set")]
    MissingKey,
}
