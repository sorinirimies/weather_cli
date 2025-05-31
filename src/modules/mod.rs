// Modules for the weather_cli project
pub mod forecaster;
pub mod location;
pub mod types;
pub mod ui;
pub mod utils;

// Re-export common types
pub use types::{CurrentWeather, DetailLevel, Forecast, Location, WeatherConfig};
