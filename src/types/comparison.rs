//! Location comparison types

use serde::{Deserialize, Serialize};
use schemars::JsonSchema;
use crate::types::weather::WeatherResponse;

/// Request to compare weather across multiple locations
#[derive(Debug, Clone, Serialize)]
pub struct ComparisonRequest {
    pub locations: Vec<LocationCoords>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub hourly: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub daily: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub forecast_days: Option<u8>,
}

/// Coordinates for a location
#[derive(Debug, Clone, Serialize)]
pub struct LocationCoords {
    pub latitude: f64,
    pub longitude: f64,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

/// Comparison response with multiple weather forecasts
#[derive(Debug, Deserialize, Serialize, JsonSchema, Clone)]
pub struct ComparisonResponse {
    pub locations: Vec<LocationWeather>,
}

/// Weather data for a single location in comparison
#[derive(Debug, Deserialize, Serialize, JsonSchema, Clone)]
pub struct LocationWeather {
    pub name: Option<String>,
    pub weather: WeatherResponse,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_comparison_request_validation() {
        let req = ComparisonRequest {
            locations: vec![
                LocationCoords {
                    latitude: 48.1,
                    longitude: 11.6,
                    name: Some("Munich".to_string()),
                },
            ],
            hourly: None,
            daily: None,
            forecast_days: Some(7),
        };

        assert!(!req.locations.is_empty());
    }
}
