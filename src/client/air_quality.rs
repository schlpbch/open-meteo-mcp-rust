//! Air Quality API client

use crate::types::air_quality::{AirQualityRequest, AirQualityResponse};
use crate::Result;
use super::{OpenMeteoClient, RetryConfig, with_retry};

impl OpenMeteoClient {
    /// Get air quality data
    ///
    /// # Arguments
    /// * `req` - Air quality request with latitude, longitude, and parameters
    ///
    /// # Returns
    /// Air quality data including AQI, pollutants, and pollen levels
    pub async fn get_air_quality(&self, req: &AirQualityRequest) -> Result<AirQualityResponse> {
        // Validate coordinates
        crate::error::validate_coordinates(req.latitude, req.longitude)?;

        let url = format!("{}/air_quality", self.base_urls.air_quality);

        let response = self.http_client
            .get(&url)
            .query(req)
            .send()
            .await?;

        OpenMeteoClient::validate_response_status(response.status())?;

        let body = response.json::<AirQualityResponse>().await?;

        tracing::debug!(
            latitude = req.latitude,
            longitude = req.longitude,
            timezone = &body.timezone,
            "Air quality data retrieved successfully"
        );

        Ok(body)
    }

    /// Get air quality data with automatic retry logic
    pub async fn get_air_quality_with_retry(
        &self,
        req: &AirQualityRequest,
        retry_config: RetryConfig,
    ) -> Result<AirQualityResponse> {
        let req = req.clone();
        with_retry(
            || {
                let client = self.clone();
                let req = req.clone();
                async move { client.get_air_quality(&req).await }
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
    fn test_air_quality_response_deserialization() {
        let json = r#"{
            "latitude": 48.1,
            "longitude": 11.6,
            "elevation": 518.0,
            "timezone": "Europe/Berlin",
            "hourly": {
                "time": ["2026-02-04T00:00", "2026-02-04T01:00"],
                "pm10": [15.5, 16.2],
                "pm2_5": [8.3, 9.1],
                "o3": [45.0, 46.5],
                "us_aqi": [35, 37],
                "european_aqi": [40, 42]
            }
        }"#;

        let response: AirQualityResponse = serde_json::from_str(json).expect("Valid JSON");
        assert_eq!(response.latitude, 48.1);
        assert!(response.hourly.is_some());

        let hourly = response.hourly.expect("Expected field exists");
        assert_eq!(hourly.time.len(), 2);
        assert_eq!(hourly.pm10, Some(vec![15.5, 16.2]));
    }
}
