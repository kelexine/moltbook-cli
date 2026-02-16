use crate::api::types::{Agent, DmRequest, Post, SearchResult, Submolt};
use colored::*;

pub fn display_post(post: &Post, index: Option<usize>) {
    let prefix = if let Some(i) = index {
        format!("[{}] ", i)
    } else {
        String::new()
    };
    
    println!("{}{}", prefix, post.title.bright_cyan().bold());
    println!("  {} | m/{} | ⬆ {} ⬇ {} | 💬 {}", 
        post.author.name.yellow(),
        post.submolt.name.green(),
        post.upvotes,
        post.downvotes,
        post.comment_count.unwrap_or(0)
    );
    
    if let Some(content) = &post.content {
        let is_listing = index.is_some();
        if is_listing && content.chars().count() > 150 {
            let truncated: String = content.chars().take(150).collect();
            println!("  {}", format!("{}...", truncated).dimmed());
        } else {
            if !is_listing {
                println!("{}", "-".repeat(60).dimmed());
            }
            println!("  {}", content);
            if !is_listing {
                println!("{}", "-".repeat(60).dimmed());
            }
        }
    }
    
    if let Some(url) = &post.url {
        println!("  🔗 {}", url.blue());
    }
    
    println!("  ID: {} | {}", post.id.dimmed(), post.created_at.dimmed());
    println!();
}

pub fn display_search_result(result: &SearchResult, index: usize) {
    println!("[{}] {}", index, 
        result.title.as_deref().unwrap_or("(comment)").bright_cyan().bold()
    );
    let score = result.similarity.unwrap_or(0.0);
    let score_label = if score > 1.0 { "Relevance" } else { "Similarity" };
    let score_display = if score > 1.0 { 
        format!("{:.1}", score) 
    } else { 
        format!("{:.0}%", score * 100.0) 
    };

    println!("  {} | {} | {}: {}", 
        result.author.name.yellow(),
        result.result_type.green(),
        score_label,
        score_display
    );
    
    if let Some(content) = &result.content {
        let preview = if content.chars().count() > 150 {
            let truncated: String = content.chars().take(150).collect();
            format!("{}...", truncated)
        } else {
            content.clone()
        };

        println!("  {}", preview.dimmed());
    }
    
    if let Some(post_id) = &result.post_id {
        println!("  Post ID: {}", post_id.dimmed());
    }
    
    println!();
}

pub fn display_profile(agent: &Agent, title: Option<&str>) {
    let title_str = title.unwrap_or("Profile");
    println!("\n{} {}", "👤".cyan(), title_str.bright_green().bold());
    println!("{:<15} {}", "Name:", agent.name.bright_white().bold());
    println!("{:<15} {}", "ID:", agent.id.dimmed());
    println!("{}", "━".repeat(60).bright_black());
    
    if let Some(desc) = &agent.description {
        let wrapped = textwrap::fill(desc, 58);
        for line in wrapped.lines() {
            println!("  {}", line.italic());
        }
        println!("{}", "─".repeat(60).dimmed());
    }
    
    println!("{:<15} {}", "✨ Karma:", agent.karma.unwrap_or(0).to_string().yellow().bold());
    
    if let Some(stats) = &agent.stats {
        println!("{:<15} {}", "📝 Posts:", stats.posts.unwrap_or(0).to_string().cyan());
        println!("{:<15} {}", "💬 Comments:", stats.comments.unwrap_or(0).to_string().cyan());
        println!("{:<15} m/ {}", "🍿 Submolts:", stats.subscriptions.unwrap_or(0).to_string().cyan());
    }
    
    if let (Some(followers), Some(following)) = (agent.follower_count, agent.following_count) {
        println!("{:<15} {}", "👥 Followers:", followers.to_string().blue());
        println!("{:<15} {}", "👀 Following:", following.to_string().blue());
    }
    
    println!("{}", "─".repeat(60).dimmed());
    
    if let Some(claimed) = agent.is_claimed {
        let status = if claimed { "✓ Claimed".green() } else { "✗ Unclaimed".red() };
        println!("{:<15} {}", "🛡️  Status:", status);
        if let Some(claimed_at) = &agent.claimed_at {
            println!("{:<15} {}", "📅 Claimed:", claimed_at.dimmed());
        }
    }
    
    if let Some(created_at) = &agent.created_at {
        println!("{:<15} {}", "🌱 Joined:", created_at.dimmed());
    }
    if let Some(last_active) = &agent.last_active {
        println!("{:<15} {}", "⏰ Active:", last_active.dimmed());
    }
    
    if let Some(owner) = &agent.owner {
        println!("\n{}", "👑 Owner".bright_yellow().underline());
        if let Some(name) = &owner.x_name {
            println!("{:<15} {}", "Name:", name);
        }
        if let Some(handle) = &owner.x_handle {
            println!("{:<15} @{}", "X (Twitter):", handle.cyan());
        }
        if let Some(owner_id) = &agent.owner_id {
            println!("{:<15} {}", "Owner ID:", owner_id.dimmed());
        }
    }

    if let Some(metadata) = &agent.metadata {
        if !metadata.is_null() && metadata.as_object().map_or(false, |o| !o.is_empty()) {
            println!("\n{}", "📂 Metadata".bright_blue().underline());
            println!("{}", serde_json::to_string_pretty(metadata).unwrap_or_default().dimmed());
        }
    }
    println!();
}

pub fn display_comment(comment: &serde_json::Value, index: usize) {
    let author = comment["author"]["name"].as_str().unwrap_or("unknown");
    let content = comment["content"].as_str().unwrap_or("");
    let upvotes = comment["upvotes"].as_i64().unwrap_or(0);
    let id = comment["id"].as_str().unwrap_or("unknown");
    
    println!("[{}] {} (⬆ {})", index, author.yellow(), upvotes);
    println!("  {}", content);
    println!("  ID: {}", id.dimmed());
    println!();
}

pub fn display_submolt(submolt: &Submolt) {
    println!("{} (m/{})", 
        submolt.display_name.bright_cyan().bold(),
        submolt.name.green()
    );
    
    if let Some(desc) = &submolt.description {
        println!("  {}", desc.dimmed());
    }
    
    println!("  Subscribers: {}", submolt.subscriber_count.unwrap_or(0));
    println!();
}

pub fn display_dm_request(req: &DmRequest) {
    let from = &req.from.name;
    let msg = req.message.as_deref().or(req.message_preview.as_deref()).unwrap_or("");
    
    println!("\nFrom: {}", from.cyan());
    println!("Message: {}", msg);
    println!("ID: {}", req.conversation_id.dimmed());
    println!("To approve: {}", format!("moltbook-cli dm-approve {}", req.conversation_id).green());
    println!("To reject:  {}", format!("moltbook-cli dm-reject {}", req.conversation_id).yellow());
    println!("{}", "─".repeat(60));
}

pub fn display_status(status: &crate::api::types::StatusResponse) {
    println!("\n{}", "Account Status".bright_green().bold());
    println!("{}", "=".repeat(50));
    
    if let Some(s) = &status.status {
        let status_display = match s.as_str() {
            "claimed" => "✓ Claimed".green(),
            "pending_claim" => "⏳ Pending Claim".yellow(),
            _ => s.normal(),
        };
        println!("Status: {}", status_display);
    }
    
    if let Some(msg) = &status.message {
        println!("\n{}", msg);
    }
    
    if let Some(next) = &status.next_step {
        println!("{}", next.dimmed());
    }
    println!();
}

pub fn display_dm_check(response: &crate::api::types::DmCheckResponse) {
    println!("\n{}", "DM Activity".bright_green().bold());
    println!("{}", "=".repeat(50));
    
    if !response.has_activity {
        println!("{}", "No new DM activity 🦞".green());
    } else {
        if let Some(summary) = &response.summary {
            println!("{}", summary.yellow());
        }
        
        // Show pending requests
        if let Some(data) = &response.requests 
            && !data.items.is_empty() {
                println!("\n{}", "Pending Requests:".bold());
                for req in &data.items {
                    let from = &req.from.name;
                    let preview = req.message_preview.as_deref().unwrap_or("");
                    let conv_id = &req.conversation_id;
                    
                    println!("\n  From: {}", from.cyan());
                    println!("  Message: {}", preview.dimmed());
                    println!("  ID: {}", conv_id);
                }
            }
        
        if let Some(data) = &response.messages 
            && data.total_unread > 0 {
                println!("\n{} unread messages", data.total_unread.to_string().yellow());
        }
    }
    println!();
}

pub fn display_conversation(conv: &crate::api::types::Conversation) {
    let unread_msg = if conv.unread_count > 0 {
        format!(" ({} unread)", conv.unread_count).yellow().to_string()
    } else {
        String::new()
    };
    
    println!("With: {}{}", conv.with_agent.name.cyan(), unread_msg);
    println!("ID: {}", conv.conversation_id.dimmed());
    println!("Read: {}", format!("moltbook-cli dm-read {}", conv.conversation_id).green());
    println!("{}", "─".repeat(60));
}

pub fn display_message(msg: &crate::api::types::Message) {
    let prefix = if msg.from_you { "You" } else { &msg.from_agent.name };
    let color = if msg.from_you { prefix.green() } else { prefix.yellow() };
    
    println!("\n{}: {}", color, msg.message);
    
    if msg.needs_human_input {
        println!("  {}", "⚠ Needs human input".red());
    }
}
