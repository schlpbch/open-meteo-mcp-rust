//! MCP Prompts - Workflow templates for weather-aware planning
//!
//! Prompts guide AI assistants through multi-step workflows:
//! - Ski trip planning with snow conditions and weather
//! - Outdoor activity planning with safety considerations
//! - Travel planning with weather integration

use crate::service::OpenMeteoService;
use crate::{CallToolResult, McpError, ToolContent};

impl OpenMeteoService {
    /// Ski trip weather planning prompt
    ///
    /// Generates a guide for checking snow conditions and weather for ski trips to Swiss resorts.
    ///
    /// Provides a workflow for assessing ski conditions combining
    /// snow depth, weather forecasts, and safety considerations.
    ///
    /// WORKFLOW:
    /// 1. Identify the resort location
    /// 2. Check snow conditions with get_snow_conditions tool
    /// 3. Check general weather with get_weather tool
    /// 4. Assess ski conditions (Excellent/Good/Fair/Poor)
    /// 5. Provide recommendations and gear suggestions
    pub async fn ski_trip_weather_prompt(
        &self,
        resort: Option<String>,
        dates: Option<String>,
    ) -> std::result::Result<CallToolResult, McpError> {
        let resort_info = resort
            .as_ref()
            .map(|r| format!("for {}", r))
            .unwrap_or_default();
        let date_info = dates
            .as_ref()
            .map(|d| format!("on {}", d))
            .unwrap_or_default();
        let dates_for_format = if date_info.is_empty() {
            "the requested dates".to_string()
        } else {
            dates.unwrap_or_default()
        };

        let prompt = format!(
            r#"# Ski Trip Weather Planning {} {}

Follow this workflow to assess ski conditions:

## Step 1: Identify the Resort Location
- If resort name provided: Use search_location to find coordinates
- If not provided: Ask user for resort name
- Suggested resorts: Zermatt, Verbier, St. Moritz, Davos, Saas-Fee, Grindelwald
- Tip: Swiss resort names can be searched directly

## Step 2: Check Snow Conditions
Use get_snow_conditions tool with:
- Resort coordinates
- 7-day forecast
- Include daily data for snow depth and snowfall analysis

Analyze:
- Current snow depth (ideal: >50cm)
- Recent snowfall (fresh powder: >10cm in last 24h)
- Temperature (ideal: -10°C to -5°C)
- Snowfall trend over forecast period

## Step 3: Check General Weather
Use get_weather tool with:
- Same coordinates
- 7-day forecast
- Include daily data for visibility and precipitation

Analyze:
- Weather code (reference weather://codes resource)
- Wind speed and gusts (concerning if >50 km/h)
- Visibility (important for safety)
- Precipitation probability
- Temperature range

## Step 4: Assess Ski Conditions
Combine data to determine:
- **Excellent**: Fresh powder (>10cm), -15°C to -5°C, clear skies, good visibility
- **Good**: Good depth (>50cm), stable temps (<0°C), mostly clear
- **Fair**: Minimal depth (>20cm), temps below freezing, acceptable visibility
- **Poor**: Insufficient snow, warm temps (>5°C), poor weather, limited visibility

## Step 5: Provide Recommendations
Based on {}:
- Best days to ski (weather + snow quality)
- Gear recommendations (layers, goggles for flat light, avalanche awareness)
- Safety warnings (wind, visibility, avalanche risk)
- Alternative dates if conditions are poor

## Resources
- Weather codes: weather://codes
- Swiss locations: weather://swiss-locations
- Weather parameters: weather://parameters

## Example Response Format
**Snow Conditions**: [Depth + recent snowfall]
**Weather**: [Conditions + temperature range]
**Ski Assessment**: [Excellent/Good/Fair/Poor with reasoning]
**Best Days**: [Specific dates with why]
**Recommendations**: [Gear, safety, alternatives]
"#,
            resort_info,
            date_info,
            dates_for_format
        );

        tracing::debug!("Generating ski trip weather prompt");
        Ok(CallToolResult::success(vec![ToolContent::Text(prompt)]))
    }

    /// Outdoor activity planning prompt
    ///
    /// Generates a weather-aware outdoor activity planning workflow for hiking, cycling, and other outdoor pursuits.
    ///
    /// Provides a workflow for weather-aware activity planning with
    /// sensitivity assessments and safety recommendations.
    pub async fn outdoor_activity_prompt(
        &self,
        activity: Option<String>,
        location: Option<String>,
        dates: Option<String>,
    ) -> std::result::Result<CallToolResult, McpError> {
        let activity_str = activity.unwrap_or_else(|| "outdoor activity".to_string());
        let location_str = location
            .as_ref()
            .map(|l| format!("near {}", l))
            .unwrap_or_else(|| "your chosen location".to_string());
        let date_str = dates
            .as_ref()
            .map(|d| format!("on {}", d))
            .unwrap_or_else(|| "your requested dates".to_string());

        let prompt = format!(
            r#"# Weather-Aware Outdoor Activity Planning

Planning: {} {}
Dates: {}

## Step 1: Identify Location & Activity Details
- Activity type: {}
- Location coordinates: Use search_location tool to find coordinates
- Elevation: Higher elevations have different weather patterns
- Activity sensitivity level:
  - **High**: Rock climbing, via ferrata, water activities (weather critical)
  - **Medium**: Day hiking, cycling, trail running (weather important)
  - **Low**: Walking, photography (weather nice-to-have)

## Step 2: Check Forecast Weather
Use get_weather tool with:
- Location coordinates
- Hourly forecast for detailed weather progression
- 7-day daily forecast

Critical parameters for outdoor activities:
- Temperature (comfort and safety)
- Precipitation probability (rain, snow)
- Wind speed and gusts (safety factor)
- Cloud cover and visibility (enjoyment)
- Apparent temperature (what it feels like)

## Step 3: Check Air Quality
Use get_air_quality tool if relevant:
- Especially important for people with respiratory conditions
- Reference weather://aqi-reference for health implications
- Consider UV index for sun-exposed activities

## Step 4: Assess Activity Conditions
Rate activity feasibility:
- **Excellent**: Perfect conditions, ideal timing
- **Good**: Suitable conditions, minor considerations
- **Fair**: Conditions manageable with precautions
- **Poor**: Not recommended, suggest alternative dates

## Step 5: Provide Detailed Recommendations
Based on the activity:
- **Timing**: Best time of day and specific dates
- **Equipment**: Necessary gear and clothing layers
- **Safety**: Specific hazards and precautions
- **Alternatives**: Other dates/locations if current conditions poor
- **Health**: Any air quality or UV concerns

## Resources
- Weather codes: weather://codes
- Weather parameters: weather://parameters
- AQI guidance: weather://aqi-reference
- Swiss locations: weather://swiss-locations (if in Switzerland)

## Example Response Format
**Conditions**: [Temperature, wind, precipitation summary]
**Air Quality**: [AQI and pollen if relevant]
**Activity Feasibility**: [Excellent/Good/Fair/Poor]
**Best Times**: [Specific dates and times]
**Recommendations**: [Equipment, safety measures, alternatives]
"#,
            activity_str,
            location_str,
            date_str,
            activity_str
        );

        tracing::debug!("Generating outdoor activity prompt");
        Ok(CallToolResult::success(vec![ToolContent::Text(prompt)]))
    }

    /// Travel planning prompt with weather integration
    ///
    /// Generates a weather-aware travel planning workflow that integrates
    /// multiple weather factors for trip decision-making.
    pub async fn travel_planning_prompt(
        &self,
        destination: Option<String>,
        dates: Option<String>,
    ) -> std::result::Result<CallToolResult, McpError> {
        let dest_str = destination
            .as_ref()
            .map(|d| format!("to {}", d))
            .unwrap_or_else(|| "your destination".to_string());
        let date_str = dates
            .as_ref()
            .map(|d| format!("from {}", d))
            .unwrap_or_else(|| "your requested dates".to_string());

        let prompt = format!(
            r#"# Weather-Aware Travel Planning

Destination: {}
Dates: {}

## Step 1: Location Research
- Primary destination coordinates
- Alternative locations nearby (backups in case of weather)
- Use search_location for coordinate lookup
- Note elevation differences affecting weather

## Step 2: Get Comprehensive Weather Forecast
Use get_weather tool for each location:
- 7-day forecast minimum
- Key metrics:
  - Temperature range (daytime/nighttime)
  - Precipitation type and probability
  - Wind conditions
  - Cloud cover and sunshine hours

## Step 3: Check Specialty Conditions if Needed
Depending on activities:
- Snow conditions: Use get_snow_conditions (winter travel)
- Marine conditions: Use get_marine_conditions (coastal/water activities)
- Air quality: Use get_air_quality (sensitive travelers)
- Astronomy: Use get_astronomy (stargazing trips)

## Step 4: Compare Locations
Use compare_locations tool for:
- Multiple potential destinations
- Different dates at same location
- Elevation-based weather variations

## Step 5: Assess Travel Conditions
Evaluate:
- **Best case**: Ideal weather, all activities possible
- **Good case**: Mostly favorable, some limitations
- **Acceptable**: Manageable with precautions
- **Challenging**: Significant constraints or risks
- **Not recommended**: Poor conditions or safety concerns

## Step 6: Provide Travel Recommendations
Include:
- **Timing**: Best dates for optimal weather
- **Packing**: Climate-appropriate clothing and gear
- **Activities**: Weather-dependent activity suggestions
- **Contingencies**: Rainy day alternatives
- **Logistics**: Weather impact on travel
- **Health**: Altitude, air quality, UV concerns

## Bonus: Historical Context
Use get_historical_weather for:
- What weather was typical for this time
- Expected vs. unusual conditions
- Pattern analysis for planning

## Resources
- weather://codes (interpret weather conditions)
- weather://parameters (understand data)
- weather://aqi-reference (health considerations)
- weather://swiss-locations (Swiss travel)

## Example Response Format
**Weather Summary**: [Expected conditions by day]
**Travel Assessment**: [Best/Good/Acceptable/Challenging]
**Best Travel Dates**: [Specific date recommendations]
**Packing List**: [Climate-appropriate gear]
**Activity Suggestions**: [What to do in given weather]
**Contingency Plan**: [Rainy day alternatives]
"#,
            dest_str, date_str
        );

        tracing::debug!("Generating travel planning prompt");
        Ok(CallToolResult::success(vec![ToolContent::Text(prompt)]))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_ski_trip_weather_prompt() {
        let config = crate::Config::default();
        let service = OpenMeteoService::new(config).expect("Valid service");

        let result = service
            .ski_trip_weather_prompt(Some("Zermatt".to_string()), Some("next weekend".to_string()))
            .await;

        assert!(result.is_ok());
        let tool_result = result.expect("Ski prompt result");
        assert!(!tool_result.is_error);

        if let ToolContent::Text(text) = &tool_result.content[0] {
            assert!(text.contains("Ski Trip"));
            assert!(text.contains("Zermatt"));
        }
    }

    #[tokio::test]
    async fn test_outdoor_activity_prompt() {
        let config = crate::Config::default();
        let service = OpenMeteoService::new(config).expect("Valid service");

        let result = service
            .outdoor_activity_prompt(
                Some("hiking".to_string()),
                Some("Munich".to_string()),
                Some("tomorrow".to_string()),
            )
            .await;

        assert!(result.is_ok());
        let tool_result = result.expect("Activity prompt result");
        assert!(!tool_result.is_error);
    }

    #[tokio::test]
    async fn test_travel_planning_prompt() {
        let config = crate::Config::default();
        let service = OpenMeteoService::new(config).expect("Valid service");

        let result = service
            .travel_planning_prompt(Some("Paris".to_string()), None)
            .await;

        assert!(result.is_ok());
        let tool_result = result.expect("Travel prompt result");
        assert!(!tool_result.is_error);
    }
}
