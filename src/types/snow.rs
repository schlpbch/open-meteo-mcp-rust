//! Snow conditions API types

use serde::{Deserialize, Serialize};
use schemars::JsonSchema;

/// Request for snow conditions data
#[derive(Debug, Clone, Serialize)]
pub struct SnowRequest {
    pub latitude: f64,
    pub longitude: f64,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub hourly: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub daily: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub timezone: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub forecast_days: Option<u8>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub temperature_unit: Option<String>,
}

impl Default for SnowRequest {
    fn default() -> Self {
        Self {
            latitude: 0.0,
            longitude: 0.0,
            hourly: None,
            daily: None,
            timezone: None,
            forecast_days: Some(7),
            temperature_unit: None,
        }
    }
}

impl SnowRequest {
    /// Validate snow request parameters
    ///
    /// Checks:
    /// - latitude: -90 to 90
    /// - longitude: -180 to 180
    /// - forecast_days: 1-16 (API limit)
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

/// Snow conditions response
#[derive(Debug, Deserialize, Serialize, JsonSchema, Clone)]
pub struct SnowResponse {
    pub latitude: f64,
    pub longitude: f64,
    pub elevation: f64,
    pub timezone: String,
    pub timezone_abbreviation: Option<String>,

    #[serde(default)]
    pub hourly: Option<SnowData>,

    #[serde(default)]
    pub hourly_units: Option<std::collections::HashMap<String, String>>,

    #[serde(default)]
    pub daily: Option<SnowData>,

    #[serde(default)]
    pub daily_units: Option<std::collections::HashMap<String, String>>,
}

/// Snow measurement data
#[derive(Debug, Deserialize, Serialize, JsonSchema, Clone)]
pub struct SnowData {
    pub time: Vec<String>,

    #[serde(default)]
    pub snowfall: Option<Vec<f64>>,

    #[serde(default)]
    pub snow_depth: Option<Vec<f64>>,

    #[serde(default)]
    pub temperature_2m: Option<Vec<f64>>,

    #[serde(default)]
    pub precipitation: Option<Vec<f64>>,

    #[serde(default)]
    pub wind_speed_10m: Option<Vec<f64>>,

    #[serde(default)]
    pub cloud_cover: Option<Vec<u8>>,

    #[serde(default)]
    pub weather_code: Option<Vec<u16>>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_snow_request_default() {
        let req = SnowRequest::default();
        assert_eq!(req.forecast_days, Some(7));
    }

    #[test]
    fn test_snow_request_validation() {
        let valid_req = SnowRequest {
            latitude: 45.5,
            longitude: 11.0,
            forecast_days: Some(7),
            ..Default::default()
        };
        assert!(valid_req.validate().is_ok());

        let invalid_req = SnowRequest {
            latitude: 45.5,
            longitude: 11.0,
            forecast_days: Some(20),
            ..Default::default()
        };
        assert!(invalid_req.validate().is_err());
    }
}
