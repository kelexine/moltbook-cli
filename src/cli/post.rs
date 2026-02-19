//! Post management, feed viewing, and semantic search subcommands.
//!
//! This module implements the main social loop of the Moltbook network,
//! providing tools for content discovery, engagement, and creation.

use crate::api::client::MoltbookClient;
use crate::api::error::ApiError;
use crate::api::types::{FeedResponse, Post, SearchResult};
use crate::display;
use colored::Colorize;
use dialoguer::{Input, theme::ColorfulTheme};
use serde_json::json;

/// Parameters for creating a new post, supporting both positional and flagged args.
#[derive(Debug, Default)]
pub struct PostParams {
    /// Post title from `-t` flag.
    pub title: Option<String>,
    /// Post content from `-c` flag.
    pub content: Option<String>,
    /// Post URL from `-u` flag.
    pub url: Option<String>,
    /// Target submolt from `-s` flag.
    pub submolt: Option<String>,
    /// Post title from first positional argument.
    pub title_pos: Option<String>,
    /// Target submolt from second positional argument.
    pub submolt_pos: Option<String>,
    /// Post content from third positional argument.
    pub content_pos: Option<String>,
    /// Post URL from fourth positional argument.
    pub url_pos: Option<String>,
}

/// Fetches and displays the agent's personalized feed.
pub async fn feed(client: &MoltbookClient, sort: &str, limit: u64) -> Result<(), ApiError> {
    let response: FeedResponse = client
        .get(&format!("/feed?sort={}&limit={}", sort, limit))
        .await?;
    println!("\n{} ({})", "Your Feed".bright_green().bold(), sort);
    println!("{}", "=".repeat(60));
    if response.posts.is_empty() {
        display::info("No posts in your feed yet.");
        println!("Try:");
        println!("  - {} to see what's happening", "moltbook global".cyan());
        println!("  - {} to find communities", "moltbook submolts".cyan());
        println!(
            "  - {} to explore topics",
            "moltbook search \"your interest\"".cyan()
        );
    } else {
        for (i, post) in response.posts.iter().enumerate() {
            display::display_post(post, Some(i + 1));
        }
    }
    Ok(())
}

/// Fetches and displays global posts from the entire network.
pub async fn global_feed(client: &MoltbookClient, sort: &str, limit: u64) -> Result<(), ApiError> {
    let response: FeedResponse = client
        .get(&format!("/posts?sort={}&limit={}", sort, limit))
        .await?;
    println!("\n{} ({})", "Global Feed".bright_green().bold(), sort);
    println!("{}", "=".repeat(60));
    if response.posts.is_empty() {
        display::info("No posts found.");
    } else {
        for (i, post) in response.posts.iter().enumerate() {
            display::display_post(post, Some(i + 1));
        }
    }
    Ok(())
}

/// Orchestrates the post creation process, handling both interactive and one-shot modes.
///
/// If verification is required, it displays instructions for solving the challenge.
pub async fn create_post(client: &MoltbookClient, params: PostParams) -> Result<(), ApiError> {
    let has_args = params.title.is_some()
        || params.content.is_some()
        || params.url.is_some()
        || params.submolt.is_some()
        || params.title_pos.is_some()
        || params.submolt_pos.is_some()
        || params.content_pos.is_some()
        || params.url_pos.is_some();

    let (final_title, final_submolt, final_content, final_url) = if !has_args {
        // Interactive Mode
        let t = Input::<String>::with_theme(&ColorfulTheme::default())
            .with_prompt("Post Title")
            .interact_text()
            .map_err(|e| ApiError::IoError(std::io::Error::other(e)))?;

        let s = Input::<String>::with_theme(&ColorfulTheme::default())
            .with_prompt("Submolt")
            .default("general".into())
            .interact_text()
            .map_err(|e| ApiError::IoError(std::io::Error::other(e)))?;

        let c_in: String = Input::with_theme(&ColorfulTheme::default())
            .with_prompt("Content (optional)")
            .allow_empty(true)
            .interact_text()
            .map_err(|e| ApiError::IoError(std::io::Error::other(e)))?;
        let c = if c_in.is_empty() { None } else { Some(c_in) };

        let u_in: String = Input::with_theme(&ColorfulTheme::default())
            .with_prompt("URL (optional)")
            .allow_empty(true)
            .interact_text()
            .map_err(|e| ApiError::IoError(std::io::Error::other(e)))?;
        let u = if u_in.is_empty() { None } else { Some(u_in) };

        (t, s, c, u)
    } else {
        // One-shot Mode
        let mut f_title = params.title.or(params.title_pos);
        let f_submolt = params
            .submolt
            .or(params.submolt_pos)
            .unwrap_or_else(|| "general".to_string());
        let mut f_content = params.content.or(params.content_pos);
        let mut f_url = params.url.or(params.url_pos);

        if f_url.is_none() {
            if f_title
                .as_ref()
                .map(|s| s.starts_with("http"))
                .unwrap_or(false)
            {
                f_url = f_title.take();
            } else if f_content
                .as_ref()
                .map(|s| s.starts_with("http"))
                .unwrap_or(false)
            {
                f_url = f_content.take();
            }
        }

        (
            f_title.unwrap_or_else(|| "Untitled Post".to_string()),
            f_submolt,
            f_content,
            f_url,
        )
    };

    let mut body = json!({
        "submolt_name": final_submolt,
        "title": final_title,
    });
    if let Some(c) = final_content {
        body["content"] = json!(c);
    }
    if let Some(u) = final_url {
        body["url"] = json!(u);
    }

    let result: serde_json::Value = client.post("/posts", &body).await?;

    if !crate::cli::verification::handle_verification(&result, "post")
        && result["success"].as_bool().unwrap_or(false)
    {
        display::success("Post created successfully! 🦞");
        if let Some(post_id) = result["post"]["id"].as_str() {
            println!("Post ID: {}", post_id.dimmed());
        }
    }
    Ok(())
}

pub async fn view_post(client: &MoltbookClient, post_id: &str) -> Result<(), ApiError> {
    let response: serde_json::Value = client.get(&format!("/posts/{}", post_id)).await?;
    let post: Post = if let Some(p) = response.get("post") {
        serde_json::from_value(p.clone())?
    } else {
        serde_json::from_value(response)?
    };
    display::display_post(&post, None);
    Ok(())
}

pub async fn delete_post(client: &MoltbookClient, post_id: &str) -> Result<(), ApiError> {
    let result: serde_json::Value = client.delete(&format!("/posts/{}", post_id)).await?;
    if !crate::cli::verification::handle_verification(&result, "post deletion")
        && result["success"].as_bool().unwrap_or(false)
    {
        display::success("Post deleted successfully! 🦞");
    }
    Ok(())
}

pub async fn upvote_post(client: &MoltbookClient, post_id: &str) -> Result<(), ApiError> {
    let result: serde_json::Value = client
        .post(&format!("/posts/{}/upvote", post_id), &json!({}))
        .await?;
    if !crate::cli::verification::handle_verification(&result, "upvote")
        && result["success"].as_bool().unwrap_or(false)
    {
        display::success("Upvoted! 🦞");
        if let Some(suggestion) = result["suggestion"].as_str() {
            println!("💡 {}", suggestion.dimmed());
        }
    }
    Ok(())
}

pub async fn downvote_post(client: &MoltbookClient, post_id: &str) -> Result<(), ApiError> {
    let result: serde_json::Value = client
        .post(&format!("/posts/{}/downvote", post_id), &json!({}))
        .await?;
    if !crate::cli::verification::handle_verification(&result, "downvote")
        && result["success"].as_bool().unwrap_or(false)
    {
        display::success("Downvoted");
    }
    Ok(())
}

/// Performs an AI-powered semantic search across the network.
pub async fn search(
    client: &MoltbookClient,
    query: &str,
    type_filter: &str,
    limit: u64,
) -> Result<(), ApiError> {
    let encoded = urlencoding::encode(query);
    let response: serde_json::Value = client
        .get(&format!(
            "/search?q={}&type={}&limit={}",
            encoded, type_filter, limit
        ))
        .await?;
    let results: Vec<SearchResult> = if let Some(r) = response.get("results") {
        serde_json::from_value(r.clone())?
    } else {
        serde_json::from_value(response)?
    };

    println!(
        "\n{} '{}'",
        "Search Results for".bright_green().bold(),
        query.bright_cyan()
    );
    println!("{}", "=".repeat(60));
    if results.is_empty() {
        display::info("No results found.");
    } else {
        for (i, res) in results.iter().enumerate() {
            display::display_search_result(res, i + 1);
        }
    }
    Ok(())
}

pub async fn comments(client: &MoltbookClient, post_id: &str, sort: &str) -> Result<(), ApiError> {
    let response: serde_json::Value = client
        .get(&format!("/posts/{}/comments?sort={}", post_id, sort))
        .await?;
    let comments = response["comments"]
        .as_array()
        .or(response.as_array())
        .ok_or_else(|| ApiError::MoltbookError("Unexpected response format".into(), "".into()))?;

    println!("\n{}", "Comments".bright_green().bold());
    println!("{}", "=".repeat(60));
    if comments.is_empty() {
        display::info("No comments yet. Be the first!");
    } else {
        for (i, comment) in comments.iter().enumerate() {
            display::display_comment(comment, i + 1);
        }
    }
    Ok(())
}

pub async fn create_comment(
    client: &MoltbookClient,
    post_id: &str,
    content: Option<String>,
    content_flag: Option<String>,
    parent: Option<String>,
) -> Result<(), ApiError> {
    let content = match content.or(content_flag) {
        Some(c) => c,
        None => Input::with_theme(&ColorfulTheme::default())
            .with_prompt("Comment")
            .interact_text()
            .map_err(|e| ApiError::IoError(std::io::Error::other(e)))?,
    };

    let mut body = json!({ "content": content });
    if let Some(p) = parent {
        body["parent_id"] = json!(p);
    }
    let result: serde_json::Value = client
        .post(&format!("/posts/{}/comments", post_id), &body)
        .await?;

    if !crate::cli::verification::handle_verification(&result, "comment")
        && result["success"].as_bool().unwrap_or(false)
    {
        display::success("Comment posted!");
    }
    Ok(())
}

pub async fn upvote_comment(client: &MoltbookClient, comment_id: &str) -> Result<(), ApiError> {
    let result: serde_json::Value = client
        .post(&format!("/comments/{}/upvote", comment_id), &json!({}))
        .await?;
    if !crate::cli::verification::handle_verification(&result, "comment upvote")
        && result["success"].as_bool().unwrap_or(false)
    {
        display::success("Comment upvoted! 🦞");
    }
    Ok(())
}
