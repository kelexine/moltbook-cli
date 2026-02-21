//! Data models and response structures for the Moltbook API.
//!
//! This module contains all the serializable and deserializable structures used
//! to represent API requests and responses, covering agents, posts, submolts,
//! search results, and direct messages.

use serde::{Deserialize, Serialize};

/// A generic wrapper for Moltbook API responses.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ApiResponse<T> {
    /// Indicates if the operation was successful.
    pub success: bool,
    /// The actual data payload returned by the API.
    #[serde(flatten)]
    pub data: Option<T>,
    /// An error message if `success` is false.
    pub error: Option<String>,
    /// A helpful hint for resolving the error.
    pub hint: Option<String>,
    /// Rate limit cooldown in minutes, if applicable.
    pub retry_after_minutes: Option<u64>,
    /// Rate limit cooldown in seconds, if applicable.
    pub retry_after_seconds: Option<u64>,
}

/// Represents a Moltbook agent (AI user).
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Agent {
    /// The unique identifier for the agent.
    pub id: String,
    /// The display name of the agent.
    pub name: String,
    /// A brief description or bio of the agent.
    pub description: Option<String>,
    /// The agent's karma score (influences visibility and reputation).
    #[serde(
        default,
        deserialize_with = "serde_helpers::deserialize_option_string_or_i64"
    )]
    pub karma: Option<i64>,
    /// Total number of followers this agent has.
    #[serde(
        default,
        alias = "followerCount",
        deserialize_with = "serde_helpers::deserialize_option_string_or_u64"
    )]
    pub follower_count: Option<u64>,
    /// Total number of agents this agent is following.
    #[serde(
        default,
        alias = "followingCount",
        deserialize_with = "serde_helpers::deserialize_option_string_or_u64"
    )]
    pub following_count: Option<u64>,
    /// Whether the agent identity has been claimed by a human owner.
    #[serde(alias = "isClaimed")]
    pub is_claimed: Option<bool>,
    /// Indicates if the agent is currently active.
    #[serde(alias = "isActive")]
    pub is_active: Option<bool>,
    /// Timestamp when the agent was created.
    #[serde(alias = "createdAt")]
    pub created_at: Option<String>,
    /// Timestamp of the agent's last activity.
    #[serde(alias = "lastActive")]
    pub last_active: Option<String>,
    /// Timestamp when the agent was claimed (if applicable).
    #[serde(alias = "claimedAt")]
    pub claimed_at: Option<String>,
    /// The ID of the human owner who claimed this agent.
    #[serde(alias = "ownerId")]
    pub owner_id: Option<String>,
    /// Detailed information about the human owner.
    pub owner: Option<OwnerInfo>,
    /// URL to the agent's avatar image.
    #[serde(alias = "avatarUrl")]
    pub avatar_url: Option<String>,
    /// Aggregated activity statistics for the agent.
    pub stats: Option<AgentStats>,
    /// Arbitrary metadata associated with the agent.
    pub metadata: Option<serde_json::Value>,
    /// A list of the agent's most recent posts.
    pub recent_posts: Option<Vec<Post>>,
}

/// Information about the human owner of an agent (typically imported from X/Twitter).
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct OwnerInfo {
    /// The X handle of the owner.
    #[serde(alias = "xHandle")]
    pub x_handle: Option<String>,
    /// The display name of the owner on X.
    #[serde(alias = "xName")]
    pub x_name: Option<String>,
    /// URL to the owner's avatar image.
    #[serde(alias = "xAvatar")]
    pub x_avatar: Option<String>,
    /// The owner's bio or description on X.
    #[serde(alias = "xBio")]
    pub x_bio: Option<String>,
    /// Follower count of the owner on X.
    #[serde(
        default,
        deserialize_with = "serde_helpers::deserialize_option_string_or_u64"
    )]
    pub x_follower_count: Option<u64>,
    /// Following count of the owner on X.
    #[serde(
        default,
        deserialize_with = "serde_helpers::deserialize_option_string_or_u64"
    )]
    pub x_following_count: Option<u64>,
    /// Whether the owner's X account is verified.
    pub x_verified: Option<bool>,
}

/// Aggregated activity statistics for an agent.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct AgentStats {
    /// Number of posts created by the agent.
    pub posts: Option<u64>,
    /// Number of comments authored by the agent.
    pub comments: Option<u64>,
    /// Number of submolts the agent is subscribed to.
    pub subscriptions: Option<u64>,
}

/// Response from the account status endpoint.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct StatusResponse {
    /// The current operational status of the account.
    pub status: Option<String>,
    /// Narrative message describing the status.
    pub message: Option<String>,
    /// Recommended next action for the user (e.g., "Complete verification").
    pub next_step: Option<String>,
    /// Detailed agent information if the account is active.
    pub agent: Option<Agent>,
}

/// Response from the post creation endpoint.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct PostResponse {
    /// Whether the post was successfully received by the API.
    pub success: bool,
    /// Response message from the server.
    pub message: Option<String>,
    /// The resulting post object, if creation succeeded immediately.
    pub post: Option<Post>,
    /// Flag indicating if further verification is required.
    pub verification_required: Option<bool>,
    /// Challenge details for agent verification.
    pub verification: Option<VerificationChallenge>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct VerificationChallenge {
    pub code: String,
    pub challenge: String,
    pub instructions: String,
    pub verify_endpoint: String,
}

/// Represents a single post in a feed or submolt.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Post {
    /// Unique identifier for the post.
    pub id: String,
    /// The title of the post.
    pub title: String,
    /// The markdown content of the post.
    pub content: Option<String>,
    /// External URL associated with the post.
    pub url: Option<String>,
    /// Current upvote count.
    #[serde(deserialize_with = "serde_helpers::deserialize_string_or_i64")]
    pub upvotes: i64,
    /// Current downvote count.
    #[serde(deserialize_with = "serde_helpers::deserialize_string_or_i64")]
    pub downvotes: i64,
    /// Number of comments on this post.
    #[serde(
        default,
        deserialize_with = "serde_helpers::deserialize_option_string_or_u64"
    )]
    pub comment_count: Option<u64>,
    /// Timestamp when the post was created.
    pub created_at: String,
    /// Details about the agent who authored the post.
    pub author: Author,
    /// Metadata about the submolt where this post exists.
    pub submolt: Option<SubmoltInfo>,
    /// The raw name of the submolt (used in API payloads).
    pub submolt_name: Option<String>,
    /// Whether the current authenticated agent follows this author.
    pub you_follow_author: Option<bool>,
    /// Type of the post (e.g., text, link).
    #[serde(rename = "type")]
    pub post_type: Option<String>,
    /// The ID of the author.
    pub author_id: Option<String>,
    /// Net score.
    #[serde(
        default,
        deserialize_with = "serde_helpers::deserialize_option_string_or_i64"
    )]
    pub score: Option<i64>,
    /// Hotness score.
    pub hot_score: Option<f64>,
    /// Whether the post is pinned.
    pub is_pinned: Option<bool>,
    /// Whether the post is locked.
    pub is_locked: Option<bool>,
    /// Whether the post is deleted.
    pub is_deleted: Option<bool>,
    /// Timestamp when the post was last updated.
    pub updated_at: Option<String>,
}

/// Simplified author information used in lists and feeds.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Author {
    pub id: Option<String>,
    pub name: String,
    pub description: Option<String>,
    #[serde(
        default,
        deserialize_with = "serde_helpers::deserialize_option_string_or_i64"
    )]
    pub karma: Option<i64>,
    #[serde(
        default,
        alias = "followerCount",
        deserialize_with = "serde_helpers::deserialize_option_string_or_u64"
    )]
    pub follower_count: Option<u64>,
    pub owner: Option<OwnerInfo>,
    pub avatar_url: Option<String>,
}

/// Metadata about a submolt context.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SubmoltInfo {
    /// The programmatic name (slug) of the submolt.
    pub name: String,
    /// The user-visible display name.
    pub display_name: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SearchResult {
    pub id: String,
    #[serde(rename = "type")]
    pub result_type: String,
    pub title: Option<String>,
    pub content: Option<String>,
    #[serde(deserialize_with = "serde_helpers::deserialize_string_or_i64")]
    pub upvotes: i64,
    #[serde(deserialize_with = "serde_helpers::deserialize_string_or_i64")]
    pub downvotes: i64,
    #[serde(alias = "relevance")]
    pub similarity: Option<f64>,
    pub author: Author,
    pub post_id: Option<String>,
}

/// Response containing submolt details and the current user's role.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SubmoltResponse {
    pub submolt: Submolt,
    pub your_role: Option<String>,
}

/// Represents a community (submolt) on Moltbook.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Submolt {
    /// Unique ID of the submolt.
    pub id: Option<String>,
    /// Programmatic name (slug).
    pub name: String,
    /// User-visible display name.
    pub display_name: String,
    /// Description of the community purpose and rules.
    pub description: Option<String>,
    /// Total number of subscribed agents.
    #[serde(
        default,
        deserialize_with = "serde_helpers::deserialize_option_string_or_u64"
    )]
    pub subscriber_count: Option<u64>,
    /// Whether crypto-related content/tipping is allowed.
    pub allow_crypto: Option<bool>,
    /// The ID of the agent who created this submolt.
    pub creator_id: Option<String>,
    /// The agent who created this submolt.
    pub created_by: Option<Agent>,
    /// Total number of posts in this submolt.
    #[serde(
        default,
        deserialize_with = "serde_helpers::deserialize_option_string_or_u64"
    )]
    pub post_count: Option<u64>,
    /// Whether this submolt is flagged as NSFW.
    pub is_nsfw: Option<bool>,
    /// Whether this submolt is private.
    pub is_private: Option<bool>,
    /// Creation timestamp.
    pub created_at: Option<String>,
    /// Timestamp of the most recent activity in this community.
    pub last_activity_at: Option<String>,
}

/// Represents a Direct Message request from another agent.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct DmRequest {
    /// The agent who sent the request.
    pub from: Author,
    /// The initial message sent with the request.
    pub message: Option<String>,
    /// A short preview of the message.
    pub message_preview: Option<String>,
    /// Unique ID for the resulting conversation if approved.
    pub conversation_id: String,
}

/// Represents an active DM conversation thread.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Conversation {
    /// Unique identifier for the conversation.
    pub conversation_id: String,
    /// The agent on the other side of the chat.
    pub with_agent: Author,
    /// Number of unread messages in this thread.
    pub unread_count: u64,
}

/// A specific message within a conversation thread.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Message {
    /// Agent who authored the message.
    pub from_agent: Author,
    /// The message text content.
    pub message: String,
    /// True if the message was authored by the current agent.
    pub from_you: bool,
    /// True if the message is flagged for human intervention.
    pub needs_human_input: bool,
    /// Message timestamp.
    pub created_at: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct FeedContext {
    pub page: Option<u64>,
    pub limit: Option<u64>,
    pub total: Option<u64>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct FeedResponse {
    pub success: bool,
    pub posts: Vec<Post>,
    pub feed_type: Option<String>,
    pub context: Option<FeedContext>,
}

/// Response from the search endpoint.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SearchResponse {
    /// A list of posts or comments matching the search query.
    pub results: Vec<SearchResult>,
}

/// Response containing a list of communities.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SubmoltsResponse {
    /// Array of submolt objects.
    pub submolts: Vec<Submolt>,
}

/// Response from the DM activity check endpoint.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct DmCheckResponse {
    /// Indicates if there are any new requests or unread messages.
    pub has_activity: bool,
    /// A short summary string of the activity.
    pub summary: Option<String>,
    /// Metadata about pending DM requests.
    pub requests: Option<DmRequestsData>,
    /// Metadata about unread messages.
    pub messages: Option<DmMessagesData>,
}

/// Paginated response for a submolt feed.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SubmoltFeedResponse {
    /// Array of posts in this submolt.
    pub posts: Vec<Post>,
    /// Total number of posts available in this community.
    #[serde(
        default,
        deserialize_with = "serde_helpers::deserialize_option_string_or_u64"
    )]
    pub total: Option<u64>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct DmRequestsData {
    #[serde(
        default,
        deserialize_with = "serde_helpers::deserialize_option_string_or_u64"
    )]
    pub count: Option<u64>,
    pub items: Vec<DmRequest>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct DmMessagesData {
    #[serde(deserialize_with = "serde_helpers::deserialize_string_or_u64")]
    pub total_unread: u64,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct DmListResponse {
    pub conversations: DmConversationsData,
    #[serde(deserialize_with = "serde_helpers::deserialize_string_or_u64")]
    pub total_unread: u64,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct DmConversationsData {
    pub items: Vec<Conversation>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_post_deserialization() {
        let json = r#"{
            "id": "123",
            "title": "Test Post",
            "content": "Content",
            "upvotes": 10,
            "downvotes": 0,
            "created_at": "2024-01-01T00:00:00Z",
            "author": {"name": "Bot"},
            "submolt": {"name": "general", "display_name": "General"}
        }"#;

        let post: Post = serde_json::from_str(json).unwrap();
        assert_eq!(post.title, "Test Post");
        assert_eq!(post.upvotes, 10);
    }

    #[test]
    fn test_api_response_success() {
        let json = r#"{"success": true, "id": "123", "name": "Test"}"#;
        let resp: ApiResponse<serde_json::Value> = serde_json::from_str(json).unwrap();
        assert!(resp.success);
        assert!(resp.data.is_some());
    }

    #[test]
    fn test_api_response_error() {
        let json =
            r#"{"success": false, "error": "Invalid key", "hint": "Check your credentials"}"#;
        let resp: ApiResponse<serde_json::Value> = serde_json::from_str(json).unwrap();
        assert!(!resp.success);
        assert_eq!(resp.error, Some("Invalid key".to_string()));
        assert_eq!(resp.hint, Some("Check your credentials".to_string()));
    }
}

/// Response from the registration endpoint.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct RegistrationResponse {
    /// Whether the registration was accepted.
    pub success: bool,
    /// The details of the newly created agent.
    pub agent: RegisteredAgent,
}

/// Details provided upon successful agent registration.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct RegisteredAgent {
    /// The assigned name of the agent.
    pub name: String,
    /// The API key to be used for future requests.
    pub api_key: String,
    /// URL to visit for claiming the agent identity.
    pub claim_url: String,
    /// Code required to complete the verification flow.
    pub verification_code: String,
}

/// Internal utilities for flexible JSON deserialization.
///
/// This module handles the "string-or-integer" ambiguity often found in JSON APIs,
/// ensuring that IDs and counts are correctly parsed regardless of their wire format.
mod serde_helpers {

    use serde::{Deserialize, Deserializer};

    pub fn deserialize_option_string_or_u64<'de, D>(
        deserializer: D,
    ) -> Result<Option<u64>, D::Error>
    where
        D: Deserializer<'de>,
    {
        #[derive(Deserialize)]
        #[serde(untagged)]
        enum StringOrInt {
            String(String),
            Int(u64),
        }

        match Option::<StringOrInt>::deserialize(deserializer)? {
            Some(StringOrInt::String(s)) => {
                s.parse::<u64>().map(Some).map_err(serde::de::Error::custom)
            }
            Some(StringOrInt::Int(i)) => Ok(Some(i)),
            None => Ok(None),
        }
    }

    pub fn deserialize_string_or_i64<'de, D>(deserializer: D) -> Result<i64, D::Error>
    where
        D: Deserializer<'de>,
    {
        #[derive(Deserialize)]
        #[serde(untagged)]
        enum StringOrInt {
            String(String),
            Int(i64),
        }

        match StringOrInt::deserialize(deserializer)? {
            StringOrInt::String(s) => s.parse::<i64>().map_err(serde::de::Error::custom),
            StringOrInt::Int(i) => Ok(i),
        }
    }

    pub fn deserialize_option_string_or_i64<'de, D>(
        deserializer: D,
    ) -> Result<Option<i64>, D::Error>
    where
        D: Deserializer<'de>,
    {
        #[derive(Deserialize)]
        #[serde(untagged)]
        enum StringOrInt {
            String(String),
            Int(i64),
        }

        match Option::<StringOrInt>::deserialize(deserializer)? {
            Some(StringOrInt::String(s)) => {
                s.parse::<i64>().map(Some).map_err(serde::de::Error::custom)
            }
            Some(StringOrInt::Int(i)) => Ok(Some(i)),
            None => Ok(None),
        }
    }

    pub fn deserialize_string_or_u64<'de, D>(deserializer: D) -> Result<u64, D::Error>
    where
        D: Deserializer<'de>,
    {
        #[derive(Deserialize)]
        #[serde(untagged)]
        enum StringOrInt {
            String(String),
            Int(u64),
        }

        match StringOrInt::deserialize(deserializer)? {
            StringOrInt::String(s) => s.parse::<u64>().map_err(serde::de::Error::custom),
            StringOrInt::Int(i) => Ok(i),
        }
    }
}
