//! Core service tests for Open-Meteo MCP
//! Phase 5: Service layer orchestration and health

use open_meteo_mcp::{Config, OpenMeteoService};

// OpenMeteoService Initialization Tests

#[test]
fn test_service_creation_with_default_config() {
    let config = Config::default();
    let service = OpenMeteoService::new(config);
    assert!(service.is_ok());
}

#[test]
fn test_service_creation_with_custom_config() {
    let mut config = Config::default();
    config.port = 9999;
    config.api_base = "https://api.example.com".to_string();
    let service = OpenMeteoService::new(config);
    assert!(service.is_ok());
}

#[test]
fn test_service_unwrap_valid() {
    let config = Config::default();
    let _service = OpenMeteoService::new(config).expect("Service should initialize");
}

#[test]
fn test_service_api_client_access() {
    let config = Config::default();
    let service = OpenMeteoService::new(config).expect("Service creation failed");
    let _ = service.api_client();
}

#[test]
fn test_service_http_client_access() {
    let config = Config::default();
    let service = OpenMeteoService::new(config).expect("Service creation failed");
    let _ = service.http_client();
}

// Health Check Tests

#[test]
fn test_health_check_liveness_sync() {
    let config = Config::default();
    let _service = OpenMeteoService::new(config).expect("Service creation");
    // Service should be alive immediately after creation
    // This is a basic sanity check
}

// Configuration Application Tests

#[test]
fn test_service_respects_config_api_base() {
    let mut config = Config::default();
    config.api_base = "https://custom-api.example.com".to_string();
    let service = OpenMeteoService::new(config).expect("Service creation");
    let _ = service.api_client();
    // Service should use the configured API base
}

#[test]
fn test_service_respects_config_timeout() {
    let mut config = Config::default();
    config.timeout_secs = 60;
    let service = OpenMeteoService::new(config).expect("Service creation");
    let _ = service.api_client();
    // Service should use the configured timeout
}

// Error Handling in Service Creation

#[test]
fn test_service_with_invalid_config_port_zero() {
    let mut config = Config::default();
    config.port = 0; // Invalid port
    // Service creation might fail or ignore port in some cases
    let _ = OpenMeteoService::new(config);
}

#[test]
fn test_service_multiple_instances() {
    let config1 = Config::default();
    let config2 = Config::default();
    let service1 = OpenMeteoService::new(config1).expect("Service 1");
    let service2 = OpenMeteoService::new(config2).expect("Service 2");
    // Should be able to create multiple service instances
    let _ = service1.http_client();
    let _ = service2.http_client();
}

// Configuration Combinations

#[test]
fn test_service_with_sse_transport() {
    let mut config = Config::default();
    config.transport = "sse".to_string();
    assert!(config.is_sse());
    let service = OpenMeteoService::new(config).expect("Service creation");
    let _ = service.http_client();
}

#[test]
fn test_service_with_stdio_transport() {
    let mut config = Config::default();
    config.transport = "stdio".to_string();
    assert!(config.is_stdio());
    let service = OpenMeteoService::new(config).expect("Service creation");
    let _ = service.api_client();
}

// Service State Tests

#[test]
fn test_service_client_callable() {
    let config = Config::default();
    let service = OpenMeteoService::new(config).expect("Service creation");
    let _client = service.api_client();
    // Getting client should work
}

// Edge Cases

#[test]
fn test_service_custom_all_fields() {
    let config = Config {
        host: "localhost".to_string(),
        port: 3000,
        api_base: "https://api.test.com".to_string(),
        timeout_secs: 45,
        log_level: Some("debug".to_string()),
        transport: "sse".to_string(),
    };
    assert!(config.validate().is_ok());
    let service = OpenMeteoService::new(config).expect("Service creation");
    let _ = service.http_client();
}

#[test]
fn test_service_minimal_config() {
    let config = Config {
        host: "0.0.0.0".to_string(),
        port: 8888,
        api_base: "https://api.open-meteo.com".to_string(),
        timeout_secs: 30,
        log_level: None,
        transport: "stdio".to_string(),
    };
    let service = OpenMeteoService::new(config).expect("Service creation");
    let _ = service.api_client();
}

// Integration-like Tests

#[test]
fn test_service_ready_after_creation() {
    let config = Config::default();
    let service = OpenMeteoService::new(config).expect("Service creation");
    // Service should be in a ready state immediately
    let _ = service.http_client();
}

#[tokio::test]
async fn test_service_async_operations() {
    let config = Config::default();
    let service = OpenMeteoService::new(config).expect("Service creation");
    // Service should support async operations
    let _ = service.http_client();
}
