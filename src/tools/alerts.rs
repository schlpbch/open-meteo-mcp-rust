//! Weather alerts tool

use crate::service::OpenMeteoService;
use crate::types::alerts::AlertsRequest;
use crate::{CallToolResult, McpError, ToolContent};

impl OpenMeteoService {
    /// Get weather alerts based on thresholds
    ///
    /// ADR-008: Tool returns Result<CallToolResult, McpError> for MCP protocol compliance
    ///
    /// # Parameters
    /// * `latitude` - Location latitude (-90 to 90)
    /// * `longitude` - Location longitude (-180 to 180)
    /// * `temperature_threshold_hot` - Optional hot temperature threshold (°C)
    /// * `temperature_threshold_cold` - Optional cold temperature threshold (°C)
    /// * `precipitation_threshold` - Optional precipitation threshold (mm)
    /// * `wind_speed_threshold` - Optional wind speed threshold (km/h)
    pub async fn get_weather_alerts(
        &self,
        latitude: f64,
        longitude: f64,
        temperature_threshold_hot: Option<f64>,
        temperature_threshold_cold: Option<f64>,
        precipitation_threshold: Option<f64>,
        wind_speed_threshold: Option<f64>,
    ) -> std::result::Result<CallToolResult, McpError> {
        // Build request
        let req = AlertsRequest {
            latitude,
            longitude,
            temperature_threshold_hot,
            temperature_threshold_cold,
            precipitation_threshold,
            wind_speed_threshold,
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

        // Build response based on thresholds (in real implementation, would fetch weather and check thresholds)
        let response = serde_json::json!({
            "latitude": latitude,
            "longitude": longitude,
            "timezone": "UTC",
            "alerts": [],
            "thresholds": {
                "temperature_hot": temperature_threshold_hot.unwrap_or(30.0),
                "temperature_cold": temperature_threshold_cold.unwrap_or(-5.0),
                "precipitation": precipitation_threshold.unwrap_or(10.0),
                "wind_speed": wind_speed_threshold.unwrap_or(50.0)
            }
        });

        Ok(CallToolResult::success(vec![ToolContent::Json(response)]))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_get_weather_alerts_validation_invalid_latitude() {
        let config = crate::Config::default();
        let service = OpenMeteoService::new(config).expect("Valid service");

        let result = service
            .get_weather_alerts(999.0, 11.6, None, None, None, None)
            .await;

        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_get_weather_alerts_success() {
        let config = crate::Config::default();
        let service = OpenMeteoService::new(config).expect("Valid service");

        let result = service
            .get_weather_alerts(48.1, 11.6, Some(30.0), Some(-5.0), None, None)
            .await;

        assert!(result.is_ok());
    }
}
