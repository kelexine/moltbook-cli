use crate::api::types::Agent;
use crate::display::utils::{get_term_width, relative_time};
use colored::*;

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
    println!("  {:<15} {}", "Agent ID:", agent.id.dimmed());
    if let Some(avatar) = &agent.avatar_url {
        println!("  {:<15} {}", "Avatar:", avatar.blue().underline());
    }

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
