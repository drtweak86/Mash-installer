//! GitHub MCP Webhook Server
//!
//! This crate provides a webhook server for receiving GitHub events
//! and processing them according to the Model Context Protocol (MCP).
//!
//! The server supports:
//! - GitHub webhook validation with HMAC signatures
//! - Multiple event types (push, pull_request, issue, etc.)
//! - Configurable endpoints and secret management
//! - Integration with AI agents and development tools

use axum::{
    Router,
    routing::{get, post},
};
use hmac::{Hmac, Mac};
use sha2::Sha256;
use std::sync::Arc;
use tokio::sync::Mutex;

pub mod config;
pub mod error;
pub mod handlers;
pub mod models;

pub use config::ServerConfig;
pub use error::ServerError;

/// Type alias for HMAC-SHA256
pub type HmacSha256 = Hmac<Sha256>;

/// Main server state
#[derive(Clone, Debug)]
pub struct AppState {
    pub config: ServerConfig,
    pub event_history: Arc<Mutex<Vec<String>>>, // Simple event logging
}

/// Start the GitHub MCP webhook server
pub async fn start_server(config: ServerConfig) -> Result<(), ServerError> {
    // Initialize tracing
    tracing_subscriber::fmt::init();

    let state = AppState {
        config: config.clone(),
        event_history: Arc::new(Mutex::new(Vec::new())),
    };

    // Build our application with routes
    let app = Router::new()
        .route("/health", get(handlers::health_check))
        .route("/webhook", post(handlers::github_webhook))
        .route("/events", get(handlers::list_events))
        .with_state(state);

    // Start the server
    let listener = tokio::net::TcpListener::bind(&config.bind_address)
        .await
        .map_err(|e| ServerError::Startup(e.to_string()))?;

    tracing::info!("Starting GitHub MCP server on {}", config.bind_address);

    axum::serve(listener, app)
        .await
        .map_err(|e| ServerError::Runtime(e.to_string()))
}

/// Validate GitHub webhook signature
pub fn validate_webhook_signature(
    secret: &str,
    signature: &str,
    body: &[u8],
) -> Result<(), ServerError> {
    if secret.is_empty() {
        return Err(ServerError::Validation(
            "Webhook secret is not configured".to_string(),
        ));
    }

    if !signature.starts_with("sha256=") {
        return Err(ServerError::Validation(
            "Invalid signature format".to_string(),
        ));
    }

    let signature =
        hex::decode(&signature[7..]).map_err(|e| ServerError::Validation(e.to_string()))?;

    let mut mac = HmacSha256::new_from_slice(secret.as_bytes())
        .map_err(|e| ServerError::Validation(e.to_string()))?;

    mac.update(body);
    mac.verify_slice(&signature)
        .map_err(|_| ServerError::Validation("Signature verification failed".to_string()))
}
