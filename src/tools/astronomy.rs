//! Astronomy tool (sunrise, sunset, moon phases)

use crate::service::OpenMeteoService;
use crate::types::astronomy::AstronomyRequest;
use crate::{CallToolResult, McpError, ToolContent};

impl OpenMeteoService {
    /// Get astronomy data (sunrise, sunset, moon phases)
    ///
    /// ADR-008: Tool returns Result<CallToolResult, McpError> for MCP protocol compliance
    ///
    /// # Parameters
    /// * `latitude` - Location latitude (-90 to 90)
    /// * `longitude` - Location longitude (-180 to 180)
    /// * `forecast_days` - Optional forecast days (1-16, default 7)
    pub async fn get_astronomy(
        &self,
        latitude: f64,
        longitude: f64,
        forecast_days: Option<u8>,
    ) -> std::result::Result<CallToolResult, McpError> {
        // Build request
        let req = AstronomyRequest {
            latitude,
            longitude,
            timezone: None,
            forecast_days,
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

        // Build weather request to get daily data with sunrise/sunset
        let weather_req = crate::types::weather::WeatherRequest {
            latitude,
            longitude,
            daily: Some("sunrise,sunset,sunset_end,sunrise_end".to_string()),
            forecast_days,
            ..Default::default()
        };

        let response = self
            .api_client()
            .get_weather(&weather_req)
            .await
            .map_err(|e| match e {
                crate::Error::HttpClient(http_err) => {
                    McpError::InternalError(format!("HTTP request failed: {}", http_err))
                }
                crate::Error::ApiError(msg) => McpError::ToolError(msg),
                crate::Error::Timeout(_) => {
                    McpError::Timeout("Astronomy API request timed out".to_string())
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
    async fn test_get_astronomy_validation_invalid_latitude() {
        let config = crate::Config::default();
        let service = OpenMeteoService::new(config).expect("Valid service");

        let result = service.get_astronomy(999.0, 11.6, None).await;

        assert!(result.is_err());
    }

    #[test]
    fn test_astronomy_request_construction() {
        let req = AstronomyRequest {
            latitude: 48.1,
            longitude: 11.6,
            forecast_days: Some(7),
            ..Default::default()
        };

        assert!(req.validate().is_ok());
    }
}
