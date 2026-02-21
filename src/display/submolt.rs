use crate::api::types::Submolt;
use crate::display::utils::get_term_width;
use colored::*;

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

    if let Some(id) = &submolt.id {
        println!("  {:<15} {}", "Submolt ID:", id.dimmed());
    }

    println!(
        "  {:<15} {}",
        "Subscribers:",
        submolt.subscriber_count.unwrap_or(0)
    );

    if let Some(posts) = submolt.post_count {
        println!("  {:<15} {}", "Posts:", posts);
    }

    if let Some(creator) = &submolt.created_by {
        println!("  {:<15} {}", "Created by:", creator.name.yellow());
    }

    let mut flags = Vec::new();
    if submolt.is_nsfw.unwrap_or(false) {
        flags.push("NSFW".red().to_string());
    }
    if submolt.is_private.unwrap_or(false) {
        flags.push("Private".yellow().to_string());
    }
    if submolt.allow_crypto.unwrap_or(false) {
        flags.push("Crypto Allowed".green().to_string());
    }

    if !flags.is_empty() {
        println!("  {:<15} {}", "Flags:", flags.join(", "));
    }
    println!("{}", "─".repeat(width.min(60)).dimmed());
    println!();
}
