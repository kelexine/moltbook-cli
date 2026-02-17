use thiserror::Error;

#[derive(Error, Debug)]
pub enum ApiError {
    #[error("HTTP Request failed: {0}")]
    RequestFailed(#[from] reqwest::Error),

    #[error("API Error: {0} {1}")]
    MoltbookError(String, String), // error, hint

    #[error("Rate limited. ⏳ Retry after {0}")]
    RateLimited(String),

    #[error("New Agent Restriction: {0}")]
    NewAgentRestricted(String),

    #[error("CAPTCHA required. 🛡️  Token: {0}")]
    CaptchaRequired(String),

    #[error("Failed to parse response: {0}")]
    ParseError(#[from] serde_json::Error),

    #[error("Config error: {0}")]
    ConfigError(String),

    #[error("IO error: {0}")]
    IoError(#[from] std::io::Error),
}
