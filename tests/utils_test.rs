use chrono::{DateTime, Utc};
use std::str::FromStr;
use weather_cli::modules::utils::{
    convert_temperature, degrees_to_direction, format_datetime, format_wind_speed, get_temp_unit,
    parse_with_default, timestamp_to_datetime, truncate_string,
};

#[test]
fn test_timestamp_to_datetime() {
    let timestamp = 1609459200; // 2020-12-31 20:00:00 UTC
    let dt = timestamp_to_datetime(timestamp);
    assert_eq!(dt.timestamp(), timestamp);
}

#[test]
fn test_format_datetime() {
    let dt = DateTime::parse_from_rfc3339("2023-01-01T12:00:00Z")
        .unwrap()
        .with_timezone(&Utc);
    let formatted = format_datetime(&dt, "%Y-%m-%d");
    assert_eq!(formatted, "2023-01-01");
}

#[test]
fn test_convert_temperature() {
    // Test Celsius to Fahrenheit
    assert_eq!(convert_temperature(0.0, "metric", "imperial"), 32.0);
    assert_eq!(convert_temperature(100.0, "metric", "imperial"), 212.0);

    // Test Fahrenheit to Celsius
    assert!((convert_temperature(32.0, "imperial", "metric") - 0.0).abs() < 0.001);
    assert!((convert_temperature(212.0, "imperial", "metric") - 100.0).abs() < 0.001);

    // Test Celsius to Kelvin
    assert_eq!(convert_temperature(0.0, "metric", "standard"), 273.15);

    // Test Kelvin to Celsius
    assert!((convert_temperature(273.15, "standard", "metric") - 0.0).abs() < 0.001);

    // Test same unit (no conversion)
    assert_eq!(convert_temperature(25.0, "metric", "metric"), 25.0);
}

#[test]
fn test_format_wind_speed() {
    // Test metric
    assert_eq!(format_wind_speed(5.5, "metric"), "5.5 m/s");

    // Test imperial
    assert_eq!(format_wind_speed(10.5, "imperial"), "10.5 mph");

    // Test default (should be metric)
    assert_eq!(format_wind_speed(7.2, "unknown"), "7.2 m/s");
}

#[test]
fn test_get_temp_unit() {
    assert_eq!(get_temp_unit("metric"), "°C");
    assert_eq!(get_temp_unit("imperial"), "°F");
    assert_eq!(get_temp_unit("standard"), "K");
    assert_eq!(get_temp_unit("unknown"), "°C"); // default
}

#[test]
fn test_parse_with_default() {
    // Test successful parse
    let result: i32 = parse_with_default("42", 0);
    assert_eq!(result, 42);

    // Test fallback to default
    let result: i32 = parse_with_default("not_a_number", 100);
    assert_eq!(result, 100);
}

#[test]
fn test_truncate_string() {
    // No truncation needed
    assert_eq!(truncate_string("Hello", 10), "Hello");

    // Truncation needed
    assert_eq!(truncate_string("Hello World", 8), "Hello...");

    // Edge case: max_len <= 3
    assert_eq!(truncate_string("Hello", 3), "...");
    assert_eq!(truncate_string("Hi", 3), "Hi");
}

#[test]
fn test_degrees_to_direction() {
    // Test cardinal directions
    assert_eq!(degrees_to_direction(0.0), "N");
    assert_eq!(degrees_to_direction(90.0), "E");
    assert_eq!(degrees_to_direction(180.0), "S");
    assert_eq!(degrees_to_direction(270.0), "W");

    // Test intermediate directions
    assert_eq!(degrees_to_direction(45.0), "NE");
    assert_eq!(degrees_to_direction(135.0), "SE");
    assert_eq!(degrees_to_direction(225.0), "SW");
    assert_eq!(degrees_to_direction(315.0), "NW");

    // Test edge cases
    assert_eq!(degrees_to_direction(360.0), "N"); // 360 wraps around to 0
    assert_eq!(degrees_to_direction(370.0), "N"); // >360 wraps around
}
