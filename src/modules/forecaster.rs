use anyhow::{anyhow, Result};
use chrono::{DateTime, Duration, TimeZone, Timelike, Utc};
use reqwest::Client;
use serde_json::Value;
use std::collections::HashMap;
use std::time::Duration as StdDuration;

use crate::modules::types::{
    CurrentWeather, DailyForecast, Forecast, HourlyForecast, Location, WeatherCondition,
    WeatherConfig, WeatherDescription,
};

/// Open-Meteo base URL (doesn't require API key)
const OPENMETEO_BASE_URL: &str = "https://api.open-meteo.com/v1";

/// Handles weather data retrieval and processing
#[derive(Clone)]
pub struct WeatherForecaster {
    client: Client,
    config: WeatherConfig,
    api_keys: HashMap<String, String>,
}

impl WeatherForecaster {
    /// Create a new weather forecaster with the given configuration
    pub fn new(config: WeatherConfig) -> Self {
        let client = Client::builder()
            .timeout(StdDuration::from_secs(30))
            .build()
            .unwrap_or_default();

        let api_keys = HashMap::new();

        Self {
            client,
            config,
            api_keys,
        }
    }

    /// Get current weather for a location
    pub async fn get_current_weather(&self, location: &Location) -> Result<CurrentWeather> {
        self.get_openmeteo_current(location).await
    }

    /// Get hourly forecast for a location (next 48 hours)
    pub async fn get_hourly_forecast(&self, location: &Location) -> Result<Vec<HourlyForecast>> {
        let forecast = self.get_openmeteo_forecast(location).await?;
        Ok(forecast.hourly)
    }

    /// Get daily forecast for a location (next 7 days)
    pub async fn get_daily_forecast(&self, location: &Location) -> Result<Vec<DailyForecast>> {
        let forecast = self.get_openmeteo_forecast(location).await?;
        Ok(forecast.daily)
    }

    /// Get complete forecast including current, hourly, and daily data
    pub async fn get_forecast(&self, location: &Location) -> Result<Forecast> {
        self.get_openmeteo_forecast(location).await
    }

    /// Get forecast from Open-Meteo API (no API key required)
    async fn get_openmeteo_forecast(&self, location: &Location) -> Result<Forecast> {
        // Build URL with parameters for both hourly and daily forecasts
        let url = format!(
            "{}/forecast?latitude={}&longitude={}&hourly=temperature_2m,relative_humidity_2m,apparent_temperature,precipitation_probability,precipitation,rain,showers,snowfall,weather_code,cloud_cover,pressure_msl,surface_pressure,wind_speed_10m,wind_direction_10m,wind_gusts_10m&daily=weather_code,temperature_2m_max,temperature_2m_min,apparent_temperature_max,apparent_temperature_min,sunrise,sunset,uv_index_max,precipitation_sum,rain_sum,snowfall_sum,precipitation_probability_max,wind_speed_10m_max,wind_direction_10m_dominant&timezone=auto&current=temperature_2m,relative_humidity_2m,apparent_temperature,is_day,precipitation,rain,showers,snowfall,weather_code,cloud_cover,pressure_msl,surface_pressure,wind_speed_10m,wind_direction_10m,wind_gusts_10m",
            OPENMETEO_BASE_URL, location.latitude, location.longitude
        );

        let response = self.client.get(&url).send().await?;
        let json: Value = response.json().await?;

        if let Some(error) = json["error"].as_bool() {
            if error {
                let reason = json["reason"].as_str().unwrap_or("Unknown error");
                return Err(anyhow!("Open-Meteo API error: {}", reason));
            }
        }

        // Parse current weather
        let current = self.parse_openmeteo_current(&json)?;

        // Parse hourly forecast
        let hourly = self.parse_openmeteo_hourly(&json)?;

        // Parse daily forecast
        let daily = self.parse_openmeteo_daily(&json)?;

        // Get timezone offset
        let timezone_offset = json["utc_offset_seconds"].as_i64().unwrap_or(0) as i32;

        // Determine units based on config
        let units = self.config.units.clone();

        // Create the Forecast object
        Ok(Forecast {
            current: Some(current),
            hourly,
            daily,
            timezone_offset,
            units,
        })
    }

    /// Get current weather from Open-Meteo API
    async fn get_openmeteo_current(&self, location: &Location) -> Result<CurrentWeather> {
        // Build URL with parameters
        let url = format!(
            "{}/forecast?latitude={}&longitude={}&current=temperature_2m,relative_humidity_2m,apparent_temperature,is_day,precipitation,rain,showers,snowfall,weather_code,cloud_cover,pressure_msl,surface_pressure,wind_speed_10m,wind_direction_10m,wind_gusts_10m&daily=sunrise,sunset&timezone=auto",
            OPENMETEO_BASE_URL, location.latitude, location.longitude
        );

        let response = self.client.get(&url).send().await?;
        let json: Value = response.json().await?;

        if let Some(error) = json["error"].as_bool() {
            if error {
                let reason = json["reason"].as_str().unwrap_or("Unknown error");
                return Err(anyhow!("Open-Meteo API error: {}", reason));
            }
        }

        self.parse_openmeteo_current(&json)
    }

    /// Parse current weather from Open-Meteo API response
    fn parse_openmeteo_current(&self, json: &Value) -> Result<CurrentWeather> {
        // Parse current weather
        let current = &json["current"];
        let current_time = current["time"].as_str().unwrap_or_default();
        let timestamp = match DateTime::parse_from_rfc3339(current_time) {
            Ok(dt) => dt.with_timezone(&Utc),
            Err(_) => Utc::now(),
        };

        // Parse weather variables
        let temp = current["temperature_2m"].as_f64().unwrap_or(0.0);
        let feels_like = current["apparent_temperature"].as_f64().unwrap_or(0.0);
        let humidity = current["relative_humidity_2m"].as_f64().unwrap_or(0.0) as u8;
        let pressure = current["surface_pressure"].as_f64().unwrap_or(0.0) as u32;
        let wind_speed = current["wind_speed_10m"].as_f64().unwrap_or(0.0);
        let wind_direction = current["wind_direction_10m"].as_f64().unwrap_or(0.0) as u16;
        let clouds = current["cloud_cover"].as_f64().unwrap_or(0.0) as u8;
        let weather_code = current["weather_code"].as_f64().unwrap_or(0.0) as u32;
        let is_day = current["is_day"].as_i64().unwrap_or(1) == 1;

        // Create weather condition from WMO code
        let main_condition = self.wmo_code_to_condition(weather_code);

        // Create weather description
        let description = self.get_weather_description_from_wmo(weather_code, is_day);

        // Precipitation data
        let rain_last_hour = current["rain"].as_f64();
        let snow_last_hour = current["snowfall"].as_f64();

        // Daily info for sunrise/sunset
        let daily = &json["daily"];
        let sunrise_time = daily["sunrise"]
            .as_array()
            .and_then(|arr| arr.first())
            .and_then(|v| v.as_str())
            .unwrap_or_default();

        let sunset_time = daily["sunset"]
            .as_array()
            .and_then(|arr| arr.first())
            .and_then(|v| v.as_str())
            .unwrap_or_default();

        let sunrise = match DateTime::parse_from_rfc3339(sunrise_time) {
            Ok(dt) => dt.with_timezone(&Utc),
            Err(_) => timestamp, // Fallback to current time
        };

        let sunset = match DateTime::parse_from_rfc3339(sunset_time) {
            Ok(dt) => dt.with_timezone(&Utc),
            Err(_) => timestamp
                .checked_add_signed(Duration::hours(12))
                .unwrap_or(timestamp), // Fallback to 12 hours later
        };

        // Create the CurrentWeather object
        Ok(CurrentWeather {
            timestamp,
            temperature: temp,
            feels_like,
            humidity,
            pressure,
            wind_speed,
            wind_direction,
            conditions: vec![description],
            main_condition,
            visibility: 10000, // Default to good visibility
            clouds,
            uv_index: 0.0, // Not provided by Open-Meteo basic API
            sunrise,
            sunset,
            rain_last_hour,
            snow_last_hour,
            air_quality_index: None,
        })
    }

    /// Parse hourly forecast from Open-Meteo API
    fn parse_openmeteo_hourly(&self, json: &Value) -> Result<Vec<HourlyForecast>> {
        let hourly = &json["hourly"];

        // Get time array
        let times = hourly["time"]
            .as_array()
            .ok_or_else(|| anyhow!("Missing time array"))?;
        let temps = hourly["temperature_2m"]
            .as_array()
            .ok_or_else(|| anyhow!("Missing temperature data"))?;
        let feels_like = hourly["apparent_temperature"]
            .as_array()
            .ok_or_else(|| anyhow!("Missing apparent temperature data"))?;
        let humidity = hourly["relative_humidity_2m"]
            .as_array()
            .ok_or_else(|| anyhow!("Missing humidity data"))?;
        let pressure = hourly["surface_pressure"]
            .as_array()
            .ok_or_else(|| anyhow!("Missing pressure data"))?;
        let wind_speed = hourly["wind_speed_10m"]
            .as_array()
            .ok_or_else(|| anyhow!("Missing wind speed data"))?;
        let wind_direction = hourly["wind_direction_10m"]
            .as_array()
            .ok_or_else(|| anyhow!("Missing wind direction data"))?;
        let clouds = hourly["cloud_cover"]
            .as_array()
            .ok_or_else(|| anyhow!("Missing cloud cover data"))?;
        let empty_vec_pop = Vec::new();
        let pop = hourly["precipitation_probability"]
            .as_array()
            .unwrap_or(&empty_vec_pop);
        let weather_codes = hourly["weather_code"]
            .as_array()
            .ok_or_else(|| anyhow!("Missing weather code data"))?;
        let empty_vec_rain = Vec::new();
        let rain = hourly["rain"].as_array().unwrap_or(&empty_vec_rain);
        let empty_vec_snow = Vec::new();
        let snow = hourly["snowfall"].as_array().unwrap_or(&empty_vec_snow);

        let mut forecasts = Vec::new();

        for i in 0..times.len().min(48) {
            // Limit to 48 hours (2 days)
            let time_str = times[i].as_str().unwrap_or_default();
            let timestamp = match DateTime::parse_from_rfc3339(time_str) {
                Ok(dt) => dt.with_timezone(&Utc),
                Err(_) => continue, // Skip invalid timestamps
            };

            let temp = temps.get(i).and_then(|v| v.as_f64()).unwrap_or(0.0);
            let feels = feels_like.get(i).and_then(|v| v.as_f64()).unwrap_or(0.0);
            let hum = humidity.get(i).and_then(|v| v.as_f64()).unwrap_or(0.0) as u8;
            let press = pressure.get(i).and_then(|v| v.as_f64()).unwrap_or(1013.0) as u32;
            let wind_spd = wind_speed.get(i).and_then(|v| v.as_f64()).unwrap_or(0.0);
            let wind_dir = wind_direction
                .get(i)
                .and_then(|v| v.as_f64())
                .unwrap_or(0.0) as u16;

            let precipitation_prob = pop.get(i).and_then(|v| v.as_f64()).unwrap_or(0.0);
            let weather_code = weather_codes.get(i).and_then(|v| v.as_f64()).unwrap_or(0.0) as u32;
            let cloud_cover = clouds.get(i).and_then(|v| v.as_f64()).unwrap_or(0.0) as u8;

            let rain_amount = rain.get(i).and_then(|v| v.as_f64());
            let snow_amount = snow.get(i).and_then(|v| v.as_f64());

            // Determine if it's day or night (simple approximation)
            let hour = timestamp.hour();
            let is_day = hour >= 6 && hour < 18;

            // Get weather condition from WMO code
            let main_condition = self.wmo_code_to_condition(weather_code);

            // Create weather description
            let description = self.get_weather_description_from_wmo(weather_code, is_day);

            forecasts.push(HourlyForecast {
                timestamp,
                temperature: temp,
                feels_like: feels,
                humidity: hum,
                pressure: press,
                wind_speed: wind_spd,
                wind_direction: wind_dir,
                conditions: vec![description],
                main_condition,
                pop: precipitation_prob / 100.0, // Convert from percentage to 0-1 scale
                visibility: 10000,               // Default to good visibility
                clouds: cloud_cover,
                rain: rain_amount,
                snow: snow_amount,
            });
        }

        Ok(forecasts)
    }

    /// Parse daily forecast from Open-Meteo API
    fn parse_openmeteo_daily(&self, json: &Value) -> Result<Vec<DailyForecast>> {
        let daily = &json["daily"];

        // Get date array
        let dates = daily["time"]
            .as_array()
            .ok_or_else(|| anyhow!("Missing date array"))?;
        let weather_codes = daily["weather_code"]
            .as_array()
            .ok_or_else(|| anyhow!("Missing weather code data"))?;
        let temp_max = daily["temperature_2m_max"]
            .as_array()
            .ok_or_else(|| anyhow!("Missing max temperature data"))?;
        let temp_min = daily["temperature_2m_min"]
            .as_array()
            .ok_or_else(|| anyhow!("Missing min temperature data"))?;
        let feels_max = daily["apparent_temperature_max"]
            .as_array()
            .ok_or_else(|| anyhow!("Missing max feels like data"))?;
        let feels_min = daily["apparent_temperature_min"]
            .as_array()
            .ok_or_else(|| anyhow!("Missing min feels like data"))?;
        let empty_vec_precip_sum = Vec::new();
        let _precip_sum = daily["precipitation_sum"]
            .as_array()
            .unwrap_or(&empty_vec_precip_sum);
        let wind_speed = daily["wind_speed_10m_max"]
            .as_array()
            .ok_or_else(|| anyhow!("Missing wind speed data"))?;
        let wind_direction = daily["wind_direction_10m_dominant"]
            .as_array()
            .ok_or_else(|| anyhow!("Missing wind direction data"))?;
        let empty_vec_prob = Vec::new();
        let precip_prob = daily["precipitation_probability_max"]
            .as_array()
            .unwrap_or(&empty_vec_prob);
        let empty_vec_rain = Vec::new();
        let rain_sum = daily["rain_sum"].as_array().unwrap_or(&empty_vec_rain);
        let empty_vec_snow = Vec::new();
        let snow_sum = daily["snowfall_sum"].as_array().unwrap_or(&empty_vec_snow);
        let empty_vec_uv = Vec::new();
        let uv_index = daily["uv_index_max"].as_array().unwrap_or(&empty_vec_uv);

        let sunrise_times = daily["sunrise"]
            .as_array()
            .ok_or_else(|| anyhow!("Missing sunrise data"))?;
        let sunset_times = daily["sunset"]
            .as_array()
            .ok_or_else(|| anyhow!("Missing sunset data"))?;

        let mut forecasts = Vec::new();

        for i in 0..dates.len().min(7) {
            // Limit to 7 days (1 week)
            let date_str = dates[i].as_str().unwrap_or_default();
            let date = match DateTime::parse_from_rfc3339(&format!("{}T12:00:00Z", date_str)) {
                Ok(dt) => dt.with_timezone(&Utc),
                Err(_) => continue, // Skip invalid dates
            };

            let sunrise_str = sunrise_times
                .get(i)
                .and_then(|v| v.as_str())
                .unwrap_or_default();
            let sunset_str = sunset_times
                .get(i)
                .and_then(|v| v.as_str())
                .unwrap_or_default();

            let sunrise = match DateTime::parse_from_rfc3339(sunrise_str) {
                Ok(dt) => dt.with_timezone(&Utc),
                Err(_) => date, // Fallback to noon
            };

            let sunset = match DateTime::parse_from_rfc3339(sunset_str) {
                Ok(dt) => dt.with_timezone(&Utc),
                Err(_) => date.checked_add_signed(Duration::hours(12)).unwrap_or(date), // Fallback to 12 hours later
            };

            let weather_code = weather_codes.get(i).and_then(|v| v.as_f64()).unwrap_or(0.0) as u32;
            let max = temp_max.get(i).and_then(|v| v.as_f64()).unwrap_or(0.0);
            let min = temp_min.get(i).and_then(|v| v.as_f64()).unwrap_or(0.0);
            let feels_like_day = feels_max.get(i).and_then(|v| v.as_f64()).unwrap_or(0.0);
            let feels_like_night = feels_min.get(i).and_then(|v| v.as_f64()).unwrap_or(0.0);
            let pop = precip_prob.get(i).and_then(|v| v.as_f64()).unwrap_or(0.0);
            let wind_spd = wind_speed.get(i).and_then(|v| v.as_f64()).unwrap_or(0.0);
            let wind_dir = wind_direction
                .get(i)
                .and_then(|v| v.as_f64())
                .unwrap_or(0.0) as u16;

            let rain_amount = rain_sum.get(i).and_then(|v| v.as_f64());
            let snow_amount = snow_sum.get(i).and_then(|v| v.as_f64());
            let uv = uv_index.get(i).and_then(|v| v.as_f64()).unwrap_or(0.0);

            // Get weather condition from WMO code
            let main_condition = self.wmo_code_to_condition(weather_code);

            // Create weather description
            let description = self.get_weather_description_from_wmo(weather_code, true);

            forecasts.push(DailyForecast {
                date,
                sunrise,
                sunset,
                temp_morning: min + (max - min) * 0.25, // Approximate morning temp
                temp_day: max,
                temp_evening: min + (max - min) * 0.5, // Approximate evening temp
                temp_night: min,
                temp_min: min,
                temp_max: max,
                feels_like_day,
                feels_like_night,
                pressure: 1013, // Default pressure as it's not provided in daily
                humidity: 50,   // Default humidity as it's not provided in daily
                wind_speed: wind_spd,
                wind_direction: wind_dir,
                conditions: vec![description],
                main_condition,
                clouds: 0,        // Not provided in daily forecast
                pop: pop / 100.0, // Convert from percentage to 0-1 scale
                rain: rain_amount,
                snow: snow_amount,
                uv_index: uv,
            });
        }

        Ok(forecasts)
    }

    /// Convert WMO weather code to our internal WeatherCondition
    pub fn wmo_code_to_condition(&self, code: u32) -> WeatherCondition {
        match code {
            0 => WeatherCondition::Clear,              // Clear sky
            1 | 2 | 3 => WeatherCondition::Clouds,     // Partly cloudy
            45 | 48 => WeatherCondition::Fog,          // Fog
            51 | 53 | 55 => WeatherCondition::Drizzle, // Drizzle
            56 | 57 => WeatherCondition::Drizzle,      // Freezing Drizzle
            61 | 63 | 65 => WeatherCondition::Rain,    // Rain
            66 | 67 => WeatherCondition::Rain,         // Freezing Rain
            71 | 73 | 75 => WeatherCondition::Snow,    // Snow
            77 => WeatherCondition::Snow,              // Snow grains
            80 | 81 | 82 => WeatherCondition::Rain,    // Rain showers
            85 | 86 => WeatherCondition::Snow,         // Snow showers
            95 => WeatherCondition::Thunderstorm,      // Thunderstorm
            96 | 99 => WeatherCondition::Thunderstorm, // Thunderstorm with hail
            _ => WeatherCondition::Unknown,
        }
    }

    /// Get weather description from WMO weather code
    pub fn get_weather_description_from_wmo(&self, code: u32, is_day: bool) -> WeatherDescription {
        let (main, description, icon) = match code {
            0 => ("Clear", "Clear sky", if is_day { "01d" } else { "01n" }),
            1 => ("Clouds", "Mainly clear", if is_day { "02d" } else { "02n" }),
            2 => (
                "Clouds",
                "Partly cloudy",
                if is_day { "03d" } else { "03n" },
            ),
            3 => ("Clouds", "Overcast", if is_day { "04d" } else { "04n" }),
            45 => ("Fog", "Fog", "50d"),
            48 => ("Fog", "Depositing rime fog", "50d"),
            51 => ("Drizzle", "Light drizzle", "09d"),
            53 => ("Drizzle", "Moderate drizzle", "09d"),
            55 => ("Drizzle", "Dense drizzle", "09d"),
            56 => ("Drizzle", "Light freezing drizzle", "09d"),
            57 => ("Drizzle", "Dense freezing drizzle", "09d"),
            61 => ("Rain", "Slight rain", "10d"),
            63 => ("Rain", "Moderate rain", "10d"),
            65 => ("Rain", "Heavy rain", "10d"),
            66 => ("Rain", "Light freezing rain", "10d"),
            67 => ("Rain", "Heavy freezing rain", "10d"),
            71 => ("Snow", "Slight snow fall", "13d"),
            73 => ("Snow", "Moderate snow fall", "13d"),
            75 => ("Snow", "Heavy snow fall", "13d"),
            77 => ("Snow", "Snow grains", "13d"),
            80 => ("Rain", "Slight rain showers", "09d"),
            81 => ("Rain", "Moderate rain showers", "09d"),
            82 => ("Rain", "Violent rain showers", "09d"),
            85 => ("Snow", "Slight snow showers", "13d"),
            86 => ("Snow", "Heavy snow showers", "13d"),
            95 => ("Thunderstorm", "Thunderstorm", "11d"),
            96 => ("Thunderstorm", "Thunderstorm with slight hail", "11d"),
            99 => ("Thunderstorm", "Thunderstorm with heavy hail", "11d"),
            _ => ("Unknown", "Unknown weather condition", "50d"),
        };

        WeatherDescription {
            id: code,
            main: main.to_string(),
            description: description.to_string(),
            icon: icon.to_string(),
        }
    }
}
