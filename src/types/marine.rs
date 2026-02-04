//! Marine API types (waves, swells)

use serde::{Deserialize, Serialize};
use schemars::JsonSchema;

/// Request for marine data
#[derive(Debug, Clone, Serialize)]
pub struct MarineRequest {
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
}

impl Default for MarineRequest {
    fn default() -> Self {
        Self {
            latitude: 0.0,
            longitude: 0.0,
            hourly: None,
            daily: None,
            timezone: None,
            forecast_days: Some(7),
        }
    }
}

/// Marine conditions response
#[derive(Debug, Deserialize, Serialize, JsonSchema, Clone)]
pub struct MarineResponse {
    pub latitude: f64,
    pub longitude: f64,
    pub timezone: String,

    #[serde(default)]
    pub hourly: Option<WaveData>,

    #[serde(default)]
    pub hourly_units: Option<std::collections::HashMap<String, String>>,

    #[serde(default)]
    pub daily: Option<WaveData>,

    #[serde(default)]
    pub daily_units: Option<std::collections::HashMap<String, String>>,
}

/// Wave and swell data
#[derive(Debug, Deserialize, Serialize, JsonSchema, Clone)]
pub struct WaveData {
    pub time: Vec<String>,

    #[serde(default)]
    pub wave_height: Option<Vec<f64>>,

    #[serde(default)]
    pub wave_direction: Option<Vec<u16>>,

    #[serde(default)]
    pub wave_period: Option<Vec<f64>>,

    #[serde(default)]
    pub swell_wave_height: Option<Vec<f64>>,

    #[serde(default)]
    pub swell_wave_direction: Option<Vec<u16>>,

    #[serde(default)]
    pub swell_wave_period: Option<Vec<f64>>,

    #[serde(default)]
    pub wind_wave_height: Option<Vec<f64>>,

    #[serde(default)]
    pub wind_wave_direction: Option<Vec<u16>>,

    #[serde(default)]
    pub wind_wave_period: Option<Vec<f64>>,

    #[serde(default)]
    pub wind_speed_10m: Option<Vec<f64>>,

    #[serde(default)]
    pub wind_direction_10m: Option<Vec<u16>>,

    #[serde(default)]
    pub weather_code: Option<Vec<u16>>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_marine_request_default() {
        let req = MarineRequest::default();
        assert_eq!(req.forecast_days, Some(7));
    }
}
