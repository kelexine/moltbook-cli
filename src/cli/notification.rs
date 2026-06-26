// author: kelexine <https://github.com/kelexine>

use crate::api::client::MoltbookClient;
use crate::api::error::ApiError;
use crate::api::types::NotificationsResponse;
use crate::display;
use serde_json::json;

pub async fn list(
    client: &MoltbookClient,
    limit: u64,
    cursor: Option<String>,
    unread: bool,
) -> Result<(), ApiError> {
    let mut query = format!("/notifications?limit={}", limit);
    if let Some(c) = &cursor {
        query.push_str(&format!("&cursor={}", c));
    }
    if unread {
        query.push_str("&filter=unread");
    }

    let response: NotificationsResponse = client.get(&query).await?;
    display::display_notifications(&response);
    Ok(())
}

pub async fn read_by_post(client: &MoltbookClient, post_id: &str) -> Result<(), ApiError> {
    let result: serde_json::Value = client
        .post(
            &format!("/notifications/read-by-post/{}", post_id),
            &json!({}),
        )
        .await?;

    if result["success"].as_bool().unwrap_or(false) {
        display::success(&format!("Notifications marked read for post {}", post_id));
    } else {
        let error = result["error"].as_str().unwrap_or("Unknown error");
        display::error(&format!("Failed to mark read: {}", error));
    }
    Ok(())
}

pub async fn read_all(client: &MoltbookClient) -> Result<(), ApiError> {
    let result: serde_json::Value =
        client.post("/notifications/read-all", &json!({})).await?;

    if result["success"].as_bool().unwrap_or(false) {
        display::success("All notifications marked as read");
    } else {
        let error = result["error"].as_str().unwrap_or("Unknown error");
        display::error(&format!("Failed to mark all read: {}", error));
    }
    Ok(())
}
