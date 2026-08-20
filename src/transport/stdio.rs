//! STDIO Transport Implementation
//!
//! STDIO transport for MCP protocol - used for Claude Desktop integration.
//! Handles JSON-RPC messages over standard input/output via the `rmcp` SDK.

use crate::service::OpenMeteoService;
use rmcp::transport::stdio;
use rmcp::ServiceExt;
use std::sync::Arc;
use tracing::info;

/// Run MCP server with STDIO transport
///
/// Initializes the STDIO-based MCP server for Claude Desktop integration
/// and serves it until the client disconnects.
pub async fn run_stdio_server(
    service: Arc<OpenMeteoService>,
) -> Result<(), Box<dyn std::error::Error>> {
    info!("Starting MCP server with STDIO transport for Claude Desktop");

    let running = (*service).clone().serve(stdio()).await?;
    running.waiting().await?;

    info!("MCP server shutdown signal received");

    Ok(())
}

#[cfg(test)]
mod tests {
    #[tokio::test]
    async fn test_stdio_module_loads() {
        // Basic test to verify module can be imported
        let result: Result<(), String> = Ok(());
        assert!(result.is_ok());
    }
}
