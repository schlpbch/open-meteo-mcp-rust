//! Comfort index API types

use serde::{Deserialize, Serialize};
use schemars::JsonSchema;

/// Request for comfort index calculation
#[derive(Debug, Clone, Serialize)]
pub struct ComfortRequest {
    pub latitude: f64,
    pub longitude: f64,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub activity_type: Option<String>, // e.g., "outdoor", "indoor", "sports"

    #[serde(skip_serializing_if = "Option::is_none")]
    pub forecast_days: Option<u8>,
}

impl Default for ComfortRequest {
    fn default() -> Self {
        Self {
            latitude: 0.0,
            longitude: 0.0,
            activity_type: Some("outdoor".to_string()),
            forecast_days: Some(7),
        }
    }
}

impl ComfortRequest {
    /// Validate comfort request parameters
    ///
    /// Checks:
    /// - latitude: -90 to 90
    /// - longitude: -180 to 180
    /// - forecast_days: 1-16
    pub fn validate(&self) -> crate::Result<()> {
        crate::error::validate_coordinates(self.latitude, self.longitude)?;

        if let Some(days) = self.forecast_days {
            if days < 1 || days > 16 {
                return Err(crate::Error::InvalidParameter(
                    format!("forecast_days must be between 1 and 16, got {}", days),
                ));
            }
        }

        Ok(())
    }
}

/// Comfort index response
#[derive(Debug, Deserialize, Serialize, JsonSchema, Clone)]
pub struct ComfortResponse {
    pub latitude: f64,
    pub longitude: f64,
    pub timezone: String,

    #[serde(default)]
    pub daily: Option<ComfortData>,
}

/// Daily comfort index data
#[derive(Debug, Deserialize, Serialize, JsonSchema, Clone)]
pub struct ComfortData {
    pub time: Vec<String>,

    // Comfort index: 0-100 (0=very uncomfortable, 100=very comfortable)
    pub comfort_index: Vec<u8>,

    #[serde(default)]
    pub comfort_level: Option<Vec<String>>, // e.g., "excellent", "good", "fair", "poor"

    #[serde(default)]
    pub temperature_2m: Option<Vec<f64>>,

    #[serde(default)]
    pub humidity: Option<Vec<u8>>,

    #[serde(default)]
    pub wind_speed_10m: Option<Vec<f64>>,

    #[serde(default)]
    pub precipitation_sum: Option<Vec<f64>>,

    #[serde(default)]
    pub recommendation: Option<Vec<String>>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_comfort_request_default() {
        let req = ComfortRequest::default();
        assert_eq!(req.activity_type, Some("outdoor".to_string()));
    }

    #[test]
    fn test_comfort_request_validation() {
        let valid_req = ComfortRequest {
            latitude: 48.1,
            longitude: 11.6,
            activity_type: Some("outdoor".to_string()),
            forecast_days: Some(7),
        };
        assert!(valid_req.validate().is_ok());
    }
}
