//! The core HTTP client for the Moltbook API.
//!
//! This module provides the `MoltbookClient` which handles authentication headers,
//! rate limit parsing, CAPTCHA detection, and JSON serialization/deserialization
//! for all API interactions.

use crate::api::error::ApiError;
use mime_guess::from_path;
use reqwest::Client;
use serde::{Serialize, de::DeserializeOwned};
use serde_json::Value;
use std::path::PathBuf;

/// The base URL for the Moltbook API.
const API_BASE: &str = "https://www.moltbook.com/api/v1";

/// A thread-safe, asynchronous client for the Moltbook API.
///
/// Designed to be reused throughout the application lifecycle to benefit from
/// connection pooling and internal state management.
pub struct MoltbookClient {
    client: Client,
    api_key: String,
    debug: bool,
}

impl MoltbookClient {
    /// Creates a new `MoltbookClient` instance.
    ///
    /// # Arguments
    ///
    /// * `api_key` - The API key for authentication.
    /// * `debug` - If true, logs all requests and responses to stderr.
    pub fn new(api_key: String, debug: bool) -> Self {
        Self {
            client: Client::new(),
            api_key,
            debug,
        }
    }

    /// Performs a GET request to the specified endpoint.
    ///
    /// # Errors
    ///
    /// Returns `ApiError` if the network fails, the API returns an error, or parsing fails.
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

    /// Performs a POST request with a JSON body.
    ///
    /// # Errors
    ///
    /// Returns `ApiError` if the network fails, the API returns an error, or serialization/parsing fails.
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

    /// Uploads a file using multipart/form-data.
    ///
    /// Typically used for avatar updates.
    ///
    /// # Errors
    ///
    /// Returns `ApiError` if the file cannot be read or the upload fails.
    pub async fn post_file<T: DeserializeOwned>(
        &self,
        endpoint: &str,
        file_path: PathBuf,
    ) -> Result<T, ApiError> {
        let url = format!("{}{}", API_BASE, endpoint);

        let file_name = file_path
            .file_name()
            .unwrap_or_default()
            .to_string_lossy()
            .to_string();

        let file_contents = std::fs::read(&file_path).map_err(ApiError::IoError)?;

        let mime_type = from_path(&file_path).first_or_octet_stream();
        let part = reqwest::multipart::Part::bytes(file_contents)
            .file_name(file_name)
            .mime_str(mime_type.as_ref())?;
        let form = reqwest::multipart::Form::new().part("file", part);

        if self.debug {
            eprintln!("POST (File) {}", url);
            eprintln!("File: {:?}", file_path);
        }

        let response = self
            .client
            .post(&url)
            .header("Authorization", format!("Bearer {}", self.api_key))
            .multipart(form)
            .send()
            .await?;

        self.handle_response(response).await
    }

    /// Performs a PATCH request with a JSON body.
    pub async fn patch<T: DeserializeOwned>(
        &self,
        endpoint: &str,
        body: &impl Serialize,
    ) -> Result<T, ApiError> {
        let url = format!("{}{}", API_BASE, endpoint);

        if self.debug {
            eprintln!("PATCH {}", url);
            eprintln!(
                "Body: {}",
                serde_json::to_string_pretty(&body).unwrap_or_default()
            );
        }

        let response = self
            .client
            .patch(&url)
            .header("Authorization", format!("Bearer {}", self.api_key))
            .header("Content-Type", "application/json")
            .json(body)
            .send()
            .await?;

        self.handle_response(response).await
    }

    /// Performs a DELETE request to the specified endpoint.
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

    /// Unified handler for API responses, managing errors and parsing.
    ///
    /// This method specifically handles:
    /// - HTTP 429 Rate Limiting with retry extraction.
    /// - CAPTCHA required status.
    /// - Flattened API errors (error message + hint).
    /// - General JSON deserialization.
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

        if !status.is_success() {
            if let Ok(json) = serde_json::from_str::<Value>(&text) {
                let error = json
                    .get("error")
                    .and_then(|v| v.as_str())
                    .unwrap_or("Unknown error");

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
