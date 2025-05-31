# Weather Man CLI

A feature-rich Rust-based command-line interface for weather forecasting with a clean, minimalist design.

![Version](https://img.shields.io/badge/version-0.1.0-blue.svg)
[![License](https://img.shields.io/badge/license-MIT-green.svg)](LICENSE)

## Features

- Current weather conditions
- Hourly and daily forecasts
- Location auto-detection (IP-based)
- Custom location specification
- No API key required (uses Open-Meteo)
- Weather recommendations
- Support for metric/imperial units
- Interactive mode with menu-based navigation
- JSON output option for scripting

## Installation

### From crates.io

```
cargo install weather_cli
```

### From source

```
git clone https://github.com/yourusername/weather_cli.git
cd weather_cli
cargo build --release
```

The executable will be available at `target/release/weather_cli`.

## Usage

```
# Current weather at auto-detected location
weather_cli

# Specify a location
weather_cli --location "New York"

# Daily forecast
weather_cli --mode daily

# Hourly forecast
weather_cli --mode hourly

# Full weather report
weather_cli --mode full

# Interactive mode
weather_cli --mode interactive

# Use imperial units
weather_cli --units imperial

# Output as JSON (for scripting)
weather_cli --json
```

## Command-line Options

| Option | Description |
|--------|-------------|
| `--mode`, `-m` | Display mode: current, forecast, hourly, daily, full, interactive |
| `--location`, `-l` | Location to check weather for (default: auto-detect) |
| `--units`, `-u` | Units to display: metric, imperial, standard (default: metric) |
| `--detail`, `-d` | Level of detail: basic, standard, detailed, debug |
| `--json`, `-j` | Output results as JSON |
| `--no-animations`, `-a` | Disable animations |

## Development

### Prerequisites

- Rust 1.70 or newer
- Cargo

### Building

```
cargo build
```

### Running Tests

```
cargo test
```

### Generating Changelog

We use [git-cliff](https://github.com/orhun/git-cliff) to generate changelogs:

```
git cliff --output CHANGELOG.md
```

## Release Process

1. Update version in Cargo.toml
2. Create a new tag: `git tag -a v0.1.0 -m "Release v0.1.0"`
3. Push the tag: `git push origin v0.1.0`
4. GitHub Actions will automatically generate the changelog and publish to crates.io

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

1. Fork the repository
2. Create your feature branch (`git checkout -b feature/amazing-feature`)
3. Commit your changes using conventional commits (`git commit -m 'feat: add some amazing feature'`)
4. Push to the branch (`git push origin feature/amazing-feature`)
5. Open a Pull Request

Please follow the [Conventional Commits](https://www.conventionalcommits.org/) format for your commit messages to ensure proper changelog generation.

## License

This project is licensed under the MIT License - see the LICENSE file for details.

## Acknowledgments

- Weather data provided by [Open-Meteo](https://open-meteo.com/)
- Geocoding services by [Nominatim/OpenStreetMap](https://nominatim.openstreetmap.org/)