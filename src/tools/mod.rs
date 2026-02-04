//! MCP Tool implementations
//!
//! Phase 3.5: Integrated with rmcp 0.3 SDK
//! - Manual ServerHandler implementation
//! - Tool registration via list_tools and call_tool methods
//! - Resource and prompt handling
//! - Full MCP protocol compliance

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
