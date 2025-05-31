use chrono::{DateTime, TimeZone, Utc};
use std::io::{self, Write};
use std::str::FromStr;
use std::time::Duration;
use colored::Colorize;
use rand::Rng;

/// Convert a timestamp (in seconds) to a DateTime
pub fn timestamp_to_datetime(timestamp: i64) -> DateTime<Utc> {
    Utc.timestamp_opt(timestamp, 0).single().unwrap_or_else(|| Utc::now())
}

/// Convert a datetime to a human-readable string
pub fn format_datetime(datetime: &DateTime<Utc>, format: &str) -> String {
    datetime.format(format).to_string()
}

/// Convert temperature between units
pub fn convert_temperature(value: f64, from: &str, to: &str) -> f64 {
    match (from, to) {
        ("metric", "imperial") => (value * 9.0/5.0) + 32.0,  // C to F
        ("imperial", "metric") => (value - 32.0) * 5.0/9.0,  // F to C
        ("metric", "standard") => value + 273.15,            // C to K
        ("standard", "metric") => value - 273.15,            // K to C
        ("imperial", "standard") => ((value - 32.0) * 5.0/9.0) + 273.15, // F to K
        ("standard", "imperial") => ((value - 273.15) * 9.0/5.0) + 32.0, // K to F
        _ => value, // Same unit or unknown conversion
    }
}

/// Format wind speed with appropriate units
pub fn format_wind_speed(speed: f64, units: &str) -> String {
    match units {
        "imperial" => format!("{:.1} mph", speed),
        "metric" => format!("{:.1} m/s", speed),
        _ => format!("{:.1} m/s", speed),
    }
}

/// Get temperature unit symbol based on units setting
pub fn get_temp_unit(units: &str) -> &'static str {
    match units {
        "imperial" => "°F",
        "metric" => "°C",
        "standard" => "K",
        _ => "°C",
    }
}

/// Parse a string to a given type, with a default value
pub fn parse_with_default<T: FromStr>(s: &str, default: T) -> T {
    s.parse().unwrap_or(default)
}

/// Truncate a string if it's longer than max_len
pub fn truncate_string(s: &str, max_len: usize) -> String {
    if s.len() <= max_len {
        s.to_string()
    } else {
        format!("{}...", &s[..max_len-3])
    }
}

/// Get a visual representation of weather condition
pub fn get_weather_ascii_art(condition: &str, is_day: bool) -> String {
    match condition.to_lowercase().as_str() {
        "clear" if is_day => "   \\   /\n    .--.\n   /    \\\n".bright_yellow().to_string(),
        "clear" => "     *  \n  *  .--.\n     /  \\\n".bright_blue().to_string(),
        "clouds" | "cloudy" => "   .--.  \n .-(    ).\n(___.___)\n".cyan().to_string(),
        "rain" | "rainy" => "  .--. \n (___) \n  |||  \n  |||  \n".bright_blue().to_string(),
        "drizzle" => "  .--. \n (___) \n  | |  \n  | |  \n".blue().to_string(),
        "thunderstorm" => r"  .--. \n (___) \n  |||  \n  /|\\\  \n".yellow().to_string(),
        "snow" | "snowy" => "   .--. \n  (___). \n  *  *  * \n * *  * * \n".white().to_string(),
        "mist" | "fog" => " _ - _ - \n  _ - _  \n _ - _   \n".cyan().to_string(),
        _ => "   ?   \n  ???  \n   ?   \n".to_string(),
    }
}

/// Create a visualization bar for values
pub fn create_visualization_bar(value: f64, max: f64, width: usize) -> String {
    let filled_width = ((value / max) * width as f64).round() as usize;
    let filled_width = filled_width.min(width); // Ensure we don't exceed width
    
    let filled = "█".repeat(filled_width);
    let empty = "▒".repeat(width - filled_width);
    
    format!("{}{}", filled, empty)
}

/// Create a colored temperature visualization
pub fn colored_temp_bar(temp: f64, units: &str, width: usize) -> String {
    // Reference values for coloring (adjusted based on units)
    let (cold, cool, mild, warm, hot) = match units {
        "imperial" => (32.0, 50.0, 70.0, 85.0, 95.0),
        "metric" => (0.0, 10.0, 20.0, 30.0, 35.0),
        "standard" => (273.15, 283.15, 293.15, 303.15, 308.15),
        _ => (0.0, 10.0, 20.0, 30.0, 35.0), // Default to metric
    };
    
    let filled_width = width.min(10);
    let bar = "■".repeat(filled_width);
    
    if temp <= cold {
        bar.bright_blue().to_string()
    } else if temp <= cool {
        bar.blue().to_string()
    } else if temp <= mild {
        bar.green().to_string()
    } else if temp <= warm {
        bar.yellow().to_string()
    } else if temp <= hot {
        bar.bright_red().to_string()
    } else {
        bar.red().to_string()
    }
}

/// Create a progress spinner with the given message
pub fn spinner_with_message(message: &str, duration_ms: u64) {
    let spinner_chars = ['⠋', '⠙', '⠹', '⠸', '⠼', '⠴', '⠦', '⠧', '⠇', '⠏'];
    let steps = duration_ms / 100; // Update spinner every 100ms
    
    for i in 0..steps {
        let char_idx = (i as usize) % spinner_chars.len();
        print!("\r{} {}", spinner_chars[char_idx].to_string().cyan(), message);
        io::stdout().flush().unwrap();
        std::thread::sleep(Duration::from_millis(100));
    }
    
    println!("\r✓ {}", message.green());
}

/// Generate random bytes for testing
pub fn generate_random_bytes(size: usize) -> Vec<u8> {
    let mut rng = rand::thread_rng();
    let mut bytes = vec![0u8; size];
    for byte in bytes.iter_mut() {
        *byte = rng.gen();
    }
    bytes
}

/// Convert coordinates to a cardinal direction (N, NE, E, etc.)
pub fn degrees_to_direction(degrees: f64) -> &'static str {
    let directions = [
        "N", "NNE", "NE", "ENE", 
        "E", "ESE", "SE", "SSE", 
        "S", "SSW", "SW", "WSW", 
        "W", "WNW", "NW", "NNW"
    ];
    
    // Normalize degrees to 0-360 range and calculate the index
    // Adding 11.25 shifts the boundaries to align with direction ranges
    let normalized_degrees = degrees % 360.0;
    let index = ((normalized_degrees + 11.25) % 360.0 / 22.5) as usize;
    directions[index]
}

/// Format a unix timestamp to a readable time
pub fn format_unix_time(timestamp: i64, format: &str) -> String {
    let dt = Utc.timestamp_opt(timestamp, 0).unwrap();
    dt.format(format).to_string()
}

/// Get emoji for UV index
pub fn uv_index_emoji(uv: f64) -> &'static str {
    match uv as u32 {
        0..=2 => "✅", // Low
        3..=5 => "⚠️", // Moderate
        6..=7 => "🟠", // High
        8..=10 => "🔴", // Very High
        _ => "☣️", // Extreme
    }
}