//! Weather forecast tool

use crate::service::OpenMeteoService;
use crate::types::weather::WeatherRequest;
use crate::{CallToolResult, McpError, ToolContent};

impl OpenMeteoService {
    /// Get weather forecast data for a location
    ///
    /// ADR-008: Tool returns Result<CallToolResult, McpError> for MCP protocol compliance
    ///
    /// # Parameters
    /// * `latitude` - Location latitude (-90 to 90)
    /// * `longitude` - Location longitude (-180 to 180)
    /// * `hourly` - Optional comma-separated hourly variables (e.g., "temperature_2m,precipitation")
    /// * `daily` - Optional comma-separated daily variables (e.g., "temperature_2m_max,precipitation_sum")
    /// * `forecast_days` - Optional forecast days (1-16, default 7)
    /// * `temperature_unit` - Optional temperature unit ("celsius", "fahrenheit", default celsius)
    pub async fn get_weather(
        &self,
        latitude: f64,
        longitude: f64,
        hourly: Option<String>,
        daily: Option<String>,
        forecast_days: Option<u8>,
        temperature_unit: Option<String>,
    ) -> std::result::Result<CallToolResult, McpError> {
        // Build request
        let req = WeatherRequest {
            latitude,
            longitude,
            hourly,
            daily,
            forecast_days,
            temperature_unit,
            ..Default::default()
        };

        // Validate request
        req.validate().map_err(|e| match e {
            crate::Error::InvalidCoordinates { lat, lon } => {
                McpError::InvalidParameter(format!(
                    "Invalid coordinates: latitude must be -90..90, got {}, longitude must be -180..180, got {}",
                    lat, lon
                ))
            }
            crate::Error::InvalidParameter(msg) => McpError::InvalidParameter(msg),
            _ => McpError::InternalError(e.to_string()),
        })?;

        // Get weather data
        let response = self
            .api_client()
            .get_weather(&req)
            .await
            .map_err(|e| match e {
                crate::Error::HttpClient(http_err) => {
                    McpError::InternalError(format!("HTTP request failed: {}", http_err))
                }
                crate::Error::ApiError(msg) => McpError::ToolError(msg),
                crate::Error::Timeout(_) => McpError::Timeout(
                    "Weather API request timed out".to_string(),
                ),
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
    async fn test_get_weather_validation_invalid_latitude() {
        let config = crate::Config::default();
        let service = OpenMeteoService::new(config).expect("Valid service");

        let result = service
            .get_weather(999.0, 11.6, None, None, None, None)
            .await;

        assert!(result.is_err());
        match result {
            Err(McpError::InvalidParameter(msg)) => {
                assert!(msg.contains("latitude"));
            }
            _ => panic!("Expected InvalidParameter error"),
        }
    }

    #[tokio::test]
    async fn test_get_weather_validation_invalid_longitude() {
        let config = crate::Config::default();
        let service = OpenMeteoService::new(config).expect("Valid service");

        let result = service
            .get_weather(48.1, 999.0, None, None, None, None)
            .await;

        assert!(result.is_err());
        match result {
            Err(McpError::InvalidParameter(msg)) => {
                assert!(msg.contains("longitude"));
            }
            _ => panic!("Expected InvalidParameter error"),
        }
    }

    #[test]
    fn test_weather_request_construction() {
        let req = WeatherRequest {
            latitude: 48.1,
            longitude: 11.6,
            hourly: Some("temperature_2m,precipitation".to_string()),
            daily: Some("temperature_2m_max,precipitation_sum".to_string()),
            forecast_days: Some(7),
            ..Default::default()
        };

        assert!(req.validate().is_ok());
        assert_eq!(req.latitude, 48.1);
        assert_eq!(req.longitude, 11.6);
    }
}
