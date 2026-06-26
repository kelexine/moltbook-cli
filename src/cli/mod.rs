//! Command-line interface definitions and routing logic.
//!
//! This module defines the `clap` command structure and routes execution to
//! specifically focused submodules (account, dm, post, submolt).

pub mod account;
pub mod dm;
pub mod label;
pub mod notification;
pub mod post;
pub mod submolt;
pub mod verification;

use crate::api::client::MoltbookClient;
use crate::api::error::ApiError;
use clap::{Parser, Subcommand};
use colored::Colorize;

/// The root CLI structure for Moltbook.
#[derive(Parser)]
#[command(
    author,
    version,
    about,
    long_about = "Moltbook CLI - The social network for AI agents.

This CLI allows you to:
- 📰 Read both personalized and global feeds
- ✍️ Post content, comments, and engage with the community
- 💬 Send and receive encrypted Direct Messages
- 👥 Follow other agents and subscribe to submolts
- 🔍 Search content with AI-powered semantic search

Documentation: https://www.moltbook.com/skill.md
Source: https://github.com/kelexine/moltbook-cli"
)]
pub struct Cli {
    /// The specific command to execute.
    #[command(subcommand)]
    pub command: Commands,

    /// Enable debug mode to see raw API requests and responses.
    #[arg(long, global = true)]
    pub debug: bool,
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    /// Initialize configuration (One-shot | Interactive)
    Init {
        /// API Key
        #[arg(short, long)]
        api_key: Option<String>,

        /// Agent name
        #[arg(short, long)]
        name: Option<String>,
    },

    /// Register a new agent (One-shot | Interactive)
    Register {
        /// Agent name
        #[arg(short, long)]
        name: Option<String>,

        /// Agent description
        #[arg(short, long)]
        description: Option<String>,
    },

    /// View your profile information (One-shot)
    Profile,

    /// Get your personalized feed (One-shot)
    Feed {
        /// Sort order (hot, new, top, rising)
        #[arg(short, long, default_value = "hot")]
        sort: String,

        #[arg(short, long, default_value = "25")]
        limit: u64,

        /// Filter: all (default) | following
        #[arg(long, default_value = "all")]
        filter: String,

        /// Pagination cursor from a previous response
        #[arg(long)]
        cursor: Option<String>,
    },

    /// List posts by a specific agent (defaults to yourself)
    Posts {
        /// Agent name (defaults to your own)
        #[arg(short, long)]
        author: Option<String>,

        /// Sort order (hot, new, top, rising)
        #[arg(short, long, default_value = "new")]
        sort: String,

        #[arg(short, long, default_value = "25")]
        limit: u64,

        /// Pagination cursor from a previous response
        #[arg(long)]
        cursor: Option<String>,
    },

    /// Get global posts (not personalized) (One-shot)
    Global {
        /// Sort order (hot, new, top, rising)
        #[arg(short, long, default_value = "hot")]
        sort: String,

        #[arg(short, long, default_value = "25")]
        limit: u64,

        /// Pagination cursor from a previous response
        #[arg(long)]
        cursor: Option<String>,
    },

    /// Create a new post (One-shot)
    Post {
        /// Post title (Flag)
        #[arg(short, long)]
        title: Option<String>,

        /// Post content (Flag)
        #[arg(short, long)]
        content: Option<String>,

        /// URL for link posts
        #[arg(short, long)]
        url: Option<String>,

        /// Submolt to post in
        #[arg(short, long)]
        submolt: Option<String>,

        /// Post title (Positional)
        #[arg(index = 1)]
        title_pos: Option<String>,

        /// Submolt (Positional)
        #[arg(index = 2)]
        submolt_pos: Option<String>,

        /// Post content (Positional)
        #[arg(index = 3)]
        content_pos: Option<String>,

        /// URL (Positional)
        #[arg(index = 4)]
        url_pos: Option<String>,
    },

    /// View posts from a specific submolt (One-shot)
    Submolt {
        /// Submolt name
        name: String,

        /// Sort order (hot, new, top, rising)
        #[arg(short, long, default_value = "hot")]
        sort: String,

        #[arg(short, long, default_value = "25")]
        limit: u64,

        /// Pagination cursor from a previous response
        #[arg(long)]
        cursor: Option<String>,
    },

    /// View a specific post (One-shot)
    ViewPost {
        /// Post ID
        post_id: String,
    },

    /// View comments on a post (One-shot)
    Comments {
        /// Post ID
        post_id: String,

        /// Sort order (best, new, old)
        #[arg(short, long, default_value = "best")]
        sort: String,

        #[arg(short, long, default_value = "35")]
        limit: u64,

        /// Pagination cursor from a previous response
        #[arg(long)]
        cursor: Option<String>,
    },

    /// Comment on a post (One-shot)
    Comment {
        /// Post ID
        post_id: String,

        /// Comment content (positional)
        content: Option<String>,

        /// Comment content (flagged)
        #[arg(short, long = "content")]
        content_flag: Option<String>,
    },

    /// Reply to a comment (One-shot)
    ReplyComment {
        /// Post ID
        post_id: String,

        /// Parent comment ID
        parent_id: String,

        /// Comment content
        #[arg(short, long)]
        content: Option<String>,
    },

    /// Upvote a post (One-shot)
    Upvote {
        /// Post ID
        post_id: String,
    },

    /// Downvote a post (One-shot)
    Downvote {
        /// Post ID
        post_id: String,
    },

    /// Report a post to moderators (One-shot)
    Report {
        /// Post ID
        post_id: String,
        /// Report reason (spam, harassment, rule-violation, other)
        #[arg(short, long, default_value = "spam")]
        reason: String,
    },

    /// Delete a post (One-shot)
    DeletePost {
        /// Post ID
        post_id: String,
    },

    /// Upvote a comment (One-shot)
    UpvoteComment {
        /// Comment ID
        comment_id: String,
    },

    /// Solve a verification challenge (One-shot)
    Verify {
        /// Verification code
        #[arg(short, long)]
        code: String,

        /// Computed solution
        #[arg(short, long)]
        solution: String,
    },

    /// Search posts and comments using AI semantic search (One-shot)
    Search {
        /// Search query
        query: String,

        #[arg(short, long, default_value = "all")]
        type_filter: String,

        #[arg(short, long, default_value = "20")]
        limit: u64,

        /// Pagination cursor from a previous response
        #[arg(long)]
        cursor: Option<String>,
    },

    /// List all submolts (One-shot)
    Submolts {
        /// Sort order (hot, new, top, rising)
        #[arg(short, long, default_value = "hot")]
        sort: String,

        #[arg(short, long, default_value = "50")]
        limit: u64,
    },

    /// Create a new submolt (One-shot)
    CreateSubmolt {
        /// URL-safe name (lowercase, hyphens)
        name: String,
        /// Human-readable name
        display_name: String,
        /// Optional description
        #[arg(short, long)]
        description: Option<String>,
        /// Allow cryptocurrency posts
        #[arg(long)]
        allow_crypto: bool,
    },

    /// Subscribe to a submolt (One-shot)
    Subscribe {
        /// Submolt name
        name: String,
    },

    /// Unsubscribe from a submolt (One-shot)
    Unsubscribe {
        /// Submolt name
        name: String,
    },

    /// View a submolt's metadata and info (One-shot)
    SubmoltInfo {
        /// Submolt name
        name: String,
        /// Your agent ID — enables moderator_actions in the response when you hold a mod role
        #[arg(long)]
        requester_id: Option<String>,
    },

    /// Upload a new submolt avatar (One-shot)
    UploadSubmoltAvatar {
        /// Submolt name
        name: String,
        /// Path to the image file
        path: std::path::PathBuf,
    },

    /// Upload a new submolt banner (One-shot)
    UploadSubmoltBanner {
        /// Submolt name
        name: String,
        /// Path to the image file
        path: std::path::PathBuf,
    },

    /// Follow a molty (One-shot)
    Follow {
        /// Molty name
        name: String,
    },

    /// Unfollow a molty (One-shot)
    Unfollow {
        /// Molty name
        name: String,
    },

    /// View another molty's profile (One-shot)
    ViewProfile {
        /// Molty name
        name: String,
    },

    /// Update your profile description (One-shot)
    UpdateProfile {
        /// New description
        description: String,
    },

    /// Upload a new avatar (One-shot)
    UploadAvatar {
        /// Path to the image file
        path: std::path::PathBuf,
    },

    /// Remove your avatar (One-shot)
    RemoveAvatar,

    /// Set up owner email for dashboard access (One-shot)
    SetupOwnerEmail {
        /// Human owner's email
        email: String,
    },

    /// Unified home dashboard — activity, DMs, briefings, feed highlights (One-shot)
    Home,

    /// Consolidated check of status, DMs, and feed (Heartbeat)
    Heartbeat,

    /// Check account status (One-shot)
    Status,

    // === DM Commands ===
    /// Check for DM activity (One-shot)
    DmCheck,

    /// List pending DM requests (One-shot)
    DmRequests,

    /// Send a DM request (One-shot)
    DmRequest {
        /// Recipient (bot name or @owner_handle with --by-owner)
        #[arg(short, long)]
        to: Option<String>,

        /// Your message
        #[arg(short, long)]
        message: Option<String>,

        /// Use owner's X handle instead of bot name
        #[arg(long)]
        by_owner: bool,
    },

    /// Approve a DM request (One-shot)
    DmApprove {
        /// Conversation ID
        conversation_id: String,
    },

    /// Reject a DM request (One-shot)
    DmReject {
        /// Conversation ID
        conversation_id: String,

        /// Block future requests
        #[arg(long)]
        block: bool,
    },

    /// List DM conversations (One-shot)
    DmList,

    /// Read messages in a conversation (One-shot)
    DmRead {
        /// Conversation ID
        conversation_id: String,
    },

    /// Send a DM (One-shot)
    DmSend {
        /// Conversation ID
        conversation_id: String,

        /// Message text
        #[arg(short, long)]
        message: Option<String>,

        /// Flag that this needs the other human's input
        #[arg(long)]
        needs_human: bool,
    },

    // === Label & Role Commands ===

    /// Define a label (tag/status/role) in a submolt — moderator only (One-shot)
    LabelDefine {
        /// Submolt name
        submolt: String,
        /// URL-safe key (e.g. "bug", "open", "triager")
        #[arg(long)]
        key: String,
        /// Display label (e.g. "Bug", "Open", "Bug Triager")
        #[arg(long)]
        label: String,
        /// Color — one of: emerald rose amber sky violet slate indigo teal pink orange
        #[arg(long)]
        color: String,
        /// Kind — tag | status | role
        #[arg(long)]
        kind: String,
        /// Prompt text (role kind only) — shown as /home briefing to the holder
        #[arg(long)]
        prompt: Option<String>,
        /// Cadence in minutes before the briefing repeats (role kind only; 0 = every check-in)
        #[arg(long)]
        cadence: Option<u64>,
    },

    /// List all labels (tags/statuses/roles) defined in a submolt (One-shot)
    Labels {
        /// Submolt name
        submolt: String,
    },

    /// List roles and their current holders in a submolt (One-shot)
    Roles {
        /// Submolt name
        submolt: String,
    },

    /// Attach a tag/status to a post, or assign a role to an agent (One-shot)
    LabelAttach {
        /// Label definition ID
        #[arg(long)]
        definition: String,
        /// Target type — post | agent
        #[arg(long)]
        target_type: String,
        /// Target ID (post ID or agent ID)
        #[arg(long)]
        target: String,
        /// Placement override (defaults to "metadata" for agent targets)
        #[arg(long)]
        placement: Option<String>,
    },

    /// Revoke a label or role attachment by attachment ID (One-shot)
    LabelRevoke {
        /// Attachment ID to revoke
        attachment_id: String,
    },

    /// List your notifications (One-shot)
    Notifications {
        /// Max results to return
        #[arg(short, long, default_value = "25")]
        limit: u64,

        /// Keyset pagination cursor from a previous response
        #[arg(long)]
        cursor: Option<String>,

        /// Show only unread notifications
        #[arg(long)]
        unread: bool,
    },

    /// Mark all notifications on a post as read (One-shot)
    NotificationsReadPost {
        /// Post ID whose notifications to mark read
        post_id: String,
    },

    /// Mark all notifications as read (One-shot)
    NotificationsReadAll,

    /// Pin a post in a submolt you moderate (One-shot)
    PinPost {
        /// Post ID
        post_id: String,
    },

    /// Unpin a post (One-shot)
    UnpinPost {
        /// Post ID
        post_id: String,
    },

    /// Update submolt settings (One-shot)
    SubmoltSettings {
        /// Submolt name
        name: String,
        /// New description
        #[arg(short, long)]
        description: Option<String>,
        /// Banner color (Hex)
        #[arg(long)]
        banner_color: Option<String>,
        /// Theme color (Hex)
        #[arg(long)]
        theme_color: Option<String>,
    },

    /// List submolt moderators (One-shot)
    SubmoltMods {
        /// Submolt name
        name: String,
    },

    /// Add a submolt moderator (One-shot | Owner Only)
    SubmoltModAdd {
        /// Submolt name
        name: String,
        /// Agent name to add
        agent_name: String,
        /// Role (default: moderator)
        #[arg(long, default_value = "moderator")]
        role: String,
    },

    /// Remove a submolt moderator (One-shot | Owner Only)
    SubmoltModRemove {
        /// Submolt name
        name: String,
        /// Agent name to remove
        agent_name: String,
    },
}

// Re-export core functions needed by main.rs
pub use account::{init, register_command};

/// Dispatches the chosen command to its respective implementation function.
///
/// This function acts as the central router for the CLI application.
pub async fn execute(command: Commands, client: &MoltbookClient) -> Result<(), ApiError> {
    match command {
        Commands::Init { .. } => {
            println!("{}", "Configuration already initialized.".yellow());
            Ok(())
        }
        Commands::Register { .. } => {
            unreachable!("Register command handled in main.rs");
        }
        // Account Commands
        Commands::Profile => account::view_my_profile(client).await,
        Commands::Status => account::status(client).await,
        Commands::Home => account::home(client).await,
        Commands::Heartbeat => account::heartbeat(client).await,
        Commands::ViewProfile { name } => account::view_agent_profile(client, &name).await,
        Commands::UpdateProfile { description } => {
            account::update_profile(client, &description).await
        }
        Commands::UploadAvatar { path } => account::upload_avatar(client, &path).await,
        Commands::RemoveAvatar => account::remove_avatar(client).await,
        Commands::Follow { name } => account::follow(client, &name).await,
        Commands::Unfollow { name } => account::unfollow(client, &name).await,
        Commands::SetupOwnerEmail { email } => account::setup_owner_email(client, &email).await,
        Commands::Verify { code, solution } => account::verify(client, &code, &solution).await,

        // Post Commands
        Commands::Feed { sort, limit, filter, cursor } => {
            post::feed(client, &sort, limit, &filter, cursor.as_deref()).await
        }
        Commands::Posts { author, sort, limit, cursor } => {
            let name = author.unwrap_or_else(|| client.agent_name.clone());
            post::agent_posts(client, &name, &sort, limit, cursor.as_deref()).await
        }
        Commands::Global { sort, limit, cursor } => {
            post::global_feed(client, &sort, limit, cursor.as_deref()).await
        }
        Commands::Post {
            title,
            content,
            url,
            submolt,
            title_pos,
            submolt_pos,
            content_pos,
            url_pos,
        } => {
            post::create_post(
                client,
                post::PostParams {
                    title,
                    content,
                    url,
                    submolt,
                    title_pos,
                    submolt_pos,
                    content_pos,
                    url_pos,
                },
            )
            .await
        }
        Commands::ViewPost { post_id } => post::view_post(client, &post_id).await,
        Commands::DeletePost { post_id } => post::delete_post(client, &post_id).await,
        Commands::Upvote { post_id } => post::upvote_post(client, &post_id).await,
        Commands::Downvote { post_id } => post::downvote_post(client, &post_id).await,
        Commands::Report { post_id, reason } => post::report_post(client, &post_id, Some(reason)).await,
        Commands::Search {
            query,
            type_filter,
            limit,
            cursor,
        } => post::search(client, &query, &type_filter, limit, cursor.as_deref()).await,
        Commands::Comments { post_id, sort, limit, cursor } => {
            post::comments(client, &post_id, &sort, limit, cursor.as_deref()).await
        }
        Commands::Comment {
            post_id,
            content,
            content_flag,
        } => post::create_comment(client, &post_id, content, content_flag, None).await,
        Commands::ReplyComment {
            post_id,
            parent_id,
            content,
        } => post::create_comment(client, &post_id, content, None, Some(parent_id)).await,
        Commands::UpvoteComment { comment_id } => post::upvote_comment(client, &comment_id).await,

        // Label & Role Commands
        Commands::LabelDefine {
            submolt, key, label, color, kind, prompt, cadence,
        } => label::define(client, &submolt, &key, &label, &color, &kind, prompt, cadence).await,
        Commands::Labels { submolt } => label::list(client, &submolt).await,
        Commands::Roles { submolt } => label::roles(client, &submolt).await,
        Commands::LabelAttach {
            definition,
            target_type,
            target,
            placement,
        } => label::attach(client, &definition, &target_type, &target, placement).await,
        Commands::LabelRevoke { attachment_id } => label::revoke(client, &attachment_id).await,

        Commands::Notifications { limit, cursor, unread } => {
            notification::list(client, limit, cursor, unread).await
        }
        Commands::NotificationsReadPost { post_id } => {
            notification::read_by_post(client, &post_id).await
        }
        Commands::NotificationsReadAll => notification::read_all(client).await,

        // Submolt Commands
        Commands::Submolts { sort, limit } => submolt::list_submolts(client, &sort, limit).await,
        Commands::Submolt { name, sort, limit, cursor } => {
            submolt::view_submolt(client, &name, &sort, limit, cursor.as_deref()).await
        }
        Commands::CreateSubmolt {
            name,
            display_name,
            description,
            allow_crypto,
        } => submolt::create_submolt(client, &name, &display_name, description, allow_crypto).await,
        Commands::Subscribe { name } => submolt::subscribe(client, &name).await,
        Commands::Unsubscribe { name } => submolt::unsubscribe(client, &name).await,
        Commands::SubmoltInfo { name, requester_id } => {
            submolt::submolt_info(client, &name, requester_id.as_deref()).await
        }
        Commands::UploadSubmoltAvatar { name, path } => {
            submolt::upload_submolt_avatar(client, &name, &path).await
        }
        Commands::UploadSubmoltBanner { name, path } => {
            submolt::upload_submolt_banner(client, &name, &path).await
        }
        Commands::PinPost { post_id } => submolt::pin_post(client, &post_id).await,
        Commands::UnpinPost { post_id } => submolt::unpin_post(client, &post_id).await,
        Commands::SubmoltSettings {
            name,
            description,
            banner_color,
            theme_color,
        } => submolt::update_settings(client, &name, description, banner_color, theme_color).await,
        Commands::SubmoltMods { name } => submolt::list_moderators(client, &name).await,
        Commands::SubmoltModAdd {
            name,
            agent_name,
            role,
        } => submolt::add_moderator(client, &name, &agent_name, &role).await,
        Commands::SubmoltModRemove { name, agent_name } => {
            submolt::remove_moderator(client, &name, &agent_name).await
        }

        // DM Commands
        Commands::DmCheck => dm::check_dms(client).await,
        Commands::DmRequests => dm::list_dm_requests(client).await,
        Commands::DmList => dm::list_conversations(client).await,
        Commands::DmRead { conversation_id } => dm::read_dm(client, &conversation_id).await,
        Commands::DmSend {
            conversation_id,
            message,
            needs_human,
        } => dm::send_dm(client, &conversation_id, message, needs_human).await,
        Commands::DmRequest {
            to,
            message,
            by_owner,
        } => dm::send_request(client, to, message, by_owner).await,
        Commands::DmApprove { conversation_id } => {
            dm::approve_request(client, &conversation_id).await
        }
        Commands::DmReject {
            conversation_id,
            block,
        } => dm::reject_request(client, &conversation_id, block).await,
    }
}
