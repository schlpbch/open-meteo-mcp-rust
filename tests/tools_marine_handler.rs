//! Tool handler tests for Marine Conditions
//! Phase 4: Comprehensive marine weather tool testing

use open_meteo_mcp::OpenMeteoService;

#[tokio::test]
async fn test_get_marine_conditions_success_minimal() {
    let config = open_meteo_mcp::Config::default();
    let service = OpenMeteoService::new(config).expect("Valid service");

    let result = service.get_marine_conditions(
        48.1,
        11.6,
        None, None, None
    ).await;

    assert!(result.is_ok());
}

#[tokio::test]
async fn test_get_marine_conditions_with_daily() {
    let config = open_meteo_mcp::Config::default();
    let service = OpenMeteoService::new(config).expect("Valid service");

    let result = service.get_marine_conditions(
        48.1,
        11.6,
        None,
        Some("wave_height_max".to_string()),
        Some(5)
    ).await;

    assert!(result.is_ok());
}

#[tokio::test]
async fn test_get_marine_conditions_validation_latitude() {
    let config = open_meteo_mcp::Config::default();
    let service = OpenMeteoService::new(config).expect("Valid service");

    let result = service.get_marine_conditions(
        90.001,
        11.6,
        None, None, None
    ).await;

    assert!(result.is_err(), "Invalid latitude should be rejected");
}

#[tokio::test]
async fn test_get_marine_conditions_validation_longitude() {
    let config = open_meteo_mcp::Config::default();
    let service = OpenMeteoService::new(config).expect("Valid service");

    let result = service.get_marine_conditions(
        48.1,
        180.001,
        None, None, None
    ).await;

    assert!(result.is_err(), "Invalid longitude should be rejected");
}

#[tokio::test]
async fn test_get_marine_conditions_boundary_coordinates() {
    let config = open_meteo_mcp::Config::default();
    let service = OpenMeteoService::new(config).expect("Valid service");

    let result = service.get_marine_conditions(
        90.0,
        180.0,
        None, None, None
    ).await;

    assert!(result.is_ok(), "Boundary coordinates should be valid");
}

#[tokio::test]
async fn test_get_marine_conditions_forecast_days_valid_max() {
    let config = open_meteo_mcp::Config::default();
    let service = OpenMeteoService::new(config).expect("Valid service");

    let result = service.get_marine_conditions(
        48.1,
        11.6,
        None, None, Some(16)
    ).await;

    assert!(result.is_ok(), "forecast_days 16 should be valid");
}

#[tokio::test]
async fn test_get_marine_conditions_forecast_days_invalid_zero() {
    let config = open_meteo_mcp::Config::default();
    let service = OpenMeteoService::new(config).expect("Valid service");

    let result = service.get_marine_conditions(
        48.1,
        11.6,
        None, None, Some(0)
    ).await;

    assert!(result.is_err(), "forecast_days 0 should be invalid");
}

#[tokio::test]
async fn test_get_marine_conditions_null_island() {
    let config = open_meteo_mcp::Config::default();
    let service = OpenMeteoService::new(config).expect("Valid service");

    let result = service.get_marine_conditions(
        0.0,
        0.0,
        None, None, None
    ).await;

    assert!(result.is_ok());
}
