//! HTTP client for Open-Meteo API

pub mod weather;
pub mod geocoding;
pub mod air_quality;
pub mod marine;
pub mod archive;

use crate::{Error, Result};
use std::sync::Arc;
use std::time::Duration;

/// Main HTTP client for Open-Meteo API
///
/// This client manages a connection pool and provides methods for querying
/// all Open-Meteo API endpoints. It's designed to be cloned and shared
/// across async tasks via Arc.
pub struct OpenMeteoClient {
    http_client: Arc<reqwest::Client>,
    base_urls: BaseUrls,
}

#[derive(Clone, Debug)]
pub(crate) struct BaseUrls {
    pub weather: String,
    pub geocoding: String,
    pub air_quality: String,
    pub marine: String,
    pub archive: String,
}

impl Default for BaseUrls {
    fn default() -> Self {
        Self {
            weather: "https://api.open-meteo.com/v1".to_string(),
            geocoding: "https://geocoding-api.open-meteo.com/v1".to_string(),
            air_quality: "https://air-quality-api.open-meteo.com/v1".to_string(),
            marine: "https://marine-api.open-meteo.com/v1".to_string(),
            archive: "https://archive-api.open-meteo.com/v1".to_string(),
        }
    }
}

impl OpenMeteoClient {
    /// Create a new OpenMeteoClient with default configuration
    pub fn new(http_client: Arc<reqwest::Client>) -> Self {
        Self {
            http_client,
            base_urls: BaseUrls::default(),
        }
    }

    /// Create a new OpenMeteoClient with custom base URLs (for testing)
    #[allow(dead_code)]
    pub(crate) fn with_base_urls(http_client: Arc<reqwest::Client>, base_urls: BaseUrls) -> Self {
        Self {
            http_client,
            base_urls,
        }
    }

    /// Get a reference to the underlying HTTP client
    pub fn http_client(&self) -> Arc<reqwest::Client> {
        self.http_client.clone()
    }

    /// Validate HTTP response status
    fn validate_response_status(status: reqwest::StatusCode) -> Result<()> {
        if status.is_success() {
            Ok(())
        } else if status.is_client_error() {
            Err(Error::ApiError(format!(
                "Client error: {} {}",
                status.as_u16(),
                status.canonical_reason().unwrap_or("Unknown")
            )))
        } else {
            Err(Error::ApiError(format!(
                "Server error: {} {}",
                status.as_u16(),
                status.canonical_reason().unwrap_or("Unknown")
            )))
        }
    }
}

/// Retry configuration for resilience
#[derive(Debug, Clone)]
pub struct RetryConfig {
    pub max_retries: u32,
    pub initial_backoff_ms: u64,
    pub max_backoff_ms: u64,
}

impl Default for RetryConfig {
    fn default() -> Self {
        Self {
            max_retries: 3,
            initial_backoff_ms: 100,
            max_backoff_ms: 5000,
        }
    }
}

/// Execute a function with exponential backoff retry logic
pub async fn with_retry<F, T>(
    mut f: impl FnMut() -> F,
    config: RetryConfig,
) -> Result<T>
where
    F: std::future::Future<Output = Result<T>>,
{
    let mut retries = 0;
    let mut backoff_ms = config.initial_backoff_ms;

    loop {
        match f().await {
            Ok(val) => return Ok(val),
            Err(e) if retries < config.max_retries => {
                retries += 1;
                tracing::warn!(
                    retry = retries,
                    backoff_ms = backoff_ms,
                    error = ?e,
                    "API request failed, retrying"
                );
                tokio::time::sleep(Duration::from_millis(backoff_ms)).await;
                backoff_ms = (backoff_ms * 2).min(config.max_backoff_ms);
            }
            Err(e) => {
                tracing::error!(
                    total_retries = retries,
                    error = ?e,
                    "API request failed after all retries"
                );
                return Err(e);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_client_creation() {
        let http_client = Arc::new(
            reqwest::Client::builder()
                .timeout(Duration::from_secs(30))
                .build()
                .expect("Valid HTTP client"),
        );
        let client = OpenMeteoClient::new(http_client);
        // Verify client was created (Arc is always Some)
        let _ = client.http_client();
    }

    #[test]
    fn test_retry_config_default() {
        let config = RetryConfig::default();
        assert_eq!(config.max_retries, 3);
        assert_eq!(config.initial_backoff_ms, 100);
    }

    #[test]
    fn test_validate_response_status() {
        assert!(OpenMeteoClient::validate_response_status(reqwest::StatusCode::OK).is_ok());
        assert!(OpenMeteoClient::validate_response_status(reqwest::StatusCode::NOT_FOUND).is_err());
        assert!(
            OpenMeteoClient::validate_response_status(reqwest::StatusCode::INTERNAL_SERVER_ERROR)
                .is_err()
        );
    }
}
