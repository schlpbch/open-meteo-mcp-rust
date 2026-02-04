//! Weather alerts API types

use serde::{Deserialize, Serialize};
use schemars::JsonSchema;

/// Request for weather alerts
#[derive(Debug, Clone, Serialize)]
pub struct AlertsRequest {
    pub latitude: f64,
    pub longitude: f64,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub temperature_threshold_hot: Option<f64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub temperature_threshold_cold: Option<f64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub precipitation_threshold: Option<f64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub wind_speed_threshold: Option<f64>,
}

impl Default for AlertsRequest {
    fn default() -> Self {
        Self {
            latitude: 0.0,
            longitude: 0.0,
            temperature_threshold_hot: None,
            temperature_threshold_cold: None,
            precipitation_threshold: None,
            wind_speed_threshold: None,
        }
    }
}

impl AlertsRequest {
    /// Validate alerts request parameters
    ///
    /// Checks:
    /// - latitude: -90 to 90
    /// - longitude: -180 to 180
    pub fn validate(&self) -> crate::Result<()> {
        crate::error::validate_coordinates(self.latitude, self.longitude)?;
        Ok(())
    }
}

/// Weather alerts response
#[derive(Debug, Deserialize, Serialize, JsonSchema, Clone)]
pub struct AlertsResponse {
    pub latitude: f64,
    pub longitude: f64,
    pub timezone: String,

    #[serde(default)]
    pub alerts: Vec<WeatherAlert>,
}

/// Individual weather alert
#[derive(Debug, Deserialize, Serialize, JsonSchema, Clone)]
pub struct WeatherAlert {
    pub alert_type: String,
    pub severity: String,
    pub start_time: String,
    pub end_time: String,
    pub description: String,

    #[serde(default)]
    pub thresholds: std::collections::HashMap<String, f64>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_alerts_request_default() {
        let req = AlertsRequest::default();
        assert_eq!(req.temperature_threshold_hot, None);
    }

    #[test]
    fn test_alerts_request_validation() {
        let valid_req = AlertsRequest {
            latitude: 48.1,
            longitude: 11.6,
            ..Default::default()
        };
        assert!(valid_req.validate().is_ok());

        let invalid_req = AlertsRequest {
            latitude: 999.0,
            longitude: 11.6,
            ..Default::default()
        };
        assert!(invalid_req.validate().is_err());
    }
}
