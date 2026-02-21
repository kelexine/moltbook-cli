use crate::api::types::Post;
use crate::display::utils::{get_term_width, relative_time};
use colored::*;

/// Renders a Moltbook post in a premium box-styled layout.
///
/// # Arguments
///
/// * `post` - The post object to display.
/// * `index` - Optional positional index for use in lists.
pub fn display_post(post: &Post, index: Option<usize>) {
    let width = get_term_width();

    let mut prefix_str = String::new();
    if let Some(i) = index {
        prefix_str.push_str(&format!("#{:<2} ", i));
    }
    if post.is_pinned.unwrap_or(false) {
        prefix_str.push_str("📌 ");
    }
    if post.is_locked.unwrap_or(false) {
        prefix_str.push_str("🔒 ");
    }

    let prefix = if !prefix_str.is_empty() {
        prefix_str.bright_white().bold()
    } else {
        "".normal()
    };

    println!("{}Title: {}", prefix, post.title.bright_cyan().bold());

    let mut author_display = post.author.name.yellow().to_string();
    if post.you_follow_author.unwrap_or(false) {
        author_display.push_str(&" [Following]".bright_blue().to_string());
    }

    // Handle submolt name fallback
    let sub_name = if let Some(s) = &post.submolt {
        &s.name
    } else if let Some(s) = &post.submolt_name {
        s
    } else {
        "unknown"
    };

    let sub = sub_name.green();

    let score_str = if let Some(score) = post.score {
        format!(" | score ({})", score)
    } else {
        String::new()
    };

    let stats = format!(
        "upvotes ({}) | downvotes ({}) | comments ({}){}",
        post.upvotes,
        post.downvotes,
        post.comment_count.unwrap_or(0),
        score_str
    );

    println!("👤 {} in m/{} {}", author_display, sub, stats.dimmed());

    if let Some(content) = &post.content {
        let is_listing = index.is_some();
        let max_lines = if is_listing { 3 } else { 1000 };

        let wrapped_width = width.saturating_sub(4);
        let wrapped = textwrap::fill(content, wrapped_width);

        for (i, line) in wrapped.lines().enumerate() {
            if i >= max_lines {
                println!("│  {}", "...".dimmed());
                break;
            }
            println!("│  {}", line);
        }
    }

    if let Some(url) = &post.url {
        println!("│  🔗 {}", url.blue().underline());
    }

    println!(
        "└─ Post ID: {} • {}",
        post.id.dimmed(),
        relative_time(&post.created_at).dimmed()
    );
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
    println!("└─ Comment ID: {}", id.dimmed());
    println!();
}
