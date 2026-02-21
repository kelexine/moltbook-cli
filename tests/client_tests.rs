use moltbook_cli::api::client::MoltbookClient;
use moltbook_cli::api::error::ApiError;
use serde_json::json;
use wiremock::matchers::{header, method, path};
use wiremock::{Mock, MockServer, ResponseTemplate};

#[tokio::test]
async fn test_successful_get_request() {
    let mock_server = MockServer::start().await;

    let client =
        MoltbookClient::new("test-key".to_string(), false).with_base_url(mock_server.uri());

    Mock::given(method("GET"))
        .and(path("/agents/me"))
        .and(header("Authorization", "Bearer test-key"))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!({
            "success": true,
            "agent": {
                "id": "123",
                "name": "TestBot"
            }
        })))
        .mount(&mock_server)
        .await;

    let response: serde_json::Value = client.get("/agents/me").await.unwrap();
    assert_eq!(response["success"], true);
    assert_eq!(response["agent"]["name"], "TestBot");
}

#[tokio::test]
async fn test_rate_limit_handling_minutes() {
    let mock_server = MockServer::start().await;
    let client =
        MoltbookClient::new("test-key".to_string(), false).with_base_url(mock_server.uri());

    Mock::given(method("POST"))
        .and(path("/posts"))
        .respond_with(ResponseTemplate::new(429).set_body_json(json!({
            "error": "rate_limit_exceeded",
            "retry_after_minutes": 30
        })))
        .mount(&mock_server)
        .await;

    let result: Result<serde_json::Value, ApiError> = client.post("/posts", &json!({})).await;

    match result {
        Err(ApiError::RateLimited(msg)) => assert_eq!(msg, "30 minutes"),
        _ => panic!("Expected RateLimited error"),
    }
}

#[tokio::test]
async fn test_rate_limit_handling_seconds() {
    let mock_server = MockServer::start().await;
    let client =
        MoltbookClient::new("test-key".to_string(), false).with_base_url(mock_server.uri());

    Mock::given(method("POST"))
        .and(path("/posts/1/comments"))
        .respond_with(ResponseTemplate::new(429).set_body_json(json!({
            "error": "rate_limit_exceeded",
            "retry_after_seconds": 15
        })))
        .mount(&mock_server)
        .await;

    let result: Result<serde_json::Value, ApiError> =
        client.post("/posts/1/comments", &json!({})).await;

    match result {
        Err(ApiError::RateLimited(msg)) => assert_eq!(msg, "15 seconds"),
        _ => panic!("Expected RateLimited error"),
    }
}

#[tokio::test]
async fn test_moltbook_error_with_hint() {
    let mock_server = MockServer::start().await;
    let client =
        MoltbookClient::new("test-key".to_string(), false).with_base_url(mock_server.uri());

    Mock::given(method("GET"))
        .and(path("/agents/unknown"))
        .respond_with(ResponseTemplate::new(404).set_body_json(json!({
            "success": false,
            "error": "Agent not found",
            "hint": "Check the spelling of the name"
        })))
        .mount(&mock_server)
        .await;

    let result: Result<serde_json::Value, ApiError> = client.get("/agents/unknown").await;

    match result {
        Err(ApiError::MoltbookError(err, hint)) => {
            assert_eq!(err, "Agent not found");
            assert_eq!(hint, "Check the spelling of the name");
        }
        _ => panic!("Expected MoltbookError"),
    }
}

#[tokio::test]
async fn test_captcha_required_error() {
    let mock_server = MockServer::start().await;
    let client =
        MoltbookClient::new("test-key".to_string(), false).with_base_url(mock_server.uri());

    Mock::given(method("POST"))
        .and(path("/verify"))
        .respond_with(ResponseTemplate::new(403).set_body_json(json!({
            "success": false,
            "error": "captcha_required",
            "token": "tok_abc123"
        })))
        .mount(&mock_server)
        .await;

    let result: Result<serde_json::Value, ApiError> = client.post("/verify", &json!({})).await;

    match result {
        Err(ApiError::CaptchaRequired(token)) => assert_eq!(token, "tok_abc123"),
        _ => panic!("Expected CaptchaRequired error, got {:?}", result),
    }
}
