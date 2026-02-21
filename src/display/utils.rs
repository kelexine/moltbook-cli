use chrono::{DateTime, Utc};
use colored::*;
use terminal_size::{Width, terminal_size};

/// Detects the available terminal width for responsive layout.
///
/// Priority:
/// 1. `COLUMNS` environment variable.
/// 2. `terminal_size` system call.
/// 3. Default fallback of 80 characters.
pub fn get_term_width() -> usize {
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
pub fn relative_time(timestamp: &str) -> String {
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
