//! Type definitions for API requests and responses

pub mod air_quality;
pub mod alerts;
pub mod astronomy;
pub mod comfort;
pub mod comparison;
pub mod location;
pub mod marine;
pub mod snow;
pub mod weather;

pub use air_quality::{AirQualityData, AirQualityRequest, AirQualityResponse};
pub use alerts::{AlertsRequest, AlertsResponse, WeatherAlert};
pub use astronomy::{AstronomyData, AstronomyRequest, AstronomyResponse};
pub use comfort::{ComfortData, ComfortRequest, ComfortResponse};
pub use comparison::{ComparisonRequest, ComparisonResponse, LocationCoords, LocationWeather};
pub use location::{GeocodeRequest, GeocodeResponse, Location};
pub use marine::{MarineRequest, MarineResponse, WaveData};
pub use snow::{SnowData, SnowRequest, SnowResponse};
pub use weather::{CurrentWeather, DailyData, HourlyData, WeatherRequest, WeatherResponse};
