//! Shared test utilities and helpers

use std::fs;

/// Load a fixture file from tests/fixtures directory
pub fn load_fixture(filename: &str) -> String {
    let path = format!("tests/fixtures/{}", filename);
    fs::read_to_string(&path)
        .unwrap_or_else(|e| panic!("Failed to load fixture {}: {}", filename, e))
}

/// Parse fixture JSON into type
pub fn parse_fixture<T: serde::de::DeserializeOwned>(filename: &str) -> T {
    let json = load_fixture(filename);
    serde_json::from_str(&json)
        .unwrap_or_else(|e| panic!("Failed to parse fixture {}: {}", filename, e))
}

/// Test coordinates - Valid
pub const VALID_LATITUDE: f64 = 48.1;
pub const VALID_LONGITUDE: f64 = 11.6;

/// Test coordinates - Boundaries
pub const LATITUDE_MAX: f64 = 90.0;
pub const LATITUDE_MIN: f64 = -90.0;
pub const LONGITUDE_MAX: f64 = 180.0;
pub const LONGITUDE_MIN: f64 = -180.0;

/// Test coordinates - Invalid (out of bounds)
pub const INVALID_LATITUDE_HIGH: f64 = 90.001;
pub const INVALID_LATITUDE_LOW: f64 = -90.001;
pub const INVALID_LONGITUDE_HIGH: f64 = 180.001;
pub const INVALID_LONGITUDE_LOW: f64 = -180.001;

/// Null Island
pub const NULL_ISLAND_LAT: f64 = 0.0;
pub const NULL_ISLAND_LON: f64 = 0.0;
