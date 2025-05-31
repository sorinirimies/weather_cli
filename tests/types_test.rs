use weather_cli::modules::types::{DetailLevel, Location, WeatherCondition, WeatherConfig};

#[test]
fn test_weather_condition_from_str() {
    assert_eq!(WeatherCondition::from_str("clear"), WeatherCondition::Clear);
    assert_eq!(
        WeatherCondition::from_str("clouds"),
        WeatherCondition::Clouds
    );
    assert_eq!(WeatherCondition::from_str("rain"), WeatherCondition::Rain);
    assert_eq!(
        WeatherCondition::from_str("drizzle"),
        WeatherCondition::Drizzle
    );
    assert_eq!(
        WeatherCondition::from_str("thunderstorm"),
        WeatherCondition::Thunderstorm
    );
    assert_eq!(WeatherCondition::from_str("snow"), WeatherCondition::Snow);
    assert_eq!(WeatherCondition::from_str("mist"), WeatherCondition::Mist);
    assert_eq!(WeatherCondition::from_str("fog"), WeatherCondition::Fog);
    assert_eq!(WeatherCondition::from_str("smoke"), WeatherCondition::Smoke);
    assert_eq!(WeatherCondition::from_str("haze"), WeatherCondition::Haze);
    assert_eq!(WeatherCondition::from_str("dust"), WeatherCondition::Dust);
    assert_eq!(WeatherCondition::from_str("sand"), WeatherCondition::Sand);
    assert_eq!(WeatherCondition::from_str("ash"), WeatherCondition::Ash);
    assert_eq!(
        WeatherCondition::from_str("squall"),
        WeatherCondition::Squall
    );
    assert_eq!(
        WeatherCondition::from_str("tornado"),
        WeatherCondition::Tornado
    );
    assert_eq!(
        WeatherCondition::from_str("unknown"),
        WeatherCondition::Unknown
    );
    assert_eq!(
        WeatherCondition::from_str("nonexistent"),
        WeatherCondition::Unknown
    );
}

#[test]
fn test_weather_condition_get_emoji() {
    assert_eq!(WeatherCondition::Clear.get_emoji(), "☀️");
    assert_eq!(WeatherCondition::Clouds.get_emoji(), "☁️");
    assert_eq!(WeatherCondition::Rain.get_emoji(), "🌧️");
    assert_eq!(WeatherCondition::Thunderstorm.get_emoji(), "⛈️");
    assert_eq!(WeatherCondition::Snow.get_emoji(), "❄️");
    assert_eq!(WeatherCondition::Mist.get_emoji(), "🌫️");
    assert_eq!(WeatherCondition::Fog.get_emoji(), "🌫️");
    assert_eq!(WeatherCondition::Unknown.get_emoji(), "❓");
}

#[test]
fn test_weather_condition_display() {
    assert_eq!(WeatherCondition::Clear.to_string(), "Clear");
    assert_eq!(WeatherCondition::Clouds.to_string(), "Cloudy");
    assert_eq!(WeatherCondition::Rain.to_string(), "Rainy");
    assert_eq!(WeatherCondition::Drizzle.to_string(), "Drizzle");
    assert_eq!(WeatherCondition::Thunderstorm.to_string(), "Thunderstorm");
    assert_eq!(WeatherCondition::Snow.to_string(), "Snowy");
    assert_eq!(WeatherCondition::Unknown.to_string(), "Unknown");
}

#[test]
fn test_detail_level() {
    // Test ordering
    assert!(DetailLevel::Basic < DetailLevel::Standard);
    assert!(DetailLevel::Standard < DetailLevel::Detailed);
    assert!(DetailLevel::Detailed < DetailLevel::Debug);

    // Test display
    assert_eq!(DetailLevel::Basic.to_string(), "Basic");
    assert_eq!(DetailLevel::Standard.to_string(), "Standard");
    assert_eq!(DetailLevel::Detailed.to_string(), "Detailed");
    assert_eq!(DetailLevel::Debug.to_string(), "Debug");
}

#[test]
fn test_weather_config_default() {
    let config = WeatherConfig::default();
    assert_eq!(config.units, "metric");
    assert_eq!(config.location, None);
    assert_eq!(config.json_output, false);
    assert_eq!(config.animation_enabled, true);
    assert_eq!(config.detail_level, DetailLevel::Standard);
}

#[test]
fn test_location_default() {
    let location = Location::default();
    assert_eq!(location.name, "Unknown");
    assert_eq!(location.country, "Unknown");
    assert_eq!(location.country_code, "UN");
    assert_eq!(location.latitude, 0.0);
    assert_eq!(location.longitude, 0.0);
    assert_eq!(location.timezone, "UTC");
    assert_eq!(location.region, None);
    assert_eq!(location.state, None);
}
