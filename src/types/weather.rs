//! Weather API types

use serde::{Deserialize, Serialize};
use schemars::JsonSchema;

/// Request for weather forecast data
#[derive(Debug, Clone, Serialize)]
pub struct WeatherRequest {
    pub latitude: f64,
    pub longitude: f64,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub elevation: Option<f64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub hourly: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub daily: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub current: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub timezone: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub forecast_days: Option<u8>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub temperature_unit: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub wind_speed_unit: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub precipitation_unit: Option<String>,
}

impl Default for WeatherRequest {
    fn default() -> Self {
        Self {
            latitude: 0.0,
            longitude: 0.0,
            elevation: None,
            hourly: None,
            daily: None,
            current: None,
            timezone: None,
            forecast_days: Some(7),
            temperature_unit: None,
            wind_speed_unit: None,
            precipitation_unit: None,
        }
    }
}

impl WeatherRequest {
    /// Validate weather request parameters
    ///
    /// Checks:
    /// - forecast_days: 1-16 (API limit)
    /// - latitude: -90 to 90
    /// - longitude: -180 to 180
    pub fn validate(&self) -> crate::Result<()> {
        // Validate coordinates
        crate::error::validate_coordinates(self.latitude, self.longitude)?;

        // Validate forecast_days range (Open-Meteo supports 1-16)
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

/// Weather forecast response
#[derive(Debug, Deserialize, Serialize, JsonSchema, Clone)]
pub struct WeatherResponse {
    pub latitude: f64,
    pub longitude: f64,
    pub elevation: f64,
    pub timezone: String,
    pub timezone_abbreviation: Option<String>,

    #[serde(default)]
    pub current: Option<CurrentWeather>,

    #[serde(default)]
    pub current_units: Option<std::collections::HashMap<String, String>>,

    #[serde(default)]
    pub hourly: Option<HourlyData>,

    #[serde(default)]
    pub hourly_units: Option<std::collections::HashMap<String, String>>,

    #[serde(default)]
    pub daily: Option<DailyData>,

    #[serde(default)]
    pub daily_units: Option<std::collections::HashMap<String, String>>,
}

/// Current weather conditions
#[derive(Debug, Deserialize, Serialize, JsonSchema, Clone)]
pub struct CurrentWeather {
    pub time: String,
    pub interval: u32,
    pub temperature: f64,
    pub relative_humidity: u8,
    pub weather_code: u16,
    pub wind_speed_10m: f64,
    pub wind_direction_10m: u16,

    #[serde(default)]
    pub is_day: Option<u8>,

    #[serde(default)]
    pub apparent_temperature: Option<f64>,

    #[serde(default)]
    pub precipitation: Option<f64>,

    #[serde(default)]
    pub rain: Option<f64>,

    #[serde(default)]
    pub showers: Option<f64>,

    #[serde(default)]
    pub snowfall: Option<f64>,

    #[serde(default)]
    pub cloud_cover: Option<u8>,
}

/// Hourly forecast data
#[derive(Debug, Deserialize, Serialize, JsonSchema, Clone)]
pub struct HourlyData {
    pub time: Vec<String>,
    pub temperature_2m: Vec<f64>,
    pub precipitation: Vec<f64>,
    pub weather_code: Vec<u16>,
    pub wind_speed_10m: Vec<f64>,
    pub wind_direction_10m: Vec<u16>,

    #[serde(default)]
    pub humidity: Option<Vec<u8>>,

    #[serde(default)]
    pub apparent_temperature: Option<Vec<f64>>,

    #[serde(default)]
    pub cloud_cover: Option<Vec<u8>>,

    #[serde(default)]
    pub dew_point_2m: Option<Vec<f64>>,

    #[serde(default)]
    pub relative_humidity_2m: Option<Vec<u8>>,
}

/// Daily forecast data
#[derive(Debug, Deserialize, Serialize, JsonSchema, Clone)]
pub struct DailyData {
    pub time: Vec<String>,
    pub weather_code: Vec<u16>,
    pub temperature_2m_max: Vec<f64>,
    pub temperature_2m_min: Vec<f64>,
    pub precipitation_sum: Vec<f64>,
    pub wind_speed_10m_max: Vec<f64>,
    pub wind_direction_10m_dominant: Vec<u16>,

    #[serde(default)]
    pub precipitation_probability_max: Option<Vec<u8>>,

    #[serde(default)]
    pub showers_sum: Option<Vec<f64>>,

    #[serde(default)]
    pub snowfall_sum: Option<Vec<f64>>,

    #[serde(default)]
    pub sunrise: Option<Vec<String>>,

    #[serde(default)]
    pub sunset: Option<Vec<String>>,

    #[serde(default)]
    pub uv_index_max: Option<Vec<f64>>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_weather_request_default() {
        let req = WeatherRequest::default();
        assert_eq!(req.latitude, 0.0);
        assert_eq!(req.forecast_days, Some(7));
    }

    #[test]
    fn test_weather_request_serialization() {
        let req = WeatherRequest {
            latitude: 48.1,
            longitude: 11.6,
            forecast_days: Some(7),
            ..Default::default()
        };

        let json = serde_json::to_value(&req).expect("Valid JSON serialization");
        assert_eq!(json["latitude"], 48.1);
        assert_eq!(json["longitude"], 11.6);
        assert!(json["hourly"].is_null()); // skip_serializing_if
    }

    #[test]
    fn test_weather_request_validation() {
        let valid_req = WeatherRequest {
            latitude: 48.1,
            longitude: 11.6,
            forecast_days: Some(7),
            ..Default::default()
        };
        assert!(valid_req.validate().is_ok());

        let invalid_forecast = WeatherRequest {
            latitude: 48.1,
            longitude: 11.6,
            forecast_days: Some(20),  // Out of range
            ..Default::default()
        };
        assert!(invalid_forecast.validate().is_err());

        let invalid_coords = WeatherRequest {
            latitude: 999.0,  // Out of range
            longitude: 11.6,
            ..Default::default()
        };
        assert!(invalid_coords.validate().is_err());
    }
}
