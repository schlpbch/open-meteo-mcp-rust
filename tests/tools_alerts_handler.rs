//! Tool handler tests for Weather Alerts
//! Phase 4: Comprehensive weather alerts tool testing

use open_meteo_mcp::OpenMeteoService;

#[tokio::test]
async fn test_get_weather_alerts_success_minimal() {
    let config = open_meteo_mcp::Config::default();
    let service = OpenMeteoService::new(config).expect("Valid service");

    let result = service.get_weather_alerts(
        48.1,
        11.6,
        None, None, None, None
    ).await;

    assert!(result.is_ok());
}

#[tokio::test]
async fn test_get_weather_alerts_with_temperature_hot() {
    let config = open_meteo_mcp::Config::default();
    let service = OpenMeteoService::new(config).expect("Valid service");

    let result = service.get_weather_alerts(
        48.1,
        11.6,
        Some(35.0),
        None, None, None
    ).await;

    assert!(result.is_ok());
}

#[tokio::test]
async fn test_get_weather_alerts_with_temperature_cold() {
    let config = open_meteo_mcp::Config::default();
    let service = OpenMeteoService::new(config).expect("Valid service");

    let result = service.get_weather_alerts(
        48.1,
        11.6,
        None, Some(-10.0), None, None
    ).await;

    assert!(result.is_ok());
}

#[tokio::test]
async fn test_get_weather_alerts_with_precipitation() {
    let config = open_meteo_mcp::Config::default();
    let service = OpenMeteoService::new(config).expect("Valid service");

    let result = service.get_weather_alerts(
        48.1,
        11.6,
        None, None, Some(50.0), None
    ).await;

    assert!(result.is_ok());
}

#[tokio::test]
async fn test_get_weather_alerts_with_wind_speed() {
    let config = open_meteo_mcp::Config::default();
    let service = OpenMeteoService::new(config).expect("Valid service");

    let result = service.get_weather_alerts(
        48.1,
        11.6,
        None, None, None, Some(60.0)
    ).await;

    assert!(result.is_ok());
}

#[tokio::test]
async fn test_get_weather_alerts_with_all_thresholds() {
    let config = open_meteo_mcp::Config::default();
    let service = OpenMeteoService::new(config).expect("Valid service");

    let result = service.get_weather_alerts(
        48.1,
        11.6,
        Some(35.0),
        Some(-10.0),
        Some(50.0),
        Some(60.0)
    ).await;

    assert!(result.is_ok());
}

#[tokio::test]
async fn test_get_weather_alerts_validation_latitude() {
    let config = open_meteo_mcp::Config::default();
    let service = OpenMeteoService::new(config).expect("Valid service");

    let result = service.get_weather_alerts(
        90.001,
        11.6,
        None, None, None, None
    ).await;

    assert!(result.is_err(), "Invalid latitude should be rejected");
}

#[tokio::test]
async fn test_get_weather_alerts_null_island() {
    let config = open_meteo_mcp::Config::default();
    let service = OpenMeteoService::new(config).expect("Valid service");

    let result = service.get_weather_alerts(
        0.0,
        0.0,
        Some(30.0),
        Some(0.0),
        None, None
    ).await;

    assert!(result.is_ok());
}
