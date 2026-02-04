//! Configuration management tests for Open-Meteo MCP
//! Phase 5: Service layer configuration

use open_meteo_mcp::Config;

// Default Configuration Tests

#[test]
fn test_config_default_host() {
    let cfg = Config::default();
    assert_eq!(cfg.host, "0.0.0.0");
}

#[test]
fn test_config_default_port() {
    let cfg = Config::default();
    assert_eq!(cfg.port, 8888);
}

#[test]
fn test_config_default_api_base() {
    let cfg = Config::default();
    assert_eq!(cfg.api_base, "https://api.open-meteo.com");
}

#[test]
fn test_config_default_timeout() {
    let cfg = Config::default();
    assert_eq!(cfg.timeout_secs, 30);
}

#[test]
fn test_config_default_transport() {
    let cfg = Config::default();
    assert_eq!(cfg.transport, "stdio");
}

#[test]
fn test_config_default_log_level() {
    let cfg = Config::default();
    assert_eq!(cfg.log_level, Some("info".to_string()));
}

#[test]
fn test_config_all_defaults() {
    let cfg = Config::default();
    assert_eq!(cfg.host, "0.0.0.0");
    assert_eq!(cfg.port, 8888);
    assert_eq!(cfg.api_base, "https://api.open-meteo.com");
    assert_eq!(cfg.timeout_secs, 30);
    assert_eq!(cfg.transport, "stdio");
    assert!(cfg.log_level.is_some());
}

// Validation Tests - Valid Cases

#[test]
fn test_config_validation_valid_defaults() {
    let cfg = Config::default();
    assert!(cfg.validate().is_ok());
}

#[test]
fn test_config_validation_valid_port_low() {
    let mut cfg = Config::default();
    cfg.port = 1;
    assert!(cfg.validate().is_ok());
}

#[test]
fn test_config_validation_valid_port_high() {
    let mut cfg = Config::default();
    cfg.port = 65535;
    assert!(cfg.validate().is_ok());
}

#[test]
fn test_config_validation_valid_timeout_low() {
    let mut cfg = Config::default();
    cfg.timeout_secs = 1;
    assert!(cfg.validate().is_ok());
}

#[test]
fn test_config_validation_valid_timeout_high() {
    let mut cfg = Config::default();
    cfg.timeout_secs = 300;
    assert!(cfg.validate().is_ok());
}

#[test]
fn test_config_validation_valid_transport_stdio() {
    let mut cfg = Config::default();
    cfg.transport = "stdio".to_string();
    assert!(cfg.validate().is_ok());
}

#[test]
fn test_config_validation_valid_transport_sse() {
    let mut cfg = Config::default();
    cfg.transport = "sse".to_string();
    assert!(cfg.validate().is_ok());
}

// Validation Tests - Invalid Cases

#[test]
fn test_config_validation_invalid_port_zero() {
    let mut cfg = Config::default();
    cfg.port = 0;
    let result = cfg.validate();
    assert!(result.is_err());
    assert!(result.unwrap_err().contains("Port"));
}

#[test]
fn test_config_validation_invalid_timeout_zero() {
    let mut cfg = Config::default();
    cfg.timeout_secs = 0;
    let result = cfg.validate();
    assert!(result.is_err());
    assert!(result.unwrap_err().contains("timeout"));
}

#[test]
fn test_config_validation_invalid_timeout_too_high() {
    let mut cfg = Config::default();
    cfg.timeout_secs = 301;
    let result = cfg.validate();
    assert!(result.is_err());
    assert!(result.unwrap_err().contains("timeout"));
}

#[test]
fn test_config_validation_invalid_transport() {
    let mut cfg = Config::default();
    cfg.transport = "invalid".to_string();
    let result = cfg.validate();
    assert!(result.is_err());
    assert!(result.unwrap_err().contains("transport"));
}

#[test]
fn test_config_validation_empty_host() {
    let mut cfg = Config::default();
    cfg.host = String::new();
    let result = cfg.validate();
    assert!(result.is_err());
    assert!(result.unwrap_err().contains("Host"));
}

#[test]
fn test_config_validation_empty_api_base() {
    let mut cfg = Config::default();
    cfg.api_base = String::new();
    let result = cfg.validate();
    assert!(result.is_err());
    assert!(result.unwrap_err().contains("API base"));
}

// Transport Check Tests

#[test]
fn test_config_is_stdio_true() {
    let mut cfg = Config::default();
    cfg.transport = "stdio".to_string();
    assert!(cfg.is_stdio());
}

#[test]
fn test_config_is_stdio_false() {
    let mut cfg = Config::default();
    cfg.transport = "sse".to_string();
    assert!(!cfg.is_stdio());
}

#[test]
fn test_config_is_sse_true() {
    let mut cfg = Config::default();
    cfg.transport = "sse".to_string();
    assert!(cfg.is_sse());
}

#[test]
fn test_config_is_sse_false() {
    let mut cfg = Config::default();
    cfg.transport = "stdio".to_string();
    assert!(!cfg.is_sse());
}

// Configuration Clone Tests

#[test]
fn test_config_cloneable() {
    let cfg1 = Config::default();
    let cfg2 = cfg1.clone();
    assert_eq!(cfg1.host, cfg2.host);
    assert_eq!(cfg1.port, cfg2.port);
    assert_eq!(cfg1.api_base, cfg2.api_base);
}

// Custom Configuration Tests

#[test]
fn test_config_custom_port() {
    let mut cfg = Config::default();
    cfg.port = 9999;
    assert_eq!(cfg.port, 9999);
    assert!(cfg.validate().is_ok());
}

#[test]
fn test_config_custom_api_base() {
    let mut cfg = Config::default();
    cfg.api_base = "https://custom-api.example.com".to_string();
    assert_eq!(cfg.api_base, "https://custom-api.example.com");
    assert!(cfg.validate().is_ok());
}

#[test]
fn test_config_custom_timeout() {
    let mut cfg = Config::default();
    cfg.timeout_secs = 60;
    assert_eq!(cfg.timeout_secs, 60);
    assert!(cfg.validate().is_ok());
}

#[test]
fn test_config_custom_host() {
    let mut cfg = Config::default();
    cfg.host = "127.0.0.1".to_string();
    assert_eq!(cfg.host, "127.0.0.1");
    assert!(cfg.validate().is_ok());
}

#[test]
fn test_config_from_env_or_default() {
    // This uses default since we're not setting env vars in test
    let cfg = Config::from_env_or_default();
    assert_eq!(cfg.host, "0.0.0.0");
    assert_eq!(cfg.port, 8888);
    assert_eq!(cfg.timeout_secs, 30);
}

// Edge Cases

#[test]
fn test_config_validation_timeout_boundary_low() {
    let mut cfg = Config::default();
    cfg.timeout_secs = 1;
    assert!(cfg.validate().is_ok());
}

#[test]
fn test_config_validation_timeout_boundary_high() {
    let mut cfg = Config::default();
    cfg.timeout_secs = 300;
    assert!(cfg.validate().is_ok());
}

#[test]
fn test_config_validation_all_valid_fields() {
    let cfg = Config {
        host: "localhost".to_string(),
        port: 3000,
        api_base: "https://api.example.com".to_string(),
        timeout_secs: 45,
        log_level: Some("debug".to_string()),
        transport: "sse".to_string(),
    };
    assert!(cfg.validate().is_ok());
}

#[test]
fn test_config_transport_case_sensitive() {
    let mut cfg = Config::default();
    cfg.transport = "STDIO".to_string();
    // Should fail because case-sensitive
    assert!(cfg.validate().is_err());
}

#[test]
fn test_config_with_none_log_level() {
    let cfg = Config {
        host: "0.0.0.0".to_string(),
        port: 8888,
        api_base: "https://api.open-meteo.com".to_string(),
        timeout_secs: 30,
        log_level: None,
        transport: "stdio".to_string(),
    };
    assert!(cfg.validate().is_ok());
}
