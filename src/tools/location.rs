//! Location search and geocoding tool

use crate::service::OpenMeteoService;
use crate::types::location::GeocodeRequest;
use crate::{CallToolResult, McpError, ToolContent};

impl OpenMeteoService {
    /// Search for locations by name
    ///
    /// ADR-008: Tool returns Result<CallToolResult, McpError> for MCP protocol compliance
    ///
    /// # Parameters
    /// * `name` - Location name to search (e.g., "Munich", "New York")
    /// * `count` - Optional maximum number of results (1-100, default 10)
    /// * `language` - Optional language code (e.g., "en", "de")
    pub async fn search_location(
        &self,
        name: String,
        count: Option<u32>,
        language: Option<String>,
    ) -> std::result::Result<CallToolResult, McpError> {
        // Build request
        let req = GeocodeRequest {
            name,
            count,
            language,
            format: None,
        };

        // Validate request
        req.validate().map_err(|e| match e {
            crate::Error::InvalidParameter(msg) => McpError::InvalidParameter(msg),
            _ => McpError::InternalError(e.to_string()),
        })?;

        // Search for locations
        let response = self
            .api_client()
            .search_location(&req)
            .await
            .map_err(|e| match e {
                crate::Error::HttpClient(http_err) => {
                    McpError::InternalError(format!("HTTP request failed: {}", http_err))
                }
                crate::Error::ApiError(msg) => McpError::ToolError(msg),
                crate::Error::Timeout(_) => {
                    McpError::Timeout("Geocoding request timed out".to_string())
                }
                crate::Error::RateLimit { seconds } => {
                    McpError::RateLimit(format!("Rate limited, retry after {} seconds", seconds))
                }
                _ => McpError::InternalError(e.to_string()),
            })?;

        // Format response as JSON
        let json_response = serde_json::to_value(&response)
            .map_err(|e| McpError::InternalError(format!("JSON serialization error: {}", e)))?;

        Ok(CallToolResult::success(vec![ToolContent::Json(json_response)]))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_search_location_validation_empty_name() {
        let config = crate::Config::default();
        let service = OpenMeteoService::new(config).expect("Valid service");

        let result = service.search_location(String::new(), None, None).await;

        assert!(result.is_err());
        match result {
            Err(McpError::InvalidParameter(msg)) => {
                assert!(msg.contains("name"));
            }
            _ => panic!("Expected InvalidParameter error"),
        }
    }

    #[tokio::test]
    async fn test_search_location_validation_invalid_count() {
        let config = crate::Config::default();
        let service = OpenMeteoService::new(config).expect("Valid service");

        let result = service
            .search_location("Munich".to_string(), Some(200), None)
            .await;

        assert!(result.is_err());
        match result {
            Err(McpError::InvalidParameter(msg)) => {
                assert!(msg.contains("count"));
            }
            _ => panic!("Expected InvalidParameter error"),
        }
    }

    #[test]
    fn test_location_request_construction() {
        let req = GeocodeRequest {
            name: "Munich".to_string(),
            count: Some(10),
            language: Some("en".to_string()),
            format: None,
        };

        assert!(req.validate().is_ok());
        assert_eq!(req.name, "Munich");
    }
}
