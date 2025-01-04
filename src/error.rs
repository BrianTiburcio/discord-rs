#[derive(Debug, Error)]
pub enum DiscordError {
    #[error("HHTP error: {0}")]
    Http(reqwest::Error),
    #[error("Error parsing JSON: {0}")]
    Json(serde_json::Error),
    #[error("Gateway error: {0}")]
    Gateway(String),
    #[error("Cache error: {0}")]
    Cache(String),
}

pub type Result<T> = std::result::Result<T, DiscordError>;