//! Verification challenge handling for the Moltbook CLI.
//!
//! This module provides generic logic for detecting and displaying 
//! verification requirements (e.g., CAPTCHAs, math problems) 
//! returned by the Moltbook API.

use colored::Colorize;

/// Checks for verification requirements in an API response and displays instructions if found.
///
/// Returns `true` if verification is required, `false` otherwise.
pub fn handle_verification(result: &serde_json::Value, action: &str) -> bool {
    if let Some(true) = result["verification_required"].as_bool() {
        if let Some(verification) = result.get("verification") {
            let instructions = verification["instructions"].as_str().unwrap_or("");
            let challenge = verification["challenge"].as_str().unwrap_or("");
            let code = verification["code"].as_str().unwrap_or("");

            println!("\n{}", "🔒 Verification Required".yellow().bold());
            println!("{}", instructions);
            println!("Challenge: {}\n", challenge.cyan().bold());
            println!("To complete your {}, run:", action);
            println!(
                "  moltbook verify --code \"{}\" --solution \"<YOUR_ANSWER>\"",
                code
            );
            return true;
        }
    }
    false
}
