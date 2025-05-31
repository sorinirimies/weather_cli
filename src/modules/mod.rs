// Modules for the weather_cli project
pub mod types;
pub mod forecaster;
pub mod location;
pub mod ui;
pub mod utils;

// Re-export common types
pub use types::{WeatherConfig, DetailLevel, Location, CurrentWeather, Forecast};