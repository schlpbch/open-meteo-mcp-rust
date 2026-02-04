//! Core MCP service implementation

use crate::client::OpenMeteoClient;
use crate::config::Config;
use crate::Result;
use std::sync::Arc;

/// Main Open-Meteo MCP service
///
/// This service is the entry point for all MCP tool and resource operations.
/// It manages the HTTP client connection pool and coordinates requests to
/// the Open-Meteo API via the OpenMeteoClient.
pub struct OpenMeteoService {
    http_client: Arc<reqwest::Client>,
    api_client: OpenMeteoClient,
    config: Config,
}

impl OpenMeteoService {
    /// Create a new OpenMeteoService with the given configuration
    pub fn new(config: Config) -> Result<Self> {
        let http_client = reqwest::Client::builder()
            .timeout(std::time::Duration::from_secs(config.timeout_secs))
            .connect_timeout(std::time::Duration::from_secs(10))
            .gzip(true)
            .user_agent(concat!("open-meteo-mcp/", env!("CARGO_PKG_VERSION")))
            .build()?;

        let http_client = Arc::new(http_client);
        let api_client = OpenMeteoClient::new(http_client.clone());

        Ok(Self {
            http_client,
            api_client,
            config,
        })
    }

    /// Get a reference to the HTTP client
    pub fn http_client(&self) -> Arc<reqwest::Client> {
        self.http_client.clone()
    }

    /// Get a reference to the API client
    pub fn api_client(&self) -> OpenMeteoClient {
        self.api_client.clone()
    }

    /// Get a reference to the configuration
    pub fn config(&self) -> &Config {
        &self.config
    }

    /// Get the base URL for the Open-Meteo API
    pub fn api_base(&self) -> &str {
        &self.config.api_base
    }

    /// Check if the service is ready (API is reachable)
    ///
    /// This performs a quick ping to verify Open-Meteo API connectivity
    /// and is used for readiness probes in Kubernetes/Docker.
    pub async fn is_ready(&self) -> bool {
        self.ping().await.is_ok()
    }

    /// Get service version
    pub fn version(&self) -> &'static str {
        crate::VERSION
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_service_creation() {
        let config = Config::default();
        let service = OpenMeteoService::new(config).expect("Valid service creation");
        assert_eq!(service.api_base(), "https://api.open-meteo.com");
    }

    #[test]
    fn test_service_timeout_config() {
        let mut config = Config::default();
        config.timeout_secs = 60;
        let service = OpenMeteoService::new(config).expect("Valid service with timeout");
        assert_eq!(service.config().timeout_secs, 60);
    }
}
