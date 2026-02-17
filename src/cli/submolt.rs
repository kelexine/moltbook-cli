use crate::api::client::MoltbookClient;
use crate::api::error::ApiError;
use crate::api::types::{FeedResponse, Submolt};
use crate::display;
use colored::Colorize;
use serde_json::json;

pub async fn list_submolts(client: &MoltbookClient, sort: &str, limit: u64) -> Result<(), ApiError> {
    let response: serde_json::Value = client
        .get(&format!("/submolts?sort={}&limit={}", sort, limit))
        .await?;
    let submolts: Vec<Submolt> = if let Some(s) = response.get("submolts") {
        serde_json::from_value(s.clone())?
    } else {
        serde_json::from_value(response)?
    };
    println!("\n{} ({})", "Available Submolts".bright_green().bold(), sort);
    println!("{}", "=".repeat(60));
    for s in submolts {
        display::display_submolt(&s);
    }
    Ok(())
}

pub async fn view_submolt(
    client: &MoltbookClient,
    name: &str,
    sort: &str,
    limit: u64,
) -> Result<(), ApiError> {
    let response: FeedResponse = client
        .get(&format!(
            "/submolts/{}/feed?sort={}&limit={}",
            name, sort, limit
        ))
        .await?;
    println!("\nSubmolt m/{} ({})", name, sort);
    println!("{}", "=".repeat(60));
    if response.posts.is_empty() {
        display::info("No posts in this submolt yet.");
    } else {
        for (i, post) in response.posts.iter().enumerate() {
            display::display_post(post, Some(i + 1));
        }
    }
    Ok(())
}

pub async fn create_submolt(
    client: &MoltbookClient,
    name: &str,
    display_name: &str,
    description: Option<String>,
    allow_crypto: bool,
) -> Result<(), ApiError> {
    let body = json!({
        "name": name,
        "display_name": display_name,
        "description": description,
        "allow_crypto": allow_crypto,
    });
    let result: serde_json::Value = client.post("/submolts", &body).await?;
    if result["success"].as_bool().unwrap_or(false) {
        display::success(&format!("Submolt m/{} created successfully! 🦞", name));
    }
    Ok(())
}

pub async fn subscribe(client: &MoltbookClient, name: &str) -> Result<(), ApiError> {
    let result: serde_json::Value = client
        .post(&format!("/submolts/{}/subscribe", name), &json!({}))
        .await?;
    if result["success"].as_bool().unwrap_or(false) {
        display::success(&format!("Subscribed to m/{}", name));
    }
    Ok(())
}

pub async fn unsubscribe(client: &MoltbookClient, name: &str) -> Result<(), ApiError> {
    let result: serde_json::Value = client
        .delete(&format!("/submolts/{}/subscribe", name))
        .await?;
    if result["success"].as_bool().unwrap_or(false) {
        display::success(&format!("Unsubscribed from m/{}", name));
    }
    Ok(())
}

pub async fn pin_post(client: &MoltbookClient, post_id: &str) -> Result<(), ApiError> {
    let result: serde_json::Value = client
        .post(&format!("/posts/{}/pin", post_id), &json!({}))
        .await?;
    if result["success"].as_bool().unwrap_or(false) {
        display::success("Post pinned successfully! 📌");
    }
    Ok(())
}

pub async fn unpin_post(client: &MoltbookClient, post_id: &str) -> Result<(), ApiError> {
    let result: serde_json::Value = client.delete(&format!("/posts/{}/pin", post_id)).await?;
    if result["success"].as_bool().unwrap_or(false) {
        display::success("Post unpinned");
    }
    Ok(())
}

pub async fn update_settings(
    client: &MoltbookClient,
    name: &str,
    description: Option<String>,
    banner_color: Option<String>,
    theme_color: Option<String>,
) -> Result<(), ApiError> {
    let mut body = json!({});
    if let Some(d) = description {
        body["description"] = json!(d);
    }
    if let Some(bc) = banner_color {
        body["banner_color"] = json!(bc);
    }
    if let Some(tc) = theme_color {
        body["theme_color"] = json!(tc);
    }

    let result: serde_json::Value = client
        .patch(&format!("/submolts/{}/settings", name), &body)
        .await?;
    if result["success"].as_bool().unwrap_or(false) {
        display::success(&format!("m/{} settings updated!", name));
    }
    Ok(())
}

pub async fn list_moderators(client: &MoltbookClient, name: &str) -> Result<(), ApiError> {
    let response: serde_json::Value = client
        .get(&format!("/submolts/{}/moderators", name))
        .await?;
    println!("\nModerators for m/{}", name.cyan());
    if let Some(mods) = response["moderators"].as_array() {
        for m in mods {
            let agent = m["agent_name"].as_str().unwrap_or("unknown");
            let role = m["role"].as_str().unwrap_or("moderator");
            println!("  - {} ({})", agent.yellow(), role.dimmed());
        }
    }
    Ok(())
}

pub async fn add_moderator(
    client: &MoltbookClient,
    name: &str,
    agent_name: &str,
    role: &str,
) -> Result<(), ApiError> {
    let body = json!({ "agent_name": agent_name, "role": role });
    let result: serde_json::Value = client
        .post(&format!("/submolts/{}/moderators", name), &body)
        .await?;
    if result["success"].as_bool().unwrap_or(false) {
        display::success(&format!("Added {} as a moderator to m/{}", agent_name, name));
    }
    Ok(())
}

pub async fn remove_moderator(
    client: &MoltbookClient,
    name: &str,
    agent_name: &str,
) -> Result<(), ApiError> {
    let result: serde_json::Value = client
        .delete(&format!("/submolts/{}/moderators/{}", name, agent_name))
        .await?;
    if result["success"].as_bool().unwrap_or(false) {
        display::success(&format!("Removed {} from moderators of m/{}", agent_name, name));
    }
    Ok(())
}
