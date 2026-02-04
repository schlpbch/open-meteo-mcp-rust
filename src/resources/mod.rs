//! MCP Resources - Static reference data for weather, air quality, and locations
//!
//! Resources provide reference data that helps interpret tool responses:
//! - Weather codes: WMO weather code reference
//! - Parameters: Available weather parameters with units
//! - AQI reference: Air quality scales and health guidance
//! - Swiss locations: Database of Swiss cities, mountains, and resorts

use crate::service::OpenMeteoService;
use crate::{CallToolResult, McpError, ToolContent};

/// Weather code resource URI
pub const URI_WEATHER_CODES: &str = "weather://codes";

/// Weather parameters resource URI
pub const URI_PARAMETERS: &str = "weather://parameters";

/// AQI reference resource URI
pub const URI_AQI_REFERENCE: &str = "weather://aqi-reference";

/// Swiss locations resource URI
pub const URI_SWISS_LOCATIONS: &str = "weather://swiss-locations";

impl OpenMeteoService {
    /// Get weather codes resource
    ///
    /// WMO (World Meteorological Organization) weather code reference.
    ///
    /// Contains WMO weather code reference with descriptions, categories,
    /// icons, and travel impact assessments.
    ///
    /// USE THIS RESOURCE WHEN:
    /// - Interpreting weather codes from get_weather tool responses
    /// - Explaining weather conditions to users (e.g., code 71 = "Light snow")
    /// - Providing travel impact guidance based on weather
    /// - Checking weather categorization (clear, cloudy, rainy, snowy, stormy)
    ///
    /// CONTAINS:
    /// - 0-99: WMO weather codes with descriptions
    /// - Each code with category, icon, and travel impact assessment
    /// - Examples: 0=Clear sky, 1=Mainly clear, 3=Overcast, 51=Light drizzle, 80=Moderate rain showers
    pub async fn get_weather_codes(
        &self,
    ) -> std::result::Result<CallToolResult, McpError> {
        let data = include_str!("data/weather-codes.json");
        tracing::debug!("Retrieving weather codes resource");
        Ok(CallToolResult::success(vec![ToolContent::Text(
            data.to_string(),
        )]))
    }

    /// Get weather parameters resource
    ///
    /// Complete reference of available weather and snow parameters from Open-Meteo API.
    ///
    /// Documents available weather and snow parameters from Open-Meteo API
    /// including hourly and daily parameters with units and descriptions.
    ///
    /// USE THIS RESOURCE WHEN:
    /// - Understanding what parameters are available in weather responses
    /// - Checking units and measurement types (°C, mm, km/h, %)
    /// - Learning about snow-specific parameters (snow depth, snowfall, snow water equivalent)
    /// - Interpreting complex metrics (apparent temperature, dew point, cloud cover)
    ///
    /// INCLUDES:
    /// - Hourly parameters: temperature, precipitation, wind, humidity, pressure, etc.
    /// - Daily parameters: min/max temperature, precipitation sum, weather codes, etc.
    /// - Snow parameters: snow depth, snowfall, snow water equivalent
    /// - Each parameter with unit, description, and data type
    pub async fn get_weather_parameters(
        &self,
    ) -> std::result::Result<CallToolResult, McpError> {
        let data = include_str!("data/weather-parameters.json");
        tracing::debug!("Retrieving weather parameters resource");
        Ok(CallToolResult::success(vec![ToolContent::Text(
            data.to_string(),
        )]))
    }

    /// Get AQI reference resource
    ///
    /// Air Quality Index (AQI) scales and health impact guidance.
    ///
    /// Contains European and US AQI scales with health implications,
    /// UV index guidance, and pollen level information.
    ///
    /// USE THIS RESOURCE WHEN:
    /// - Interpreting AQI values from get_air_quality tool responses
    /// - Providing health guidance based on air quality (sensitive groups warnings, etc.)
    /// - Explaining UV index levels and sun exposure risks
    /// - Interpreting pollen counts and allergy risk levels
    /// - Planning outdoor activities for people with respiratory conditions
    ///
    /// CONTAINS:
    /// - European AQI (0-100+): Good, Fair, Moderate, Poor, Very Poor, Extremely Poor
    /// - US AQI (0-500): Good, Moderate, Unhealthy for Sensitive, Unhealthy, Very Unhealthy, Hazardous
    /// - UV Index levels (0-11+): Low, Moderate, High, Very High, Extreme
    /// - Pollen levels and health recommendations
    pub async fn get_aqi_reference(
        &self,
    ) -> std::result::Result<CallToolResult, McpError> {
        let data = include_str!("data/aqi-reference.json");
        tracing::debug!("Retrieving AQI reference resource");
        Ok(CallToolResult::success(vec![ToolContent::Text(
            data.to_string(),
        )]))
    }

    /// Get Swiss locations resource
    ///
    /// Reference database of major Swiss locations with geographical coordinates.
    ///
    /// Contains major Swiss cities, mountains, passes, and lakes
    /// with GPS coordinates and elevation data.
    ///
    /// USE THIS RESOURCE WHEN:
    /// - Looking up coordinates for Swiss locations without using search_location
    /// - Identifying ski resorts and mountain peaks in Switzerland
    /// - Planning trips to specific Swiss regions
    /// - Comparing elevations for weather impact analysis
    /// - Quick reference for major Swiss towns, mountains, and landmarks
    ///
    /// LOCATION CATEGORIES:
    /// - Major cities: Zurich, Bern, Basel, Geneva, Lausanne, Lucerne, etc.
    /// - Mountains & Peaks: Matterhorn, Eiger, Jungfrau, Monte Rosa, etc.
    /// - Ski Resorts: Zermatt, Verbier, St. Moritz, Davos, Saas-Fee, etc.
    /// - Mountain Passes: Gotthard, Simplon, Furka, etc.
    /// - Lakes: Lake Geneva, Lake Zurich, Lake Lucerne, etc.
    ///
    /// Each location includes: name, latitude, longitude, elevation (meters)
    pub async fn get_swiss_locations(
        &self,
    ) -> std::result::Result<CallToolResult, McpError> {
        let data = include_str!("data/swiss-locations.json");
        tracing::debug!("Retrieving Swiss locations resource");
        Ok(CallToolResult::success(vec![ToolContent::Text(
            data.to_string(),
        )]))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_get_weather_codes() {
        let config = crate::Config::default();
        let service = OpenMeteoService::new(config).expect("Valid service");

        let result = service.get_weather_codes().await;

        assert!(result.is_ok());
        let tool_result = result.expect("Weather codes result");
        assert!(!tool_result.is_error);
        assert!(!tool_result.content.is_empty());

        // Verify JSON content
        if let ToolContent::Text(text) = &tool_result.content[0] {
            assert!(text.contains("weather") || text.contains("code"));
        }
    }

    #[tokio::test]
    async fn test_get_weather_parameters() {
        let config = crate::Config::default();
        let service = OpenMeteoService::new(config).expect("Valid service");

        let result = service.get_weather_parameters().await;

        assert!(result.is_ok());
        let tool_result = result.expect("Parameters result");
        assert!(!tool_result.is_error);
        assert!(!tool_result.content.is_empty());
    }

    #[tokio::test]
    async fn test_get_aqi_reference() {
        let config = crate::Config::default();
        let service = OpenMeteoService::new(config).expect("Valid service");

        let result = service.get_aqi_reference().await;

        assert!(result.is_ok());
        let tool_result = result.expect("AQI reference result");
        assert!(!tool_result.is_error);
        assert!(!tool_result.content.is_empty());
    }

    #[tokio::test]
    async fn test_get_swiss_locations() {
        let config = crate::Config::default();
        let service = OpenMeteoService::new(config).expect("Valid service");

        let result = service.get_swiss_locations().await;

        assert!(result.is_ok());
        let tool_result = result.expect("Swiss locations result");
        assert!(!tool_result.is_error);
        assert!(!tool_result.content.is_empty());
    }
}
