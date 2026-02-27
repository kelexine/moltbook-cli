//! Direct messaging (DM) and private conversation subcommands.
//!
//! This module implements the secure messaging layer of the Moltbook CLI,
//! including request-based chat initiation and human-in-the-loop signaling.

use crate::api::client::MoltbookClient;
use crate::api::error::ApiError;
use crate::api::types::{Conversation, DmCheckResponse, DmRequest, Message};
use crate::display;
use colored::Colorize;
use dialoguer::{Input, theme::ColorfulTheme};
use serde_json::json;

/// Checks for any new DM activity (requests or unread messages).
pub async fn check_dms(client: &MoltbookClient) -> Result<(), ApiError> {
    let response: DmCheckResponse = client.get("/agents/dm/check").await?;
    display::display_dm_check(&response);
    Ok(())
}

/// Lists all pending DM requests received by the agent.
pub async fn list_dm_requests(client: &MoltbookClient) -> Result<(), ApiError> {
    let response: serde_json::Value = client.get("/agents/dm/requests").await?;
    let items: Vec<DmRequest> = if let Some(r) = response.get("requests") {
        if r.is_array() {
            serde_json::from_value(r.clone())?
        } else if let Some(items) = r.get("items") {
            serde_json::from_value(items.clone())?
        } else {
            vec![]
        }
    } else if response.is_array() {
        serde_json::from_value(response)?
    } else {
        vec![]
    };

    println!("\n{}", "Pending DM Requests".bright_green().bold());
    println!("{}", "=".repeat(60));
    if items.is_empty() {
        display::info("No pending requests.");
    } else {
        for req in items {
            display::display_dm_request(&req);
        }
    }
    Ok(())
}

pub async fn list_conversations(client: &MoltbookClient) -> Result<(), ApiError> {
    let response: serde_json::Value = client.get("/agents/dm/conversations").await?;
    let items: Vec<Conversation> = if let Some(c) = response.get("conversations") {
        if c.is_array() {
            serde_json::from_value(c.clone())?
        } else if let Some(i) = c.get("items") {
            serde_json::from_value(i.clone())?
        } else {
            vec![]
        }
    } else {
        vec![]
    };

    println!("\n{}", "DM Conversations".bright_green().bold());
    println!("{}", "=".repeat(60));
    if items.is_empty() {
        display::info("No active conversations.");
    } else {
        for conv in items {
            display::display_conversation(&conv);
        }
    }
    Ok(())
}

pub async fn read_dm(client: &MoltbookClient, conversation_id: &str) -> Result<(), ApiError> {
    let response: serde_json::Value = client
        .get(&format!("/agents/dm/conversations/{}", conversation_id))
        .await?;
    let messages: Vec<Message> = if let Some(m) = response.get("messages") {
        serde_json::from_value(m.clone())?
    } else {
        vec![]
    };

    println!("\n{}", "Messages".bright_green().bold());
    println!("{}", "=".repeat(60));
    for msg in messages {
        display::display_message(&msg, &client.agent_name);
    }
    Ok(())
}

/// Sends a direct message in an established conversation.
pub async fn send_dm(
    client: &MoltbookClient,
    conversation_id: &str,
    message: Option<String>,
    needs_human: bool,
) -> Result<(), ApiError> {
    let message = match message {
        Some(m) => m,
        None => Input::with_theme(&ColorfulTheme::default())
            .with_prompt("Message")
            .interact_text()
            .map_err(|e| ApiError::IoError(std::io::Error::other(e)))?,
    };

    let body = json!({ "message": message, "needs_human_input": needs_human });
    let result: serde_json::Value = client
        .post(
            &format!("/agents/dm/conversations/{}/send", conversation_id),
            &body,
        )
        .await?;

    if !crate::cli::verification::handle_verification(&result, "message")
        && result["success"].as_bool().unwrap_or(false)
    {
        display::success("Message sent! 🦞");
    }
    Ok(())
}

/// Sends a new DM request to another agent.
pub async fn send_request(
    client: &MoltbookClient,
    to: Option<String>,
    message: Option<String>,
    by_owner: bool,
) -> Result<(), ApiError> {
    let to = match to {
        Some(t) => t,
        None => Input::with_theme(&ColorfulTheme::default())
            .with_prompt("To (Agent Name)")
            .interact_text()
            .map_err(|e| ApiError::IoError(std::io::Error::other(e)))?,
    };

    let message = match message {
        Some(m) => m,
        None => Input::with_theme(&ColorfulTheme::default())
            .with_prompt("Message")
            .interact_text()
            .map_err(|e| ApiError::IoError(std::io::Error::other(e)))?,
    };

    let body = if by_owner {
        json!({ "to_owner": to, "message": message })
    } else {
        json!({ "to": to, "message": message })
    };
    let result: serde_json::Value = client.post("/agents/dm/request", &body).await?;

    if !crate::cli::verification::handle_verification(&result, "request")
        && result["success"].as_bool().unwrap_or(false)
    {
        display::success("DM request sent! 🦞");
    }
    Ok(())
}

pub async fn approve_request(
    client: &MoltbookClient,
    conversation_id: &str,
) -> Result<(), ApiError> {
    let result: serde_json::Value = client
        .post(
            &format!("/agents/dm/requests/{}/approve", conversation_id),
            &json!({}),
        )
        .await?;
    if result["success"].as_bool().unwrap_or(false) {
        display::success("Request approved! 🦞");
    }
    Ok(())
}

pub async fn reject_request(
    client: &MoltbookClient,
    conversation_id: &str,
    block: bool,
) -> Result<(), ApiError> {
    let body = json!({ "block": block });
    let result: serde_json::Value = client
        .post(
            &format!("/agents/dm/requests/{}/reject", conversation_id),
            &body,
        )
        .await?;
    if result["success"].as_bool().unwrap_or(false) {
        if block {
            display::success("Request rejected and blocked");
        } else {
            display::success("Request rejected");
        }
    }
    Ok(())
}
