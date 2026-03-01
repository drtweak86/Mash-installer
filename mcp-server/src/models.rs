//! Data models for GitHub webhooks

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// GitHub webhook payload envelope
#[derive(Debug, Serialize, Deserialize)]
pub struct WebhookPayload {
    pub action: Option<String>,
    pub repository: Option<Repository>,
    pub sender: Option<User>,
    pub installation: Option<Installation>,
    #[serde(flatten)]
    pub event_data: HashMap<String, serde_json::Value>,
}

/// Repository information
#[derive(Debug, Serialize, Deserialize)]
pub struct Repository {
    pub id: u64,
    pub name: String,
    pub full_name: String,
    pub owner: User,
    pub private: bool,
    pub html_url: String,
    pub description: Option<String>,
}

/// User information
#[derive(Debug, Serialize, Deserialize)]
pub struct User {
    pub login: String,
    pub id: u64,
    pub node_id: String,
    pub avatar_url: String,
    pub gravatar_id: Option<String>,
    pub url: String,
    pub html_url: String,
    pub followers_url: String,
    pub following_url: String,
    pub gists_url: String,
    pub starred_url: String,
    pub subscriptions_url: String,
    pub organizations_url: String,
    pub repos_url: String,
    pub events_url: String,
    pub received_events_url: String,
    pub r#type: String,
    pub site_admin: bool,
}

/// Installation information
#[derive(Debug, Serialize, Deserialize)]
pub struct Installation {
    pub id: u64,
    pub node_id: String,
}

/// Webhook event header information
#[derive(Debug, Clone)]
pub struct WebhookHeaders {
    pub event: String,
    pub delivery: String,
    pub signature: Option<String>,
}

/// Processed webhook event
#[derive(Debug, Serialize, Deserialize)]
pub struct ProcessedEvent {
    pub event_type: String,
    pub repository: String,
    pub action: Option<String>,
    pub sender: String,
    pub timestamp: String,
    pub payload: serde_json::Value,
}
