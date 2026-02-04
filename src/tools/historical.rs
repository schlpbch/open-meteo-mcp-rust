//! Historical weather tool (archive data 1940-present)

use crate::service::OpenMeteoService;
use crate::types::weather::WeatherRequest;
use crate::{CallToolResult, McpError, ToolContent};

impl OpenMeteoService {
    /// Get historical weather data (1940-present)
    ///
    /// ADR-008: Tool returns Result<CallToolResult, McpError> for MCP protocol compliance
    ///
    /// # Parameters
    /// * `latitude` - Location latitude (-90 to 90)
    /// * `longitude` - Location longitude (-180 to 180)
    /// * `start_date` - Start date in YYYY-MM-DD format
    /// * `end_date` - End date in YYYY-MM-DD format
    /// * `hourly` - Optional comma-separated hourly variables
    /// * `daily` - Optional comma-separated daily variables
    pub async fn get_historical_weather(
        &self,
        latitude: f64,
        longitude: f64,
        start_date: String,
        end_date: String,
        hourly: Option<String>,
        daily: Option<String>,
    ) -> std::result::Result<CallToolResult, McpError> {
        // Validate coordinates
        crate::error::validate_coordinates(latitude, longitude).map_err(|e| {
            McpError::InvalidParameter(e.to_string())
        })?;

        // Validate date formats
        if !is_valid_date_format(&start_date) {
            return Err(McpError::InvalidParameter(
                "Invalid start_date format, use YYYY-MM-DD".to_string(),
            ));
        }
        if !is_valid_date_format(&end_date) {
            return Err(McpError::InvalidParameter(
                "Invalid end_date format, use YYYY-MM-DD".to_string(),
            ));
        }

        // Fetch historical data
        let response = self
            .api_client()
            .get_historical_weather(&WeatherRequest {
                latitude,
                longitude,
                hourly,
                daily,
                ..Default::default()
            }, &start_date, &end_date)
            .await
            .map_err(|e| match e {
                crate::Error::HttpClient(http_err) => {
                    McpError::InternalError(format!("HTTP request failed: {}", http_err))
                }
                crate::Error::ApiError(msg) => McpError::ToolError(msg),
                crate::Error::Timeout(_) => {
                    McpError::Timeout("Historical weather request timed out".to_string())
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

/// Validate date format and semantics (YYYY-MM-DD)
fn is_valid_date_format(date_str: &str) -> bool {
    let parts: Vec<&str> = date_str.split('-').collect();
    if parts.len() != 3 {
        return false;
    }

    // Format check
    if !(parts[0].len() == 4 && parts[0].chars().all(|c| c.is_ascii_digit())) {
        return false;
    }
    if !(parts[1].len() == 2 && parts[1].chars().all(|c| c.is_ascii_digit())) {
        return false;
    }
    if !(parts[2].len() == 2 && parts[2].chars().all(|c| c.is_ascii_digit())) {
        return false;
    }

    // Semantic validation: month 1-12, day 1-31
    let month: u32 = parts[1].parse().unwrap_or(0);
    let day: u32 = parts[2].parse().unwrap_or(0);

    month >= 1 && month <= 12 && day >= 1 && day <= 31
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_get_historical_weather_validation_invalid_latitude() {
        let config = crate::Config::default();
        let service = OpenMeteoService::new(config).expect("Valid service");

        let result = service
            .get_historical_weather(999.0, 11.6, "2020-01-01".to_string(), "2020-12-31".to_string(), None, None)
            .await;

        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_get_historical_weather_validation_invalid_start_date() {
        let config = crate::Config::default();
        let service = OpenMeteoService::new(config).expect("Valid service");

        let result = service
            .get_historical_weather(48.1, 11.6, "2020/01/01".to_string(), "2020-12-31".to_string(), None, None)
            .await;

        assert!(result.is_err());
    }

    #[test]
    fn test_date_format_validation() {
        assert!(is_valid_date_format("2020-01-01"));
        assert!(is_valid_date_format("2026-02-04"));
        assert!(!is_valid_date_format("2020/01/01"));
        assert!(!is_valid_date_format("2020-13-01"));
        assert!(!is_valid_date_format("2020-02-32"));
    }
}
