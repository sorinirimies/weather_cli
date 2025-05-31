use clap::Parser;
use colored::*;
use std::process;
use std::time::Duration;

mod modules;

use modules::forecaster::WeatherForecaster;
use modules::location::LocationService;
use modules::types::{DetailLevel, WeatherConfig};
use modules::ui::WeatherUI;

#[derive(Parser)]
#[command(
    name = "weather_man",
    author = "Sorin Albu-Irimies",
    version = "0.1.0",
    about = "A cyberpunk-themed weather forecasting CLI",
    long_about = "A feature-rich Rust-based CLI to get weather forecasts with cyberpunk-themed animations"
)]
struct Cli {
    /// Display mode for the application
    #[arg(short, long, default_value = "current")]
    mode: String,

    /// Location to check weather for (default: auto-detect from IP)
    #[arg(short, long)]
    location: Option<String>,

    /// Units to display (metric, imperial, standard)
    #[arg(short, long, default_value = "metric")]
    units: String,

    /// Level of detail to display
    #[arg(short, long, default_value = "standard")]
    detail: String,

    /// Output results as JSON
    #[arg(short, long, default_value = "false")]
    json: bool,

    /// Disable animations
    #[arg(short = 'a', long, default_value = "false")]
    no_animations: bool,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::parse();

    // Configure based on command-line arguments
    let config = WeatherConfig {
        units: cli.units,
        location: cli.location,
        json_output: cli.json,
        animation_enabled: !cli.no_animations,
        detail_level: parse_detail_level(&cli.detail),
    };

    // Initialize components
    let ui = WeatherUI::new(config.animation_enabled, config.json_output);
    let location_service = LocationService::new();
    let forecaster = WeatherForecaster::new(config.clone());

    match cli.mode.as_str() {
        "current" => run_current_weather(forecaster, location_service, ui, config).await?,
        "forecast" => run_forecast(forecaster, location_service, ui, config).await?,
        "hourly" => run_hourly_forecast(forecaster, location_service, ui, config).await?,
        "daily" => run_daily_forecast(forecaster, location_service, ui, config).await?,
        "full" => run_full_weather(forecaster, location_service, ui, config).await?,
        "interactive" => run_interactive_menu(forecaster, location_service, ui, config).await?,
        _ => {
            eprintln!("{}", "Invalid mode specified!".bright_red());
            eprintln!("Valid modes: current, forecast, hourly, daily, full, interactive");
            process::exit(1);
        }
    }

    Ok(())
}

async fn run_current_weather(
    forecaster: WeatherForecaster,
    location_service: LocationService,
    ui: WeatherUI,
    config: WeatherConfig,
) -> Result<(), Box<dyn std::error::Error>> {
    if !config.json_output {
        ui.show_welcome_banner()?;
        ui.show_connecting_animation()?;
    }

    // Determine location (auto-detect or use provided)
    let location = match &config.location {
        Some(loc) => location_service.get_location_by_name(loc).await?,
        None => location_service.get_location_from_ip().await?,
    };

    if !config.json_output {
        ui.show_location_info(&location)?;
    }

    // Get current weather
    let weather = forecaster.get_current_weather(&location).await?;

    // Display results
    if config.json_output {
        println!("{}", serde_json::to_string_pretty(&weather)?);
    } else {
        ui.show_current_weather(&weather, &location)?;
        ui.show_weather_recommendations(&weather)?;
    }

    Ok(())
}

async fn run_forecast(
    forecaster: WeatherForecaster,
    location_service: LocationService,
    ui: WeatherUI,
    config: WeatherConfig,
) -> Result<(), Box<dyn std::error::Error>> {
    if !config.json_output {
        ui.show_welcome_banner()?;
        ui.show_connecting_animation()?;
    }

    // Determine location
    let location = match &config.location {
        Some(loc) => location_service.get_location_by_name(loc).await?,
        None => location_service.get_location_from_ip().await?,
    };

    if !config.json_output {
        ui.show_location_info(&location)?;
    }

    // Get weather forecast
    let forecast = forecaster.get_forecast(&location).await?;

    // Display results
    if config.json_output {
        println!("{}", serde_json::to_string_pretty(&forecast)?);
    } else {
        ui.show_forecast(&forecast, &location)?;
    }

    Ok(())
}

async fn run_hourly_forecast(
    forecaster: WeatherForecaster,
    location_service: LocationService,
    ui: WeatherUI,
    config: WeatherConfig,
) -> Result<(), Box<dyn std::error::Error>> {
    if !config.json_output {
        ui.show_welcome_banner()?;
        ui.show_connecting_animation()?;
    }

    // Determine location
    let location = match &config.location {
        Some(loc) => location_service.get_location_by_name(loc).await?,
        None => location_service.get_location_from_ip().await?,
    };

    if !config.json_output {
        ui.show_location_info(&location)?;
    }

    // Get hourly forecast
    let forecast = forecaster.get_hourly_forecast(&location).await?;

    // Display results
    if config.json_output {
        println!("{}", serde_json::to_string_pretty(&forecast)?);
    } else {
        ui.show_hourly_forecast(&forecast, &location)?;
    }

    Ok(())
}

async fn run_daily_forecast(
    forecaster: WeatherForecaster,
    location_service: LocationService,
    ui: WeatherUI,
    config: WeatherConfig,
) -> Result<(), Box<dyn std::error::Error>> {
    if !config.json_output {
        ui.show_welcome_banner()?;
        ui.show_connecting_animation()?;
    }

    // Determine location
    let location = match &config.location {
        Some(loc) => location_service.get_location_by_name(loc).await?,
        None => location_service.get_location_from_ip().await?,
    };

    if !config.json_output {
        ui.show_location_info(&location)?;
    }

    // Get daily forecast
    let forecast = forecaster.get_daily_forecast(&location).await?;

    // Display results
    if config.json_output {
        println!("{}", serde_json::to_string_pretty(&forecast)?);
    } else {
        ui.show_daily_forecast(&forecast, &location)?;
    }

    Ok(())
}

async fn run_full_weather(
    forecaster: WeatherForecaster,
    location_service: LocationService,
    ui: WeatherUI,
    config: WeatherConfig,
) -> Result<(), Box<dyn std::error::Error>> {
    if !config.json_output {
        ui.show_welcome_banner()?;
        ui.show_connecting_animation()?;
    }

    // Determine location
    let location = match &config.location {
        Some(loc) => location_service.get_location_by_name(loc).await?,
        None => location_service.get_location_from_ip().await?,
    };

    if !config.json_output {
        ui.show_location_info(&location)?;
    }

    // Get current weather, hourly and daily forecasts
    let current = forecaster.get_current_weather(&location).await?;
    let hourly = forecaster.get_hourly_forecast(&location).await?;
    let daily = forecaster.get_daily_forecast(&location).await?;

    // Display results
    if config.json_output {
        let full_data = serde_json::json!({
            "current": current,
            "hourly": hourly,
            "daily": daily,
        });
        println!("{}", serde_json::to_string_pretty(&full_data)?);
    } else {
        ui.show_current_weather(&current, &location)?;

        if config.animation_enabled {
            std::thread::sleep(Duration::from_millis(800));
        }

        ui.show_hourly_forecast(&hourly, &location)?;

        if config.animation_enabled {
            std::thread::sleep(Duration::from_millis(800));
        }

        ui.show_daily_forecast(&daily, &location)?;
        ui.show_weather_recommendations(&current)?;
    }

    Ok(())
}

async fn run_interactive_menu(
    forecaster: WeatherForecaster,
    location_service: LocationService,
    ui: WeatherUI,
    config: WeatherConfig,
) -> Result<(), Box<dyn std::error::Error>> {
    ui.show_welcome_banner()?;

    // Loop until exit
    loop {
        let choice = ui.show_interactive_menu()?;

        match choice.as_str() {
            "current" => {
                run_current_weather(
                    forecaster.clone(),
                    location_service.clone(),
                    ui.clone(),
                    config.clone(),
                )
                .await?;
            }
            "hourly" => {
                run_hourly_forecast(
                    forecaster.clone(),
                    location_service.clone(),
                    ui.clone(),
                    config.clone(),
                )
                .await?;
            }
            "daily" => {
                run_daily_forecast(
                    forecaster.clone(),
                    location_service.clone(),
                    ui.clone(),
                    config.clone(),
                )
                .await?;
            }
            "full" => {
                run_full_weather(
                    forecaster.clone(),
                    location_service.clone(),
                    ui.clone(),
                    config.clone(),
                )
                .await?;
            }
            "change_location" => {
                // Prompt for a new location
                let new_location = ui.prompt_for_location()?;
                let mut new_config = config.clone();
                new_config.location = Some(new_location);

                run_full_weather(
                    forecaster.clone(),
                    location_service.clone(),
                    ui.clone(),
                    new_config,
                )
                .await?;
            }
            "change_units" => {
                // Prompt for units
                let new_units = ui.prompt_for_units()?;
                let mut new_config = config.clone();
                new_config.units = new_units;

                run_full_weather(
                    forecaster.clone(),
                    location_service.clone(),
                    ui.clone(),
                    new_config,
                )
                .await?;
            }
            "exit" => break,
            _ => {
                eprintln!("{}", "Invalid option selected!".bright_red());
            }
        }
    }

    Ok(())
}

fn parse_detail_level(detail: &str) -> DetailLevel {
    match detail.to_lowercase().as_str() {
        "basic" => DetailLevel::Basic,
        "detailed" => DetailLevel::Detailed,
        "debug" => DetailLevel::Debug,
        _ => DetailLevel::Standard,
    }
}
