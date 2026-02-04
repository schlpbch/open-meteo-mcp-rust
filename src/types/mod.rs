//! Type definitions for API requests and responses

pub mod weather;
pub mod location;
pub mod air_quality;
pub mod marine;
pub mod snow;
pub mod alerts;
pub mod astronomy;
pub mod comparison;
pub mod comfort;

pub use weather::{WeatherRequest, WeatherResponse, CurrentWeather, HourlyData, DailyData};
pub use location::{GeocodeRequest, GeocodeResponse, Location};
pub use air_quality::{AirQualityRequest, AirQualityResponse, AirQualityData};
pub use marine::{MarineRequest, MarineResponse, WaveData};
pub use snow::{SnowRequest, SnowResponse, SnowData};
pub use alerts::{AlertsRequest, AlertsResponse, WeatherAlert};
pub use astronomy::{AstronomyRequest, AstronomyResponse, AstronomyData};
pub use comparison::{ComparisonRequest, ComparisonResponse, LocationCoords, LocationWeather};
pub use comfort::{ComfortRequest, ComfortResponse, ComfortData};
