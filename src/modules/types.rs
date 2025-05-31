use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::fmt;
use strum::EnumString;
use strum_macros::Display;

/// Represents configuration options for the weather forecasting tool
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WeatherConfig {
    pub units: String,
    pub location: Option<String>,
    pub json_output: bool,
    pub animation_enabled: bool,
    pub detail_level: DetailLevel,
}

impl Default for WeatherConfig {
    fn default() -> Self {
        Self {
            units: "metric".to_string(),
            location: None,
            json_output: false,
            animation_enabled: true,
            detail_level: DetailLevel::Standard,
        }
    }
}

/// Level of detail for weather output
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord, Display, EnumString)]
pub enum DetailLevel {
    #[strum(to_string = "Basic")]
    Basic,
    #[strum(to_string = "Standard")]
    Standard,
    #[strum(to_string = "Detailed")]
    Detailed,
    #[strum(to_string = "Debug")]
    Debug,
}

/// Represents weather condition categories
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum WeatherCondition {
    Clear,
    Clouds,
    Rain,
    Drizzle,
    Thunderstorm,
    Snow,
    Mist,
    Fog,
    Smoke,
    Haze,
    Dust,
    Sand,
    Ash,
    Squall,
    Tornado,
    Unknown,
}

impl WeatherCondition {
    pub fn from_str(condition: &str) -> Self {
        match condition.to_lowercase().as_str() {
            "clear" => WeatherCondition::Clear,
            "clouds" => WeatherCondition::Clouds,
            "rain" => WeatherCondition::Rain,
            "drizzle" => WeatherCondition::Drizzle,
            "thunderstorm" => WeatherCondition::Thunderstorm,
            "snow" => WeatherCondition::Snow,
            "mist" => WeatherCondition::Mist,
            "fog" => WeatherCondition::Fog,
            "smoke" => WeatherCondition::Smoke,
            "haze" => WeatherCondition::Haze,
            "dust" => WeatherCondition::Dust,
            "sand" => WeatherCondition::Sand,
            "ash" => WeatherCondition::Ash,
            "squall" => WeatherCondition::Squall,
            "tornado" => WeatherCondition::Tornado,
            _ => WeatherCondition::Unknown,
        }
    }

    pub fn get_emoji(&self) -> &'static str {
        match self {
            WeatherCondition::Clear => "☀️",
            WeatherCondition::Clouds => "☁️",
            WeatherCondition::Rain => "🌧️",
            WeatherCondition::Drizzle => "🌦️",
            WeatherCondition::Thunderstorm => "⛈️",
            WeatherCondition::Snow => "❄️",
            WeatherCondition::Mist => "🌫️",
            WeatherCondition::Fog => "🌫️",
            WeatherCondition::Smoke => "🌫️",
            WeatherCondition::Haze => "🌫️",
            WeatherCondition::Dust => "🌫️",
            WeatherCondition::Sand => "🌫️",
            WeatherCondition::Ash => "🌫️",
            WeatherCondition::Squall => "💨",
            WeatherCondition::Tornado => "🌪️",
            WeatherCondition::Unknown => "❓",
        }
    }
}

impl fmt::Display for WeatherCondition {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            WeatherCondition::Clear => "Clear",
            WeatherCondition::Clouds => "Cloudy",
            WeatherCondition::Rain => "Rainy",
            WeatherCondition::Drizzle => "Drizzle",
            WeatherCondition::Thunderstorm => "Thunderstorm",
            WeatherCondition::Snow => "Snowy",
            WeatherCondition::Mist => "Misty",
            WeatherCondition::Fog => "Foggy",
            WeatherCondition::Smoke => "Smoky",
            WeatherCondition::Haze => "Hazy",
            WeatherCondition::Dust => "Dusty",
            WeatherCondition::Sand => "Sandy",
            WeatherCondition::Ash => "Ashy",
            WeatherCondition::Squall => "Squall",
            WeatherCondition::Tornado => "Tornado",
            WeatherCondition::Unknown => "Unknown",
        };
        write!(f, "{}", s)
    }
}

/// Represents location information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Location {
    pub name: String,
    pub country: String,
    pub country_code: String,
    pub latitude: f64,
    pub longitude: f64,
    pub timezone: String,
    pub region: Option<String>,
    pub state: Option<String>,
}

impl Default for Location {
    fn default() -> Self {
        Self {
            name: "Unknown".to_string(),
            country: "Unknown".to_string(),
            country_code: "UN".to_string(),
            latitude: 0.0,
            longitude: 0.0,
            timezone: "UTC".to_string(),
            region: None,
            state: None,
        }
    }
}

/// Represents current weather data
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CurrentWeather {
    pub timestamp: DateTime<Utc>,
    pub temperature: f64,
    pub feels_like: f64,
    pub humidity: u8,
    pub pressure: u32,
    pub wind_speed: f64,
    pub wind_direction: u16,
    pub conditions: Vec<WeatherDescription>,
    pub main_condition: WeatherCondition,
    pub visibility: u32,
    pub clouds: u8,
    pub uv_index: f64,
    pub sunrise: DateTime<Utc>,
    pub sunset: DateTime<Utc>,
    pub rain_last_hour: Option<f64>,
    pub snow_last_hour: Option<f64>,
    pub air_quality_index: Option<u8>,
}

/// Represents detailed weather description
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WeatherDescription {
    pub id: u32,
    pub main: String,
    pub description: String,
    pub icon: String,
}

/// Represents hourly forecast data
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HourlyForecast {
    pub timestamp: DateTime<Utc>,
    pub temperature: f64,
    pub feels_like: f64,
    pub humidity: u8,
    pub pressure: u32,
    pub wind_speed: f64,
    pub wind_direction: u16,
    pub conditions: Vec<WeatherDescription>,
    pub main_condition: WeatherCondition,
    pub pop: f64, // Probability of precipitation
    pub visibility: u32,
    pub clouds: u8,
    pub rain: Option<f64>,
    pub snow: Option<f64>,
}

/// Represents daily forecast data
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DailyForecast {
    pub date: DateTime<Utc>,
    pub sunrise: DateTime<Utc>,
    pub sunset: DateTime<Utc>,
    pub temp_morning: f64,
    pub temp_day: f64,
    pub temp_evening: f64,
    pub temp_night: f64,
    pub temp_min: f64,
    pub temp_max: f64,
    pub feels_like_day: f64,
    pub feels_like_night: f64,
    pub pressure: u32,
    pub humidity: u8,
    pub wind_speed: f64,
    pub wind_direction: u16,
    pub conditions: Vec<WeatherDescription>,
    pub main_condition: WeatherCondition,
    pub clouds: u8,
    pub pop: f64,
    pub rain: Option<f64>,
    pub snow: Option<f64>,
    pub uv_index: f64,
}

/// Represents a complete weather forecast
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Forecast {
    pub current: Option<CurrentWeather>,
    pub hourly: Vec<HourlyForecast>,
    pub daily: Vec<DailyForecast>,
    pub timezone_offset: i32,
    pub units: String,
}

/// Represents air quality data
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AirQuality {
    pub aqi: u8, // 1-5 scale (1: Good, 2: Fair, 3: Moderate, 4: Poor, 5: Very Poor)
    pub co: f64, // Carbon monoxide (μg/m3)
    pub no: f64, // Nitrogen monoxide (μg/m3)
    pub no2: f64, // Nitrogen dioxide (μg/m3)
    pub o3: f64,  // Ozone (μg/m3)
    pub so2: f64, // Sulphur dioxide (μg/m3)
    pub pm2_5: f64, // Fine particles (μg/m3)
    pub pm10: f64,  // Coarse particles (μg/m3)
    pub nh3: f64,   // Ammonia (μg/m3)
}

/// Represents alert information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WeatherAlert {
    pub sender: String,
    pub event: String,
    pub start: DateTime<Utc>,
    pub end: DateTime<Utc>,
    pub description: String,
    pub tags: Vec<String>,
}