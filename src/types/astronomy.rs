//! Astronomy API types (sunrise, sunset, moon phases)

use serde::{Deserialize, Serialize};
use schemars::JsonSchema;

/// Request for astronomy data
#[derive(Debug, Clone, Serialize)]
pub struct AstronomyRequest {
    pub latitude: f64,
    pub longitude: f64,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub timezone: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub forecast_days: Option<u8>,
}

impl Default for AstronomyRequest {
    fn default() -> Self {
        Self {
            latitude: 0.0,
            longitude: 0.0,
            timezone: None,
            forecast_days: Some(7),
        }
    }
}

impl AstronomyRequest {
    /// Validate astronomy request parameters
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

/// Astronomy data response
#[derive(Debug, Deserialize, Serialize, JsonSchema, Clone)]
pub struct AstronomyResponse {
    pub latitude: f64,
    pub longitude: f64,
    pub timezone: String,

    #[serde(default)]
    pub daily: Option<AstronomyData>,

    #[serde(default)]
    pub daily_units: Option<std::collections::HashMap<String, String>>,
}

/// Daily astronomy data
#[derive(Debug, Deserialize, Serialize, JsonSchema, Clone)]
pub struct AstronomyData {
    pub time: Vec<String>,

    #[serde(default)]
    pub sunrise: Option<Vec<String>>,

    #[serde(default)]
    pub sunset: Option<Vec<String>>,

    #[serde(default)]
    pub sunrise_end: Option<Vec<String>>,

    #[serde(default)]
    pub sunset_start: Option<Vec<String>>,

    #[serde(default)]
    pub moon_phase: Option<Vec<f64>>,

    #[serde(default)]
    pub moon_altitude: Option<Vec<f64>>,

    #[serde(default)]
    pub moon_distance: Option<Vec<f64>>,

    #[serde(default)]
    pub moon_illumination: Option<Vec<f64>>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_astronomy_request_default() {
        let req = AstronomyRequest::default();
        assert_eq!(req.forecast_days, Some(7));
    }

    #[test]
    fn test_astronomy_request_validation() {
        let valid_req = AstronomyRequest {
            latitude: 48.1,
            longitude: 11.6,
            forecast_days: Some(7),
            ..Default::default()
        };
        assert!(valid_req.validate().is_ok());
    }
}
