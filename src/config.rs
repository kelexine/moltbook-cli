//! Configuration management for the Moltbook CLI.
//!
//! This module handles loading and saving the agent's credentials (API key and agent name)
//! to a local configuration file, typically located at `~/.config/moltbook/credentials.json`.
//! It also enforces secure file permissions (0600) on Unix-like systems.

use crate::api::error::ApiError;
use dirs::home_dir;
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;

/// The default configuration directory relative to the user's home.
const CONFIG_DIR: &str = ".config/moltbook";
/// The filename for storing agent credentials.
const CONFIG_FILE: &str = "credentials.json";

/// Represents the CLI configuration and credentials.
#[derive(Serialize, Deserialize, Debug)]
pub struct Config {
    /// The Moltbook API key used for authentication.
    pub api_key: String,
    /// The name of the AI agent associated with this key.
    pub agent_name: String,
}

impl Config {
    /// Loads the configuration from the disk.
    ///
    /// # Errors
    ///
    /// Returns an `ApiError::ConfigError` if:
    /// - The configuration file does not exist.
    /// - The file cannot be read or parsed as valid JSON.
    pub fn load() -> Result<Self, ApiError> {
        let config_path = Self::get_config_path()?;

        if !config_path.exists() {
            return Err(ApiError::ConfigError(format!(
                "Config file not found at: {}\nPlease create it with your API key.",
                config_path.display()
            )));
        }

        let content = fs::read_to_string(&config_path)
            .map_err(|e| ApiError::ConfigError(format!("Failed to read config: {}", e)))?;

        let config: Config = serde_json::from_str(&content)
            .map_err(|e| ApiError::ConfigError(format!("Failed to parse config: {}", e)))?;

        Ok(config)
    }

    /// Resolves the path to the configuration file.
    ///
    /// Priority:
    /// 1. `MOLTBOOK_CONFIG_DIR` environment variable.
    /// 2. Default `~/.config/moltbook/credentials.json` path.
    fn get_config_path() -> Result<PathBuf, ApiError> {
        if let Ok(config_dir) = std::env::var("MOLTBOOK_CONFIG_DIR") {
            return Ok(PathBuf::from(config_dir).join(CONFIG_FILE));
        }

        let home = home_dir().ok_or_else(|| {
            ApiError::ConfigError("Could not determine home directory".to_string())
        })?;

        Ok(home.join(CONFIG_DIR).join(CONFIG_FILE))
    }

    /// Saves the current configuration to disk.
    ///
    /// On Unix systems, this method strictly enforces `0600` permissions
    /// to protect the API key from unauthorized local access.
    pub fn save(&self) -> Result<(), ApiError> {
        let config_path = Self::get_config_path()?;
        let config_dir = config_path.parent().unwrap();

        if !config_dir.exists() {
            fs::create_dir_all(config_dir).map_err(|e| {
                ApiError::ConfigError(format!("Failed to create config dir: {}", e))
            })?;
        }

        let content = serde_json::to_string_pretty(self)
            .map_err(|e| ApiError::ConfigError(format!("Failed to serialize config: {}", e)))?;

        fs::write(&config_path, content)
            .map_err(|e| ApiError::ConfigError(format!("Failed to write config: {}", e)))?;

        #[cfg(unix)]
        {
            use std::os::unix::fs::PermissionsExt;
            let mut perms = fs::metadata(&config_path)
                .map_err(|e| ApiError::ConfigError(format!("Failed to get metadata: {}", e)))?
                .permissions();
            perms.set_mode(0o600);
            fs::set_permissions(&config_path, perms)
                .map_err(|e| ApiError::ConfigError(format!("Failed to set permissions: {}", e)))?;
        }

        Ok(())
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_config_deserialization() {
        let json = r#"{"api_key": "test_key", "agent_name": "test_agent"}"#;
        let config: Config = serde_json::from_str(json).unwrap();
        assert_eq!(config.api_key, "test_key");
        assert_eq!(config.agent_name, "test_agent");
    }

    #[test]
    fn test_missing_fields() {
        let json = r#"{"api_key": "test_key"}"#;
        let result: Result<Config, _> = serde_json::from_str(json);
        assert!(result.is_err());
    }
}
