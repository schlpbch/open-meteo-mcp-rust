//! Shared test helpers for integration tests that exercise the tool-handler
//! layer against a mocked Open-Meteo API (no live network calls).

use open_meteo_mcp::client::BaseUrls;
use open_meteo_mcp::{Config, OpenMeteoClient, OpenMeteoService};
use std::sync::Arc;
use wiremock::MockServer;

/// Builds an `OpenMeteoService` with every API base URL pointed at
/// `mock_server`, so `latitude`/`longitude`-only tests never need to know
/// which specific endpoint (weather, geocoding, marine, ...) they exercise.
pub fn mock_service(mock_server: &MockServer) -> OpenMeteoService {
    let http_client = Arc::new(reqwest::Client::new());
    let uri = mock_server.uri();
    let base_urls = BaseUrls {
        weather: uri.clone(),
        geocoding: uri.clone(),
        air_quality: uri.clone(),
        marine: uri.clone(),
        archive: uri,
    };
    let api_client = OpenMeteoClient::with_base_urls(http_client, base_urls);
    OpenMeteoService::with_api_client(Config::default(), api_client)
}
