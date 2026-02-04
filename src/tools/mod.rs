//! MCP Tool implementations

use crate::service::OpenMeteoService;
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
    /// This tool is used for Phase 0 validation to ensure the MCP stack
    /// is working correctly. It returns immediately with the current timestamp.
    pub async fn ping(&self) -> crate::Result<PingResponse> {
        Ok(PingResponse {
            message: "pong".to_string(),
            timestamp: chrono::Utc::now().to_rfc3339(),
            version: crate::VERSION.to_string(),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_ping_success() {
        let config = crate::Config::default();
        let service = OpenMeteoService::new(config).unwrap();
        let result = service.ping().await;

        assert!(result.is_ok());
        let response = result.unwrap();
        assert_eq!(response.message, "pong");
        assert!(!response.timestamp.is_empty());
        assert_eq!(response.version, crate::VERSION);
    }
}
