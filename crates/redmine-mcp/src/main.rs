//! Redmine MCP server binary.
//!
//! Parses configuration from CLI/environment, creates a [`RedmineClient`],
//! wraps it in a [`RedmineServer`](tools::RedmineServer) with all MCP tools,
//! and serves over stdio transport.

use redmine_core::client::RedmineClient;
use redmine_core::config::Config;
use rmcp::transport::stdio;
use rmcp::ServiceExt;

mod tools;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let config = Config::parse();
    let client = RedmineClient::new(config)?;
    let server = tools::RedmineServer::new(client);

    let service = server.serve(stdio()).await?;
    service.waiting().await?;
    Ok(())
}
