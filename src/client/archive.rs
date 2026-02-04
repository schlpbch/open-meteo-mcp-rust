//! Archive API client (historical weather data)

use crate::types::weather::{WeatherRequest, WeatherResponse};
use crate::{Error, Result};
use super::{OpenMeteoClient, RetryConfig, with_retry};

impl OpenMeteoClient {
    /// Get historical weather data (1940-present)
    ///
    /// # Arguments
    /// * `req` - Weather request with latitude, longitude, and historical parameters
    /// * `start_date` - Start date in YYYY-MM-DD format
    /// * `end_date` - End date in YYYY-MM-DD format
    ///
    /// # Returns
    /// Historical weather data for the specified date range
    pub async fn get_historical_weather(
        &self,
        req: &WeatherRequest,
        start_date: &str,
        end_date: &str,
    ) -> Result<WeatherResponse> {
        // Validate coordinates
        crate::error::validate_coordinates(req.latitude, req.longitude)?;

        // Validate dates (basic format check)
        if !is_valid_date_format(start_date) {
            return Err(Error::InvalidParameter(
                "Invalid start_date format, use YYYY-MM-DD".to_string(),
            ));
        }
        if !is_valid_date_format(end_date) {
            return Err(Error::InvalidParameter(
                "Invalid end_date format, use YYYY-MM-DD".to_string(),
            ));
        }

        let url = format!("{}/archive", self.base_urls.archive);

        let response = self.http_client
            .get(&url)
            .query(req)
            .query(&[("start_date", start_date)])
            .query(&[("end_date", end_date)])
            .send()
            .await?;

        OpenMeteoClient::validate_response_status(response.status())?;

        let body = response.json::<WeatherResponse>().await?;

        tracing::debug!(
            latitude = req.latitude,
            longitude = req.longitude,
            start_date = start_date,
            end_date = end_date,
            "Historical weather retrieved successfully"
        );

        Ok(body)
    }

    /// Get historical weather data with automatic retry logic
    pub async fn get_historical_weather_with_retry(
        &self,
        req: &WeatherRequest,
        start_date: &str,
        end_date: &str,
        retry_config: RetryConfig,
    ) -> Result<WeatherResponse> {
        let req = req.clone();
        let start_date = start_date.to_string();
        let end_date = end_date.to_string();
        with_retry(
            || {
                let client = self.clone();
                let req = req.clone();
                let start_date = start_date.clone();
                let end_date = end_date.clone();
                async move {
                    client
                        .get_historical_weather(&req, &start_date, &end_date)
                        .await
                }
            },
            retry_config,
        )
        .await
    }
}

/// Validate date format and semantics (YYYY-MM-DD)
///
/// Checks format and validates month (1-12) and day (1-31) ranges
fn is_valid_date_format(date_str: &str) -> bool {
    let parts: Vec<&str> = date_str.split('-').collect();
    if parts.len() != 3 {
        return false;
    }

    // Format check
    if !(parts[0].len() == 4 && parts[0].chars().all(|c| c.is_ascii_digit())) {
        return false;
    }
    if !(parts[1].len() == 2 && parts[1].chars().all(|c| c.is_ascii_digit())) {
        return false;
    }
    if !(parts[2].len() == 2 && parts[2].chars().all(|c| c.is_ascii_digit())) {
        return false;
    }

    // Semantic validation: month 1-12, day 1-31
    let month: u32 = parts[1].parse().unwrap_or(0);
    let day: u32 = parts[2].parse().unwrap_or(0);

    month >= 1 && month <= 12 && day >= 1 && day <= 31
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_valid_date_format() {
        assert!(is_valid_date_format("2026-02-04"));
        assert!(is_valid_date_format("1940-01-01"));
        assert!(is_valid_date_format("2025-12-31"));
    }

    #[test]
    fn test_invalid_date_format() {
        assert!(!is_valid_date_format("2026/02/04"));
        assert!(!is_valid_date_format("02-04-2026"));
        assert!(!is_valid_date_format("2026-2-4"));
        assert!(!is_valid_date_format("invalid"));
    }

    #[test]
    fn test_date_semantic_validation() {
        // Valid dates
        assert!(is_valid_date_format("2026-01-01"));
        assert!(is_valid_date_format("2026-12-31"));

        // Invalid month
        assert!(!is_valid_date_format("2026-00-01"));
        assert!(!is_valid_date_format("2026-13-01"));

        // Invalid day
        assert!(!is_valid_date_format("2026-02-00"));
        assert!(!is_valid_date_format("2026-02-32"));
        assert!(!is_valid_date_format("2026-99-99"));
    }

    #[test]
    fn test_historical_response_deserialization() {
        let json = r#"{
            "latitude": 48.1,
            "longitude": 11.6,
            "elevation": 518.0,
            "timezone": "Europe/Berlin",
            "daily": {
                "time": ["2026-02-01", "2026-02-02"],
                "weather_code": [2, 3],
                "temperature_2m_max": [8.5, 9.2],
                "temperature_2m_min": [2.1, 2.8],
                "precipitation_sum": [0.5, 1.2],
                "wind_speed_10m_max": [12.3, 13.5],
                "wind_direction_10m_dominant": [200, 210]
            }
        }"#;

        let response: WeatherResponse = serde_json::from_str(json).expect("Valid JSON");
        assert_eq!(response.latitude, 48.1);
        assert!(response.daily.is_some());

        let daily = response.daily.expect("Expected field exists");
        assert_eq!(daily.time.len(), 2);
    }
}
