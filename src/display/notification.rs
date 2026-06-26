// author: kelexine <https://github.com/kelexine>

use crate::api::types::{Notification, NotificationsResponse};
use crate::display::utils::{get_term_width, info, relative_time};
use colored::*;

pub fn display_notifications(response: &NotificationsResponse) {
    let width = get_term_width();

    let total = response.total.unwrap_or(0);
    let unread = response.unread_count.unwrap_or(0);

    println!("\n{} {}", "🔔".bright_yellow(), "Notifications".bright_yellow().bold());
    println!("{}", "━".repeat(width).dimmed());

    if unread > 0 {
        println!(
            "  {} unread  •  {} total",
            unread.to_string().yellow().bold(),
            total.to_string().dimmed()
        );
    } else {
        println!("  {} total  •  all read", total.to_string().dimmed());
    }
    println!();

    let notifications = match &response.notifications {
        Some(n) if !n.is_empty() => n,
        _ => {
            info("No notifications yet.");
            return;
        }
    };

    for notif in notifications {
        display_notification(notif, width);
    }

    if response.has_more.unwrap_or(false) {
        if let Some(cursor) = &response.next_cursor {
            println!(
                "  {} More notifications available — {}",
                "→".dimmed(),
                format!("moltbook notifications --cursor {}", cursor).cyan()
            );
        }
    }
    println!();
}

fn display_notification(notif: &Notification, width: usize) {
    let is_read = notif.is_read.unwrap_or(false);

    let indicator = if is_read {
        " ".normal()
    } else {
        "●".bright_yellow()
    };

    let type_icon = match notif.notification_type.as_deref() {
        Some("comment") => "💬",
        Some("reply") => "↩️ ",
        Some("upvote") => "▲",
        Some("mention") => "@",
        Some("dm") => "✉️ ",
        Some("follow") => "👤",
        _ => "•",
    };

    let from = notif
        .from_agent
        .as_ref()
        .map(|a| a.name.as_str())
        .unwrap_or("unknown");

    println!(
        "  {} {} {}  {}",
        indicator,
        type_icon,
        from.yellow(),
        notif.message.as_deref().unwrap_or("").dimmed()
    );

    if let Some(title) = &notif.post_title {
        let truncated = if title.chars().count() > width.saturating_sub(12) {
            format!("{}…", title.chars().take(width.saturating_sub(13)).collect::<String>())
        } else {
            title.clone()
        };
        println!("       {}", truncated.bright_cyan().italic());
    }

    let mut meta_parts: Vec<String> = Vec::new();
    if let Some(t) = &notif.created_at {
        meta_parts.push(relative_time(t));
    }
    if let Some(post_id) = &notif.post_id {
        meta_parts.push(format!("moltbook view-post {}", post_id));
    }
    if !meta_parts.is_empty() {
        println!("       {}", meta_parts.join("  •  ").dimmed());
    }
    println!();
}
