//! Marine API client (waves, swells)

use crate::types::marine::{MarineRequest, MarineResponse};
use crate::{Error, Result};
use super::{OpenMeteoClient, RetryConfig, with_retry};

impl OpenMeteoClient {
    /// Get marine conditions (wave and swell data)
    ///
    /// # Arguments
    /// * `req` - Marine request with latitude, longitude, and parameters
    ///
    /// # Returns
    /// Wave and swell data for maritime conditions
    pub async fn get_marine_conditions(&self, req: &MarineRequest) -> Result<MarineResponse> {
        // Validate coordinates
        crate::error::validate_coordinates(req.latitude, req.longitude)?;

        let url = format!("{}/marine", self.base_urls.marine);

        let response = self.http_client
            .get(&url)
            .query(req)
            .send()
            .await?;

        OpenMeteoClient::validate_response_status(response.status())?;

        let body = response.json::<MarineResponse>().await?;

        tracing::debug!(
            latitude = req.latitude,
            longitude = req.longitude,
            timezone = &body.timezone,
            "Marine conditions retrieved successfully"
        );

        Ok(body)
    }

    /// Get marine conditions with automatic retry logic
    pub async fn get_marine_conditions_with_retry(
        &self,
        req: &MarineRequest,
        retry_config: RetryConfig,
    ) -> Result<MarineResponse> {
        let req = req.clone();
        with_retry(
            || {
                let client = self.clone();
                let req = req.clone();
                async move { client.get_marine_conditions(&req).await }
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
    fn test_marine_response_deserialization() {
        let json = r#"{
            "latitude": 48.1,
            "longitude": 11.6,
            "timezone": "Europe/Berlin",
            "hourly": {
                "time": ["2026-02-04T00:00", "2026-02-04T01:00"],
                "wave_height": [1.5, 1.6],
                "wave_direction": [180, 185],
                "wave_period": [8.5, 8.7],
                "swell_wave_height": [0.8, 0.9],
                "wind_speed_10m": [5.2, 5.5],
                "weather_code": [0, 1]
            }
        }"#;

        let response: MarineResponse = serde_json::from_str(json).expect("Valid JSON");
        assert_eq!(response.latitude, 48.1);
        assert!(response.hourly.is_some());

        let hourly = response.hourly.expect("Expected field exists");
        assert_eq!(hourly.time.len(), 2);
        assert_eq!(hourly.wave_height, Some(vec![1.5, 1.6]));
    }
}
