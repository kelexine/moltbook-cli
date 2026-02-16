use crate::api::error::ApiError;
use dirs::home_dir;
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;

const CONFIG_DIR: &str = ".config/moltbook";
const CONFIG_FILE: &str = "credentials.json";

#[derive(Serialize, Deserialize, Debug)]
pub struct Config {
    pub api_key: String,
    pub agent_name: String,
}

impl Config {
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
    
    fn get_config_path() -> Result<PathBuf, ApiError> {
        let home = home_dir()
            .ok_or_else(|| ApiError::ConfigError("Could not determine home directory".to_string()))?;
        
        Ok(home.join(CONFIG_DIR).join(CONFIG_FILE))
    }

    pub fn save(&self) -> Result<(), ApiError> {
        let config_path = Self::get_config_path()?;
        let config_dir = config_path.parent().unwrap();
        
        if !config_dir.exists() {
            fs::create_dir_all(config_dir)
                .map_err(|e| ApiError::ConfigError(format!("Failed to create config dir: {}", e)))?;
        }
        
        let content = serde_json::to_string_pretty(self)
            .map_err(|e| ApiError::ConfigError(format!("Failed to serialize config: {}", e)))?;
            
        fs::write(&config_path, content)
            .map_err(|e| ApiError::ConfigError(format!("Failed to write config: {}", e)))?;
            
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
