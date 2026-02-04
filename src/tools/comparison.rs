//! Location comparison tool (multi-location weather)

use crate::service::OpenMeteoService;
use crate::types::comparison::{ComparisonRequest, LocationCoords};
use crate::{CallToolResult, McpError, ToolContent};

impl OpenMeteoService {
    /// Compare weather across multiple locations
    ///
    /// ADR-008: Tool returns Result<CallToolResult, McpError> for MCP protocol compliance
    ///
    /// # Parameters
    /// * `locations` - Array of latitude/longitude pairs to compare
    /// * `hourly` - Optional comma-separated hourly variables
    /// * `daily` - Optional comma-separated daily variables
    /// * `forecast_days` - Optional forecast days (1-16, default 7)
    pub async fn compare_locations(
        &self,
        locations: Vec<(f64, f64)>, // (latitude, longitude) pairs
        hourly: Option<String>,
        daily: Option<String>,
        forecast_days: Option<u8>,
    ) -> std::result::Result<CallToolResult, McpError> {
        // Validate we have at least 2 locations
        if locations.len() < 2 {
            return Err(McpError::InvalidParameter(
                "At least 2 locations required for comparison".to_string(),
            ));
        }

        if locations.len() > 10 {
            return Err(McpError::InvalidParameter(
                "Maximum 10 locations allowed for comparison".to_string(),
            ));
        }

        // Validate all locations
        for (lat, lon) in &locations {
            crate::error::validate_coordinates(*lat, *lon).map_err(|e| {
                McpError::InvalidParameter(e.to_string())
            })?;
        }

        // Fetch weather for each location
        let mut location_weathers = Vec::new();

        for (latitude, longitude) in locations {
            let req = crate::types::weather::WeatherRequest {
                latitude,
                longitude,
                hourly: hourly.clone(),
                daily: daily.clone(),
                forecast_days,
                ..Default::default()
            };

            match self.client.get_weather(&req).await {
                Ok(weather) => {
                    location_weathers.push(serde_json::json!({
                        "latitude": latitude,
                        "longitude": longitude,
                        "weather": weather
                    }));
                }
                Err(_) => {
                    return Err(McpError::ToolError(
                        format!("Failed to fetch weather for location ({}, {})", latitude, longitude)
                    ));
                }
            }
        }

        // Format comparison response
        let comparison_response = serde_json::json!({
            "locations": location_weathers,
            "count": location_weathers.len()
        });

        Ok(CallToolResult::success(vec![ToolContent::Json(comparison_response)]))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_compare_locations_validation_too_few() {
        let config = crate::Config::default();
        let service = OpenMeteoService::new(config).expect("Valid service");

        let result = service.compare_locations(vec![(48.1, 11.6)], None, None, None).await;

        assert!(result.is_err());
        match result {
            Err(McpError::InvalidParameter(msg)) => {
                assert!(msg.contains("at least 2"));
            }
            _ => panic!("Expected InvalidParameter error"),
        }
    }

    #[tokio::test]
    async fn test_compare_locations_validation_too_many() {
        let config = crate::Config::default();
        let service = OpenMeteoService::new(config).expect("Valid service");

        let locations = vec![
            (48.1, 11.6),
            (51.5, -0.1),
            (48.8, 2.3),
            (52.5, 13.4),
            (41.9, 12.5),
            (55.75, 37.6),
            (50.1, 14.4),
            (60.1, 24.9),
            (59.3, 18.1),
            (52.2, 21.0),
            (47.5, 19.0), // 11th location
        ];

        let result = service.compare_locations(locations, None, None, None).await;

        assert!(result.is_err());
    }
}
