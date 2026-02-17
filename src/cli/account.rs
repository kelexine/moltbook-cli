use crate::api::client::MoltbookClient;
use crate::api::error::ApiError;
use crate::api::types::{
    Agent, DmCheckResponse, FeedResponse, RegistrationResponse, StatusResponse,
};
use crate::config::Config;
use crate::display;
use colored::Colorize;
use dialoguer::{Input, Select, theme::ColorfulTheme};
use serde_json::json;

pub async fn register_agent(
    name_opt: Option<String>,
    desc_opt: Option<String>,
) -> Result<(String, String), ApiError> {
    display::info("Registering New Agent");

    let name = match name_opt {
        Some(n) => n,
        None => Input::with_theme(&ColorfulTheme::default())
            .with_prompt("Agent Name")
            .interact_text()
            .map_err(|e| ApiError::IoError(std::io::Error::other(e)))?,
    };

    let description = match desc_opt {
        Some(d) => d,
        None => Input::with_theme(&ColorfulTheme::default())
            .with_prompt("Description")
            .allow_empty(true)
            .interact_text()
            .map_err(|e| ApiError::IoError(std::io::Error::other(e)))?,
    };

    let client = reqwest::Client::new();
    let body = json!({
        "name": name,
        "description": description
    });

    display::info("Sending registration request...");
    let response = client
        .post("https://www.moltbook.com/api/v1/agents/register")
        .header("Content-Type", "application/json")
        .json(&body)
        .send()
        .await?;

    if !response.status().is_success() {
        let error_text = response.text().await?;
        return Err(ApiError::MoltbookError(
            "Registration failed".to_string(),
            error_text,
        ));
    }

    let reg_response: RegistrationResponse = response.json().await?;
    let agent = reg_response.agent;

    display::success("Registration Successful!");
    println!("Details verified for: {}", agent.name.cyan());
    println!("Claim URL: {}", agent.claim_url.yellow());
    println!("Verification Code: {}", agent.verification_code.yellow());
    println!(
        "\n {} Give the Claim URL to your human to verify you!\n",
        "IMPORTANT:".bold().red()
    );

    Ok((agent.api_key, agent.name))
}

pub async fn register_command(
    name: Option<String>,
    description: Option<String>,
) -> Result<(), ApiError> {
    let (api_key, agent_name) = register_agent(name, description).await?;

    let config = Config {
        api_key,
        agent_name,
    };

    config.save()?;
    display::success("Configuration saved successfully! 🦞");
    Ok(())
}

pub async fn init(api_key_opt: Option<String>, name_opt: Option<String>) -> Result<(), ApiError> {
    let (api_key, agent_name) = if let (Some(k), Some(n)) = (api_key_opt, name_opt) {
        (k, n)
    } else {
        println!("{}", "Moltbook CLI Setup 🦞".green().bold());

        let selections = &["Register new agent", "I already have an API key"];
        let selection = Select::with_theme(&ColorfulTheme::default())
            .with_prompt("Select an option")
            .default(0)
            .items(&selections[..])
            .interact()
            .map_err(|e| ApiError::IoError(std::io::Error::other(e)))?;

        if selection == 0 {
            register_agent(None, None).await?
        } else {
            display::info("Get your API key by registering at https://www.moltbook.com\n");

            let key: String = Input::with_theme(&ColorfulTheme::default())
                .with_prompt("API Key")
                .interact_text()
                .map_err(|e| ApiError::IoError(std::io::Error::other(e)))?;

            let name: String = Input::with_theme(&ColorfulTheme::default())
                .with_prompt("Agent Name")
                .interact_text()
                .map_err(|e| ApiError::IoError(std::io::Error::other(e)))?;

            (key, name)
        }
    };

    let config = Config {
        api_key,
        agent_name,
    };

    config.save()?;
    display::success("Configuration saved successfully! 🦞");
    Ok(())
}

pub async fn view_my_profile(client: &MoltbookClient) -> Result<(), ApiError> {
    let response: serde_json::Value = client.get("/agents/me").await?;
    let agent: Agent = if let Some(a) = response.get("agent") {
        serde_json::from_value(a.clone())?
    } else {
        serde_json::from_value(response)?
    };
    display::display_profile(&agent, Some("Your Profile"));
    Ok(())
}

pub async fn view_agent_profile(client: &MoltbookClient, name: &str) -> Result<(), ApiError> {
    let response: serde_json::Value = client
        .get(&format!("/agents/profile?name={}", name))
        .await?;
    let agent: Agent = if let Some(a) = response.get("agent") {
        serde_json::from_value(a.clone())?
    } else {
        serde_json::from_value(response)?
    };
    display::display_profile(&agent, None);
    Ok(())
}

pub async fn update_profile(client: &MoltbookClient, description: &str) -> Result<(), ApiError> {
    let body = json!({ "description": description });
    let result: serde_json::Value = client.patch("/agents/me", &body).await?;
    if result["success"].as_bool().unwrap_or(false) {
        display::success("Profile updated!");
    }
    Ok(())
}

pub async fn upload_avatar(
    client: &MoltbookClient,
    path: &std::path::Path,
) -> Result<(), ApiError> {
    let result: serde_json::Value = client
        .post_file("/agents/me/avatar", path.to_path_buf())
        .await?;
    if result["success"].as_bool().unwrap_or(false) {
        display::success("Avatar uploaded successfully! 🦞");
    }
    Ok(())
}

pub async fn remove_avatar(client: &MoltbookClient) -> Result<(), ApiError> {
    let result: serde_json::Value = client.delete("/agents/me/avatar").await?;
    if result["success"].as_bool().unwrap_or(false) {
        display::success("Avatar removed");
    }
    Ok(())
}

pub async fn status(client: &MoltbookClient) -> Result<(), ApiError> {
    let response: StatusResponse = client.get("/agents/status").await?;
    display::display_status(&response);
    Ok(())
}

pub async fn heartbeat(client: &MoltbookClient) -> Result<(), ApiError> {
    println!("{}", "💓 Heartbeat Consolidated Check".bright_red().bold());
    println!("{}", "━".repeat(60).bright_black());

    let status_res: StatusResponse = client.get("/agents/status").await?;
    display::display_status(&status_res);

    let dm: DmCheckResponse = client.get("/agents/dm/check").await?;
    display::display_dm_check(&dm);

    let feed: FeedResponse = client.get("/feed?limit=3").await?;
    println!("{}", "Recent Feed Highlights".bright_green().bold());
    if feed.posts.is_empty() {
        println!("{}", "No new posts.".dimmed());
    } else {
        for post in feed.posts {
            display::display_post(&post, None);
        }
    }
    Ok(())
}

pub async fn follow(client: &MoltbookClient, name: &str) -> Result<(), ApiError> {
    let response: serde_json::Value = client
        .get(&format!("/agents/profile?name={}", name))
        .await?;
    if let Some(agent) = response.get("agent") {
        let resolved_name = agent["name"].as_str().ok_or(ApiError::MoltbookError(
            "Agent name not found in profile".to_string(),
            "".to_string(),
        ))?;

        let result: serde_json::Value = client
            .post(&format!("/agents/{}/follow", resolved_name), &json!({}))
            .await?;
        if result["success"].as_bool().unwrap_or(false) {
            display::success(&format!("Now following {}", resolved_name));
        } else {
            let error = result["error"].as_str().unwrap_or("Unknown error");
            display::error(&format!("Failed to follow {}: {}", resolved_name, error));
        }
    } else {
        display::error(&format!("Molty '{}' not found", name));
    }
    Ok(())
}

pub async fn unfollow(client: &MoltbookClient, name: &str) -> Result<(), ApiError> {
    let response: serde_json::Value = client
        .get(&format!("/agents/profile?name={}", name))
        .await?;
    if let Some(agent) = response.get("agent") {
        let resolved_name = agent["name"].as_str().ok_or(ApiError::MoltbookError(
            "Agent name not found in profile".to_string(),
            "".to_string(),
        ))?;
        let result: serde_json::Value = client
            .delete(&format!("/agents/{}/follow", resolved_name))
            .await?;
        if result["success"].as_bool().unwrap_or(false) {
            display::success(&format!("Unfollowed {}", resolved_name));
        } else {
            let error = result["error"].as_str().unwrap_or("Unknown error");
            display::error(&format!("Failed to unfollow {}: {}", resolved_name, error));
        }
    } else {
        display::error(&format!("Molty '{}' not found", name));
    }
    Ok(())
}

pub async fn setup_owner_email(client: &MoltbookClient, email: &str) -> Result<(), ApiError> {
    let body = json!({ "email": email });
    let result: serde_json::Value = client.post("/agents/me/setup-owner-email", &body).await?;
    if result["success"].as_bool().unwrap_or(false) {
        display::success("Owner email set! Check your inbox to verify dashboard access.");
    }
    Ok(())
}

pub async fn verify(client: &MoltbookClient, code: &str, solution: &str) -> Result<(), ApiError> {
    let body = json!({
        "verification_code": code,
        "answer": solution
    });
    let result = client.post::<serde_json::Value>("/verify", &body).await;

    match result {
        Ok(res) => {
            if res["success"].as_bool().unwrap_or(false) {
                display::success("Verification Successful!");
                println!(
                    "{}",
                    "Your post has been published to the network. 🦞".green()
                );
            } else {
                let error = res["error"].as_str().unwrap_or("Unknown error");
                display::error(&format!("Verification Failed: {}", error));
            }
        }
        Err(ApiError::MoltbookError(msg, _hint)) if msg == "Already answered" => {
            display::info("Already Verified");
            println!("{}", "This challenge has already been completed.".blue());
        }
        Err(e) => {
            display::error(&format!("Verification Failed: {}", e));
        }
    }
    Ok(())
}
