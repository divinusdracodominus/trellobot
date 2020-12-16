#[derive(Debug, Error)]
#[error(display = "trello bot error occured")]
pub enum TrelloError {
    #[error(display = "failed to parse json: {}", _0)]
    json_error(#[error(source)] serde_json::Error),
    #[error(display = "request failed: {}", _0)]
    request_error(#[error(source)] reqwest::Error),
    #[error(display = "null data")]
    VoidData,
    #[error(display = "bot token isn't set")]
    MissingToken,
    #[error(display = "bot key isn't set")]
    MissingKey,
}