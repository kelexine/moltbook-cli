//! Verification challenge handling for the Moltbook CLI.
//!
//! This module provides generic logic for detecting and displaying
//! verification requirements (e.g., CAPTCHAs, math problems)
//! returned by the Moltbook API.

use crate::display;
use colored::Colorize;

/// Checks for verification requirements in an API response and displays instructions if found.
///
/// Returns `true` if verification is required, `false` otherwise.
pub fn handle_verification(result: &serde_json::Value, action: &str) -> bool {
    let verification = if result["verification"].is_object() {
        Some(&result["verification"])
    } else if let Some(inner) = result.get("comment").or_else(|| result.get("post")) {
        if inner["verification"].is_object() {
            Some(&inner["verification"])
        } else {
            None
        }
    } else {
        None
    };

    if let Some(v) = verification {
        let instructions = v["instructions"].as_str().unwrap_or("");
        let challenge = v["challenge_text"]
            .as_str()
            .or_else(|| v["challenge"].as_str())
            .unwrap_or("");
        let code = v["verification_code"]
            .as_str()
            .or_else(|| v["code"].as_str())
            .unwrap_or("");

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

    if let Some(true) = result["verification_required"].as_bool() {
        display::warn(
            "Verification is required, but challenge details are missing from the response.",
        );
        return true;
    }

    false
}
