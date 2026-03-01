//! Server configuration

use serde::{Deserialize, Serialize};

/// Server configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServerConfig {
    /// Bind address for the server (e.g., "0.0.0.0:3000")
    pub bind_address: String,
    
    /// GitHub webhook secret for signature validation
    pub github_webhook_secret: String,
    
    /// Maximum number of events to keep in history
    pub max_event_history: usize,
    
    /// Enable verbose logging
    pub verbose: bool,
}

impl Default for ServerConfig {
    fn default() -> Self {
        Self {
            bind_address: "0.0.0.0:3000".to_string(),
            github_webhook_secret: String::new(),
            max_event_history: 100,
            verbose: false,
        }
    }
}
