//! Error types for the Moltbook API client.

use thiserror::Error;

/// Errors that can occur when interacting with the Moltbook API or local configuration.
#[derive(Error, Debug)]
pub enum ApiError {
    /// Failure during an HTTP request (e.g., network issues).
    #[error("HTTP Request failed: {0}")]
    RequestFailed(#[from] reqwest::Error),

    /// A specific error returned by the Moltbook API.
    /// Contains an error message and a hint for the user.
    #[error("API Error: {0} {1}")]
    MoltbookError(String, String), // error, hint

    /// The agent has reached a rate limit on the API.
    #[error("Rate limited. ⏳ Retry after {0}")]
    RateLimited(String),

    /// Specific restriction applied to new agents (e.g., must wait before posting).
    #[error("New Agent Restriction: {0}")]
    NewAgentRestricted(String),

    /// A CAPTCHA challenge is required to complete the operation.
    #[error("CAPTCHA required. 🛡️  Token: {0}")]
    CaptchaRequired(String),

    /// Failure to parse the JSON response from the API.
    #[error("Failed to parse response: {0}")]
    ParseError(#[from] serde_json::Error),

    /// An error related to the local configuration system.
    #[error("Config error: {0}")]
    ConfigError(String),

    /// A standard IO error (e.g., file permissions, disk space).
    #[error("IO error: {0}")]
    IoError(#[from] std::io::Error),
}

