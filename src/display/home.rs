// author: kelexine <https://github.com/kelexine>

use crate::api::types::{HomeFollowingPost, HomePostActivity, HomeResponse};
use crate::display::utils::{get_term_width, info, relative_time, warn};
use colored::*;

pub fn display_home(home: &HomeResponse) {
    let width = get_term_width();
    let sep = "━".repeat(width);
    let thin = "─".repeat(width);

    println!("\n{} {}", "🏠".cyan(), "Home Dashboard".bright_green().bold());
    println!("{}", sep.dimmed());

    display_account_bar(home, width);
    display_dm_activity(home, &thin);
    display_post_activity(home, &thin, width);
    display_briefings(home, &thin, width);
    display_announcement(home, &thin);
    display_following_posts(home, &thin, width);
    display_explore(home, &thin);
    display_next_steps(home, &sep);
}

fn display_account_bar(home: &HomeResponse, width: usize) {
    if let Some(acct) = &home.your_account {
        let karma = acct.karma.unwrap_or(0);
        let notifs = acct.unread_notification_count.unwrap_or(0);

        let notif_str = if notifs > 0 {
            format!("🔔 {} unread", notifs).bright_yellow().to_string()
        } else {
            "🔔 all clear".dimmed().to_string()
        };

        println!(
            "  {:<width$}",
            format!(
                "👤 {}   ✨ {} karma   {}",
                acct.name.bright_white().bold(),
                karma.to_string().yellow(),
                notif_str
            ),
            width = width.saturating_sub(2)
        );
        println!();
    }
}

fn display_dm_activity(home: &HomeResponse, thin: &str) {
    let Some(dms) = &home.your_direct_messages else {
        return;
    };

    let unread = dms.unread_count.unwrap_or(0);
    let pending = dms.pending_requests.unwrap_or(0);

    if unread == 0 && pending == 0 {
        return;
    }

    println!("{} {}", "💬".bright_blue(), "Direct Messages".bright_blue().bold());
    println!("{}", thin.dimmed());

    if unread > 0 {
        println!("  {} unread message(s)", unread.to_string().yellow());
    }
    if pending > 0 {
        println!(
            "  {} pending request(s) — {}",
            pending.to_string().yellow(),
            "moltbook dm-requests".cyan()
        );
    }
    if let Some(summary) = &dms.summary {
        println!("  {}", summary.dimmed());
    }
    println!();
}

fn display_post_activity(home: &HomeResponse, thin: &str, width: usize) {
    let activity = match &home.activity_on_your_posts {
        Some(a) if !a.is_empty() => a,
        _ => return,
    };

    println!(
        "{} {}",
        "📬".bright_red(),
        "Activity on Your Posts".bright_red().bold()
    );
    println!("{}", thin.dimmed());

    for item in activity {
        display_post_activity_item(item, width);
    }
}

fn display_post_activity_item(item: &HomePostActivity, width: usize) {
    let title = item.post_title.as_deref().unwrap_or("(no title)");
    let submolt = item.submolt_name.as_deref().unwrap_or("?");
    let count = item.new_notification_count.unwrap_or(0);

    println!(
        "  {} {} {}",
        "▸".bright_white(),
        title.bright_cyan().bold(),
        format!("m/{}", submolt).green().dimmed()
    );

    if count > 0 {
        println!(
            "    {} new notification(s)",
            count.to_string().yellow().bold()
        );
    }

    if let Some(commenters) = &item.latest_commenters
        && !commenters.is_empty()
    {
        println!("    From: {}", commenters.join(", ").dimmed());
    }

    if let Some(preview) = &item.preview {
        let wrapped = textwrap::fill(preview, width.saturating_sub(8));
        for line in wrapped.lines() {
            println!("    {}", line.italic().dimmed());
        }
    }

    if let Some(t) = &item.latest_at {
        println!("    {}", relative_time(t).dimmed());
    }

    println!(
        "    {} {}",
        "→".dimmed(),
        format!("moltbook comments {}", item.post_id).cyan()
    );
    println!();
}

fn display_briefings(home: &HomeResponse, thin: &str, width: usize) {
    let briefings = match home.check_in.as_ref().and_then(|c| c.briefings.as_ref()) {
        Some(b) if !b.is_empty() => b,
        _ => return,
    };

    println!(
        "{} {}",
        "🎭".bright_magenta(),
        "Role Briefings".bright_magenta().bold()
    );
    println!("{}", thin.dimmed());

    for briefing in briefings {
        let role = briefing.your_role.as_deref().unwrap_or("unknown");
        let submolt = briefing.submolt_name.as_deref().unwrap_or("?");

        println!(
            "  {} {} in {}",
            "▸".bright_white(),
            role.bright_magenta().bold(),
            format!("m/{}", submolt).green()
        );

        if let Some(prompt) = &briefing.prompt {
            let wrapped = textwrap::fill(prompt, width.saturating_sub(6));
            for line in wrapped.lines() {
                println!("    {}", line.italic());
            }
        }

        if let Some(msg) = &briefing.message {
            println!("    {}", msg.dimmed());
        }

        if let Some(cadence) = briefing.cadence_minutes {
            if cadence > 0 {
                println!("    Cadence: every {}m", cadence.to_string().dimmed());
            }
        }
        println!();
    }
}

fn display_announcement(home: &HomeResponse, thin: &str) {
    let Some(ann) = &home.latest_moltbook_announcement else {
        return;
    };

    if ann.title.is_none() && ann.preview.is_none() {
        return;
    }

    println!(
        "{} {}",
        "📣".bright_yellow(),
        "Latest Announcement".bright_yellow().bold()
    );
    println!("{}", thin.dimmed());

    if let Some(title) = &ann.title {
        println!("  {}", title.bright_white().bold());
    }
    if let Some(preview) = &ann.preview {
        println!("  {}", preview.dimmed());
    }
    if let Some(id) = &ann.post_id {
        println!("  {} {}", "→".dimmed(), format!("moltbook view-post {}", id).cyan());
    }
    println!();
}

fn display_following_posts(home: &HomeResponse, thin: &str, width: usize) {
    let Some(section) = &home.posts_from_accounts_you_follow else {
        return;
    };

    let posts = match &section.posts {
        Some(p) if !p.is_empty() => p,
        _ => {
            if section.total_following.unwrap_or(0) == 0 {
                info("You're not following anyone yet. Follow moltys you enjoy to personalise your feed.");
            }
            return;
        }
    };

    let total = section.total_following.unwrap_or(0);
    println!(
        "{} {} (following {})",
        "👥".bright_blue(),
        "From Moltys You Follow".bright_blue().bold(),
        total.to_string().yellow()
    );
    println!("{}", thin.dimmed());

    for post in posts {
        display_following_post(post, width);
    }

    if let Some(hint) = &section.hint {
        println!("  {}", hint.dimmed());
    }
    if let Some(see_more) = &section.see_more {
        // strip the "GET " prefix if present for cleaner display
        let cmd = see_more
            .trim_start_matches("GET /api/v1/")
            .replace("feed?filter=following", "feed --filter following");
        println!("  {} {}", "→".dimmed(), format!("moltbook {}", cmd).cyan());
    }
    println!();
}

fn display_following_post(post: &HomeFollowingPost, width: usize) {
    let title = post.title.as_deref().unwrap_or("(no title)");
    let author = post.author_name.as_deref().unwrap_or("unknown");
    let submolt = post.submolt_name.as_deref().unwrap_or("?");

    println!(
        "  {} {} {}",
        "▸".bright_white(),
        title.bright_cyan(),
        format!("by {}", author).yellow().dimmed()
    );
    println!(
        "    {} | ▲ {} | 💬 {}",
        format!("m/{}", submolt).green().dimmed(),
        post.upvotes.unwrap_or(0),
        post.comment_count.unwrap_or(0)
    );

    if let Some(preview) = &post.content_preview {
        let truncated = if preview.chars().count() > width.saturating_sub(8) {
            format!("{}…", preview.chars().take(width.saturating_sub(9)).collect::<String>())
        } else {
            preview.clone()
        };
        println!("    {}", truncated.dimmed().italic());
    }

    if let Some(id) = &post.post_id {
        println!("    {} {}", "→".dimmed(), format!("moltbook view-post {}", id).cyan());
    }
    println!();
}

fn display_explore(home: &HomeResponse, thin: &str) {
    let Some(explore) = &home.explore else { return };

    println!("{} {}", "🌍".bright_green(), "Explore".bright_green().bold());
    println!("{}", thin.dimmed());

    if let Some(desc) = &explore.description {
        println!("  {}", desc.dimmed());
    }
    println!("  {} {}", "→".dimmed(), "moltbook feed".cyan());
    println!();
}

fn display_next_steps(home: &HomeResponse, sep: &str) {
    let steps = match &home.what_to_do_next {
        Some(s) if !s.is_empty() => s,
        _ => return,
    };

    println!("{}", sep.dimmed());
    println!("{} {}", "💡".bright_white(), "What to do next".bright_white().bold());

    for step in steps {
        println!("  {} {}", "•".dimmed(), step);
    }
    println!();
}

/// Displays a warning if the home response came back empty or malformed.
pub fn display_home_fallback() {
    warn("Home endpoint returned an empty response. Try running `moltbook status` or `moltbook heartbeat`.");
}
