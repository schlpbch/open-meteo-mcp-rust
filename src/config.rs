//! Configuration management

use serde::Deserialize;

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

    /// Validate configuration settings
    pub fn validate(&self) -> Result<(), String> {
        // Validate port
        if self.port == 0 {
            return Err("Port must be between 1 and 65535".to_string());
        }

        // Validate transport
        if !["stdio", "sse"].contains(&self.transport.as_str()) {
            return Err(format!(
                "Invalid transport '{}' (must be 'stdio' or 'sse')",
                self.transport
            ));
        }

        // Validate timeout
        if self.timeout_secs == 0 || self.timeout_secs > 300 {
            return Err(format!(
                "Invalid timeout {} seconds (must be 1-300)",
                self.timeout_secs
            ));
        }

        // Validate host is not empty
        if self.host.is_empty() {
            return Err("Host cannot be empty".to_string());
        }

        // Validate API base URL is not empty
        if self.api_base.is_empty() {
            return Err("API base URL cannot be empty".to_string());
        }

        Ok(())
    }

    /// Check if STDIO transport is configured
    pub fn is_stdio(&self) -> bool {
        self.transport == "stdio"
    }

    /// Check if SSE transport is configured
    pub fn is_sse(&self) -> bool {
        self.transport == "sse"
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
