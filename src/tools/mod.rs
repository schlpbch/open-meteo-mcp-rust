//! MCP Tool implementations
//!
//! Tools follow ADR-008 pattern:
//! - Decorated with `#[tool]` macro (once rmcp integration is complete)
//! - `#[tool_param]` for parameters with descriptions
//! - Return `CallToolResult` for MCP protocol compliance
//! - Implement proper error handling with `McpError` conversion

pub mod weather;
pub mod location;
pub mod location_swiss;
pub mod air_quality;
pub mod marine;
pub mod snow;
pub mod alerts;
pub mod astronomy;
pub mod comfort;
pub mod comparison;
pub mod historical;

use crate::service::OpenMeteoService;
use crate::{CallToolResult, McpError, ToolContent};
use serde::{Deserialize, Serialize};
use schemars::JsonSchema;

/// Ping response for connectivity testing
#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct PingResponse {
    pub message: String,
    pub timestamp: String,
    pub version: String,
}

impl OpenMeteoService {
    /// Simple ping tool to test MCP connectivity
    ///
    /// ADR-008: Implemented with MCP error handling pattern
    ///
    /// In Phase 2, this will be decorated with:
    /// ```ignore
    /// #[tool(description = "Ping MCP server for connectivity test")]
    /// pub async fn ping(&self) -> Result<CallToolResult, McpError> { ... }
    /// ```
    pub async fn ping(&self) -> std::result::Result<CallToolResult, McpError> {
        let response = PingResponse {
            message: "pong".to_string(),
            timestamp: chrono::Utc::now().to_rfc3339(),
            version: crate::VERSION.to_string(),
        };

        let json = serde_json::to_string_pretty(&response)
            .map_err(|e| McpError::InternalError(format!("Serialization error: {}", e)))?;

        Ok(CallToolResult::success(vec![ToolContent::Text(json)]))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_ping_success() {
        let config = crate::Config::default();
        let service = OpenMeteoService::new(config).expect("Valid service");
        let result = service.ping().await;

        assert!(result.is_ok());
        let tool_result = result.expect("Ping CallToolResult");
        assert!(!tool_result.is_error);
        assert!(!tool_result.content.is_empty());

        // Verify JSON response
        if let ToolContent::Text(text) = &tool_result.content[0] {
            let response: PingResponse = serde_json::from_str(text).expect("Valid JSON");
            assert_eq!(response.message, "pong");
            assert!(!response.timestamp.is_empty());
            assert_eq!(response.version, crate::VERSION);
        } else {
            panic!("Expected Text content");
        }
    }
}
