//! Geocoding API client

use crate::types::location::{GeocodeRequest, GeocodeResponse};
use crate::{Error, Result};
use super::{OpenMeteoClient, RetryConfig, with_retry};

impl OpenMeteoClient {
    /// Search for locations by name (geocoding)
    ///
    /// # Arguments
    /// * `req` - Geocode request with location name and optional parameters
    ///
    /// # Returns
    /// List of matching locations with coordinates and metadata
    pub async fn search_location(&self, req: &GeocodeRequest) -> Result<GeocodeResponse> {
        if req.name.is_empty() {
            return Err(Error::InvalidParameter(
                "Location name cannot be empty".to_string(),
            ));
        }

        let url = format!("{}/search", self.base_urls.geocoding);

        let response = self.http_client
            .get(&url)
            .query(req)
            .send()
            .await?;

        OpenMeteoClient::validate_response_status(response.status())?;

        let body = response.json::<GeocodeResponse>().await?;

        tracing::debug!(
            location = &req.name,
            results = body.results.len(),
            "Location search completed"
        );

        Ok(body)
    }

    /// Search for locations with automatic retry logic
    pub async fn search_location_with_retry(
        &self,
        req: &GeocodeRequest,
        retry_config: RetryConfig,
    ) -> Result<GeocodeResponse> {
        let req = req.clone();
        with_retry(
            || {
                let client = self.clone();
                let req = req.clone();
                async move { client.search_location(&req).await }
            },
            retry_config,
        )
        .await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_geocode_request_validation() {
        let req_invalid = GeocodeRequest {
            name: String::new(),
            count: Some(10),
            language: None,
            format: None,
        };

        assert!(req_invalid.name.is_empty());
    }

    #[test]
    fn test_geocode_response_deserialization() {
        let json = r#"{
            "results": [
                {
                    "id": 2867714,
                    "name": "Munich",
                    "latitude": 48.13743,
                    "longitude": 11.5754,
                    "elevation": 518.0,
                    "feature_code": "CITY",
                    "country_code": "DE",
                    "country": "Germany",
                    "admin1": "Bavaria",
                    "timezone": "Europe/Berlin",
                    "population": 1484226
                }
            ],
            "generationtime_ms": 1.23
        }"#;

        let response: GeocodeResponse = serde_json::from_str(json).expect("Valid geocode JSON");
        assert_eq!(response.results.len(), 1);

        let loc = &response.results[0];
        assert_eq!(loc.name, "Munich");
        assert_eq!(loc.latitude, 48.13743);
        assert_eq!(loc.country_code, Some("DE".to_string()));
    }
}
