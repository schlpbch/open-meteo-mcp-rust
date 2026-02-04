//! Tool handler tests for Ping
//! Phase 4: Basic connectivity and health check test

use open_meteo_mcp::OpenMeteoService;

#[tokio::test]
async fn test_ping_success() {
    let config = open_meteo_mcp::Config::default();
    let service = OpenMeteoService::new(config).expect("Valid service");

    let result = service.ping().await;

    assert!(result.is_ok(), "Ping should succeed");
    let call_result = result.unwrap();
    assert!(!call_result.is_error, "Ping response should not be an error");
}
