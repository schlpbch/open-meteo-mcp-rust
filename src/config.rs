//! Configuration management

use serde::Deserialize;
use std::env;

/// Application configuration loaded from environment variables
#[derive(Debug, Clone, Deserialize)]
pub struct Config {
    /// HTTP host to bind to (default: 0.0.0.0)
    #[serde(default = "default_host")]
    pub host: String,

    /// HTTP port to bind to (default: 8888)
    #[serde(default = "default_port")]
    pub port: u16,

    /// Base URL for Open-Meteo API (default: https://api.open-meteo.com)
    #[serde(default = "default_api_base")]
    pub api_base: String,

    /// HTTP request timeout in seconds (default: 30)
    #[serde(default = "default_timeout")]
    pub timeout_secs: u64,

    /// Log level (default: info)
    #[serde(default)]
    pub log_level: Option<String>,

    /// Transport mode: "stdio" or "sse" (default: stdio)
    #[serde(default = "default_transport")]
    pub transport: String,
}

fn default_host() -> String {
    "0.0.0.0".to_string()
}

fn default_port() -> u16 {
    8888
}

fn default_api_base() -> String {
    "https://api.open-meteo.com".to_string()
}

fn default_timeout() -> u64 {
    30
}

fn default_transport() -> String {
    "stdio".to_string()
}

impl Config {
    /// Load configuration from environment variables
    ///
    /// Supported env vars:
    /// - HOST: HTTP host (default: 0.0.0.0)
    /// - PORT: HTTP port (default: 8888)
    /// - API_BASE_URL: Base URL for Open-Meteo API (default: https://api.open-meteo.com)
    /// - TIMEOUT_SECS: Request timeout in seconds (default: 30)
    /// - LOG_LEVEL: Log level (default: info)
    /// - TRANSPORT: Transport mode "stdio" or "sse" (default: stdio)
    pub fn from_env() -> Result<Self, envy::Error> {
        envy::from_env()
    }

    /// Load configuration from environment variables, with defaults
    pub fn from_env_or_default() -> Self {
        Self::from_env().unwrap_or_default()
    }
}

impl Default for Config {
    fn default() -> Self {
        Self {
            host: default_host(),
            port: default_port(),
            api_base: default_api_base(),
            timeout_secs: default_timeout(),
            log_level: Some("info".to_string()),
            transport: default_transport(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_config_defaults() {
        let cfg = Config::default();
        assert_eq!(cfg.host, "0.0.0.0");
        assert_eq!(cfg.port, 8888);
        assert_eq!(cfg.api_base, "https://api.open-meteo.com");
        assert_eq!(cfg.timeout_secs, 30);
        assert_eq!(cfg.transport, "stdio");
    }
}
