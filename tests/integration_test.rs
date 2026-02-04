//! Integration tests with wiremock fixtures

use open_meteo_mcp::{OpenMeteoClient, Config};
use std::sync::Arc;
use wiremock::{Mock, MockServer, ResponseTemplate, matchers};

#[tokio::test]
async fn test_get_weather_success() {
    // Start mock server
    let mock_server = MockServer::start().await;

    // Load fixture
    let fixture = include_str!("fixtures/weather_response.json");

    // Setup mock endpoint
    Mock::given(matchers::path("/v1/forecast"))
        .respond_with(ResponseTemplate::new(200).set_body_string(fixture))
        .mount(&mock_server)
        .await;

    // Create client pointing to mock server
    let http_client = Arc::new(
        reqwest::Client::builder()
            .timeout(std::time::Duration::from_secs(30))
            .build()
            .unwrap(),
    );

    let client = OpenMeteoClient::new(http_client);

    // Test weather request
    let req = open_meteo_mcp::types::weather::WeatherRequest {
        latitude: 48.1,
        longitude: 11.6,
        forecast_days: Some(7),
        hourly: Some("temperature_2m,precipitation".to_string()),
        daily: Some("temperature_2m_max,temperature_2m_min".to_string()),
        ..Default::default()
    };

    let result = client.get_weather(&req).await;

    assert!(result.is_ok());
    let response = result.unwrap();
    assert_eq!(response.latitude, 48.1);
    assert_eq!(response.longitude, 11.6);
    assert_eq!(response.timezone, "Europe/Berlin");
    assert!(response.current.is_some());
    assert!(response.hourly.is_some());
    assert!(response.daily.is_some());
}

#[tokio::test]
async fn test_search_location_success() {
    // Start mock server
    let mock_server = MockServer::start().await;

    // Load fixture
    let fixture = include_str!("fixtures/geocode_response.json");

    // Setup mock endpoint
    Mock::given(matchers::path("/v1/search"))
        .respond_with(ResponseTemplate::new(200).set_body_string(fixture))
        .mount(&mock_server)
        .await;

    // Create client
    let http_client = Arc::new(
        reqwest::Client::builder()
            .timeout(std::time::Duration::from_secs(30))
            .build()
            .unwrap(),
    );

    let client = OpenMeteoClient::new(http_client);

    // Test geocoding request
    let req = open_meteo_mcp::types::location::GeocodeRequest::new("Munich".to_string());

    let result = client.search_location(&req).await;

    assert!(result.is_ok());
    let response = result.unwrap();
    assert!(!response.results.is_empty());

    // Check first result (Munich, Germany)
    let munich = &response.results[0];
    assert_eq!(munich.name, "Munich");
    assert_eq!(munich.country_code, Some("DE".to_string()));
    assert!((munich.latitude - 48.13743).abs() < 0.01);
}

#[tokio::test]
async fn test_invalid_coordinates_rejected() {
    let http_client = Arc::new(
        reqwest::Client::builder()
            .timeout(std::time::Duration::from_secs(30))
            .build()
            .unwrap(),
    );

    let client = OpenMeteoClient::new(http_client);

    // Try with invalid latitude
    let req_invalid = open_meteo_mcp::types::weather::WeatherRequest {
        latitude: 999.0,  // Out of range
        longitude: 11.6,
        ..Default::default()
    };

    let result = client.get_weather(&req_invalid).await;
    assert!(result.is_err());
}

#[tokio::test]
async fn test_empty_location_name_rejected() {
    let http_client = Arc::new(
        reqwest::Client::builder()
            .timeout(std::time::Duration::from_secs(30))
            .build()
            .unwrap(),
    );

    let client = OpenMeteoClient::new(http_client);

    // Try with empty location name
    let req_invalid = open_meteo_mcp::types::location::GeocodeRequest {
        name: String::new(),
        count: Some(10),
        language: None,
        format: None,
    };

    let result = client.search_location(&req_invalid).await;
    assert!(result.is_err());
}

#[tokio::test]
async fn test_http_client_400_error() {
    // Start mock server
    let mock_server = MockServer::start().await;

    // Setup mock endpoint returning error
    Mock::given(matchers::path("/v1/forecast"))
        .respond_with(ResponseTemplate::new(400))
        .mount(&mock_server)
        .await;

    // Create client
    let http_client = Arc::new(
        reqwest::Client::builder()
            .timeout(std::time::Duration::from_secs(30))
            .build()
            .unwrap(),
    );

    let client = OpenMeteoClient::new(http_client);

    let req = open_meteo_mcp::types::weather::WeatherRequest {
        latitude: 48.1,
        longitude: 11.6,
        ..Default::default()
    };

    let result = client.get_weather(&req).await;
    assert!(result.is_err());
}
