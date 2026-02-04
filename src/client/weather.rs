//! Weather API client

use crate::types::weather::{WeatherRequest, WeatherResponse};
use crate::{Error, Result};
use super::{OpenMeteoClient, RetryConfig, with_retry};

impl OpenMeteoClient {
    /// Get weather forecast data
    ///
    /// # Arguments
    /// * `req` - Weather request with latitude, longitude, and parameters
    ///
    /// # Returns
    /// Weather forecast response with current, hourly, and daily data
    pub async fn get_weather(&self, req: &WeatherRequest) -> Result<WeatherResponse> {
        // Validate coordinates
        crate::error::validate_coordinates(req.latitude, req.longitude)?;

        let url = format!("{}/forecast", self.base_urls.weather);

        let response = self.http_client
            .get(&url)
            .query(req)
            .send()
            .await?;

        OpenMeteoClient::validate_response_status(response.status())?;

        let body = response.json::<WeatherResponse>().await?;

        tracing::debug!(
            latitude = req.latitude,
            longitude = req.longitude,
            timezone = &body.timezone,
            "Weather forecast retrieved successfully"
        );

        Ok(body)
    }

    /// Get weather forecast with automatic retry logic
    pub async fn get_weather_with_retry(
        &self,
        req: &WeatherRequest,
        retry_config: RetryConfig,
    ) -> Result<WeatherResponse> {
        let req = req.clone();
        with_retry(
            || {
                let client = self.clone();
                let req = req.clone();
                async move { client.get_weather(&req).await }
            },
            retry_config,
        )
        .await
    }
}

impl Clone for OpenMeteoClient {
    fn clone(&self) -> Self {
        Self {
            http_client: self.http_client.clone(),
            base_urls: self.base_urls.clone(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::weather::CurrentWeather;

    #[test]
    fn test_weather_request_validation() {
        let req_invalid = WeatherRequest {
            latitude: 999.0,
            longitude: 11.6,
            ..Default::default()
        };

        assert!(crate::error::validate_coordinates(req_invalid.latitude, req_invalid.longitude)
            .is_err());
    }

    #[test]
    fn test_weather_response_deserialization() {
        let json = r#"{
            "latitude": 48.1,
            "longitude": 11.6,
            "elevation": 518.0,
            "timezone": "Europe/Berlin",
            "timezone_abbreviation": "CET",
            "current": {
                "time": "2026-02-04T12:00:00",
                "interval": 900,
                "temperature": 5.2,
                "relative_humidity": 65,
                "weather_code": 2,
                "wind_speed_10m": 8.5,
                "wind_direction_10m": 220,
                "is_day": 1
            }
        }"#;

        let response: WeatherResponse = serde_json::from_str(json).expect("Valid weather JSON");
        assert_eq!(response.latitude, 48.1);
        assert_eq!(response.timezone, "Europe/Berlin");
        assert!(response.current.is_some());

        let current = response.current.expect("Current weather exists");
        assert_eq!(current.temperature, 5.2);
        assert_eq!(current.weather_code, 2);
    }
}
