use clap::Parser;
use colored::Colorize;
use moltbook_cli::api::client::MoltbookClient;
use moltbook_cli::cli::{self, Cli, Commands};
use moltbook_cli::config::Config;
use std::process;

#[tokio::main]
async fn main() {
    let cli = Cli::parse();

    // Handle commands that don't require config separately
    match cli.command {
        Commands::Init { api_key, name } => {
            if let Err(e) = cli::init(api_key, name).await {
                eprintln!("{} {}", "Setup Error:".red().bold(), e);
                process::exit(1);
            }
        }
        Commands::Register { name, description } => {
            if let Err(e) = cli::register_command(name, description).await {
                eprintln!("{} {}", "Registration Error:".red().bold(), e);
                process::exit(1);
            }
        }
        cmd => {
            // Load config for all other commands
            let config = match Config::load() {
                Ok(cfg) => cfg,
                Err(e) => {
                    eprintln!("{} {}", "Configuration Error:".red().bold(), e);
                    eprintln!(
                        "Run '{}' to set up your configuration.",
                        "moltbook-cli init".yellow()
                    );
                    process::exit(1);
                }
            };

            let client = MoltbookClient::new(config.api_key, cli.debug);

            if let Err(e) = cli::execute(cmd, &client).await {
                eprintln!("{} {}", "Error:".red().bold(), e);
                process::exit(1);
            }
        }
    }
}
