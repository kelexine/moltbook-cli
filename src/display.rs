//! Visual presentation and terminal formatting for the Moltbook CLI.
//!
//! This module provides utilities for relative time calculation, terminal width
//! detection, and high-fidelity rendering of Moltbook data structures using
//! Unicode box-drawing characters and ANSI colors.

use crate::api::types::{Agent, DmRequest, Post, SearchResult, Submolt};
use chrono::{DateTime, Utc};
use colored::*;
use terminal_size::{Width, terminal_size};

/// Detects the available terminal width for responsive layout.
///
/// Priority:
/// 1. `COLUMNS` environment variable.
/// 2. `terminal_size` system call.
/// 3. Default fallback of 80 characters.
fn get_term_width() -> usize {

    if let Some(width) = std::env::var("COLUMNS")
        .ok()
        .and_then(|c| c.parse::<usize>().ok())
    {
        return width.saturating_sub(2).max(40);
    }

    if let Some((Width(w), _)) = terminal_size() {
        (w as usize).saturating_sub(2).max(40)
    } else {
        80
    }
}

/// Formats a UTC timestamp into a human-readable relative string (e.g., "2h ago").
///
/// Supports: "just now", minutes, hours, days, or YYYY-MM-DD for older items.
fn relative_time(timestamp: &str) -> String {

    if let Ok(dt) = DateTime::parse_from_rfc3339(timestamp) {
        let now = Utc::now();
        let diff = now.signed_duration_since(dt);

        if diff.num_seconds() < 60 {
            "just now".to_string()
        } else if diff.num_minutes() < 60 {
            format!("{}m ago", diff.num_minutes())
        } else if diff.num_hours() < 24 {
            format!("{}h ago", diff.num_hours())
        } else if diff.num_days() < 7 {
            format!("{}d ago", diff.num_days())
        } else {
            dt.format("%Y-%m-%d").to_string()
        }
    } else {
        timestamp.to_string()
    }
}

/// Prints a success message with a green checkmark.
pub fn success(msg: &str) {
    println!("{} {}", "✅".green(), msg.bright_green());
}

/// Prints an error message with a red cross.
pub fn error(msg: &str) {
    eprintln!("{} {}", "❌".red().bold(), msg.bright_red());
}

/// Prints an informational message with a cyan icon.
pub fn info(msg: &str) {
    println!("{} {}", "ℹ️ ".cyan(), msg.bright_cyan());
}

/// Prints a warning message with a yellow triangle.
pub fn warn(msg: &str) {
    println!("{} {}", "⚠️ ".yellow(), msg.bright_yellow());
}


/// Renders a Moltbook post in a premium box-styled layout.
///
/// # Arguments
///
/// * `post` - The post object to display.
/// * `index` - Optional positional index for use in lists.
pub fn display_post(post: &Post, index: Option<usize>) {

    let width = get_term_width();
    let inner_width = width.saturating_sub(4);

    println!(
        "{}",
        format!("╭{}╮", "─".repeat(width.saturating_sub(2))).dimmed()
    );

    let prefix = if let Some(i) = index {
        format!("#{:<2} ", i).bright_white().bold()
    } else {
        "".normal()
    };

    let title_space = inner_width.saturating_sub(if index.is_some() { 4 } else { 0 });

    let title = if post.title.chars().count() > title_space {
        let t: String = post
            .title
            .chars()
            .take(title_space.saturating_sub(3))
            .collect();
        format!("{}...", t)
    } else {
        post.title.clone()
    };

    let padding =
        inner_width.saturating_sub(title.chars().count() + if index.is_some() { 4 } else { 0 });
    println!(
        "│ {}{} {:>p$} │",
        prefix,
        title.bright_cyan().bold(),
        "",
        p = padding
    );

    println!(
        "{}",
        format!("├{}┤", "─".repeat(width.saturating_sub(2))).dimmed()
    );

    let karma = post.author.karma.unwrap_or(0);
    let author = post.author.name.yellow();

    // Handle submolt name fallback
    let sub_name = if let Some(s) = &post.submolt {
        &s.name
    } else if let Some(s) = &post.submolt_name {
        s
    } else {
        "unknown"
    };

    let sub = sub_name.green();
    let stats = format!(
        "⬆ {} ⬇ {} 💬 {} ✨ {}",
        post.upvotes,
        post.downvotes,
        post.comment_count.unwrap_or(0),
        karma
    );

    let left_meta = format!("👤 {}  m/{} ", author, sub);
    let left_len = post.author.name.chars().count() + sub_name.chars().count() + 8;
    let stats_len = stats.chars().count();

    let meta_padding = inner_width.saturating_sub(left_len + stats_len);

    println!(
        "│ {}{:>p$} │",
        left_meta,
        stats.dimmed(),
        p = meta_padding + stats_len
    );

    println!("│ {:>w$} │", "", w = inner_width);
    if let Some(content) = &post.content {
        let is_listing = index.is_some();
        let max_lines = if is_listing { 3 } else { 1000 };

        let wrapped_width = inner_width.saturating_sub(2);
        let wrapped = textwrap::fill(content, wrapped_width);

        for (i, line) in wrapped.lines().enumerate() {
            if i >= max_lines {
                println!("│  {: <w$} │", "...".dimmed(), w = wrapped_width);
                break;
            }
            println!("│  {:<w$}│", line, w = wrapped_width);
        }
    }

    if let Some(url) = &post.url {
        println!("│ {:>w$} │", "", w = inner_width);
        let url_width = inner_width.saturating_sub(3);
        let truncated_url = if url.chars().count() > url_width {
            let t: String = url.chars().take(url_width.saturating_sub(3)).collect();
            format!("{}...", t)
        } else {
            url.clone()
        };
        println!(
            "│  🔗 {:<w$} │",
            truncated_url.blue().underline(),
            w = inner_width.saturating_sub(4)
        );
    }

    println!(
        "{}",
        format!("╰{}╯", "─".repeat(width.saturating_sub(2))).dimmed()
    );

    println!(
        "   ID: {} • {}",
        post.id.dimmed(),
        relative_time(&post.created_at).dimmed()
    );
    println!();
}

pub fn display_search_result(result: &SearchResult, index: usize) {
    let width = get_term_width();
    let inner_width = width.saturating_sub(4);

    println!(
        "{}",
        format!("╭{}╮", "─".repeat(width.saturating_sub(2))).dimmed()
    );

    let title = result.title.as_deref().unwrap_or("(comment)");
    let score = result.similarity.unwrap_or(0.0);
    let score_display = if score > 1.0 {
        format!("{:.1}", score)
    } else {
        format!("{:.0}%", score * 100.0)
    };

    let title_space = inner_width.saturating_sub(score_display.chars().count() + 6); // #1 + space + space + score
    let title_display = if title.chars().count() > title_space {
        let t: String = title.chars().take(title_space.saturating_sub(3)).collect();
        format!("{}...", t)
    } else {
        title.to_string()
    };

    let padding = inner_width
        .saturating_sub(4 + title_display.chars().count() + score_display.chars().count());
    println!(
        "│ #{:<2} {}{:>p$} │",
        index,
        title_display.bright_cyan().bold(),
        score_display.green(),
        p = padding + score_display.chars().count()
    );

    println!(
        "{}",
        format!("├{}┤", "─".repeat(width.saturating_sub(2))).dimmed()
    );

    let author = result.author.name.yellow();
    let type_label = result.result_type.blue();

    let left_len = result.author.name.chars().count() + result.result_type.chars().count() + 8;
    let meta_padding = inner_width.saturating_sub(left_len);

    println!(
        "│ 👤 {}  •  {}{:>p$} │",
        author,
        type_label,
        "",
        p = meta_padding
    );

    println!("│ {:>w$} │", "", w = inner_width);
    if let Some(content) = &result.content {
        let wrapped_width = inner_width.saturating_sub(2);
        let wrapped = textwrap::fill(content, wrapped_width);
        for (i, line) in wrapped.lines().enumerate() {
            if i >= 3 {
                println!("│  {: <w$} │", "...".dimmed(), w = wrapped_width);
                break;
            }
            println!("│  {:<w$}│", line, w = wrapped_width);
        }
    }

    println!(
        "{}",
        format!("╰{}╯", "─".repeat(width.saturating_sub(2))).dimmed()
    );
    if let Some(post_id) = &result.post_id {
        println!("   Post ID: {}", post_id.dimmed());
    }
    println!();
}

/// Renders a comprehensive profile view for an agent.
///
/// Displays agent stats, karma, following/follower counts, and owner information
/// in a structured, multi-section layout.
pub fn display_profile(agent: &Agent, title: Option<&str>) {

    let width = get_term_width();

    let title_str = title.unwrap_or("Profile");
    println!("\n{} {}", "👤".cyan(), title_str.bright_green().bold());
    println!("{}", "━".repeat(width).dimmed());

    println!("  {:<15} {}", "Name:", agent.name.bright_white().bold());
    println!("  {:<15} {}", "ID:", agent.id.dimmed());

    if let Some(desc) = &agent.description {
        println!("{}", "─".repeat(width).dimmed());
        let wrapped = textwrap::fill(desc, width.saturating_sub(4));
        for line in wrapped.lines() {
            println!("  {}", line.italic());
        }
    }
    println!("{}", "─".repeat(width).dimmed());

    println!(
        "  {:<15} {}",
        "✨ Karma:",
        agent.karma.unwrap_or(0).to_string().yellow().bold()
    );

    if let Some(stats) = &agent.stats {
        println!(
            "  {:<15} {}",
            "📝 Posts:",
            stats.posts.unwrap_or(0).to_string().cyan()
        );
        println!(
            "  {:<15} {}",
            "💬 Comments:",
            stats.comments.unwrap_or(0).to_string().cyan()
        );
        println!(
            "  {:<15} m/ {}",
            "🍿 Submolts:",
            stats.subscriptions.unwrap_or(0).to_string().cyan()
        );
    }

    if let (Some(followers), Some(following)) = (agent.follower_count, agent.following_count) {
        println!("  {:<15} {}", "👥 Followers:", followers.to_string().blue());
        println!("  {:<15} {}", "👀 Following:", following.to_string().blue());
    }

    println!("{}", "─".repeat(width).dimmed());

    if let Some(claimed) = agent.is_claimed {
        let status = if claimed {
            "✓ Claimed".green()
        } else {
            "✗ Unclaimed".red()
        };
        println!("  {:<15} {}", "🛡️  Status:", status);
        if let Some(claimed_at) = &agent.claimed_at {
            println!(
                "  {:<15} {}",
                "📅 Claimed:",
                relative_time(claimed_at).dimmed()
            );
        }
    }

    if let Some(created_at) = &agent.created_at {
        println!(
            "  {:<15} {}",
            "🌱 Joined:",
            relative_time(created_at).dimmed()
        );
    }
    if let Some(last_active) = &agent.last_active {
        println!(
            "  {:<15} {}",
            "⏰ Active:",
            relative_time(last_active).dimmed()
        );
    }

    if let Some(owner) = &agent.owner {
        println!("\n  {}", "👑 Owner".bright_yellow().underline());
        if let Some(name) = &owner.x_name {
            println!("  {:<15} {}", "Name:", name);
        }
        if let Some(handle) = &owner.x_handle {
            let verified = if owner.x_verified.unwrap_or(false) {
                " (Verified)".blue()
            } else {
                "".normal()
            };
            println!("  {:<15} @{}{}", "X (Twitter):", handle.cyan(), verified);
        }
        if let (Some(foll), Some(follg)) = (owner.x_follower_count, owner.x_following_count) {
            println!(
                "  {:<15} {} followers | {} following",
                "X Stats:",
                foll.to_string().dimmed(),
                follg.to_string().dimmed()
            );
        }
        if let Some(owner_id) = &agent.owner_id {
            println!("  {:<15} {}", "Owner ID:", owner_id.dimmed());
        }
    }

    if let Some(metadata) = &agent.metadata
        && !metadata.is_null()
        && metadata.as_object().is_some_and(|o| !o.is_empty())
    {
        println!("\n  {}", "📂 Metadata".bright_blue().underline());
        println!(
            "  {}",
            serde_json::to_string_pretty(metadata)
                .unwrap_or_default()
                .dimmed()
        );
    }
    println!();
}

pub fn display_comment(comment: &serde_json::Value, index: usize) {
    let author = comment["author"]["name"].as_str().unwrap_or("unknown");
    let content = comment["content"].as_str().unwrap_or("");
    let upvotes = comment["upvotes"].as_i64().unwrap_or(0);
    let id = comment["id"].as_str().unwrap_or("unknown");

    let width = get_term_width();

    println!(
        "{} {} (⬆ {})",
        format!("#{:<2}", index).dimmed(),
        author.yellow().bold(),
        upvotes
    );

    let wrapped = textwrap::fill(content, width.saturating_sub(4));
    for line in wrapped.lines() {
        println!("│ {}", line);
    }
    println!("└─ ID: {}", id.dimmed());
    println!();
}

pub fn display_submolt(submolt: &Submolt) {
    let width = get_term_width();
    println!(
        "{} (m/{})",
        submolt.display_name.bright_cyan().bold(),
        submolt.name.green()
    );

    if let Some(desc) = &submolt.description {
        println!("  {}", desc.dimmed());
    }

    println!("  Subscribers: {}", submolt.subscriber_count.unwrap_or(0));
    println!("{}", "─".repeat(width.min(60)).dimmed());
    println!();
}

/// Displays a DM request with action guidance.
pub fn display_dm_request(req: &DmRequest) {

    let width = get_term_width();
    let inner_width = width.saturating_sub(4);

    let from = &req.from.name;
    let msg = req
        .message
        .as_deref()
        .or(req.message_preview.as_deref())
        .unwrap_or("");

    println!(
        "{}",
        format!("╭{}╮", "─".repeat(width.saturating_sub(2))).dimmed()
    );

    // Calculate padding for the 'from' line
    let from_line_len = 15 + from.chars().count();
    let padding = inner_width.saturating_sub(from_line_len);

    println!(
        "│ 📨 Request from {} {:>p$} │",
        from.cyan().bold(),
        "",
        p = padding
    );
    println!(
        "{}",
        format!("├{}┤", "─".repeat(width.saturating_sub(2))).dimmed()
    );

    if let Some(handle) = req.from.owner.as_ref().and_then(|o| o.x_handle.as_ref()) {
        println!(
            "│ 👑 Owner: @{:<w$} │",
            handle.blue(),
            w = inner_width.saturating_sub(14)
        );
    }

    let wrapped = textwrap::fill(msg, inner_width.saturating_sub(2));
    for line in wrapped.lines() {
        println!("│  {:<w$}│", line, w = inner_width.saturating_sub(2));
    }

    println!(
        "{}",
        format!("├{}┤", "─".repeat(width.saturating_sub(2))).dimmed()
    );
    println!(
        "│ ID: {:<w$} │",
        req.conversation_id.dimmed(),
        w = inner_width.saturating_sub(4)
    );
    println!(
        "│ {:<w$} │",
        format!("✔ Approve: moltbook dm-approve {}", req.conversation_id).green(),
        w = inner_width.saturating_sub(2) + 9
    ); // +9 roughly for ansi
    println!(
        "│ {:<w$} │",
        format!("✘ Reject:  moltbook dm-reject {}", req.conversation_id).red(),
        w = inner_width.saturating_sub(2) + 9
    );
    println!(
        "{}",
        format!("╰{}╯", "─".repeat(width.saturating_sub(2))).dimmed()
    );
    println!();
}

pub fn display_status(status: &crate::api::types::StatusResponse) {
    let width = get_term_width();
    println!(
        "\n{} {}",
        "🛡️".cyan(),
        "Account Status".bright_green().bold()
    );
    println!("{}", "━".repeat(width).dimmed());

    if let Some(agent) = &status.agent {
        println!(
            "  {:<15} {}",
            "Agent Name:",
            agent.name.bright_white().bold()
        );
        println!("  {:<15} {}", "Agent ID:", agent.id.dimmed());
        if let Some(claimed_at) = &agent.claimed_at {
            println!(
                "  {:<15} {}",
                "Claimed At:",
                relative_time(claimed_at).dimmed()
            );
        }
        println!("{}", "─".repeat(width).dimmed());
    }

    if let Some(s) = &status.status {
        let status_display = match s.as_str() {
            "claimed" => "✓ Claimed".green(),
            "pending_claim" => "⏳ Pending Claim".yellow(),
            _ => s.normal(),
        };
        println!("  {:<15} {}", "Status:", status_display);
    }

    if let Some(msg) = &status.message {
        println!("\n  {}", msg);
    }

    if let Some(next) = &status.next_step {
        println!("  {}", next.dimmed());
    }
    println!();
}

pub fn display_dm_check(response: &crate::api::types::DmCheckResponse) {
    let width = get_term_width();
    println!("\n{}", "DM Activity".bright_green().bold());
    println!("{}", "━".repeat(width).dimmed());

    if !response.has_activity {
        println!("  {}", "No new DM activity 🦞".green());
    } else {
        if let Some(summary) = &response.summary {
            println!("  {}", summary.yellow());
        }

        if let Some(data) = &response.requests
            && !data.items.is_empty()
        {
            println!("\n  {}", "Pending Requests:".bold());
            for req in &data.items {
                let from = &req.from.name;
                let preview = req.message_preview.as_deref().unwrap_or("");
                let conv_id = &req.conversation_id;

                println!("\n    From: {}", from.cyan());
                println!("    Message: {}", preview.dimmed());
                println!("    ID: {}", conv_id);
            }
        }

        if let Some(data) = &response.messages
            && data.total_unread > 0
        {
            println!(
                "\n  {} unread messages",
                data.total_unread.to_string().yellow()
            );
        }
    }
    println!();
}

pub fn display_conversation(conv: &crate::api::types::Conversation) {
    let width = get_term_width();
    let unread_msg = if conv.unread_count > 0 {
        format!(" ({} unread)", conv.unread_count)
            .yellow()
            .to_string()
    } else {
        String::new()
    };

    println!(
        "{} {}{}",
        "💬".cyan(),
        conv.with_agent.name.bright_cyan().bold(),
        unread_msg
    );
    println!("   ID: {}", conv.conversation_id.dimmed());
    println!(
        "   Read: {}",
        format!("moltbook dm-read {}", conv.conversation_id).green()
    );
    println!("{}", "─".repeat(width).dimmed());
}

pub fn display_message(msg: &crate::api::types::Message) {
    let width = get_term_width();
    let prefix = if msg.from_you {
        "You"
    } else {
        &msg.from_agent.name
    };

    let (icon, color) = if msg.from_you {
        ("📤", prefix.green())
    } else {
        ("📥", prefix.yellow())
    };

    let time = relative_time(&msg.created_at);

    println!("\n{} {} ({})", icon, color.bold(), time.dimmed());

    let wrapped = textwrap::fill(&msg.message, width.saturating_sub(4));
    for line in wrapped.lines() {
        println!("  {}", line);
    }

    if msg.needs_human_input {
        println!("  {}", "⚠ Needs human input".red());
    }
    println!("{}", "─".repeat(width.min(40)).dimmed());
}
