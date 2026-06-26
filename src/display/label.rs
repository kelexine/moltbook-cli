// author: kelexine <https://github.com/kelexine>

use crate::api::types::{ConsiderLabel, LabelDefinition, LabelsResponse, RolesResponse};
use crate::display::utils::{get_term_width, info};
use colored::*;

/// Maps a Moltbook label color name to a terminal color approximation.
fn colorize_label(text: &str, color: Option<&str>) -> ColoredString {
    match color {
        Some("emerald") | Some("teal") => text.bright_green(),
        Some("rose") | Some("pink") => text.bright_red(),
        Some("amber") | Some("orange") => text.yellow(),
        Some("sky") | Some("indigo") => text.bright_blue(),
        Some("violet") => text.bright_magenta(),
        Some("slate") => text.bright_black(),
        _ => text.white(),
    }
}

/// Kind badge with icon.
fn kind_badge(kind: &str) -> ColoredString {
    match kind {
        "tag" => "[tag]".cyan(),
        "status" => "[status]".yellow(),
        "role" => "[role]".bright_magenta(),
        other => format!("[{}]", other).normal(),
    }
}

pub fn display_labels(response: &LabelsResponse, submolt_name: &str) {
    let width = get_term_width();

    println!(
        "\n{} {} {}",
        "🏷️ ".bright_cyan(),
        format!("Labels — m/{}", submolt_name).bright_cyan().bold(),
        ""
    );
    println!("{}", "━".repeat(width).dimmed());

    let labels = match &response.labels {
        Some(l) if !l.is_empty() => l,
        _ => {
            info("No labels defined for this submolt.");
            return;
        }
    };

    // Group by kind for readability
    for kind in &["tag", "status", "role"] {
        let group: Vec<&LabelDefinition> =
            labels.iter().filter(|l| l.kind == *kind).collect();
        if group.is_empty() {
            continue;
        }

        println!(
            "\n  {} {}",
            kind_badge(kind),
            format!("{}s", kind).dimmed()
        );
        println!("  {}", "─".repeat(width.saturating_sub(4)).dimmed());

        for label in group {
            display_label_row(label, width);
        }
    }
    println!();
}

fn display_label_row(label: &LabelDefinition, width: usize) {
    let color = label.color.as_deref();
    let colored_name = colorize_label(&label.label, color);
    let color_tag = color.unwrap_or("—").dimmed();

    println!(
        "  ● {}  {}  key: {}",
        colored_name.bold(),
        color_tag,
        label.key.dimmed()
    );
    println!("    id: {}", label.id.dimmed());

    if label.kind == "role" {
        if let Some(prompt) = &label.prompt {
            let wrapped = textwrap::fill(prompt, width.saturating_sub(8));
            for line in wrapped.lines() {
                println!("    prompt: {}", line.italic().dimmed());
            }
        }
        if let Some(cadence) = label.cadence_minutes {
            if cadence > 0 {
                println!("    cadence: every {}m", cadence.to_string().dimmed());
            }
        }
    }
    println!();
}

pub fn display_roles(response: &RolesResponse, submolt_name: &str) {
    let width = get_term_width();

    println!(
        "\n{} {}",
        "🎭".bright_magenta(),
        format!("Roles — m/{}", submolt_name).bright_magenta().bold()
    );
    println!("{}", "━".repeat(width).dimmed());

    let roles = match &response.roles {
        Some(r) if !r.is_empty() => r,
        _ => {
            info("No roles defined for this submolt.");
            return;
        }
    };

    for role in roles {
        let color = role.color.as_deref();
        let colored_name = colorize_label(&role.label, color);

        println!(
            "  ● {}  key: {}  id: {}",
            colored_name.bold(),
            role.key.dimmed(),
            role.id.dimmed()
        );

        if let Some(prompt) = &role.prompt {
            let wrapped = textwrap::fill(prompt, width.saturating_sub(8));
            for line in wrapped.lines() {
                println!("    {}", line.italic().dimmed());
            }
        }

        if let Some(cadence) = role.cadence_minutes {
            if cadence > 0 {
                println!("    cadence: every {}m", cadence.to_string().dimmed());
            }
        }

        let holders = role.holders.as_deref().unwrap_or(&[]);
        if holders.is_empty() {
            println!("    {}", "no current holders".dimmed());
        } else {
            let names: Vec<&str> = holders
                .iter()
                .filter_map(|h| h.agent.as_ref().map(|a| a.name.as_str()))
                .collect();
            println!("    holders: {}", names.join(", ").yellow());

            // Show attachment IDs for revocation
            for h in holders {
                if let (Some(id), Some(agent)) = (&h.attachment_id, &h.agent) {
                    println!(
                        "      {} {} → {}",
                        "attachment:".dimmed(),
                        id.dimmed(),
                        agent.name.yellow()
                    );
                }
            }
        }
        println!();
    }
}

/// Shown after post creation when the submolt has labels but none were attached.
pub fn display_consider_labels(labels: &[ConsiderLabel]) {
    if labels.is_empty() {
        return;
    }

    println!(
        "\n  {} {}",
        "🏷️ ".bright_cyan(),
        "This submolt has labels — consider attaching one:".bright_cyan()
    );
    for label in labels {
        let name = label.label.as_deref().unwrap_or("?");
        let kind = label.kind.as_deref().unwrap_or("tag");
        let color = label.color.as_deref();
        let id = label.id.as_deref().unwrap_or("?");

        println!(
            "    {} {} {}  {}",
            kind_badge(kind),
            colorize_label(name, color).bold(),
            id.dimmed(),
            format!("moltbook label-attach --definition {} --target-type post --target <POST_ID>", id).cyan().dimmed()
        );
    }
    println!();
}
