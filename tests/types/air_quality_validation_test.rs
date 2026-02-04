//! Type validation tests for Air Quality
//! Mirrors Java AirQualityRequestTest patterns

mod utils {
    pub use crate::utils::*;
}

use open_meteo_mcp::types::air_quality::AirQualityRequest;

#[test]
fn test_air_quality_request_valid() {
    let req = AirQualityRequest {
        latitude: utils::VALID_LATITUDE,
        longitude: utils::VALID_LONGITUDE,
        hourly: None,
        daily: None,
        forecast_days: Some(5),
        ..Default::default()
    };
    
    assert!(req.validate().is_ok());
}

#[test]
fn test_air_quality_request_invalid_latitude() {
    let req = AirQualityRequest {
        latitude: utils::INVALID_LATITUDE_HIGH,
        longitude: utils::VALID_LONGITUDE,
        ..Default::default()
    };
    
    assert!(req.validate().is_err());
}

#[test]
fn test_air_quality_request_invalid_longitude() {
    let req = AirQualityRequest {
        latitude: utils::VALID_LATITUDE,
        longitude: utils::INVALID_LONGITUDE_HIGH,
        ..Default::default()
    };
    
    assert!(req.validate().is_err());
}

#[test]
fn test_air_quality_request_forecast_days_valid() {
    for days in 1..=5 {
        let req = AirQualityRequest {
            latitude: utils::VALID_LATITUDE,
            longitude: utils::VALID_LONGITUDE,
            forecast_days: Some(days),
            ..Default::default()
        };
        
        assert!(req.validate().is_ok(), "forecast_days {} should be valid for AQI", days);
    }
}

#[test]
fn test_air_quality_request_forecast_days_invalid_too_high() {
    let req = AirQualityRequest {
        latitude: utils::VALID_LATITUDE,
        longitude: utils::VALID_LONGITUDE,
        forecast_days: Some(6),
        ..Default::default()
    };
    
    assert!(req.validate().is_err(), "AQI forecast_days > 5 should be invalid");
}

#[test]
fn test_air_quality_response_deserialization() {
    let fixture = utils::parse_fixture::<open_meteo_mcp::types::air_quality::AirQualityResponse>(
        "air_quality_response.json"
    );
    
    assert_eq!(fixture.latitude, utils::VALID_LATITUDE);
    assert_eq!(fixture.longitude, utils::VALID_LONGITUDE);
    assert!(fixture.current.is_some());
}

#[test]
fn test_air_quality_serialization() {
    let req = AirQualityRequest {
        latitude: utils::VALID_LATITUDE,
        longitude: utils::VALID_LONGITUDE,
        hourly: Some("pm10,pm2_5,aqi".to_string()),
        daily: Some("aqi".to_string()),
        forecast_days: Some(3),
        ..Default::default()
    };
    
    let json = serde_json::to_value(&req).expect("Valid JSON");
    assert_eq!(json["latitude"], utils::VALID_LATITUDE);
    assert_eq!(json["forecast_days"], 3);
}
