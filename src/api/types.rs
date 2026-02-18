use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ApiResponse<T> {
    pub success: bool,
    #[serde(flatten)]
    pub data: Option<T>,
    pub error: Option<String>,
    pub hint: Option<String>,
    pub retry_after_minutes: Option<u64>,
    pub retry_after_seconds: Option<u64>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Agent {
    pub id: String,
    pub name: String,
    pub description: Option<String>,
    #[serde(default, deserialize_with = "serde_helpers::deserialize_option_string_or_i64")]
    pub karma: Option<i64>,
    #[serde(default, deserialize_with = "serde_helpers::deserialize_option_string_or_u64")]
    pub follower_count: Option<u64>,
    #[serde(default, deserialize_with = "serde_helpers::deserialize_option_string_or_u64")]
    pub following_count: Option<u64>,
    pub is_claimed: Option<bool>,
    pub is_active: Option<bool>,
    pub created_at: Option<String>,
    pub last_active: Option<String>,
    pub claimed_at: Option<String>,
    pub owner_id: Option<String>,
    pub owner: Option<OwnerInfo>,
    pub stats: Option<AgentStats>,
    pub metadata: Option<serde_json::Value>,
    pub recent_posts: Option<Vec<Post>>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct OwnerInfo {
    #[serde(alias = "xHandle")]
    pub x_handle: Option<String>,
    #[serde(alias = "xName")]
    pub x_name: Option<String>,
    #[serde(alias = "xAvatar")]
    pub x_avatar: Option<String>,
    #[serde(alias = "xBio")]
    pub x_bio: Option<String>,
    #[serde(default, deserialize_with = "serde_helpers::deserialize_option_string_or_u64")]
    pub x_follower_count: Option<u64>,
    #[serde(default, deserialize_with = "serde_helpers::deserialize_option_string_or_u64")]
    pub x_following_count: Option<u64>,
    pub x_verified: Option<bool>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct AgentStats {
    pub posts: Option<u64>,
    pub comments: Option<u64>,
    pub subscriptions: Option<u64>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct StatusResponse {
    pub status: Option<String>,
    pub message: Option<String>,
    pub next_step: Option<String>,
    pub agent: Option<Agent>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct PostResponse {
    pub success: bool,
    pub message: Option<String>,
    pub post: Option<Post>,
    pub verification_required: Option<bool>,
    pub verification: Option<VerificationChallenge>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct VerificationChallenge {
    pub code: String,
    pub challenge: String,
    pub instructions: String,
    pub verify_endpoint: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Post {
    pub id: String,
    pub title: String,
    pub content: Option<String>,
    pub url: Option<String>,
    #[serde(deserialize_with = "serde_helpers::deserialize_string_or_i64")]
    pub upvotes: i64,
    #[serde(deserialize_with = "serde_helpers::deserialize_string_or_i64")]
    pub downvotes: i64,
    #[serde(default, deserialize_with = "serde_helpers::deserialize_option_string_or_u64")]
    pub comment_count: Option<u64>,
    pub created_at: String,
    pub author: Author,
    pub submolt: Option<SubmoltInfo>,
    pub submolt_name: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Author {
    pub id: Option<String>,
    pub name: String,
    pub description: Option<String>,
    #[serde(default, deserialize_with = "serde_helpers::deserialize_option_string_or_i64")]
    pub karma: Option<i64>,
    #[serde(default, alias = "followerCount", deserialize_with = "serde_helpers::deserialize_option_string_or_u64")]
    pub follower_count: Option<u64>,
    pub owner: Option<OwnerInfo>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SubmoltInfo {
    pub name: String,
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

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Submolt {
    pub id: Option<String>,
    pub name: String,
    pub display_name: String,
    pub description: Option<String>,
    #[serde(default, deserialize_with = "serde_helpers::deserialize_option_string_or_u64")]
    pub subscriber_count: Option<u64>,
    pub allow_crypto: Option<bool>,
    pub created_at: Option<String>,
    pub last_activity_at: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct DmRequest {
    pub from: Author,
    pub message: Option<String>,
    pub message_preview: Option<String>,
    pub conversation_id: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Conversation {
    pub conversation_id: String,
    pub with_agent: Author,
    pub unread_count: u64,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Message {
    pub from_agent: Author,
    pub message: String,
    pub from_you: bool,
    pub needs_human_input: bool,
    pub created_at: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct FeedResponse {
    pub success: bool,
    pub posts: Vec<Post>,
    #[serde(default, deserialize_with = "serde_helpers::deserialize_option_string_or_u64")]
    pub count: Option<u64>,
    pub has_more: Option<bool>,
    #[serde(default, deserialize_with = "serde_helpers::deserialize_option_string_or_u64")]
    pub next_offset: Option<u64>,
    pub authenticated: Option<bool>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SearchResponse {
    pub results: Vec<SearchResult>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SubmoltsResponse {
    pub submolts: Vec<Submolt>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct DmCheckResponse {
    pub has_activity: bool,
    pub summary: Option<String>,
    pub requests: Option<DmRequestsData>,
    pub messages: Option<DmMessagesData>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct DmRequestsData {
    pub items: Vec<DmRequest>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct DmMessagesData {
    pub total_unread: u64,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct DmListResponse {
    pub conversations: DmConversationsData,
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

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct RegistrationResponse {
    pub success: bool,
    pub agent: RegisteredAgent,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct RegisteredAgent {
    pub name: String,
    pub api_key: String,
    pub claim_url: String,
    pub verification_code: String,
}

mod serde_helpers {
    use serde::{Deserialize, Deserializer};

    pub fn deserialize_option_string_or_u64<'de, D>(deserializer: D) -> Result<Option<u64>, D::Error>
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
            Some(StringOrInt::String(s)) => s.parse::<u64>().map(Some).map_err(serde::de::Error::custom),
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
    
    pub fn deserialize_option_string_or_i64<'de, D>(deserializer: D) -> Result<Option<i64>, D::Error>
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
            Some(StringOrInt::String(s)) => s.parse::<i64>().map(Some).map_err(serde::de::Error::custom),
            Some(StringOrInt::Int(i)) => Ok(Some(i)),
            None => Ok(None),
        }
    }
}
