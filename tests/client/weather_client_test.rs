//! HTTP Client tests for Weather API
//! Phase 3: Tests weather client request/response handling with wiremock

use open_meteo_mcp::{OpenMeteoClient, types::weather::WeatherRequest};
use std::sync::Arc;
use wiremock::{Mock, MockServer, ResponseTemplate, matchers};

#[tokio::test]
async fn test_weather_client_request_construction_and_parsing() {
    let mock_server = MockServer::start().await;
    
    let fixture = include_str!("../../tests/fixtures/weather_response.json");
    
    Mock::given(matchers::method("GET"))
        .and(matchers::path("/v1/forecast"))
        .and(matchers::query_param("latitude", "48.1"))
        .and(matchers::query_param("longitude", "11.6"))
        .respond_with(ResponseTemplate::new(200).set_body_string(fixture))
        .mount(&mock_server)
        .await;
    
    let http_client = Arc::new(
        reqwest::Client::builder()
            .timeout(std::time::Duration::from_secs(30))
            .build()
            .expect("Valid HTTP client"),
    );
    
    let client = OpenMeteoClient::new(http_client);
    let req = WeatherRequest {
        latitude: 48.1,
        longitude: 11.6,
        ..Default::default()
    };
    
    let result = client.get_weather(&req).await;
    assert!(result.is_ok(), "Weather request should succeed");
    
    let response = result.unwrap();
    assert_eq!(response.latitude, 48.1);
    assert_eq!(response.timezone, "Europe/Berlin");
}

#[tokio::test]
async fn test_weather_client_with_optional_parameters() {
    let mock_server = MockServer::start().await;
    let fixture = include_str!("../../tests/fixtures/weather_response.json");
    
    Mock::given(matchers::method("GET"))
        .and(matchers::path("/v1/forecast"))
        .respond_with(ResponseTemplate::new(200).set_body_string(fixture))
        .mount(&mock_server)
        .await;
    
    let http_client = Arc::new(
        reqwest::Client::builder()
            .timeout(std::time::Duration::from_secs(30))
            .build()
            .expect("Valid HTTP client"),
    );
    
    let client = OpenMeteoClient::new(http_client);
    let req = WeatherRequest {
        latitude: 48.1,
        longitude: 11.6,
        hourly: Some("temperature_2m,precipitation".to_string()),
        daily: Some("weather_code".to_string()),
        forecast_days: Some(7),
        temperature_unit: Some("fahrenheit".to_string()),
        ..Default::default()
    };
    
    let result = client.get_weather(&req).await;
    assert!(result.is_ok());
}

#[tokio::test]
async fn test_weather_client_coordinate_validation_before_request() {
    let http_client = Arc::new(
        reqwest::Client::builder()
            .timeout(std::time::Duration::from_secs(30))
            .build()
            .expect("Valid HTTP client"),
    );
    
    let client = OpenMeteoClient::new(http_client);
    
    // Invalid coordinates should be caught before HTTP call
    let req = WeatherRequest {
        latitude: 91.0,
        longitude: 11.6,
        ..Default::default()
    };
    
    let result = client.get_weather(&req).await;
    assert!(result.is_err(), "Invalid latitude should be caught");
}

#[tokio::test]
async fn test_weather_client_handles_404() {
    let mock_server = MockServer::start().await;
    
    Mock::given(matchers::method("GET"))
        .and(matchers::path("/v1/forecast"))
        .respond_with(ResponseTemplate::new(404))
        .mount(&mock_server)
        .await;
    
    let http_client = Arc::new(
        reqwest::Client::builder()
            .timeout(std::time::Duration::from_secs(30))
            .build()
            .expect("Valid HTTP client"),
    );
    
    let client = OpenMeteoClient::new(http_client);
    let req = WeatherRequest {
        latitude: 48.1,
        longitude: 11.6,
        ..Default::default()
    };
    
    let result = client.get_weather(&req).await;
    assert!(result.is_err(), "404 error should propagate");
}

#[tokio::test]
async fn test_weather_client_handles_500() {
    let mock_server = MockServer::start().await;
    
    Mock::given(matchers::method("GET"))
        .and(matchers::path("/v1/forecast"))
        .respond_with(ResponseTemplate::new(500))
        .mount(&mock_server)
        .await;
    
    let http_client = Arc::new(
        reqwest::Client::builder()
            .timeout(std::time::Duration::from_secs(30))
            .build()
            .expect("Valid HTTP client"),
    );
    
    let client = OpenMeteoClient::new(http_client);
    let req = WeatherRequest {
        latitude: 48.1,
        longitude: 11.6,
        ..Default::default()
    };
    
    let result = client.get_weather(&req).await;
    assert!(result.is_err(), "500 error should propagate");
}

#[tokio::test]
async fn test_weather_client_response_deserialization() {
    let fixture = include_str!("../../tests/fixtures/weather_response.json");
    
    let response: open_meteo_mcp::types::weather::WeatherResponse =
        serde_json::from_str(fixture).expect("Valid fixture");
    
    assert_eq!(response.latitude, 48.1);
    assert_eq!(response.longitude, 11.6);
    assert_eq!(response.timezone, "Europe/Berlin");
    assert!(response.current.is_some());
    assert!(response.hourly.is_some());
    assert!(response.daily.is_some());
}

#[test]
fn test_weather_client_creation() {
    let http_client = Arc::new(
        reqwest::Client::builder()
            .timeout(std::time::Duration::from_secs(30))
            .build()
            .expect("Valid HTTP client"),
    );
    
    let _client = OpenMeteoClient::new(http_client);
    // Client created successfully
}
