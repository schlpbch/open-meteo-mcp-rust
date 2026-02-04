//! Type validation tests for Weather requests and responses
//! Mirrors Java WeatherRequestTest and WeatherForecastTest patterns

mod utils {
    pub use crate::utils::*;
}

use open_meteo_mcp::types::weather::WeatherRequest;

#[test]
fn test_weather_request_valid_coordinates() {
    let req = WeatherRequest {
        latitude: utils::VALID_LATITUDE,
        longitude: utils::VALID_LONGITUDE,
        ..Default::default()
    };
    
    assert!(req.validate().is_ok());
}

#[test]
fn test_weather_request_invalid_latitude_high() {
    let req = WeatherRequest {
        latitude: utils::INVALID_LATITUDE_HIGH,
        longitude: utils::VALID_LONGITUDE,
        ..Default::default()
    };
    
    assert!(req.validate().is_err());
}

#[test]
fn test_weather_request_invalid_latitude_low() {
    let req = WeatherRequest {
        latitude: utils::INVALID_LATITUDE_LOW,
        longitude: utils::VALID_LONGITUDE,
        ..Default::default()
    };
    
    assert!(req.validate().is_err());
}

#[test]
fn test_weather_request_invalid_longitude_high() {
    let req = WeatherRequest {
        latitude: utils::VALID_LATITUDE,
        longitude: utils::INVALID_LONGITUDE_HIGH,
        ..Default::default()
    };
    
    assert!(req.validate().is_err());
}

#[test]
fn test_weather_request_invalid_longitude_low() {
    let req = WeatherRequest {
        latitude: utils::VALID_LATITUDE,
        longitude: utils::INVALID_LONGITUDE_LOW,
        ..Default::default()
    };
    
    assert!(req.validate().is_err());
}

#[test]
fn test_weather_request_boundary_latitude_max() {
    let req = WeatherRequest {
        latitude: utils::LATITUDE_MAX,
        longitude: 0.0,
        ..Default::default()
    };
    
    assert!(req.validate().is_ok(), "Latitude 90.0 should be valid");
}

#[test]
fn test_weather_request_boundary_latitude_min() {
    let req = WeatherRequest {
        latitude: utils::LATITUDE_MIN,
        longitude: 0.0,
        ..Default::default()
    };
    
    assert!(req.validate().is_ok(), "Latitude -90.0 should be valid");
}

#[test]
fn test_weather_request_boundary_longitude_max() {
    let req = WeatherRequest {
        latitude: 0.0,
        longitude: utils::LONGITUDE_MAX,
        ..Default::default()
    };
    
    assert!(req.validate().is_ok(), "Longitude 180.0 should be valid");
}

#[test]
fn test_weather_request_boundary_longitude_min() {
    let req = WeatherRequest {
        latitude: 0.0,
        longitude: utils::LONGITUDE_MIN,
        ..Default::default()
    };
    
    assert!(req.validate().is_ok(), "Longitude -180.0 should be valid");
}

#[test]
fn test_weather_request_null_island() {
    let req = WeatherRequest {
        latitude: utils::NULL_ISLAND_LAT,
        longitude: utils::NULL_ISLAND_LON,
        ..Default::default()
    };
    
    assert!(req.validate().is_ok(), "Null Island (0,0) should be valid");
}

#[test]
fn test_weather_request_forecast_days_valid_range() {
    for days in 1..=16 {
        let req = WeatherRequest {
            latitude: utils::VALID_LATITUDE,
            longitude: utils::VALID_LONGITUDE,
            forecast_days: Some(days),
            ..Default::default()
        };
        
        assert!(req.validate().is_ok(), "forecast_days {} should be valid", days);
    }
}

#[test]
fn test_weather_request_forecast_days_invalid_zero() {
    let req = WeatherRequest {
        latitude: utils::VALID_LATITUDE,
        longitude: utils::VALID_LONGITUDE,
        forecast_days: Some(0),
        ..Default::default()
    };
    
    assert!(req.validate().is_err(), "forecast_days 0 should be invalid");
}

#[test]
fn test_weather_request_forecast_days_invalid_too_high() {
    let req = WeatherRequest {
        latitude: utils::VALID_LATITUDE,
        longitude: utils::VALID_LONGITUDE,
        forecast_days: Some(17),
        ..Default::default()
    };
    
    assert!(req.validate().is_err(), "forecast_days 17 should be invalid");
}

#[test]
fn test_weather_request_serialization() {
    let req = WeatherRequest {
        latitude: utils::VALID_LATITUDE,
        longitude: utils::VALID_LONGITUDE,
        hourly: Some("temperature_2m,precipitation".to_string()),
        daily: Some("weather_code".to_string()),
        forecast_days: Some(7),
        ..Default::default()
    };
    
    let json = serde_json::to_value(&req).expect("Valid JSON");
    assert_eq!(json["latitude"], utils::VALID_LATITUDE);
    assert_eq!(json["longitude"], utils::VALID_LONGITUDE);
    assert_eq!(json["forecast_days"], 7);
}

#[test]
fn test_weather_response_deserialization_complete() {
    let fixture = utils::parse_fixture::<open_meteo_mcp::types::weather::WeatherResponse>(
        "weather_response.json"
    );
    
    assert_eq!(fixture.latitude, utils::VALID_LATITUDE);
    assert_eq!(fixture.longitude, utils::VALID_LONGITUDE);
    assert_eq!(fixture.timezone, "Europe/Berlin");
    assert!(fixture.current.is_some());
}

#[test]
fn test_weather_response_deserialization_minimal() {
    let json = r#"{
        "latitude": 48.1,
        "longitude": 11.6,
        "elevation": 518.0,
        "timezone": "Europe/Berlin"
    }"#;
    
    let response: Result<open_meteo_mcp::types::weather::WeatherResponse, _> = 
        serde_json::from_str(json);
    assert!(response.is_ok());
}
