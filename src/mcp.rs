//! rmcp SDK wiring
//!
//! Bridges the existing business-logic methods on [`OpenMeteoService`] (which
//! return the project's own [`crate::CallToolResult`]/[`crate::McpError`]) to
//! the real `rmcp` 3.1 [`ServerHandler`] trait: tools via `#[tool_router]`,
//! resources and prompts via manual `list_resources`/`read_resource`/
//! `list_prompts`/`get_prompt` overrides (rmcp has no macro for those).

use crate::service::OpenMeteoService;
use crate::{resources, CallToolResult as LocalCallToolResult, McpError, ToolContent};
use rmcp::handler::server::wrapper::Parameters;
use rmcp::model::{
    ErrorData, GetPromptRequestParams, GetPromptResult, ListPromptsResult, ListResourcesResult,
    Prompt, PromptArgument, PromptMessage, ReadResourceRequestParams, ReadResourceResult, Resource,
    ResourceContents, Role, ServerCapabilities, ServerInfo,
};
use rmcp::service::RequestContext;
use rmcp::{tool, tool_handler, tool_router, RoleServer, ServerHandler};
use schemars::JsonSchema;
use serde::Deserialize;
use serde_json::Value;

/// Converts the project's internal tool result/error types into rmcp's.
fn to_rmcp_result(
    result: std::result::Result<LocalCallToolResult, McpError>,
) -> std::result::Result<rmcp::model::CallToolResult, ErrorData> {
    match result {
        Ok(r) => {
            let content: Vec<_> = r
                .content
                .into_iter()
                .map(|c| match c {
                    ToolContent::Text(text) => rmcp::model::ContentBlock::text(text),
                    ToolContent::Json(value) => rmcp::model::ContentBlock::text(
                        serde_json::to_string_pretty(&value).unwrap_or_else(|_| value.to_string()),
                    ),
                })
                .collect();
            if r.is_error {
                Ok(rmcp::model::CallToolResult::error(content))
            } else {
                Ok(rmcp::model::CallToolResult::success(content))
            }
        }
        Err(e) => Err(to_rmcp_error(e)),
    }
}

fn to_rmcp_error(err: McpError) -> ErrorData {
    match err {
        McpError::InvalidRequest(msg) => ErrorData::invalid_request(msg, None),
        McpError::InvalidParameter(msg) => ErrorData::invalid_params(msg, None),
        McpError::ResourceNotFound(msg) => ErrorData::resource_not_found(msg, None),
        McpError::InternalError(msg) => ErrorData::internal_error(msg, None),
        McpError::ToolError(msg) => ErrorData::internal_error(msg, None),
        McpError::RateLimit(msg) => ErrorData::internal_error(msg, None),
        McpError::Timeout(msg) => ErrorData::internal_error(msg, None),
    }
}

/// Extracts a `String`-valued `text` result out of a local `CallToolResult`,
/// used for resources and prompts (which are always plain text).
fn extract_text(
    result: std::result::Result<LocalCallToolResult, McpError>,
) -> std::result::Result<String, ErrorData> {
    let r = result.map_err(to_rmcp_error)?;
    match r.content.into_iter().next() {
        Some(ToolContent::Text(text)) => Ok(text),
        Some(ToolContent::Json(value)) => Ok(value.to_string()),
        None => Err(ErrorData::internal_error("Empty response", None)),
    }
}

// ---------------------------------------------------------------------------
// Tool parameter structs
// ---------------------------------------------------------------------------

#[derive(Debug, Deserialize, JsonSchema)]
pub struct GetWeatherParams {
    #[schemars(description = "Location latitude (-90 to 90)")]
    pub latitude: f64,
    #[schemars(description = "Location longitude (-180 to 180)")]
    pub longitude: f64,
    #[schemars(
        description = "Comma-separated hourly variables, e.g. temperature_2m,precipitation"
    )]
    pub hourly: Option<String>,
    #[schemars(
        description = "Comma-separated daily variables, e.g. temperature_2m_max,precipitation_sum"
    )]
    pub daily: Option<String>,
    #[schemars(description = "Forecast days (1-16, default 7)")]
    pub forecast_days: Option<u8>,
    #[schemars(description = "Temperature unit: celsius or fahrenheit (default celsius)")]
    pub temperature_unit: Option<String>,
}

#[derive(Debug, Deserialize, JsonSchema)]
pub struct GetSnowConditionsParams {
    #[schemars(description = "Location latitude (-90 to 90)")]
    pub latitude: f64,
    #[schemars(description = "Location longitude (-180 to 180)")]
    pub longitude: f64,
    #[schemars(description = "Comma-separated hourly variables, e.g. snowfall,snow_depth")]
    pub hourly: Option<String>,
    #[schemars(description = "Comma-separated daily variables")]
    pub daily: Option<String>,
    #[schemars(description = "Forecast days (1-16, default 7)")]
    pub forecast_days: Option<u8>,
}

#[derive(Debug, Deserialize, JsonSchema)]
pub struct GetAirQualityParams {
    #[schemars(description = "Location latitude (-90 to 90)")]
    pub latitude: f64,
    #[schemars(description = "Location longitude (-180 to 180)")]
    pub longitude: f64,
    #[schemars(description = "Comma-separated hourly variables")]
    pub hourly: Option<String>,
    #[schemars(description = "Comma-separated daily variables")]
    pub daily: Option<String>,
}

#[derive(Debug, Deserialize, JsonSchema)]
pub struct SearchLocationParams {
    #[schemars(description = "Location name to search for")]
    pub name: String,
    #[schemars(description = "Maximum number of results")]
    pub count: Option<u32>,
    #[schemars(description = "Language code for results, e.g. en, de")]
    pub language: Option<String>,
}

#[derive(Debug, Deserialize, JsonSchema)]
pub struct SearchLocationSwissParams {
    #[schemars(description = "Location name to search for")]
    pub name: String,
    #[schemars(description = "Maximum number of results")]
    pub count: Option<u32>,
}

#[derive(Debug, Deserialize, JsonSchema)]
pub struct GetWeatherAlertsParams {
    #[schemars(description = "Location latitude (-90 to 90)")]
    pub latitude: f64,
    #[schemars(description = "Location longitude (-180 to 180)")]
    pub longitude: f64,
    #[schemars(description = "Temperature threshold (°C) above which a heat alert triggers")]
    pub temperature_threshold_hot: Option<f64>,
    #[schemars(description = "Temperature threshold (°C) below which a cold alert triggers")]
    pub temperature_threshold_cold: Option<f64>,
    #[schemars(description = "Precipitation threshold (mm) above which an alert triggers")]
    pub precipitation_threshold: Option<f64>,
    #[schemars(description = "Wind speed threshold (km/h) above which an alert triggers")]
    pub wind_speed_threshold: Option<f64>,
}

#[derive(Debug, Deserialize, JsonSchema)]
pub struct GetComfortIndexParams {
    #[schemars(description = "Location latitude (-90 to 90)")]
    pub latitude: f64,
    #[schemars(description = "Location longitude (-180 to 180)")]
    pub longitude: f64,
    #[schemars(description = "Activity type, e.g. hiking, running, cycling")]
    pub activity_type: Option<String>,
    #[schemars(description = "Forecast days (1-16, default 7)")]
    pub forecast_days: Option<u8>,
}

#[derive(Debug, Deserialize, JsonSchema)]
pub struct GetAstronomyParams {
    #[schemars(description = "Location latitude (-90 to 90)")]
    pub latitude: f64,
    #[schemars(description = "Location longitude (-180 to 180)")]
    pub longitude: f64,
    #[schemars(description = "Forecast days (1-16, default 7)")]
    pub forecast_days: Option<u8>,
}

#[derive(Debug, Deserialize, JsonSchema)]
pub struct LocationCoord {
    #[schemars(description = "Location latitude (-90 to 90)")]
    pub latitude: f64,
    #[schemars(description = "Location longitude (-180 to 180)")]
    pub longitude: f64,
}

#[derive(Debug, Deserialize, JsonSchema)]
pub struct CompareLocationsParams {
    #[schemars(description = "Locations to compare (2-10 entries)")]
    pub locations: Vec<LocationCoord>,
    #[schemars(description = "Comma-separated hourly variables")]
    pub hourly: Option<String>,
    #[schemars(description = "Comma-separated daily variables")]
    pub daily: Option<String>,
    #[schemars(description = "Forecast days (1-16, default 7)")]
    pub forecast_days: Option<u8>,
}

#[derive(Debug, Deserialize, JsonSchema)]
pub struct GetHistoricalWeatherParams {
    #[schemars(description = "Location latitude (-90 to 90)")]
    pub latitude: f64,
    #[schemars(description = "Location longitude (-180 to 180)")]
    pub longitude: f64,
    #[schemars(description = "Start date (YYYY-MM-DD)")]
    pub start_date: String,
    #[schemars(description = "End date (YYYY-MM-DD)")]
    pub end_date: String,
    #[schemars(description = "Comma-separated hourly variables")]
    pub hourly: Option<String>,
    #[schemars(description = "Comma-separated daily variables")]
    pub daily: Option<String>,
}

#[derive(Debug, Deserialize, JsonSchema)]
pub struct GetMarineConditionsParams {
    #[schemars(description = "Location latitude (-90 to 90)")]
    pub latitude: f64,
    #[schemars(description = "Location longitude (-180 to 180)")]
    pub longitude: f64,
    #[schemars(description = "Comma-separated hourly variables")]
    pub hourly: Option<String>,
    #[schemars(description = "Comma-separated daily variables")]
    pub daily: Option<String>,
    #[schemars(description = "Forecast days (1-16, default 7)")]
    pub forecast_days: Option<u8>,
}

// ---------------------------------------------------------------------------
// Tool router
// ---------------------------------------------------------------------------

#[tool_router(vis = "pub")]
impl OpenMeteoService {
    #[tool(name = "ping", description = "Simple connectivity check")]
    pub async fn tool_ping(&self) -> std::result::Result<rmcp::model::CallToolResult, ErrorData> {
        to_rmcp_result(self.ping().await)
    }

    #[tool(
        name = "get_weather",
        description = "Get weather forecast with temperature, precipitation, and wind"
    )]
    pub async fn tool_get_weather(
        &self,
        Parameters(p): Parameters<GetWeatherParams>,
    ) -> std::result::Result<rmcp::model::CallToolResult, ErrorData> {
        to_rmcp_result(
            self.get_weather(
                p.latitude,
                p.longitude,
                p.hourly,
                p.daily,
                p.forecast_days,
                p.temperature_unit,
            )
            .await,
        )
    }

    #[tool(
        name = "get_snow_conditions",
        description = "Get snow depth, snowfall, and mountain weather"
    )]
    pub async fn tool_get_snow_conditions(
        &self,
        Parameters(p): Parameters<GetSnowConditionsParams>,
    ) -> std::result::Result<rmcp::model::CallToolResult, ErrorData> {
        to_rmcp_result(
            self.get_snow_conditions(p.latitude, p.longitude, p.hourly, p.daily, p.forecast_days)
                .await,
        )
    }

    #[tool(
        name = "get_air_quality",
        description = "Get AQI, pollutants, UV index, and pollen data"
    )]
    pub async fn tool_get_air_quality(
        &self,
        Parameters(p): Parameters<GetAirQualityParams>,
    ) -> std::result::Result<rmcp::model::CallToolResult, ErrorData> {
        to_rmcp_result(
            self.get_air_quality(p.latitude, p.longitude, p.hourly, p.daily)
                .await,
        )
    }

    #[tool(
        name = "search_location",
        description = "Search for locations by name via geocoding"
    )]
    pub async fn tool_search_location(
        &self,
        Parameters(p): Parameters<SearchLocationParams>,
    ) -> std::result::Result<rmcp::model::CallToolResult, ErrorData> {
        to_rmcp_result(self.search_location(p.name, p.count, p.language).await)
    }

    #[tool(
        name = "search_location_swiss",
        description = "Search for Swiss locations by name"
    )]
    pub async fn tool_search_location_swiss(
        &self,
        Parameters(p): Parameters<SearchLocationSwissParams>,
    ) -> std::result::Result<rmcp::model::CallToolResult, ErrorData> {
        to_rmcp_result(self.search_location_swiss(p.name, p.count).await)
    }

    #[tool(
        name = "get_weather_alerts",
        description = "Get weather alerts based on configurable thresholds"
    )]
    pub async fn tool_get_weather_alerts(
        &self,
        Parameters(p): Parameters<GetWeatherAlertsParams>,
    ) -> std::result::Result<rmcp::model::CallToolResult, ErrorData> {
        to_rmcp_result(
            self.get_weather_alerts(
                p.latitude,
                p.longitude,
                p.temperature_threshold_hot,
                p.temperature_threshold_cold,
                p.precipitation_threshold,
                p.wind_speed_threshold,
            )
            .await,
        )
    }

    #[tool(
        name = "get_comfort_index",
        description = "Get an outdoor activity comfort score (0-100)"
    )]
    pub async fn tool_get_comfort_index(
        &self,
        Parameters(p): Parameters<GetComfortIndexParams>,
    ) -> std::result::Result<rmcp::model::CallToolResult, ErrorData> {
        to_rmcp_result(
            self.get_comfort_index(p.latitude, p.longitude, p.activity_type, p.forecast_days)
                .await,
        )
    }

    #[tool(
        name = "get_astronomy",
        description = "Get sunrise, sunset, golden hour, and moon phase"
    )]
    pub async fn tool_get_astronomy(
        &self,
        Parameters(p): Parameters<GetAstronomyParams>,
    ) -> std::result::Result<rmcp::model::CallToolResult, ErrorData> {
        to_rmcp_result(
            self.get_astronomy(p.latitude, p.longitude, p.forecast_days)
                .await,
        )
    }

    #[tool(
        name = "compare_locations",
        description = "Compare weather across multiple locations (2-10)"
    )]
    pub async fn tool_compare_locations(
        &self,
        Parameters(p): Parameters<CompareLocationsParams>,
    ) -> std::result::Result<rmcp::model::CallToolResult, ErrorData> {
        let locations = p
            .locations
            .into_iter()
            .map(|c| (c.latitude, c.longitude))
            .collect();
        to_rmcp_result(
            self.compare_locations(locations, p.hourly, p.daily, p.forecast_days)
                .await,
        )
    }

    #[tool(
        name = "get_historical_weather",
        description = "Get historical weather data (1940-present)"
    )]
    pub async fn tool_get_historical_weather(
        &self,
        Parameters(p): Parameters<GetHistoricalWeatherParams>,
    ) -> std::result::Result<rmcp::model::CallToolResult, ErrorData> {
        to_rmcp_result(
            self.get_historical_weather(
                p.latitude,
                p.longitude,
                p.start_date,
                p.end_date,
                p.hourly,
                p.daily,
            )
            .await,
        )
    }

    #[tool(
        name = "get_marine_conditions",
        description = "Get wave/swell data for lakes and coasts"
    )]
    pub async fn tool_get_marine_conditions(
        &self,
        Parameters(p): Parameters<GetMarineConditionsParams>,
    ) -> std::result::Result<rmcp::model::CallToolResult, ErrorData> {
        to_rmcp_result(
            self.get_marine_conditions(p.latitude, p.longitude, p.hourly, p.daily, p.forecast_days)
                .await,
        )
    }
}

// ---------------------------------------------------------------------------
// Resources (no rmcp macro support — implemented manually)
// ---------------------------------------------------------------------------

const RESOURCE_DEFS: [(&str, &str, &str); 4] = [
    (
        resources::URI_WEATHER_CODES,
        "weather-codes",
        "WMO weather code reference",
    ),
    (
        resources::URI_PARAMETERS,
        "weather-parameters",
        "Available weather and snow API parameters",
    ),
    (
        resources::URI_AQI_REFERENCE,
        "aqi-reference",
        "AQI scales and health recommendations",
    ),
    (
        resources::URI_SWISS_LOCATIONS,
        "swiss-locations",
        "Swiss cities, mountains, and passes",
    ),
];

// ---------------------------------------------------------------------------
// Prompts (no rmcp macro support — implemented manually)
// ---------------------------------------------------------------------------

const PROMPT_SKI_TRIP: &str = "ski-trip-weather";
const PROMPT_OUTDOOR_ACTIVITY: &str = "plan-outdoor-activity";
const PROMPT_TRAVEL_PLANNING: &str = "weather-aware-travel";

fn arg_str(arguments: &Option<rmcp::model::JsonObject>, key: &str) -> Option<String> {
    arguments
        .as_ref()
        .and_then(|a| a.get(key))
        .and_then(Value::as_str)
        .map(str::to_string)
}

#[tool_handler(router = self.tool_router)]
impl ServerHandler for OpenMeteoService {
    fn get_info(&self) -> ServerInfo {
        ServerInfo::new(
            ServerCapabilities::builder()
                .enable_tools()
                .enable_resources()
                .enable_prompts()
                .build(),
        )
        .with_server_info(rmcp::model::Implementation::new(
            "open-meteo-mcp",
            env!("CARGO_PKG_VERSION"),
        ))
        .with_instructions("Weather, snow, air quality, and location data via the Open-Meteo API.")
    }

    async fn list_resources(
        &self,
        _request: Option<rmcp::model::PaginatedRequestParams>,
        _context: RequestContext<RoleServer>,
    ) -> std::result::Result<ListResourcesResult, ErrorData> {
        let resources = RESOURCE_DEFS
            .iter()
            .map(|(uri, name, description)| {
                Resource::new(*uri, *name)
                    .with_description(*description)
                    .with_mime_type("application/json")
            })
            .collect();
        Ok(ListResourcesResult::with_all_items(resources))
    }

    async fn read_resource(
        &self,
        request: ReadResourceRequestParams,
        _context: RequestContext<RoleServer>,
    ) -> std::result::Result<rmcp::model::ReadResourceResponse, ErrorData> {
        let text = match request.uri.as_str() {
            resources::URI_WEATHER_CODES => extract_text(self.get_weather_codes().await)?,
            resources::URI_PARAMETERS => extract_text(self.get_weather_parameters().await)?,
            resources::URI_AQI_REFERENCE => extract_text(self.get_aqi_reference().await)?,
            resources::URI_SWISS_LOCATIONS => extract_text(self.get_swiss_locations().await)?,
            other => {
                return Err(ErrorData::resource_not_found(
                    format!("Unknown resource URI: {}", other),
                    None,
                ))
            }
        };
        let contents = ResourceContents::text(text, request.uri).with_mime_type("application/json");
        Ok(ReadResourceResult::new(vec![contents]).into())
    }

    async fn list_prompts(
        &self,
        _request: Option<rmcp::model::PaginatedRequestParams>,
        _context: RequestContext<RoleServer>,
    ) -> std::result::Result<ListPromptsResult, ErrorData> {
        let prompts = vec![
            Prompt::new(
                PROMPT_SKI_TRIP,
                Some("Ski trip planning with snow conditions and weather"),
                Some(vec![
                    PromptArgument::new("resort").with_description("Ski resort name"),
                    PromptArgument::new("dates").with_description("Trip dates"),
                ]),
            ),
            Prompt::new(
                PROMPT_OUTDOOR_ACTIVITY,
                Some("Weather-aware outdoor activity planning"),
                Some(vec![
                    PromptArgument::new("activity")
                        .with_description("Activity type, e.g. hiking, running"),
                    PromptArgument::new("location").with_description("Location name"),
                    PromptArgument::new("dates").with_description("Planned dates"),
                ]),
            ),
            Prompt::new(
                PROMPT_TRAVEL_PLANNING,
                Some("Travel planning with weather integration"),
                Some(vec![
                    PromptArgument::new("destination").with_description("Travel destination"),
                    PromptArgument::new("dates").with_description("Travel dates"),
                ]),
            ),
        ];
        Ok(ListPromptsResult::with_all_items(prompts))
    }

    async fn get_prompt(
        &self,
        request: GetPromptRequestParams,
        _context: RequestContext<RoleServer>,
    ) -> std::result::Result<rmcp::model::GetPromptResponse, ErrorData> {
        let text = match request.name.as_str() {
            PROMPT_SKI_TRIP => {
                let resort = arg_str(&request.arguments, "resort");
                let dates = arg_str(&request.arguments, "dates");
                extract_text(self.ski_trip_weather_prompt(resort, dates).await)?
            }
            PROMPT_OUTDOOR_ACTIVITY => {
                let activity = arg_str(&request.arguments, "activity");
                let location = arg_str(&request.arguments, "location");
                let dates = arg_str(&request.arguments, "dates");
                extract_text(
                    self.outdoor_activity_prompt(activity, location, dates)
                        .await,
                )?
            }
            PROMPT_TRAVEL_PLANNING => {
                let destination = arg_str(&request.arguments, "destination");
                let dates = arg_str(&request.arguments, "dates");
                extract_text(self.travel_planning_prompt(destination, dates).await)?
            }
            other => {
                return Err(ErrorData::invalid_params(
                    format!("Unknown prompt: {}", other),
                    None,
                ))
            }
        };
        let result = GetPromptResult::new(vec![PromptMessage::new_text(Role::Assistant, text)]);
        Ok(result.into())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn tool_router_registers_all_tools() {
        let router = OpenMeteoService::tool_router();
        let tools = router.list_all();
        assert_eq!(tools.len(), 12);
        let names: Vec<_> = tools.iter().map(|t| t.name.to_string()).collect();
        for expected in [
            "ping",
            "get_weather",
            "get_snow_conditions",
            "get_air_quality",
            "search_location",
            "search_location_swiss",
            "get_weather_alerts",
            "get_comfort_index",
            "get_astronomy",
            "compare_locations",
            "get_historical_weather",
            "get_marine_conditions",
        ] {
            assert!(
                names.contains(&expected.to_string()),
                "missing tool: {}",
                expected
            );
        }
    }

    #[test]
    fn get_info_declares_capabilities() {
        let config = crate::Config::default();
        let service = OpenMeteoService::new(config).expect("valid service");
        let info = service.get_info();
        assert!(info.capabilities.tools.is_some());
        assert!(info.capabilities.resources.is_some());
        assert!(info.capabilities.prompts.is_some());
    }
}
