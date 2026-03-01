//! GitHub MCP Webhook Server binary
//!
//! This binary starts the GitHub MCP webhook server with configuration
//! from environment variables or command line arguments.

use clap::Parser;
use mcp_server::{ServerConfig, start_server};

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Bind address for the server
    #[arg(long, env = "MCP_SERVER_BIND_ADDRESS", default_value = "0.0.0.0:3000")]
    bind_address: String,

    /// GitHub webhook secret
    #[arg(long, env = "GITHUB_WEBHOOK_SECRET", default_value = "")]
    github_webhook_secret: String,

    /// Maximum number of events to keep in history
    #[arg(long, env = "MCP_SERVER_MAX_EVENTS", default_value_t = 100)]
    max_event_history: usize,

    /// Enable verbose logging
    #[arg(long, short, env = "MCP_SERVER_VERBOSE", default_value_t = false)]
    verbose: bool,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();

    let config = ServerConfig {
        bind_address: args.bind_address,
        github_webhook_secret: args.github_webhook_secret,
        max_event_history: args.max_event_history,
        verbose: args.verbose,
    };

    start_server(config).await?;
    Ok(())
}
