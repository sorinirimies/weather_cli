use anyhow::Result;
use chrono::{DateTime, Datelike, Duration, Local, NaiveTime, Utc, Weekday, Timelike};
use colored::*;
use console::Term;
use dialoguer::{theme::ColorfulTheme, Input, Select};
use indicatif::{ProgressBar, ProgressStyle};
use spinners::{Spinner, Spinners};
use std::io::Write;
use std::thread::sleep;
use std::time::Duration as StdDuration;

use crate::modules::types::{CurrentWeather, DailyForecast, Forecast, HourlyForecast, Location, WeatherCondition, WeatherConfig};
// use crate::modules::utils::*;

/// Handles UI rendering and animations
#[derive(Clone)]
pub struct WeatherUI {
    animation_enabled: bool,
    json_output: bool,
    term: Term,
}

impl WeatherUI {
    /// Create a new UI handler
    pub fn new(animation_enabled: bool, json_output: bool) -> Self {
        Self {
            animation_enabled,
            json_output,
            term: Term::stdout(),
        }
    }

    /// Show welcome banner
    pub fn show_welcome_banner(&self) -> Result<()> {
        if self.json_output {
            return Ok(());
        }
        
        self.term.clear_screen()?;
        
        let banner = r#"
 _       __           __  __                 __  ___          
| |     / /__  ____ _/ /_/ /_  ___  _____   /  |/  /___ _____ 
| | /| / / _ \/ __ `/ __/ __ \/ _ \/ ___/  / /|_/ / __ `/ __ \
| |/ |/ /  __/ /_/ / /_/ / / /  __/ /     / /  / / /_/ / / / /
|__/|__/\___/\__,_/\__/_/ /_/\___/_/     /_/  /_/\__,_/_/ /_/
            "#;

        // Always display the banner directly without animations
        println!("{}", banner.bright_cyan());
        println!("\n{}", "⟨⟨⟨ WEATHER MAN ACTIVATED ⟩⟩⟩".bright_cyan());
        
        println!();
        Ok(())
    }
    
    /// Show animation when connecting to weather services
    /// Show connecting message
    pub fn show_connecting_animation(&self) -> Result<()> {
        if !self.json_output {
            println!("Fetching weather data...");
            println!();
        }
        Ok(())
    }
    
    /// Display current weather information
    pub fn show_current_weather(&self, weather: &CurrentWeather, location: &Location) -> Result<()> {
        println!("{}", "╔═══════════════════════════════════════════════════╗".bright_cyan());
        println!("{}", "║               🌡️ CURRENT CONDITIONS 🌡️             ║".bright_cyan());
        println!("{}", "╚═══════════════════════════════════════════════════╝".bright_cyan());
        println!();
        
        if self.animation_enabled {
            sleep(StdDuration::from_millis(300));
        }
        
        // Format local time based on location's timezone
        let local_time = format_local_time(&weather.timestamp, &location.timezone);
        
        // Display location and time
        println!("📍 {}: {}, {}", "Location".bold(), location.name, location.country);
        println!("🕓 {}: {} ({})", "Local Time".bold(), local_time, location.timezone);
        println!();
        
        if self.animation_enabled {
            sleep(StdDuration::from_millis(300));
        }
        
        // Display main weather info with condition emoji
        let emoji = weather.main_condition.get_emoji();
        let conditions = if let Some(desc) = weather.conditions.first() {
            desc.description.clone()
        } else {
            weather.main_condition.to_string()
        };
        
        println!("{} {} {}", emoji, "Conditions:".bold(), conditions.to_title_case());
        
        // Format temperatures based on units
        let temp_unit = if self.config().units == "imperial" { "°F" } else { "°C" };
        println!("🌡️ {}: {:.1}{} (Feels like: {:.1}{})", 
            "Temperature".bold(), weather.temperature, temp_unit, weather.feels_like, temp_unit);
        
        if self.animation_enabled {
            sleep(StdDuration::from_millis(300));
        }
        
        // Wind info
        let wind_unit = if self.config().units == "imperial" { "mph" } else { "m/s" };
        let wind_direction = get_wind_direction_arrow(weather.wind_direction);
        println!("💨 {}: {:.1} {} {}", "Wind".bold(), weather.wind_speed, wind_unit, wind_direction);
        
        // Humidity and pressure
        println!("💧 {}: {}%", "Humidity".bold(), weather.humidity);
        println!("🔄 {}: {} hPa", "Pressure".bold(), weather.pressure);
        
        if self.animation_enabled {
            sleep(StdDuration::from_millis(300));
        }
        
        // Sunrise and sunset
        let sunrise = format_local_time(&weather.sunrise, &location.timezone);
        let sunset = format_local_time(&weather.sunset, &location.timezone);
        println!("🌅 {}: {}", "Sunrise".bold(), sunrise);
        println!("🌇 {}: {}", "Sunset".bold(), sunset);
        
        // UV index with color coding
        let uv_display = match weather.uv_index as u32 {
            0..=2 => format!("{:.1} (Low)", weather.uv_index).green(),
            3..=5 => format!("{:.1} (Moderate)", weather.uv_index).yellow(),
            6..=7 => format!("{:.1} (High)", weather.uv_index).bright_yellow(),
            8..=10 => format!("{:.1} (Very High)", weather.uv_index).bright_red(),
            _ => format!("{:.1} (Extreme)", weather.uv_index).red(),
        };
        println!("☀️ {}: {}", "UV Index".bold(), uv_display);
        
        // Precipitation if available
        if let Some(rain) = weather.rain_last_hour {
            println!("🌧️ {}: {:.1} mm", "Rain (last hour)".bold(), rain);
        }
        
        if let Some(snow) = weather.snow_last_hour {
            println!("❄️ {}: {:.1} mm", "Snow (last hour)".bold(), snow);
        }
        
        println!();
        
        Ok(())
    }
    
    /// Display hourly forecast
    pub fn show_hourly_forecast(&self, forecast: &[HourlyForecast], location: &Location) -> Result<()> {
        println!("{}", "╔═══════════════════════════════════════════════════╗".bright_cyan());
        println!("{}", "║             🕓 HOURLY FORECAST (24h) 🕓            ║".bright_cyan());
        println!("{}", "╚═══════════════════════════════════════════════════╝".bright_cyan());
        println!();
        
        if forecast.is_empty() {
            println!("No hourly forecast data available.");
            return Ok(());
        }
        
        // Limit to next 24 hours for display
        let hours_to_show = std::cmp::min(forecast.len(), 24);
        let temp_unit = if self.config().units == "imperial" { "°F" } else { "°C" };
        
        for (i, hour) in forecast.iter().take(hours_to_show).enumerate() {
            // Convert to local time
            let local_time = format_hour_only(&hour.timestamp, &location.timezone);
            let emoji = hour.main_condition.get_emoji();
            
            let mut line = format!("{}  {}: {:.1}{} {}", 
                emoji,
                local_time.bold(),
                hour.temperature,
                temp_unit,
                get_temp_bar(hour.temperature, self.config().units == "imperial")
            );
            
            // Add precipitation chance if significant
            if hour.pop > 0.1 {
                let pop_pct = (hour.pop * 100.0) as u8;
                let rain_emoji = if pop_pct > 50 { "🌧️" } else { "💧" };
                line.push_str(&format!(" {} {}%", rain_emoji, pop_pct));
            }
            
            // Add wind if significant
            if hour.wind_speed > 5.0 {
                let wind_dir = get_wind_direction_arrow(hour.wind_direction);
                line.push_str(&format!(" 💨 {}", wind_dir));
            }
            
            println!("{}", line);
            
            if self.animation_enabled && i % 6 == 5 {
                sleep(StdDuration::from_millis(200));
            }
        }
        
        println!();
        Ok(())
    }
    
    /// Display daily forecast
    pub fn show_daily_forecast(&self, forecast: &[DailyForecast], location: &Location) -> Result<()> {
        println!("{}", "╔═══════════════════════════════════════════════════╗".bright_cyan());
        println!("{}", "║              📅 7-DAY FORECAST 📅                 ║".bright_cyan());
        println!("{}", "╚═══════════════════════════════════════════════════╝".bright_cyan());
        println!();
        
        if forecast.is_empty() {
            println!("No daily forecast data available.");
            return Ok(());
        }
        
        let temp_unit = if self.config().units == "imperial" { "°F" } else { "°C" };
        
        for (i, day) in forecast.iter().enumerate() {
            // Format day name
            let day_name = if i == 0 {
                "Today".to_string()
            } else if i == 1 {
                "Tomorrow".to_string()
            } else {
                format_weekday(&day.date)
            };
            
            let emoji = day.main_condition.get_emoji();
            let date_str = format_date_short(&day.date, &location.timezone);
            
            println!("{} {} ({})", day_name.bold(), date_str, emoji);
            
            // Temperature range with visualization
            println!("   🌡️ {}/{}: {:.0}{} / {:.0}{} {}",
                "High".bold(), "Low".bold(),
                day.temp_max, temp_unit,
                day.temp_min, temp_unit,
                get_temp_range_bar(day.temp_min, day.temp_max, self.config().units == "imperial")
            );
            
            // Weather description
            let conditions = if let Some(desc) = day.conditions.first() {
                desc.description.clone()
            } else {
                day.main_condition.to_string()
            };
            
            println!("   ☁️ {}: {}", "Conditions".bold(), conditions.to_title_case());
            
            // Precipitation
            if day.pop > 0.0 {
                let pop_pct = (day.pop * 100.0) as u8;
                println!("   🌧️ {}: {}%", "Precipitation Chance".bold(), pop_pct);
            }
            
            // Wind info
            let wind_unit = if self.config().units == "imperial" { "mph" } else { "m/s" };
            let wind_direction = get_wind_direction_arrow(day.wind_direction);
            println!("   💨 {}: {:.1} {} {}", "Wind".bold(), day.wind_speed, wind_unit, wind_direction);
            
            // UV index
            let uv_display = match day.uv_index as u32 {
                0..=2 => format!("{:.1} (Low)", day.uv_index).green(),
                3..=5 => format!("{:.1} (Moderate)", day.uv_index).yellow(),
                6..=7 => format!("{:.1} (High)", day.uv_index).bright_yellow(),
                8..=10 => format!("{:.1} (Very High)", day.uv_index).bright_red(),
                _ => format!("{:.1} (Extreme)", day.uv_index).red(),
            };
            println!("   ☀️ {}: {}", "UV Index".bold(), uv_display);
            
            if i < forecast.len() - 1 {
                println!("   ------------------------------");
            }
            
            if self.animation_enabled {
                sleep(StdDuration::from_millis(300));
            }
        }
        
        println!();
        Ok(())
    }
    
    /// Display full forecast (combines current, hourly, and daily)
    pub fn show_forecast(&self, forecast: &Forecast, location: &Location) -> Result<()> {
        if let Some(current) = &forecast.current {
            self.show_current_weather(current, location)?;
        }
        
        if !forecast.hourly.is_empty() {
            self.show_hourly_forecast(&forecast.hourly, location)?;
        }
        
        if !forecast.daily.is_empty() {
            self.show_daily_forecast(&forecast.daily, location)?;
        }
        
        Ok(())
    }
    
    /// Display location information
    pub fn show_location_info(&self, location: &Location) -> Result<()> {
        println!("{}", "╔═══════════════════════════════════════════════════╗".bright_cyan());
        println!("{}", "║               📍 LOCATION INFO 📍                 ║".bright_cyan());
        println!("{}", "╚═══════════════════════════════════════════════════╝".bright_cyan());
        println!();
        
        println!("📍 {}: {}", "City".bold(), location.name);
        
        if let Some(region) = &location.region {
            println!("🏙️ {}: {}", "Region".bold(), region);
        }
        
        if let Some(state) = &location.state {
            println!("🗾 {}: {}", "State".bold(), state);
        }
        
        println!("🌎 {}: {} ({})", "Country".bold(), location.country, location.country_code);
        println!("🧭 {}: {:.4}°, {:.4}°", "Coordinates".bold(), location.latitude, location.longitude);
        println!("🕒 {}: {}", "Timezone".bold(), location.timezone);
        
        println!();
        
        if self.animation_enabled {
            sleep(StdDuration::from_millis(800));
        }
        
        Ok(())
    }
    
    /// Show weather recommendations based on conditions
    pub fn show_weather_recommendations(&self, weather: &CurrentWeather) -> Result<()> {
        println!("{}", "╔═══════════════════════════════════════════════════╗".bright_cyan());
        println!("{}", "║              💡 RECOMMENDATIONS 💡                ║".bright_cyan());
        println!("{}", "╚═══════════════════════════════════════════════════╝".bright_cyan());
        println!();
        
        // General recommendation based on temperature
        let _temp = weather.temperature;
        let feels_like = weather.feels_like;
        let is_imperial = self.config().units == "imperial";
        
        // Temperature thresholds (adjusted for units)
        let very_cold = if is_imperial { 32.0 } else { 0.0 };
        let cold = if is_imperial { 50.0 } else { 10.0 };
        let mild = if is_imperial { 68.0 } else { 20.0 };
        let warm = if is_imperial { 77.0 } else { 25.0 };
        let hot = if is_imperial { 86.0 } else { 30.0 };
        
        // Clothing/comfort recommendations
        if feels_like < very_cold {
            println!("🧣 {}", "Very cold! Wear heavy winter clothing, hat, gloves and scarf.".yellow());
        } else if feels_like < cold {
            println!("🧥 {}", "Cold conditions. Wear a warm jacket and layers.".yellow());
        } else if feels_like < mild {
            println!("🧥 {}", "Cool weather. A light jacket or sweater recommended.".bright_blue());
        } else if feels_like < warm {
            println!("👕 {}", "Pleasant temperature. Light clothing should be comfortable.".green());
        } else if feels_like < hot {
            println!("👕 {}", "Warm weather. Light clothing and sun protection advised.".bright_yellow());
        } else {
            println!("🌡️ {}", "Hot weather! Stay hydrated and seek shade during peak hours.".bright_red());
        }
        
        // UV index recommendations
        if weather.uv_index > 5.0 {
            println!("🧴 {}", "High UV levels! Wear sunscreen, hat and sunglasses.".bright_yellow());
        } else if weather.uv_index > 2.0 {
            println!("🧴 {}", "Moderate UV levels. Sun protection advised during peak hours.".yellow());
        }
        
        // Weather-specific recommendations
        match weather.main_condition {
            WeatherCondition::Rain | WeatherCondition::Drizzle => {
                println!("☔ {}", "Rainy conditions. Bring an umbrella or raincoat.".bright_blue());
            },
            WeatherCondition::Thunderstorm => {
                println!("⛈️ {}", "Thunderstorms in the area. Seek shelter and avoid open spaces.".bright_red());
            },
            WeatherCondition::Snow => {
                println!("❄️ {}", "Snowy conditions. Dress warmly and take care on roads.".bright_blue());
            },
            WeatherCondition::Fog | WeatherCondition::Mist => {
                println!("🌫️ {}", "Reduced visibility due to fog. Drive carefully.".yellow());
            },
            WeatherCondition::Clear => {
                if weather.temperature > warm {
                    println!("☀️ {}", "Clear and warm. Great day for outdoor activities!".green());
                } else {
                    println!("☀️ {}", "Clear skies. Enjoy the weather!".green());
                }
            },
            WeatherCondition::Clouds => {
                println!("☁️ {}", "Cloudy conditions. Good for outdoor activities without direct sun.".bright_blue());
            },
            _ => {}
        }
        
        // Wind recommendations
        if weather.wind_speed > 10.0 {
            println!("💨 {}", "Strong winds. Secure loose objects and be careful outdoors.".yellow());
        }
        
        println!();
        Ok(())
    }
    
    /// Show interactive menu
    pub fn show_interactive_menu(&self) -> Result<String> {
        let items = vec![
            "Current Weather",
            "Hourly Forecast",
            "Daily Forecast",
            "Full Weather Report",
            "Change Location",
            "Change Units",
            "Exit",
        ];
        
        let selection = Select::with_theme(&ColorfulTheme::default())
            .with_prompt("Select an option:")
            .default(0)
            .items(&items)
            .interact_on_opt(&self.term)?;
            
        let choice = match selection {
            Some(index) => match index {
                0 => "current",
                1 => "hourly",
                2 => "daily",
                3 => "full",
                4 => "change_location",
                5 => "change_units",
                6 => "exit",
                _ => "exit",
            },
            None => "exit",
        };
        
        Ok(choice.to_string())
    }
    
    /// Prompt for location
    pub fn prompt_for_location(&self) -> Result<String> {
        let location = Input::<String>::with_theme(&ColorfulTheme::default())
            .with_prompt("Enter city name or address")
            .interact_text()?;
            
        Ok(location)
    }
    
    /// Prompt for units
    pub fn prompt_for_units(&self) -> Result<String> {
        let items = vec![
            "Metric (°C, m/s)",
            "Imperial (°F, mph)",
            "Standard (K, m/s)",
        ];
        
        let selection = Select::with_theme(&ColorfulTheme::default())
            .with_prompt("Select units:")
            .default(0)
            .items(&items)
            .interact_on_opt(&self.term)?;
            
        let units = match selection {
            Some(index) => match index {
                0 => "metric",
                1 => "imperial",
                2 => "standard",
                _ => "metric",
            },
            None => "metric",
        };
        
        Ok(units.to_string())
    }
    
    /// Create a custom spinner with cyberpunk style
    pub fn create_spinner(&self, message: &str) -> ProgressBar {
        let pb = ProgressBar::new_spinner();
        pb.set_style(
            ProgressStyle::default_spinner()
                .tick_chars("⠁⠂⠄⡀⢀⠠⠐⠈")
                .template("{spinner:.cyan} {msg}")
                .unwrap(),
        );
        pb.set_message(message.bright_cyan().to_string());
        pb
    }
    
    /// Create a custom spinner for weather icons
    pub fn create_weather_spinner(&self, message: &str) -> ProgressBar {
        let pb = ProgressBar::new_spinner();
        pb.set_style(
            ProgressStyle::default_spinner()
                .tick_chars("☁️🌨️🌧️🌦️🌥️⛅🌤️☀️")
                .template("{spinner} {msg}")
                .unwrap(),
        );
        pb.set_message(message.bright_cyan().to_string());
        pb
    }
    
    // Simplified animation methods - kept for compatibility but don't do anything fancy
    
    /// Placeholder for matrix effect (now disabled)
    pub fn show_matrix_rain_effect(&self) -> Result<()> {
        Ok(())
    }
    
    /// Show text (no pulse effect)
    pub fn show_pulse_text(&self, text: &str, _pulses: usize) -> Result<()> {
        println!("{}", text.bright_cyan());
        Ok(())
    }
    
    /// Show text (no typing animation)
    pub fn typing_animation(&self, text: &str, _speed_factor: u64) -> Result<()> {
        println!("{}", text.bright_cyan());
        Ok(())
    }
}

// Helper functions for formatting

/// Format date to weekday name
fn format_weekday(date: &DateTime<Utc>) -> String {
    match date.weekday() {
        Weekday::Mon => "Monday",
        Weekday::Tue => "Tuesday",
        Weekday::Wed => "Wednesday",
        Weekday::Thu => "Thursday",
        Weekday::Fri => "Friday",
        Weekday::Sat => "Saturday",
        Weekday::Sun => "Sunday",
    }.to_string()
}

/// Format a date to short form
fn format_date_short(date: &DateTime<Utc>, timezone: &str) -> String {
    let local_time = convert_to_local(date, timezone);
    format!("{}/{}", local_time.month(), local_time.day())
}

/// Format a timestamp to local time
fn format_local_time(time: &DateTime<Utc>, timezone: &str) -> String {
    let local_time = convert_to_local(time, timezone);
    format!("{:02}:{:02}", local_time.hour(), local_time.minute())
}

/// Format time to show only hour
fn format_hour_only(time: &DateTime<Utc>, timezone: &str) -> String {
    let local_time = convert_to_local(time, timezone);
    let hour = local_time.hour();
    
    if hour == 0 {
        "12 AM".to_string()
    } else if hour < 12 {
        format!("{} AM", hour)
    } else if hour == 12 {
        "12 PM".to_string()
    } else {
        format!("{} PM", hour - 12)
    }
}

/// Convert UTC time to local time in the specified timezone
fn convert_to_local(time: &DateTime<Utc>, _timezone: &str) -> DateTime<Utc> {
    // This is a simplified version - in a real app, use a proper timezone library
    // For now, we'll just add the offset for demo purposes
    *time
}

/// Get wind direction as an arrow
fn get_wind_direction_arrow(degrees: u16) -> &'static str {
    match degrees {
        337..=360 | 0..=22 => "↓", // N
        23..=67 => "↙",            // NE
        68..=112 => "←",           // E
        113..=157 => "↖",          // SE
        158..=202 => "↑",          // S
        203..=247 => "↗",          // SW
        248..=292 => "→",          // W
        293..=336 => "↘",          // NW
        _ => "•",
    }
}

/// Create a temperature bar visualization
fn get_temp_bar(temp: f64, is_imperial: bool) -> ColoredString {
    let (very_cold, cold, mild, warm, hot, very_hot) = if is_imperial {
        (32.0, 50.0, 68.0, 77.0, 86.0, 95.0)
    } else {
        (0.0, 10.0, 20.0, 25.0, 30.0, 35.0)
    };
    
    let bar = match temp {
        t if t < very_cold => "▁▁▁▁▁▁▁▁▁▁",
        t if t < cold => "▁▁▁▁▁▁▁▁▁▁",
        t if t < mild => "▁▁▁▁▁▁▁▁▁▁",
        t if t < warm => "▁▁▁▁▁▁▁▁▁▁",
        t if t < hot => "▁▁▁▁▁▁▁▁▁▁",
        t if t < very_hot => "▁▁▁▁▁▁▁▁▁▁",
        _ => "▁▁▁▁▁▁▁▁▁▁",
    };
    
    match temp {
        t if t < very_cold => bar.bright_blue(),
        t if t < cold => bar.blue(),
        t if t < mild => bar.cyan(),
        t if t < warm => bar.green(),
        t if t < hot => bar.yellow(),
        t if t < very_hot => bar.bright_red(),
        _ => bar.red(),
    }
}

/// Create a temperature range bar
fn get_temp_range_bar(min: f64, max: f64, is_imperial: bool) -> ColoredString {
    let range = "────────────";
    
    let (very_cold, cold, mild, _warm, hot) = if is_imperial {
        (32.0, 50.0, 68.0, 77.0, 86.0)
    } else {
        (0.0, 10.0, 20.0, 25.0, 30.0)
    };
    
    if max < very_cold {
        range.bright_blue()
    } else if max < cold {
        range.blue()
    } else if min > hot {
        range.red()
    } else if min > mild {
        range.yellow()
    } else if max > mild {
        range.green()
    } else {
        range.cyan()
    }
}

/// String extension to make title case conversions easier
trait TitleCase {
    fn to_title_case(&self) -> String;
}

impl TitleCase for String {
    fn to_title_case(&self) -> String {
        let mut result = String::new();
        let mut capitalize_next = true;
        
        for c in self.chars() {
            if c.is_whitespace() || c == '-' {
                capitalize_next = true;
                result.push(c);
            } else if capitalize_next {
                result.push(c.to_uppercase().next().unwrap_or(c));
                capitalize_next = false;
            } else {
                result.push(c);
            }
        }
        
        result
    }
}

impl TitleCase for str {
    fn to_title_case(&self) -> String {
        self.to_string().to_title_case()
    }
}

impl WeatherUI {
    /// Get configuration for the UI
    fn config(&self) -> WeatherConfig {
        WeatherConfig {
            units: "metric".to_string(),
            location: None,
            json_output: self.json_output,
            animation_enabled: self.animation_enabled,
            detail_level: crate::modules::types::DetailLevel::Standard,
        }
    }
}