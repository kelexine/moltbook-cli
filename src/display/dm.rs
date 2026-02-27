use crate::api::types::{Conversation, DmCheckResponse, DmRequest, Message};
use crate::display::utils::{get_term_width, relative_time};
use colored::*;

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
        "│ Request ID: {:<w$} │",
        req.conversation_id.dimmed(),
        w = inner_width.saturating_sub(12)
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

pub fn display_dm_check(response: &DmCheckResponse) {
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
                println!("    Request ID: {}", conv_id);
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

pub fn display_conversation(conv: &Conversation) {
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
    println!("   Conversation ID: {}", conv.conversation_id.dimmed());
    println!(
        "   Read: {}",
        format!("moltbook dm-read {}", conv.conversation_id).green()
    );
    println!("{}", "─".repeat(width).dimmed());
}

pub fn display_message(msg: &Message, my_name: &str) {
    let width = get_term_width();
    let from_you = msg.sender.name == my_name;
    let prefix = if from_you {
        "You"
    } else {
        &msg.sender.name
    };

    let (icon, color) = if from_you {
        ("📤", prefix.green())
    } else {
        ("📥", prefix.yellow())
    };

    let time = relative_time(&msg.created_at);

    println!("\n{} {} ({})", icon, color.bold(), time.dimmed());

    let wrapped = textwrap::fill(&msg.content, width.saturating_sub(4));
    for line in wrapped.lines() {
        println!("  {}", line);
    }

    if msg.needs_human_input {
        println!("  {}", "⚠ Needs human input".red());
    }
    println!("{}", "─".repeat(width.min(40)).dimmed());
}
