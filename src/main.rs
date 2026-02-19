//! Binary entry point for the Moltbook CLI.
//!
//! This module handles the top-level CLI execution flow:
//! 1. Parses command-line arguments.
//! 2. Routes commands that don't require configuration (init, register).
//! 3. Loads agent credentials and initializes the API client for all other commands.
//! 4. Executes requested operations and handles top-level errors.

use clap::Parser;
use colored::Colorize;
use moltbook_cli::api::client::MoltbookClient;
use moltbook_cli::cli::{self, Cli, Commands};
use moltbook_cli::config::Config;
use moltbook_cli::display;
use std::process;

#[tokio::main]
async fn main() {
    let cli = Cli::parse();

    // Handle commands that don't require config separately
    match cli.command {
        Commands::Init { api_key, name } => {
            if let Err(e) = cli::init(api_key, name).await {
                display::error(&format!("Setup Error: {}", e));
                process::exit(1);
            }
        }
        Commands::Register { name, description } => {
            if let Err(e) = cli::register_command(name, description).await {
                display::error(&format!("Registration Error: {}", e));
                process::exit(1);
            }
        }
        cmd => {
            // Load config for all other commands
            let config = match Config::load() {
                Ok(cfg) => cfg,
                Err(e) => {
                    display::error(&format!("Configuration Error: {}", e));
                    println!(
                        "Run '{}' to set up your configuration.",
                        "moltbook init".yellow()
                    );
                    process::exit(1);
                }
            };

            let client = MoltbookClient::new(config.api_key, cli.debug);

            if let Err(e) = cli::execute(cmd, &client).await {
                display::error(&format!("{}", e));
                process::exit(1);
            }
        }
    }
}

