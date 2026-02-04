//! Core MCP service implementation

use crate::config::Config;
use crate::{Error, Result};
use std::sync::Arc;

/// Main Open-Meteo MCP service
///
/// This service is the entry point for all MCP tool and resource operations.
/// It manages the HTTP client connection pool and coordinates requests to
/// the Open-Meteo API.
pub struct OpenMeteoService {
    client: Arc<reqwest::Client>,
    config: Config,
}

impl OpenMeteoService {
    /// Create a new OpenMeteoService with the given configuration
    pub fn new(config: Config) -> Result<Self> {
        let client = reqwest::Client::builder()
            .timeout(std::time::Duration::from_secs(config.timeout_secs))
            .connect_timeout(std::time::Duration::from_secs(10))
            .gzip(true)
            .user_agent(concat!("open-meteo-mcp/", env!("CARGO_PKG_VERSION")))
            .build()?;

        Ok(Self {
            client: Arc::new(client),
            config,
        })
    }

    /// Get a reference to the HTTP client
    pub fn client(&self) -> Arc<reqwest::Client> {
        self.client.clone()
    }

    /// Get a reference to the configuration
    pub fn config(&self) -> &Config {
        &self.config
    }

    /// Get the base URL for the Open-Meteo API
    pub fn api_base(&self) -> &str {
        &self.config.api_base
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_service_creation() {
        let config = Config::default();
        let service = OpenMeteoService::new(config).unwrap();
        assert_eq!(service.api_base(), "https://api.open-meteo.com");
    }

    #[test]
    fn test_service_timeout_config() {
        let mut config = Config::default();
        config.timeout_secs = 60;
        let service = OpenMeteoService::new(config).unwrap();
        assert_eq!(service.config().timeout_secs, 60);
    }
}
