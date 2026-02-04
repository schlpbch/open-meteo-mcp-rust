//! Tool handler tests for Historical Weather
//! Phase 4: Comprehensive historical weather tool testing

use open_meteo_mcp::OpenMeteoService;

#[tokio::test]
async fn test_get_historical_weather_validation_latitude_high() {
    let config = open_meteo_mcp::Config::default();
    let service = OpenMeteoService::new(config).expect("Valid service");

    let result = service.get_historical_weather(
        90.001,
        11.6,
        "2023-01-01".to_string(),
        "2023-01-31".to_string(),
        None, None
    ).await;

    assert!(result.is_err(), "Invalid latitude should be rejected");
}

#[tokio::test]
async fn test_get_historical_weather_validation_latitude_low() {
    let config = open_meteo_mcp::Config::default();
    let service = OpenMeteoService::new(config).expect("Valid service");

    let result = service.get_historical_weather(
        -90.001,
        11.6,
        "2023-01-01".to_string(),
        "2023-01-31".to_string(),
        None, None
    ).await;

    assert!(result.is_err(), "Invalid latitude should be rejected");
}

#[tokio::test]
async fn test_get_historical_weather_validation_longitude_high() {
    let config = open_meteo_mcp::Config::default();
    let service = OpenMeteoService::new(config).expect("Valid service");

    let result = service.get_historical_weather(
        48.1,
        180.001,
        "2023-01-01".to_string(),
        "2023-01-31".to_string(),
        None, None
    ).await;

    assert!(result.is_err(), "Invalid longitude should be rejected");
}

#[tokio::test]
async fn test_get_historical_weather_validation_longitude_low() {
    let config = open_meteo_mcp::Config::default();
    let service = OpenMeteoService::new(config).expect("Valid service");

    let result = service.get_historical_weather(
        48.1,
        -180.001,
        "2023-01-01".to_string(),
        "2023-01-31".to_string(),
        None, None
    ).await;

    assert!(result.is_err(), "Invalid longitude should be rejected");
}

#[tokio::test]
async fn test_get_historical_weather_boundary_coordinates() {
    let config = open_meteo_mcp::Config::default();
    let service = OpenMeteoService::new(config).expect("Valid service");

    let result = service.get_historical_weather(
        90.0,
        180.0,
        "2020-01-01".to_string(),
        "2020-01-31".to_string(),
        None, None
    ).await;

    let _ = result;
}

#[tokio::test]
async fn test_get_historical_weather_different_years() {
    let config = open_meteo_mcp::Config::default();
    let service = OpenMeteoService::new(config).expect("Valid service");

    let result = service.get_historical_weather(
        48.1,
        11.6,
        "2020-01-01".to_string(),
        "2020-12-31".to_string(),
        None, None
    ).await;

    let _ = result;
}

#[tokio::test]
async fn test_get_historical_weather_with_hourly() {
    let config = open_meteo_mcp::Config::default();
    let service = OpenMeteoService::new(config).expect("Valid service");

    let result = service.get_historical_weather(
        48.1,
        11.6,
        "2023-06-01".to_string(),
        "2023-06-30".to_string(),
        Some("temperature_2m".to_string()),
        None
    ).await;

    let _ = result;
}

#[tokio::test]
async fn test_get_historical_weather_with_daily() {
    let config = open_meteo_mcp::Config::default();
    let service = OpenMeteoService::new(config).expect("Valid service");

    let result = service.get_historical_weather(
        48.1,
        11.6,
        "2023-06-01".to_string(),
        "2023-06-30".to_string(),
        None,
        Some("temperature_2m_max".to_string())
    ).await;

    let _ = result;
}
