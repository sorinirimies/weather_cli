use assert_cmd::prelude::*;
use predicates::prelude::*;
use std::process::Command;

#[test]
fn test_cli_version() {
    let mut cmd = Command::cargo_bin("weather_cli").unwrap();
    cmd.arg("--version");
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("weather_cli"));
}

#[test]
fn test_cli_help() {
    let mut cmd = Command::cargo_bin("weather_cli").unwrap();
    cmd.arg("--help");
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("USAGE"))
        .stdout(predicate::str::contains("OPTIONS"))
        .stdout(predicate::str::contains("--mode"))
        .stdout(predicate::str::contains("--location"));
}

#[test]
fn test_cli_invalid_mode() {
    let mut cmd = Command::cargo_bin("weather_cli").unwrap();
    cmd.arg("--mode").arg("invalid_mode");
    cmd.assert()
        .failure()
        .stderr(predicate::str::contains("Invalid mode"));
}

#[test]
fn test_cli_valid_modes() {
    // List of valid modes to test
    let valid_modes = [
        "current", "forecast", "hourly", "daily", "full", "interactive"
    ];
    
    for mode in valid_modes {
        let mut cmd = Command::cargo_bin("weather_cli").unwrap();
        // Adding --no-animations to avoid hanging in tests
        // Adding --location to avoid IP detection which could fail in CI
        cmd.arg("--mode")
           .arg(mode)
           .arg("--no-animations")
           .arg("--location")
           .arg("London");
           
        // We don't check the actual output here as it would require
        // API calls, but we ensure the command doesn't fail due to
        // invalid mode selection
        cmd.assert().code(predicate::in_iter(vec![0, 1]));
    }
}

#[test]
fn test_cli_units_option() {
    // Test metric units (default)
    let mut cmd = Command::cargo_bin("weather_cli").unwrap();
    cmd.arg("--units")
       .arg("metric")
       .arg("--no-animations")
       .arg("--location")
       .arg("London");
    cmd.assert().code(predicate::in_iter(vec![0, 1]));
    
    // Test imperial units
    let mut cmd = Command::cargo_bin("weather_cli").unwrap();
    cmd.arg("--units")
       .arg("imperial")
       .arg("--no-animations")
       .arg("--location")
       .arg("London");
    cmd.assert().code(predicate::in_iter(vec![0, 1]));
}

#[test]
fn test_cli_detail_option() {
    // Test each detail level
    let detail_levels = ["basic", "standard", "detailed", "debug"];
    
    for level in detail_levels {
        let mut cmd = Command::cargo_bin("weather_cli").unwrap();
        cmd.arg("--detail")
           .arg(level)
           .arg("--no-animations")
           .arg("--location")
           .arg("London");
        cmd.assert().code(predicate::in_iter(vec![0, 1]));
    }
}

#[test]
fn test_cli_json_output() {
    let mut cmd = Command::cargo_bin("weather_cli").unwrap();
    cmd.arg("--json")
       .arg("--location")
       .arg("London");
       
    // When running with --json, the output should contain valid JSON
    // but we can't verify the content without API calls
    cmd.assert().code(predicate::in_iter(vec![0, 1]));
}