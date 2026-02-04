//! Air quality API types

use serde::{Deserialize, Serialize};
use schemars::JsonSchema;

/// Request for air quality data
#[derive(Debug, Clone, Serialize)]
pub struct AirQualityRequest {
    pub latitude: f64,
    pub longitude: f64,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub hourly: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub daily: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub timezone: Option<String>,
}

impl Default for AirQualityRequest {
    fn default() -> Self {
        Self {
            latitude: 0.0,
            longitude: 0.0,
            hourly: None,
            daily: None,
            timezone: None,
        }
    }
}

impl AirQualityRequest {
    /// Validate air quality request parameters
    ///
    /// Checks:
    /// - latitude: -90 to 90
    /// - longitude: -180 to 180
    pub fn validate(&self) -> crate::Result<()> {
        crate::error::validate_coordinates(self.latitude, self.longitude)?;
        Ok(())
    }
}

/// Air quality response
#[derive(Debug, Deserialize, Serialize, JsonSchema, Clone)]
pub struct AirQualityResponse {
    pub latitude: f64,
    pub longitude: f64,
    pub elevation: f64,
    pub timezone: String,

    #[serde(default)]
    pub hourly: Option<AirQualityData>,

    #[serde(default)]
    pub hourly_units: Option<std::collections::HashMap<String, String>>,

    #[serde(default)]
    pub daily: Option<AirQualityData>,

    #[serde(default)]
    pub daily_units: Option<std::collections::HashMap<String, String>>,
}

/// Air quality measurements
#[derive(Debug, Deserialize, Serialize, JsonSchema, Clone)]
pub struct AirQualityData {
    pub time: Vec<String>,

    #[serde(default)]
    pub pm10: Option<Vec<f64>>,

    #[serde(default)]
    pub pm2_5: Option<Vec<f64>>,

    #[serde(default)]
    pub o3: Option<Vec<f64>>,

    #[serde(default)]
    pub no2: Option<Vec<f64>>,

    #[serde(default)]
    pub so2: Option<Vec<f64>>,

    #[serde(default)]
    pub co: Option<Vec<f64>>,

    #[serde(default)]
    pub us_aqi: Option<Vec<u16>>,

    #[serde(default)]
    pub european_aqi: Option<Vec<u16>>,

    #[serde(default)]
    pub uv_index: Option<Vec<f64>>,

    #[serde(default)]
    pub pollen_alder: Option<Vec<u32>>,

    #[serde(default)]
    pub pollen_birch: Option<Vec<u32>>,

    #[serde(default)]
    pub pollen_grass: Option<Vec<u32>>,

    #[serde(default)]
    pub pollen_mugwort: Option<Vec<u32>>,

    #[serde(default)]
    pub pollen_olive: Option<Vec<u32>>,

    #[serde(default)]
    pub pollen_ragweed: Option<Vec<u32>>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_air_quality_request_default() {
        let req = AirQualityRequest::default();
        assert_eq!(req.latitude, 0.0);
        assert_eq!(req.longitude, 0.0);
    }
}
