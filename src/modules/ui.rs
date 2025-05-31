use anyhow::Result;
use chrono::{DateTime, Datelike, Timelike, Utc, Weekday};
use colored::*;
use console::Term;
use dialoguer::{theme::ColorfulTheme, Input, Select};

use std::thread::sleep;
use std::time::Duration as StdDuration;

use crate::modules::types::{
    CurrentWeather, DailyForecast, Forecast, HourlyForecast, Location, WeatherCondition,
    WeatherConfig,
};
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
    pub fn show_current_weather(
        &self,
        weather: &CurrentWeather,
        location: &Location,
    ) -> Result<()> {
        println!(
            "{}",
            "╔═══════════════════════════════════════════════════╗".bright_cyan()
        );
        println!(
            "{}",
            "║               🌡️ CURRENT CONDITIONS 🌡️              ║".bright_cyan()
        );
        println!(
            "{}",
            "╚═══════════════════════════════════════════════════╝".bright_cyan()
        );
        println!();

        if self.animation_enabled {
            sleep(StdDuration::from_millis(300));
        }

        // Format local time based on location's timezone
        let local_time = format_local_time(&weather.timestamp, &location.timezone);

        // Get the main weather information
        let emoji = weather.main_condition.get_emoji();
        let conditions = if let Some(desc) = weather.conditions.first() {
            desc.description.to_title_case()
        } else {
            weather.main_condition.to_string()
        };

        // Format temperatures based on units
        let temp_unit = if self.config().units == "imperial" {
            "°F"
        } else {
            "°C"
        };

        // Location and time
        println!("📍 {}: {}, {}", "Location".bold(), location.name, location.country);
        println!("🕓 {}: {} ({})", "Local Time".bold(), local_time, location.timezone);
        println!();
        
        // Main weather display
        println!("{} {}: {}", emoji, "Conditions".bold(), conditions);
        
        println!("🌡️ {}: {:.1}{} (Feels like: {:.1}{})",
            "Temperature".bold(),
            weather.temperature,
            temp_unit,
            weather.feels_like,
            temp_unit
        );

        if self.animation_enabled {
            sleep(StdDuration::from_millis(300));
        }

        // Wind info
        let wind_unit = if self.config().units == "imperial" {
            "mph"
        } else {
            "m/s"
        };
        let wind_direction = get_wind_direction_arrow(weather.wind_direction);
        println!("💨 {}: {:.1} {} {}", 
            "Wind".bold(),
            weather.wind_speed,
            wind_unit,
            wind_direction
        );

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
            println!("🌧️ {}: {:.1} mm (last hour)", "Rain".bold(), rain);
        }

        if let Some(snow) = weather.snow_last_hour {
            println!("❄️ {}: {:.1} mm (last hour)", "Snow".bold(), snow);
        }

        println!();

        Ok(())
    }

    /// Display hourly forecast
    pub fn show_hourly_forecast(
        &self,
        forecast: &[HourlyForecast],
        location: &Location,
    ) -> Result<()> {
        println!(
            "{}",
            "╔═══════════════════════════════════════════════════╗".bright_cyan()
        );
        println!(
            "{}",
            "║             🕓 HOURLY FORECAST (24h) 🕓            ║".bright_cyan()
        );
        println!(
            "{}",
            "╚═══════════════════════════════════════════════════╝".bright_cyan()
        );
        println!();

        if forecast.is_empty() {
            println!("No hourly forecast data available.");
            return Ok(());
        }

        // Limit to next 24 hours for display
        let hours_to_show = std::cmp::min(forecast.len(), 24);
        let temp_unit = if self.config().units == "imperial" {
            "°F"
        } else {
            "°C"
        };

        // Get current hour for highlighting
        let now = Utc::now();
        let current_hour = now.hour();
        
        // Print table header
        println!("┌────────┬───────────┬────────┬─────────┬────────┬─────────┐");
        println!("│  Hour  │  Weather  │  Temp  │  Precip │  Wind  │ Humidity│");
        println!("├────────┼───────────┼────────┼─────────┼────────┼─────────┤");

        for (i, hour) in forecast.iter().take(hours_to_show).enumerate() {
            // Convert to local time
            let hour_dt = convert_to_local(&hour.timestamp, &location.timezone);
            let hour_num = hour_dt.hour();
            let local_time = format_hour_only(&hour.timestamp, &location.timezone);
            let emoji = hour.main_condition.get_emoji();
            
            // Format conditions description
            let conditions = if let Some(desc) = hour.conditions.first() {
                desc.description.to_title_case()[..std::cmp::min(8, desc.description.len())].to_string()
            } else {
                hour.main_condition.to_string()
            };

            // Precipitation percentage
            let precip = if hour.pop > 0.0 {
                format!("{}%", (hour.pop * 100.0) as u8)
            } else {
                "0%".to_string()
            };
            
            // Wind information
            let wind_info = if hour.wind_speed > 0.0 {
                let wind_dir = get_wind_direction_arrow(hour.wind_direction);
                format!("{:.1} {}", hour.wind_speed, wind_dir)
            } else {
                "Calm".to_string()
            };
            
            // Highlight current hour
            let line = if hour_num == current_hour {
                format!(
                    "│{:^8}│ {:<2} {:<7} │ {:.1}{:<3} │ {:<7} │ {:<6} │ {:<7} │",
                    local_time.bold(),
                    emoji,
                    conditions,
                    hour.temperature,
                    temp_unit,
                    precip,
                    wind_info,
                    format!("{}%", hour.humidity)
                ).bright_yellow()
            } else {
                format!(
                    "│{:^8}│ {:<2} {:<7} │ {:.1}{:<3} │ {:<7} │ {:<6} │ {:<7} │",
                    local_time,
                    emoji,
                    conditions,
                    hour.temperature,
                    temp_unit,
                    precip,
                    wind_info,
                    format!("{}%", hour.humidity)
                ).normal()
            };

            println!("{}", line);

            if self.animation_enabled && i % 6 == 5 {
                sleep(StdDuration::from_millis(200));
            }
        }

        println!("└────────┴───────────┴────────┴─────────┴────────┴─────────┘");
        println!();
        Ok(())
    }

    /// Display daily forecast
    pub fn show_daily_forecast(
        &self,
        forecast: &[DailyForecast],
        location: &Location,
    ) -> Result<()> {
        println!(
            "{}",
            "╔═══════════════════════════════════════════════════╗".bright_cyan()
        );
        println!(
            "{}",
            "║              📅 7-DAY FORECAST 📅                 ║".bright_cyan()
        );
        println!(
            "{}",
            "╚═══════════════════════════════════════════════════╝".bright_cyan()
        );
        println!();

        if forecast.is_empty() {
            println!("No daily forecast data available.");
            return Ok(());
        }

        let temp_unit = if self.config().units == "imperial" {
            "°F"
        } else {
            "°C"
        };
        
        // Next Days Forecast - Enhanced visualization
        println!("{}", "📊 NEXT DAYS AT A GLANCE".bold().bright_cyan());
        println!();
        
        // Display forecast information in a clean format
        for (i, day) in forecast.iter().enumerate().take(7) {
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
            
            // Format temperatures
            let temp_high = format!("{:.0}{}", day.temp_max, temp_unit);
            let temp_low = format!("{:.0}{}", day.temp_min, temp_unit);
            
            // Precipitation percentage
            let precip = if day.pop > 0.0 {
                format!("{}%", (day.pop * 100.0) as u8)
            } else {
                "0%".to_string()
            };
            
            // Format humidity
            let humidity = format!("{}%", day.humidity);
            
            // Print day header
            println!("{}", "─".repeat(50));
            
            // Print forecast with color highlighting based on conditions
            let day_header = format!("{} {} {}", day_name.bold(), date_str, emoji);
            println!("{}", day_header);
            
            // Get weather description
            let weather_desc = if let Some(desc) = day.conditions.first() {
                desc.description.to_title_case()
            } else {
                day.main_condition.to_string()
            };
            
            // Print details in a clean format
            match day.main_condition {
                WeatherCondition::Rain | WeatherCondition::Drizzle | WeatherCondition::Thunderstorm => {
                    println!("  Weather: {}", weather_desc);
                    println!("  Temp: {} / {}", temp_high, temp_low);
                    println!("  Precipitation: {}", precip.bright_blue());
                    println!("  Humidity: {}", humidity);
                },
                WeatherCondition::Clear => {
                    println!("  Weather: {}", weather_desc);
                    println!("  Temp: {} / {}", temp_high.bright_yellow(), temp_low);
                    println!("  Precipitation: {}", precip);
                    println!("  Humidity: {}", humidity);
                },
                _ => {
                    println!("  Weather: {}", weather_desc);
                    println!("  Temp: {} / {}", temp_high, temp_low);
                    println!("  Precipitation: {}", precip);
                    println!("  Humidity: {}", humidity);
                }
            }
        }
        println!();
        
        // Add temperature summary and activity forecast
        println!("{}", "📈 TEMPERATURE TRENDS & ACTIVITIES".bold().bright_cyan());
        println!();
        
        // Print temperature trends in a simple format
        println!("  TEMPERATURE OUTLOOK:");
        for (i, day) in forecast.iter().enumerate().take(7) {
            let label = if i == 0 {
                "Today".to_string()
            } else if i == 1 {
                "Tomorrow".to_string()
            } else {
                let weekday = format_weekday(&day.date);
                format!("{} {}/{}", &weekday[..3], day.date.month(), day.date.day())
            };
            
            // Create a simple visual indicator
            let temp_indicator = if day.temp_max > 28.0 {
                "🔥 Hot".bright_red()
            } else if day.temp_max > 22.0 {
                "☀️ Warm".bright_yellow()
            } else if day.temp_max > 15.0 {
                "😎 Mild".green()
            } else if day.temp_max > 5.0 {
                "❄️ Cool".bright_blue()
            } else {
                "❄️ Cold".blue()
            };
            
            println!("  • {:<12} {:<10} {:.0}{} / {:.0}{}", 
                     label, temp_indicator, day.temp_max, temp_unit, day.temp_min, temp_unit);
        }
        println!();
        
        // Add activity recommendations in a simpler format
        println!("{}", "🎯 BEST ACTIVITIES FOR UPCOMING DAYS".bold().bright_cyan());
        println!();
        
        // Simplified activity recommendations for next 3 days
        for (i, day) in forecast.iter().enumerate().take(3) {
            let day_name = if i == 0 {
                "TODAY".to_string()
            } else if i == 1 {
                "TOMORROW".to_string()
            } else {
                format_weekday(&day.date).to_uppercase()
            };
            
            println!("  {} ({})", day_name.bold(), day.main_condition.get_emoji());
            
            // Best activities based on weather
            let temp_avg = (day.temp_max + day.temp_min) / 2.0;
            let is_rainy = matches!(day.main_condition, WeatherCondition::Rain | WeatherCondition::Drizzle | WeatherCondition::Thunderstorm);
            let is_clear = matches!(day.main_condition, WeatherCondition::Clear);
            
            // Recommended activities
            println!("  Best for:");
            
            if is_rainy {
                println!("  • Indoor: 👍 Museums, movies, shopping, home activities");
                println!("  • Outdoor: 👎 Not recommended");
            } else if is_clear && temp_avg > 25.0 {
                println!("  • Outdoor: 👍 Beach, parks, hiking, outdoor dining");
                println!("  • Sports: 👍 Swimming, cycling, team sports");
            } else if is_clear {
                println!("  • Outdoor: 👍 Hiking, sightseeing, parks");
                println!("  • Sports: 👍 Running, cycling, team sports");
            } else {
                println!("  • Outdoor: 👍 Walking, urban exploration, photography");
                println!("  • Indoor/Outdoor: 👍 Shopping, museums, casual dining");
            }
            
            println!();
        }
        
        // Show detailed view for today and tomorrow
        println!("{}", "🔍 DETAILED FORECAST:".bold().bright_cyan());
        println!();
        
        // Show expanded information for next 5 days
        for (i, day) in forecast.iter().enumerate().take(5) {
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

            // Create a header box for each day
            println!("┌───────────────────────────────────────────────────┐");
            println!(
                "│ {} {} ({}){}│",
                day_name.bold().bright_cyan(),
                date_str,
                emoji,
                " ".repeat(43 - day_name.len() - date_str.len() - 4)
            );
            println!("└───────────────────────────────────────────────────┘");

            // Temperature range with visualization
            println!(
                "   🌡️ {}/{}: {:.0}{} / {:.0}{} {}",
                "High".bold(),
                "Low".bold(),
                day.temp_max,
                temp_unit,
                day.temp_min,
                temp_unit,
                get_temp_range_bar(
                    day.temp_min,
                    day.temp_max,
                    self.config().units == "imperial"
                )
            );

            // Weather description
            let conditions = if let Some(desc) = day.conditions.first() {
                desc.description.clone()
            } else {
                day.main_condition.to_string()
            };

            println!(
                "   ☁️ {}: {}",
                "Conditions".bold(),
                conditions.to_title_case()
            );

            // Sunrise and sunset
            let sunrise = format_local_time(&day.sunrise, &location.timezone);
            let sunset = format_local_time(&day.sunset, &location.timezone);
            println!("   🌅 {}: {}", "Sunrise".bold(), sunrise);
            println!("   🌇 {}: {}", "Sunset".bold(), sunset);

            // Precipitation
            if day.pop > 0.0 {
                let pop_pct = (day.pop * 100.0) as u8;
                let rain_icon = match pop_pct {
                    0..=20 => "🌂",
                    21..=50 => "💧",
                    51..=70 => "💦",
                    71..=90 => "🌧️",
                    _ => "⛈️",
                };
                println!("   {} {}: {}%", rain_icon, "Precipitation Chance".bold(), pop_pct);
            }

            // Wind info
            let wind_unit = if self.config().units == "imperial" {
                "mph"
            } else {
                "m/s"
            };
            let wind_direction = get_wind_direction_arrow(day.wind_direction);
            println!(
                "   💨 {}: {:.1} {} {}",
                "Wind".bold(),
                day.wind_speed,
                wind_unit,
                wind_direction
            );

            // Humidity info
            println!(
                "   💧 {}: {}%",
                "Humidity".bold(),
                day.humidity
            );

            // UV index
            let uv_display = match day.uv_index as u32 {
                0..=2 => format!("{:.1} (Low)", day.uv_index).green(),
                3..=5 => format!("{:.1} (Moderate)", day.uv_index).yellow(),
                6..=7 => format!("{:.1} (High)", day.uv_index).bright_yellow(),
                8..=10 => format!("{:.1} (Very High)", day.uv_index).bright_red(),
                _ => format!("{:.1} (Extreme)", day.uv_index).red(),
            };
            println!("   ☀️ {}: {}", "UV Index".bold(), uv_display);
            
            // Daily recommendations based on conditions
            let temp_avg = (day.temp_max + day.temp_min) / 2.0;
            
            // Activity recommendations based on weather and temperature
            println!("   🔮 {}: ", "Outlook".bold());
            
            match day.main_condition {
                WeatherCondition::Rain | WeatherCondition::Drizzle => {
                    if day.pop > 0.7 {
                        println!("      ☔ {}", "Heavy rain expected. Plan for indoor activities.".bright_blue());
                        println!("      🏠 {}", "Recommended: Movies, museums, shopping, or home cooking.".bright_blue());
                    } else {
                        println!("      ☔ {}", "Light rain expected. Bring an umbrella if going out.".bright_blue());
                        println!("      🏠 {}", "Recommended: Quick errands, covered venues, or indoor sports.".bright_blue());
                    }
                },
                WeatherCondition::Thunderstorm => {
                    println!("      ⛈️ {}", "Thunderstorms expected. Stay safe indoors.".bright_red());
                    println!("      ⚠️ {}", "Not recommended: Any outdoor activities or travel if avoidable.".bright_red());
                    println!("      🏠 {}", "Recommended: Home activities, reading, cooking, or gaming.".bright_red());
                },
                WeatherCondition::Snow => {
                    println!("      ❄️ {}", "Snowy conditions. Prepare for potential travel disruptions.".bright_blue());
                    println!("      ⚠️ {}", "Not recommended: Long trips or driving if inexperienced on snow.".bright_blue());
                    println!("      🏂 {}", "Recommended: Snow sports if conditions permit, or cozy indoor activities.".bright_blue());
                },
                WeatherCondition::Clear => {
                    if temp_avg > 25.0 {
                        println!("      ☀️ {}", "Clear and warm! Perfect for outdoor activities.".green());
                        println!("      🏊 {}", "Recommended: Swimming, beach visits, park outings, or outdoor dining.".green());
                    } else if temp_avg < 10.0 {
                        println!("      ☀️ {}", "Clear but cool. Good for active outdoor activities.".green());
                        println!("      🏃 {}", "Recommended: Hiking, running, cycling, or sightseeing with warm clothing.".green());
                    } else {
                        println!("      ☀️ {}", "Perfect weather conditions. Ideal for almost any outdoor activity.".green());
                        println!("      🌳 {}", "Recommended: Parks, hiking, cycling, outdoor sports, or dining al fresco.".green());
                    }
                },
                WeatherCondition::Clouds => {
                    println!("      ☁️ {}", "Cloudy but pleasant. Good for outdoor activities without direct sun.".bright_blue());
                    println!("      🚶 {}", "Recommended: Walking tours, shopping districts, light hikes, or photography.".bright_blue());
                },
                WeatherCondition::Fog | WeatherCondition::Mist => {
                    println!("      🌫️ {}", "Foggy conditions. Be cautious while driving or in unfamiliar areas.".yellow());
                    println!("      ⚠️ {}", "Not recommended: Activities requiring good visibility or long drives.".yellow());
                    println!("      🏙️ {}", "Recommended: City exploration, museums, or atmospheric photography.".yellow());
                },
                _ => {
                    println!("      📋 {}", "Check local forecasts for specific activity recommendations.".normal());
                }
            }
            
            // UV index specific advice
            if day.uv_index > 7.0 {
                println!("      🧴 {}", "Very high UV index! Sunscreen and protective clothing essential.".bright_yellow());
            } else if day.uv_index > 5.0 {
                println!("      🧴 {}", "High UV index. Wear sunscreen and seek shade during midday hours.".yellow());
            }

            println!();

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
        println!(
            "{}",
            "╔═══════════════════════════════════════════════════╗".bright_cyan()
        );
        println!(
            "{}",
            "║               📍 LOCATION INFO 📍                 ║".bright_cyan()
        );
        println!(
            "{}",
            "╚═══════════════════════════════════════════════════╝".bright_cyan()
        );
        println!();

        println!("📍 {}: {}", "City".bold(), location.name);

        if let Some(region) = &location.region {
            println!("🏙️ {}: {}", "Region".bold(), region);
        }

        if let Some(state) = &location.state {
            println!("🗾 {}: {}", "State".bold(), state);
        }

        println!(
            "🌎 {}: {} ({})",
            "Country".bold(),
            location.country,
            location.country_code
        );
        println!(
            "🧭 {}: {:.4}°, {:.4}°",
            "Coordinates".bold(),
            location.latitude,
            location.longitude
        );
        println!("🕒 {}: {}", "Timezone".bold(), location.timezone);

        println!();

        if self.animation_enabled {
            sleep(StdDuration::from_millis(800));
        }

        Ok(())
    }

    /// Show weather recommendations based on conditions
    pub fn show_weather_recommendations(&self, weather: &CurrentWeather) -> Result<()> {
        println!(
            "{}",
            "╔═══════════════════════════════════════════════════╗".bright_cyan()
        );
        println!(
            "{}",
            "║              💡 RECOMMENDATIONS 💡                ║".bright_cyan()
        );
        println!(
            "{}",
            "╚═══════════════════════════════════════════════════╝".bright_cyan()
        );
        println!();

        // Get the current hour to determine time of day
        let now = Utc::now();
        let hour = now.hour();
        
        // Define time periods
        let is_morning = hour >= 5 && hour < 12;
        let is_afternoon = hour >= 12 && hour < 17;
        let is_evening = hour >= 17 && hour < 21;
        let is_night = hour >= 21 || hour < 5;
        
        let time_of_day = if is_morning {
            "morning"
        } else if is_afternoon {
            "afternoon"
        } else if is_evening {
            "evening"
        } else {
            "night"
        };

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

        // Clothing/comfort recommendations based on time of day and temperature
        if feels_like < very_cold {
            println!(
                "🧣 {}",
                format!("Very cold {}! Wear heavy winter clothing, hat, gloves and scarf.", time_of_day).yellow()
            );
        } else if feels_like < cold {
            println!(
                "🧥 {}",
                format!("Cold {} conditions. Wear a warm jacket and layers.", time_of_day).yellow()
            );
        } else if feels_like < mild {
            println!(
                "🧥 {}",
                format!("Cool {} weather. A light jacket or sweater recommended.", time_of_day).bright_blue()
            );
        } else if feels_like < warm {
            println!(
                "👕 {}",
                format!("Pleasant {} temperature. Light clothing should be comfortable.", time_of_day).green()
            );
        } else if feels_like < hot {
            println!(
                "👕 {}",
                format!("Warm {} weather. Light clothing and sun protection advised.", time_of_day).bright_yellow()
            );
        } else {
            println!(
                "🌡️ {}",
                format!("Hot {} weather! Stay hydrated and seek shade.", time_of_day).bright_red()
            );
        }

        // UV index recommendations - only relevant during daylight hours
        if !is_night {
            if weather.uv_index > 5.0 {
                println!(
                    "🧴 {}",
                    "High UV levels! Wear sunscreen, hat and sunglasses.".bright_yellow()
                );
            } else if weather.uv_index > 2.0 {
                println!(
                    "🧴 {}",
                    "Moderate UV levels. Sun protection advised.".yellow()
                );
            }
        }

        // Weather-specific recommendations adjusted for time of day
        match weather.main_condition {
            WeatherCondition::Rain | WeatherCondition::Drizzle => {
                println!(
                    "☔ {}",
                    format!("Rainy {} conditions. Bring an umbrella or raincoat.", time_of_day).bright_blue()
                );
            }
            WeatherCondition::Thunderstorm => {
                println!(
                    "⛈️ {}",
                    format!("Thunderstorms in the area this {}. Seek shelter and avoid open spaces.", time_of_day).bright_red()
                );
            }
            WeatherCondition::Snow => {
                println!(
                    "❄️ {}",
                    format!("Snowy {} conditions. Dress warmly and take care on roads.", time_of_day).bright_blue()
                );
            }
            WeatherCondition::Fog | WeatherCondition::Mist => {
                if is_night || is_evening {
                    println!(
                        "🌫️ {}",
                        "Reduced visibility due to fog in the dark. Drive very carefully.".yellow()
                    );
                } else {
                    println!(
                        "🌫️ {}",
                        "Reduced visibility due to fog. Drive carefully.".yellow()
                    );
                }
            }
            WeatherCondition::Clear => {
                if is_night {
                    println!("🌙 {}", "Clear night sky. Great for stargazing!".bright_blue());
                } else if weather.temperature > warm {
                    println!(
                        "☀️ {}",
                        format!("Clear and warm {}. Great for outdoor activities!", time_of_day).green()
                    );
                } else {
                    println!("☀️ {}", format!("Clear {} skies. Enjoy the weather!", time_of_day).green());
                }
            }
            WeatherCondition::Clouds => {
                if is_night {
                    println!(
                        "☁️ {}",
                        "Cloudy night. No stargazing tonight.".bright_blue()
                    );
                } else {
                    println!(
                        "☁️ {}",
                        format!("Cloudy {} conditions. Good for outdoor activities without direct sun.", time_of_day).bright_blue()
                    );
                }
            }
            _ => {}
        }

        // Wind recommendations
        if weather.wind_speed > 10.0 {
            println!(
                "💨 {}",
                format!("Strong winds this {}. Secure loose objects and be careful outdoors.", time_of_day).yellow()
            );
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
    }
    .to_string()
}

/// Format a date to short form
fn format_date_short(date: &DateTime<Utc>, timezone: &str) -> String {
    let local_time = convert_to_local(date, timezone);
    format!("{}/{}", local_time.month(), local_time.day())
}

// Removed unused function

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
fn convert_to_local(time: &DateTime<Utc>, timezone: &str) -> DateTime<Utc> {
    // This is a simplified version - in a real app, use a proper timezone library
    // For now, we'll parse the timezone offset from the timezone string
    let hours_offset = match timezone {
        // Common US timezones
        "America/New_York" | "EST" | "EDT" => -5,
        "America/Chicago" | "CST" | "CDT" => -6,
        "America/Denver" | "MST" | "MDT" => -7,
        "America/Los_Angeles" | "PST" | "PDT" => -8,
        "America/Anchorage" | "AKST" | "AKDT" => -9,
        "Pacific/Honolulu" | "HST" => -10,
        // European timezones
        "Europe/London" | "GMT" | "BST" => 0,
        "Europe/Paris" | "Europe/Berlin" | "Europe/Rome" | "CET" | "CEST" => 1,
        "Europe/Athens" | "Europe/Istanbul" | "EET" | "EEST" => 2,
        // Asian timezones
        "Asia/Dubai" => 4,
        "Asia/Kolkata" | "IST" => 5,
        "Asia/Shanghai" | "Asia/Singapore" => 8,
        "Asia/Tokyo" | "JST" => 9,
        // Australian timezones
        "Australia/Sydney" | "AEST" | "AEDT" => 10,
        // Default to UTC if timezone is unknown
        _ => 0,
    };
    
    *time + chrono::Duration::hours(hours_offset)
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
// Function has been removed as it's no longer used

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
