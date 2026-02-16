use crate::api::error::ApiError;
use reqwest::Client;
use serde::{Serialize, de::DeserializeOwned};
use serde_json::Value;

const API_BASE: &str = "https://www.moltbook.com/api/v1";

pub struct MoltbookClient {
    client: Client,
    api_key: String,
    debug: bool,
}

impl MoltbookClient {
    pub fn new(api_key: String, debug: bool) -> Self {
        Self {
            client: Client::new(),
            api_key,
            debug,
        }
    }

    pub async fn get<T: DeserializeOwned>(&self, endpoint: &str) -> Result<T, ApiError> {
        let url = format!("{}{}", API_BASE, endpoint);

        if self.debug {
            eprintln!("GET {}", url);
        }

        let response = self
            .client
            .get(&url)
            .header("Authorization", format!("Bearer {}", self.api_key))
            .send()
            .await?;

        self.handle_response(response).await
    }

    pub async fn post<T: DeserializeOwned>(
        &self,
        endpoint: &str,
        body: &impl Serialize,
    ) -> Result<T, ApiError> {
        let url = format!("{}{}", API_BASE, endpoint);

        if self.debug {
            eprintln!("POST {}", url);
            eprintln!(
                "Body: {}",
                serde_json::to_string_pretty(&body).unwrap_or_default()
            );
        }

        let response = self
            .client
            .post(&url)
            .header("Authorization", format!("Bearer {}", self.api_key))
            .header("Content-Type", "application/json")
            .json(body)
            .send()
            .await?;

        self.handle_response(response).await
    }

    pub async fn delete<T: DeserializeOwned>(&self, endpoint: &str) -> Result<T, ApiError> {
        let url = format!("{}{}", API_BASE, endpoint);

        if self.debug {
            eprintln!("DELETE {}", url);
        }

        let response = self
            .client
            .delete(&url)
            .header("Authorization", format!("Bearer {}", self.api_key))
            .send()
            .await?;

        self.handle_response(response).await
    }

    async fn handle_response<T: DeserializeOwned>(
        &self,
        response: reqwest::Response,
    ) -> Result<T, ApiError> {
        let status = response.status();
        let text = response.text().await?;

        if self.debug {
            eprintln!("Response Status: {}", status);
            eprintln!("Response Body: {}", text);
        }

        if status.as_u16() == 429 {
            // Try to parse rate limit info
            if let Ok(json) = serde_json::from_str::<Value>(&text) {
                if let Some(retry) = json.get("retry_after_minutes").and_then(|v| v.as_u64()) {
                    return Err(ApiError::RateLimited(format!("{} minutes", retry)));
                }
                if let Some(retry) = json.get("retry_after_seconds").and_then(|v| v.as_u64()) {
                    return Err(ApiError::RateLimited(format!("{} seconds", retry)));
                }
            }
            return Err(ApiError::RateLimited("Wait before retrying".to_string()));
        }

        // Handle generic errors
        if !status.is_success() {
            if let Ok(json) = serde_json::from_str::<Value>(&text) {
                let error = json
                    .get("error")
                    .and_then(|v| v.as_str())
                    .unwrap_or("Unknown error");

                // Handle Captcha
                if error == "captcha_required" {
                    let token = json
                        .get("token")
                        .and_then(|v| v.as_str())
                        .unwrap_or("unknown_token");
                    return Err(ApiError::CaptchaRequired(token.to_string()));
                }

                let hint = json.get("hint").and_then(|v| v.as_str()).unwrap_or("");
                return Err(ApiError::MoltbookError(error.to_string(), hint.to_string()));
            }
            return Err(ApiError::MoltbookError(format!("HTTP {}", status), text));
        }

        serde_json::from_str(&text).map_err(ApiError::ParseError)
    }
}
