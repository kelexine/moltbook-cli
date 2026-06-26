// author: kelexine <https://github.com/kelexine>

use crate::api::client::MoltbookClient;
use crate::api::error::ApiError;
use crate::api::types::{LabelAttachResponse, LabelsResponse, RolesResponse};
use crate::display;
use serde_json::{Value, json};

pub async fn define(
    client: &MoltbookClient,
    submolt: &str,
    key: &str,
    label: &str,
    color: &str,
    kind: &str,
    prompt: Option<String>,
    cadence: Option<u64>,
) -> Result<(), ApiError> {
    let mut body = json!({
        "key": key,
        "label": label,
        "color": color,
        "kind": kind,
    });

    // prompt and cadence_minutes are only meaningful for role kind
    if let Some(p) = prompt {
        body["prompt"] = json!(p);
    }
    if let Some(c) = cadence {
        body["cadence_minutes"] = json!(c);
    }

    let result: Value = client
        .post(&format!("/submolts/{}/labels", submolt), &body)
        .await?;

    if result["success"].as_bool().unwrap_or(false) {
        let id = result["label"]["id"]
            .as_str()
            .or_else(|| result["id"].as_str())
            .unwrap_or("?");
        display::success(&format!(
            "Label '{}' ({}) defined in m/{} — id: {}",
            label, kind, submolt, id
        ));
    } else {
        let error = result["error"].as_str().unwrap_or("Unknown error");
        display::error(&format!("Failed to define label: {}", error));
    }
    Ok(())
}

pub async fn list(client: &MoltbookClient, submolt: &str) -> Result<(), ApiError> {
    let response: LabelsResponse = client
        .get(&format!("/submolts/{}/labels", submolt))
        .await?;
    display::display_labels(&response, submolt);
    Ok(())
}

pub async fn roles(client: &MoltbookClient, submolt: &str) -> Result<(), ApiError> {
    let response: RolesResponse = client
        .get(&format!("/submolts/{}/roles", submolt))
        .await?;
    display::display_roles(&response, submolt);
    Ok(())
}

pub async fn attach(
    client: &MoltbookClient,
    definition_id: &str,
    target_type: &str,
    target_id: &str,
    placement: Option<String>,
) -> Result<(), ApiError> {
    let mut body = json!({
        "label_definition_id": definition_id,
        "target_type": target_type,
        "target_id": target_id,
    });

    // placement is required when assigning a role to an agent
    if let Some(p) = placement {
        body["placement"] = json!(p);
    } else if target_type == "agent" {
        // default for role-to-agent assignments per the skill.md spec
        body["placement"] = json!("metadata");
    }

    let result: LabelAttachResponse = client.post("/labels/attach", &body).await?;

    if result.success {
        let attachment_id = result
            .attachment
            .as_ref()
            .and_then(|a| a.id.as_deref())
            .unwrap_or("?");
        display::success(&format!(
            "Label attached — attachment id: {}",
            attachment_id
        ));
        if let Some(msg) = &result.message {
            display::info(msg);
        }
    } else {
        let error = result.message.as_deref().unwrap_or("Unknown error");
        display::error(&format!("Failed to attach label: {}", error));
    }
    Ok(())
}

pub async fn revoke(client: &MoltbookClient, attachment_id: &str) -> Result<(), ApiError> {
    let result: Value = client
        .delete(&format!("/labels/attach/{}", attachment_id))
        .await?;

    if result["success"].as_bool().unwrap_or(false) {
        display::success(&format!("Label attachment {} revoked", attachment_id));
    } else {
        let error = result["error"].as_str().unwrap_or("Unknown error");
        display::error(&format!("Failed to revoke: {}", error));
    }
    Ok(())
}
