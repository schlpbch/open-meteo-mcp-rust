//! Geocoding/Location types

use serde::{Deserialize, Serialize};
use schemars::JsonSchema;

/// Request for location search/geocoding
#[derive(Debug, Clone, Serialize)]
pub struct GeocodeRequest {
    pub name: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub count: Option<u32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub language: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub format: Option<String>,
}

impl GeocodeRequest {
    pub fn new(name: String) -> Self {
        Self {
            name,
            count: Some(10),
            language: None,
            format: None,
        }
    }
}

/// Geocoding response
#[derive(Debug, Deserialize, Serialize, JsonSchema, Clone)]
pub struct GeocodeResponse {
    #[serde(default)]
    pub results: Vec<Location>,

    #[serde(default)]
    pub generationtime_ms: f64,
}

/// A single location result
#[derive(Debug, Deserialize, Serialize, JsonSchema, Clone)]
pub struct Location {
    pub id: u32,
    pub name: String,

    #[serde(default)]
    pub latitude: f64,

    #[serde(default)]
    pub longitude: f64,

    #[serde(default)]
    pub elevation: Option<f64>,

    #[serde(default)]
    pub feature_code: Option<String>,

    #[serde(default)]
    pub country_code: Option<String>,

    #[serde(default)]
    pub country: Option<String>,

    #[serde(default)]
    pub admin1: Option<String>,

    #[serde(default)]
    pub admin2: Option<String>,

    #[serde(default)]
    pub admin3: Option<String>,

    #[serde(default)]
    pub timezone: Option<String>,

    #[serde(default)]
    pub population: Option<u32>,

    #[serde(default)]
    pub distance_m: Option<f64>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_geocode_request_new() {
        let req = GeocodeRequest::new("Munich".to_string());
        assert_eq!(req.name, "Munich");
        assert_eq!(req.count, Some(10));
    }

    #[test]
    fn test_location_serialization() {
        let loc = Location {
            id: 1,
            name: "Munich".to_string(),
            latitude: 48.1,
            longitude: 11.6,
            elevation: Some(518.0),
            feature_code: Some("CITY".to_string()),
            country_code: Some("DE".to_string()),
            country: Some("Germany".to_string()),
            admin1: Some("Bavaria".to_string()),
            admin2: None,
            admin3: None,
            timezone: Some("Europe/Berlin".to_string()),
            population: Some(1500000),
            distance_m: None,
        };

        let json = serde_json::to_value(&loc).unwrap();
        assert_eq!(json["name"], "Munich");
        assert_eq!(json["latitude"], 48.1);
    }
}
