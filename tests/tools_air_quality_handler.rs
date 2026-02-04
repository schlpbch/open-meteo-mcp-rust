//! Tool handler tests for Air Quality
//! Phase 4: Comprehensive air quality tool testing

use open_meteo_mcp::OpenMeteoService;

#[tokio::test]
async fn test_get_air_quality_validation_latitude_high() {
    let config = open_meteo_mcp::Config::default();
    let service = OpenMeteoService::new(config).expect("Valid service");

    let result = service.get_air_quality(
        90.001,
        11.6,
        None, None
    ).await;

    assert!(result.is_err(), "Invalid latitude should be rejected");
}

#[tokio::test]
async fn test_get_air_quality_validation_latitude_low() {
    let config = open_meteo_mcp::Config::default();
    let service = OpenMeteoService::new(config).expect("Valid service");

    let result = service.get_air_quality(
        -90.001,
        11.6,
        None, None
    ).await;

    assert!(result.is_err(), "Invalid latitude should be rejected");
}

#[tokio::test]
async fn test_get_air_quality_validation_longitude_high() {
    let config = open_meteo_mcp::Config::default();
    let service = OpenMeteoService::new(config).expect("Valid service");

    let result = service.get_air_quality(
        48.1,
        180.001,
        None, None
    ).await;

    assert!(result.is_err(), "Invalid longitude should be rejected");
}

#[tokio::test]
async fn test_get_air_quality_validation_longitude_low() {
    let config = open_meteo_mcp::Config::default();
    let service = OpenMeteoService::new(config).expect("Valid service");

    let result = service.get_air_quality(
        48.1,
        -180.001,
        None, None
    ).await;

    assert!(result.is_err(), "Invalid longitude should be rejected");
}

#[test]
fn test_air_quality_request_serialization() {
    let req = open_meteo_mcp::types::air_quality::AirQualityRequest {
        latitude: 48.1,
        longitude: 11.6,
        hourly: Some("pm10,pm2_5".to_string()),
        daily: Some("aqi".to_string()),
        ..Default::default()
    };

    let json = serde_json::to_value(&req).expect("Valid JSON");
    assert_eq!(json["latitude"], 48.1);
    assert_eq!(json["longitude"], 11.6);
}

#[test]
fn test_air_quality_request_validation_empty_hourly_daily() {
    let req = open_meteo_mcp::types::air_quality::AirQualityRequest {
        latitude: 48.1,
        longitude: 11.6,
        hourly: None,
        daily: None,
        ..Default::default()
    };

    assert!(req.validate().is_ok(), "AirQuality with coordinates should be valid");
}
