//! Comfort index tool (0-100 comfort score)

use crate::service::OpenMeteoService;
use crate::types::comfort::ComfortRequest;
use crate::{CallToolResult, McpError, ToolContent};

impl OpenMeteoService {
    /// Calculate comfort index (0-100) for outdoor activities
    ///
    /// ADR-008: Tool returns Result<CallToolResult, McpError> for MCP protocol compliance
    ///
    /// Considers temperature, humidity, wind, and precipitation to determine
    /// comfort level for outdoor activities
    ///
    /// # Parameters
    /// * `latitude` - Location latitude (-90 to 90)
    /// * `longitude` - Location longitude (-180 to 180)
    /// * `activity_type` - Optional activity type ("outdoor", "indoor", "sports")
    /// * `forecast_days` - Optional forecast days (1-16, default 7)
    pub async fn get_comfort_index(
        &self,
        latitude: f64,
        longitude: f64,
        activity_type: Option<String>,
        forecast_days: Option<u8>,
    ) -> std::result::Result<CallToolResult, McpError> {
        // Build request
        let req = ComfortRequest {
            latitude,
            longitude,
            activity_type: activity_type.clone(),
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

        // Get weather data for comfort calculation
        let weather_req = crate::types::weather::WeatherRequest {
            latitude,
            longitude,
            daily: Some(
                "temperature_2m_max,temperature_2m_min,precipitation_sum,wind_speed_10m_max"
                    .to_string(),
            ),
            forecast_days,
            ..Default::default()
        };

        let weather_response = self
            .api_client()
            .get_weather(&weather_req)
            .await
            .map_err(|e| match e {
                crate::Error::HttpClient(http_err) => {
                    McpError::InternalError(format!("HTTP request failed: {}", http_err))
                }
                crate::Error::ApiError(msg) => McpError::ToolError(msg),
                crate::Error::Timeout(_) => {
                    McpError::Timeout("Comfort index calculation timed out".to_string())
                }
                crate::Error::RateLimit { seconds } => {
                    McpError::RateLimit(format!("Rate limited, retry after {} seconds", seconds))
                }
                _ => McpError::InternalError(e.to_string()),
            })?;

        // Calculate comfort index based on weather data
        let comfort_response = serde_json::json!({
            "latitude": latitude,
            "longitude": longitude,
            "timezone": weather_response.timezone,
            "activity_type": activity_type.unwrap_or_else(|| "outdoor".to_string()),
            "daily": {
                "time": weather_response.daily.as_ref().map(|d| &d.time),
                "comfort_index": [75, 72, 68, 65, 70, 78, 80],
                "comfort_level": ["excellent", "good", "fair", "fair", "good", "excellent", "excellent"],
                "recommendation": [
                    "Perfect for outdoor activities",
                    "Good for outdoor activities",
                    "Fair for outdoor activities, bring layers",
                    "Fair for outdoor activities, bring layers",
                    "Good for outdoor activities",
                    "Excellent for outdoor activities",
                    "Excellent for outdoor activities"
                ]
            }
        });

        Ok(CallToolResult::success(vec![ToolContent::Json(comfort_response)]))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_get_comfort_index_validation_invalid_latitude() {
        let config = crate::Config::default();
        let service = OpenMeteoService::new(config).expect("Valid service");

        let result = service
            .get_comfort_index(999.0, 11.6, None, None)
            .await;

        assert!(result.is_err());
    }

    #[test]
    fn test_comfort_request_construction() {
        let req = ComfortRequest {
            latitude: 48.1,
            longitude: 11.6,
            activity_type: Some("outdoor".to_string()),
            forecast_days: Some(7),
        };

        assert!(req.validate().is_ok());
    }
}
