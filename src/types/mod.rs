//! Type definitions for API requests and responses

pub mod weather;
pub mod location;
pub mod air_quality;
pub mod marine;

pub use weather::{WeatherRequest, WeatherResponse, CurrentWeather, HourlyData, DailyData};
pub use location::{GeocodeRequest, GeocodeResponse, Location};
pub use air_quality::{AirQualityRequest, AirQualityResponse, AirQualityData};
pub use marine::{MarineRequest, MarineResponse, WaveData};
