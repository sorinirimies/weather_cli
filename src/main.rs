use clap::Parser;
use colored::*;
use std::process;
use std::time::Duration;

mod modules;

use modules::forecaster::WeatherForecaster;
use modules::location::LocationService;
use modules::types::{DetailLevel, WeatherConfig};
use modules::tui::WeatherTui;
use modules::ui::WeatherUI;

#[derive(Parser)]
#[command(
    name = "weather_man",
    author = "Sorin Albu-Irimies",
    version = "0.2.1",
    about = "A cyberpunk-themed weather forecasting CLI with atmospheric weather visualizations",
    long_about = "A feature-rich Rust-based CLI to get weather forecasts with cyberpunk-themed animations and atmospheric weather canvas scenes"
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
    
    /// Disable weather canvas (use text output only)
    #[arg(long, default_value = "false")]
    no_charts: bool,

    /// Run test weather canvas with mock data
    #[arg(long, default_value = "false")]
    test_charts: bool,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::parse();



    // Configure based on command-line arguments
    let config = WeatherConfig {
        units: cli.units,
        location: cli.location.clone(),
        json_output: cli.json,
        animation_enabled: !cli.no_animations,
        detail_level: parse_detail_level(&cli.detail),
        no_charts: cli.no_charts,
    };



    // Initialize components
    let ui = WeatherUI::new(config.animation_enabled, config.json_output);
    let location_service = LocationService::new();
    let forecaster = WeatherForecaster::new(config.clone());
    
    // Check for test charts flag first
    if cli.test_charts {
        return run_test_charts(config).await;
    }

    // Run selected mode
    match cli.mode.as_str() {
            "current" => run_current_weather(forecaster.clone(), location_service.clone(), ui.clone(), config.clone()).await?,
            "forecast" => run_forecast(forecaster.clone(), location_service.clone(), ui.clone(), config.clone()).await?,
            "hourly" => run_hourly_forecast(forecaster.clone(), location_service.clone(), ui.clone(), config.clone()).await?,
            "daily" => run_daily_forecast(forecaster.clone(), location_service.clone(), ui.clone(), config.clone()).await?,
            "full" => run_full_weather(forecaster.clone(), location_service.clone(), ui.clone(), config.clone()).await?,
            "interactive" => run_interactive_menu(forecaster.clone(), location_service.clone(), ui.clone(), config.clone()).await?,
            "canvas" => run_charts_mode(forecaster.clone(), location_service.clone(), config.clone()).await?,
            _ => {
                eprintln!("{}", "Invalid mode specified!".bright_red());
                eprintln!("Valid modes: current, forecast, hourly, daily, full, interactive, canvas");
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
        
        // Show weather canvas unless disabled
        if !config.no_charts {
            println!("\n🌤️  Loading interactive weather view...");
            if let Err(e) = run_charts_mode(forecaster, location_service, config).await {
                eprintln!("⚠️  Weather view unavailable: {}", e);
                eprintln!("💡 Try running with --no-charts for text-only output");
            }
        }
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
        
        // Show weather canvas unless disabled
        if !config.no_charts {
            println!("\n🌤️  Loading interactive weather view...");
            if let Err(e) = run_charts_mode(forecaster, location_service, config).await {
                eprintln!("⚠️  Weather view unavailable: {}", e);
                eprintln!("💡 Try running with --no-charts for text-only output");
            }
        }
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
        
        // Show weather canvas unless disabled
        if !config.no_charts {
            println!("\n🌤️  Loading interactive weather view...");
            if let Err(e) = run_charts_mode(forecaster, location_service, config).await {
                eprintln!("⚠️  Weather view unavailable: {}", e);
                eprintln!("💡 Try running with --no-charts for text-only output");
            }
        }
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
        
        // Show weather canvas unless disabled
        if !config.no_charts {
            println!("\n🌤️  Loading interactive weather view...");
            if let Err(e) = run_charts_mode(forecaster, location_service, config).await {
                eprintln!("⚠️  Weather view unavailable: {}", e);
                eprintln!("💡 Try running with --no-charts for text-only output");
            }
        }
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
        
        // Show weather canvas unless disabled
        if !config.no_charts {
            // First run the weather canvas mode in a separate function
            run_charts_mode(forecaster, location_service, config).await?;
        }
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
        let choice = ui.show_interactive_menu(!config.no_charts)?;

        match choice.as_str() {
            "current" => {
                // Clear terminal first for clean output
                print!("\x1B[2J\x1B[1;1H");
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
            "canvas" => {
                // Get hourly and daily forecasts for weather canvas
                let hourly = forecaster.get_hourly_forecast(&location_service.get_location_from_ip().await?).await?;
                let daily = forecaster.get_daily_forecast(&location_service.get_location_from_ip().await?).await?;
                
                // Create and run the TUI
                let mut tui = WeatherTui::new(hourly, daily, location_service.get_location_from_ip().await?, config.clone())?;
                tui.run()?;
            }
            "exit" => break,
            _ => {
                eprintln!("{}", "Invalid option selected!".bright_red());
            }
        }
    }

    Ok(())
}

async fn run_charts_mode(
    forecaster: WeatherForecaster,
    location_service: LocationService,
    config: WeatherConfig,
) -> Result<(), Box<dyn std::error::Error>> {
    // Determine location (auto-detect or use provided)
    let location = match &config.location {
        Some(loc) => location_service.get_location_by_name(loc).await?,
        None => location_service.get_location_from_ip().await?,
    };
    
    // Get the data we need for the charts
    let hourly = forecaster.get_hourly_forecast(&location).await?;
    let daily = forecaster.get_daily_forecast(&location).await?;
    
    // Clear screen for clean TUI transition
    print!("\x1B[2J\x1B[1;1H");
    std::io::Write::flush(&mut std::io::stdout()).unwrap_or(());
    
    // Create and run the TUI directly
    let mut tui = WeatherTui::new(hourly, daily, location, config)?;
    tui.run()?;
    Ok(())
}

async fn run_test_charts(config: WeatherConfig) -> Result<(), Box<dyn std::error::Error>> {
    use modules::types::{DailyForecast, HourlyForecast, Location, WeatherCondition};
    use chrono::Utc;

    println!("🧪 Testing Weather Canvas TUI");
    println!("===============================");
    
    // Create test location
    let location = Location {
        name: "Test City".to_string(),
        country: "Test Country".to_string(),
        country_code: "TC".to_string(),
        latitude: 40.7128,
        longitude: -74.0060,
        timezone: "UTC".to_string(),
        region: Some("Test Region".to_string()),
        state: Some("Test State".to_string()),
    };
    
    // Generate test hourly data
    let mut hourly_data = Vec::new();
    let base_time = Utc::now();
    
    for i in 0..24 {
        let forecast = HourlyForecast {
            timestamp: base_time + chrono::Duration::hours(i),
            temperature: 20.0 + (i as f64 * 0.5),
            feels_like: 18.0 + (i as f64 * 0.5),
            humidity: 60 + (i % 20) as u8,
            pressure: 1013 + (i % 10) as u32,
            wind_speed: 5.0 + (i as f64 * 0.2),
            wind_direction: (i * 15) as u16,
            conditions: vec![],
            main_condition: if i % 4 == 0 { WeatherCondition::Rain } else { WeatherCondition::Clear },
            pop: (i as f64 * 0.04).min(1.0),
            visibility: 10000,
            clouds: (i * 5) as u8,
            rain: if i % 4 == 0 { Some(0.5) } else { None },
            snow: None,
        };
        hourly_data.push(forecast);
    }
    
    // Generate test daily data
    let mut daily_data = Vec::new();
    
    for i in 0..7 {
        let forecast = DailyForecast {
            date: base_time + chrono::Duration::days(i),
            sunrise: base_time + chrono::Duration::days(i) + chrono::Duration::hours(6),
            sunset: base_time + chrono::Duration::days(i) + chrono::Duration::hours(18),
            temp_morning: 15.0 + (i as f64),
            temp_day: 25.0 + (i as f64),
            temp_evening: 20.0 + (i as f64),
            temp_night: 10.0 + (i as f64),
            temp_min: 8.0 + (i as f64),
            temp_max: 28.0 + (i as f64),
            feels_like_day: 23.0 + (i as f64),
            feels_like_night: 8.0 + (i as f64),
            pressure: 1015 + (i % 5) as u32,
            humidity: 65 + (i % 15) as u8,
            wind_speed: 4.0 + (i as f64 * 0.3),
            wind_direction: (i * 30) as u16,
            conditions: vec![],
            main_condition: match i % 5 {
                0 => WeatherCondition::Clear,
                1 => WeatherCondition::Clouds,
                2 => WeatherCondition::Rain,
                3 => WeatherCondition::Snow,
                _ => WeatherCondition::Thunderstorm,
            },
            clouds: (i * 15) as u8,
            pop: (i as f64 * 0.15).min(1.0),
            rain: if i % 3 == 0 { Some(1.5) } else { None },
            snow: if i == 3 { Some(2.0) } else { None },
            uv_index: (i as f64 * 1.5).min(10.0),
        };
        daily_data.push(forecast);
    }
    
    println!("📊 Created {} hourly forecasts", hourly_data.len());
    println!("📅 Created {} daily forecasts", daily_data.len());
    println!("🎯 Starting TUI in 2 seconds...");
    println!("💡 Use arrow keys or 1-5 to switch tabs, 'q' to exit");
    
    std::thread::sleep(std::time::Duration::from_millis(2000));
    
    // Create and run TUI
    let mut tui = WeatherTui::new(hourly_data, daily_data, location, config)?;
    tui.run()?;
    
    println!("✅ TUI test completed successfully!");
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
