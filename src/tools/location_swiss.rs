//! Swiss location search tool

use crate::service::OpenMeteoService;
use crate::types::location::GeocodeRequest;
use crate::{CallToolResult, McpError, ToolContent};

impl OpenMeteoService {
    /// Search for locations within Switzerland
    ///
    /// ADR-008: Tool returns Result<CallToolResult, McpError> for MCP protocol compliance
    ///
    /// Returns Swiss cities, mountains, lakes, and passes
    ///
    /// # Parameters
    /// * `name` - Location name to search within Switzerland
    /// * `count` - Optional maximum number of results (1-100, default 10)
    pub async fn search_location_swiss(
        &self,
        name: String,
        count: Option<u8>,
    ) -> std::result::Result<CallToolResult, McpError> {
        // Build request with Swiss country filter
        let mut req = GeocodeRequest {
            name,
            count,
            language: None,
            format: None,
        };

        // Set default language to English for Swiss results
        if req.language.is_none() {
            req.language = Some("en".to_string());
        }

        // Validate request
        req.validate().map_err(|e| match e {
            crate::Error::InvalidParameter(msg) => McpError::InvalidParameter(msg),
            _ => McpError::InternalError(e.to_string()),
        })?;

        // Search for locations (in real implementation, would filter by Switzerland)
        let response = self
            .client
            .search_location(&req)
            .await
            .map_err(|e| match e {
                crate::Error::HttpClient(http_err) => {
                    McpError::InternalError(format!("HTTP request failed: {}", http_err))
                }
                crate::Error::ApiError(msg) => McpError::ToolError(msg),
                crate::Error::Timeout(_) => {
                    McpError::Timeout("Swiss location search timed out".to_string())
                }
                crate::Error::RateLimit { seconds } => {
                    McpError::RateLimit(format!("Rate limited, retry after {} seconds", seconds))
                }
                _ => McpError::InternalError(e.to_string()),
            })?;

        // Filter results to Swiss locations only
        let swiss_results = response.results.into_iter()
            .filter(|loc| {
                loc.country_code.as_ref().map(|c| c == "CH").unwrap_or(false)
                    || loc.country.as_ref().map(|c| c.contains("Switzerland")).unwrap_or(false)
            })
            .collect();

        let filtered_response = serde_json::json!({
            "results": swiss_results
        });

        Ok(CallToolResult::success(vec![ToolContent::Json(filtered_response)]))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_search_location_swiss_empty_name() {
        let config = crate::Config::default();
        let service = OpenMeteoService::new(config).expect("Valid service");

        let result = service.search_location_swiss(String::new(), None).await;

        assert!(result.is_err());
    }

    #[test]
    fn test_swiss_location_request_construction() {
        let req = GeocodeRequest {
            name: "Zurich".to_string(),
            count: Some(5),
            language: Some("en".to_string()),
            format: None,
        };

        assert!(req.validate().is_ok());
    }
}
