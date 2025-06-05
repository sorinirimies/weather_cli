use crate::modules::types::{DailyForecast, HourlyForecast, Location, WeatherConfig, WeatherCondition};
use crate::modules::ui::convert_to_local;
use anyhow::Result;
use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode, KeyEventKind},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};

use ratatui::{
    backend::{CrosstermBackend},
    layout::{Constraint, Direction, Layout},
    style::{Color, Modifier, Style},
    text::{Line, Span, Text},
    widgets::{Block, BorderType, Borders, Paragraph, Tabs, Wrap},
    Terminal,
};
use std::io;
use std::io::Stdout;

/// Enum representing the available tabs in the TUI
#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub enum TuiTab {
    CurrentWeather,
    WeatherForecast,
    Calendar,
}

impl TuiTab {
    fn next(&self) -> Self {
        match self {
            TuiTab::CurrentWeather => TuiTab::WeatherForecast,
            TuiTab::WeatherForecast => TuiTab::Calendar,
            TuiTab::Calendar => TuiTab::CurrentWeather,
        }
    }

    fn prev(&self) -> Self {
        match self {
            TuiTab::CurrentWeather => TuiTab::Calendar,
            TuiTab::WeatherForecast => TuiTab::CurrentWeather,
            TuiTab::Calendar => TuiTab::WeatherForecast,
        }
    }

    fn to_string(&self) -> &'static str {
        match self {
            TuiTab::CurrentWeather => "Current Weather",
            TuiTab::WeatherForecast => "Weather Forecast",
            TuiTab::Calendar => "Weather Calendar",
        }
    }
}

struct UiState {
    active_tab: TuiTab,
    hourly_data: Vec<HourlyForecast>,
    daily_data: Vec<DailyForecast>,
    location: Location,
    config: WeatherConfig,
}

/// The main TUI application state
pub struct WeatherTui {
    state: UiState,
    terminal: Terminal<CrosstermBackend<Stdout>>,
}

impl WeatherTui {
    /// Create a new TUI with the provided weather data
    pub fn new(
        hourly_data: Vec<HourlyForecast>,
        daily_data: Vec<DailyForecast>,
        location: Location,
        config: WeatherConfig,
    ) -> Result<Self> {
        // Setup terminal properly
        enable_raw_mode()?;
        let mut stdout = io::stdout();
        execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
        let backend = CrosstermBackend::new(stdout);
        let terminal = Terminal::new(backend)?;

        let state = UiState {
            active_tab: TuiTab::CurrentWeather,
            hourly_data,
            daily_data,
            location,
            config,
        };

        Ok(Self {
            state,
            terminal,
        })
    }

    /// Run the TUI application
    pub fn run(&mut self) -> Result<()> {
        loop {
            // Clone the active tab before drawing to avoid borrowing issues
            let active_tab = self.state.active_tab;
            let hourly_data = self.state.hourly_data.clone();
            let daily_data = self.state.daily_data.clone();
            let location = self.state.location.clone();
            let config = self.state.config.clone();
            
            self.terminal.draw(|f| {
                let chunks = Layout::default()
                    .direction(Direction::Vertical)
                    .margin(1)
                    .constraints(
                        [
                            Constraint::Length(3), // Title
                            Constraint::Length(3), // Tabs
                            Constraint::Min(0),    // Content
                            Constraint::Length(3), // Help
                        ]
                        .as_ref(),
                    )
                    .split(f.size());
                
                // Render title
                let units_text = match config.units.as_str() {
                    "metric" => "°C",
                    "imperial" => "°F",
                    _ => "K",
                };
                
                let title = Paragraph::new(Text::from(vec![Line::from(vec![
                    Span::styled(
                        format!("Weather Man - {}", location.name),
                        Style::default()
                            .fg(Color::Cyan)
                            .add_modifier(Modifier::BOLD),
                    ),
                    Span::raw(" "),
                    Span::styled(
                        format!(
                            "[{}, {}]",
                            location.country,
                            location.country_code
                        ),
                        Style::default().fg(Color::Gray),
                    ),
                    Span::raw(" "),
                    Span::styled(format!("({})", units_text), Style::default().fg(Color::Yellow)),
                ])]))
                .block(
                    Block::default()
                        .borders(Borders::ALL)
                        .border_type(BorderType::Rounded)
                        .style(Style::default().fg(Color::Cyan)),
                );
                
                f.render_widget(title, chunks[0]);
                
                // Render tabs
                let titles = [
                    TuiTab::CurrentWeather,
                    TuiTab::WeatherForecast,
                    TuiTab::Calendar,
                ]
                .iter()
                .map(|t| {
                    let (first, rest) = t.to_string().split_at(1);
                    Line::from(vec![
                        Span::styled(
                            first,
                            Style::default()
                                .fg(Color::Yellow)
                                .add_modifier(Modifier::UNDERLINED),
                        ),
                        Span::styled(rest, Style::default().fg(Color::White)),
                    ])
                })
                .collect::<Vec<_>>();
                
                let tabs = Tabs::new(titles)
                    .block(
                        Block::default()
                            .borders(Borders::ALL)
                            .border_type(BorderType::Rounded)
                            .title("Tabs")
                            .style(Style::default().fg(Color::Cyan)),
                    )
                    .select(match active_tab {
                        TuiTab::CurrentWeather => 0,
                        TuiTab::WeatherForecast => 1,
                        TuiTab::Calendar => 2,
                    })
                    .style(Style::default().fg(Color::White))
                    .highlight_style(
                        Style::default()
                            .fg(Color::Cyan)
                            .add_modifier(Modifier::BOLD),
                    );
                
                f.render_widget(tabs, chunks[1]);
                
                // Render content based on selected tab
                match active_tab {
                    TuiTab::CurrentWeather => {
                        use crate::modules::canvas::render_current_weather_canvas;
                        render_current_weather_canvas(&hourly_data, f, chunks[2]);
                    }
                    TuiTab::WeatherForecast => {
                        use crate::modules::canvas::render_forecast_canvas;
                        render_forecast_canvas(&daily_data, f, chunks[2]);
                    }
                    TuiTab::Calendar => {
                        render_weather_calendar(&daily_data, &location, f, chunks[2]);
                    }
                }
                
                // Render help
                let help_text = Text::from(vec![Line::from(vec![
                    Span::styled("Keys: ", Style::default().fg(Color::Cyan)),
                    Span::styled("←/→", Style::default().fg(Color::Yellow)),
                    Span::raw(" Switch tabs | "),
                    Span::styled("1-3", Style::default().fg(Color::Yellow)),
                    Span::raw(" Select tab | "),
                    Span::styled("q", Style::default().fg(Color::Yellow)),
                    Span::raw(" Quit | "),
                    Span::styled("ESC", Style::default().fg(Color::Yellow)),
                    Span::raw(" Exit weather view"),
                ])]);
                
                let help = Paragraph::new(help_text)
                    .block(
                        Block::default()
                            .borders(Borders::ALL)
                            .border_type(BorderType::Rounded)
                            .style(Style::default().fg(Color::Cyan)),
                    )
                    .wrap(Wrap { trim: true });
                
                f.render_widget(help, chunks[3]);
            })?;

            match event::read()? {
                Event::Key(key) => {
                    if key.kind == KeyEventKind::Press {
                        match key.code {
                            KeyCode::Char('q') | KeyCode::Esc => {
                                break;
                            }
                            KeyCode::Right | KeyCode::Tab => {
                                self.state.active_tab = self.state.active_tab.next();
                            }
                            KeyCode::Left | KeyCode::BackTab => {
                                self.state.active_tab = self.state.active_tab.prev();
                            }
                            KeyCode::Char('1') => {
                                self.state.active_tab = TuiTab::CurrentWeather;
                            }
                            KeyCode::Char('2') => {
                                self.state.active_tab = TuiTab::WeatherForecast;
                            }
                            KeyCode::Char('3') => {
                                self.state.active_tab = TuiTab::Calendar;
                            }
                            _ => {}
                        }
                    }
                }
                _ => {
                    // Ignore other events
                }
            }
        }

        // Restore terminal
        disable_raw_mode()?;
        execute!(
            self.terminal.backend_mut(),
            LeaveAlternateScreen,
            DisableMouseCapture
        )?;
        self.terminal.show_cursor()?;

        Ok(())
    }

    // The UI drawing methods have been moved into the run() function to avoid borrowing issues
}



/// Render a weather calendar showing conditions for a range of dates
fn render_weather_calendar<B: ratatui::backend::Backend>(
    daily_data: &[DailyForecast],
    location: &Location,
    frame: &mut ratatui::Frame<B>,
    area: ratatui::layout::Rect,
) {
    // Create a simple text-based calendar view
    let mut calendar_text = Vec::new();
    
    calendar_text.push(Line::from(vec![
        Span::styled("7-Day Weather Calendar", Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD)),
    ]));
    calendar_text.push(Line::from(vec![Span::raw("")]));
    
    // Show next 7 days with weather info
    for (_i, day) in daily_data.iter().take(7).enumerate() {
        let local_date = convert_to_local(&day.date, &location.timezone);
        let weekday = local_date.format("%A").to_string();
        let date_str = local_date.format("%m/%d").to_string();
        
        let condition_emoji = day.main_condition.get_emoji();
        let color = match day.main_condition {
            WeatherCondition::Clear => Color::Yellow,
            WeatherCondition::Clouds => Color::Gray,
            WeatherCondition::Rain | WeatherCondition::Drizzle => Color::Blue,
            WeatherCondition::Thunderstorm => Color::Magenta,
            WeatherCondition::Snow => Color::White,
            _ => Color::Gray,
        };
        
        let pop_percent = (day.pop * 100.0) as u8;
        
        calendar_text.push(Line::from(vec![
            Span::styled(format!("{:9}", weekday), Style::default().fg(Color::Cyan)),
            Span::raw(" "),
            Span::styled(date_str, Style::default().fg(Color::Gray)),
            Span::raw("  "),
            Span::styled(condition_emoji, Style::default()),
            Span::raw(" "),
            Span::styled(format!("{}", day.main_condition), Style::default().fg(color)),
            Span::raw("  "),
            Span::styled(format!("{}°-{}°C", day.temp_min as i32, day.temp_max as i32), Style::default().fg(Color::White)),
            Span::raw("  "),
            Span::styled(format!("{}%", pop_percent), Style::default().fg(Color::Blue)),
        ]));
    }
    
    calendar_text.push(Line::from(vec![Span::raw("")]));
    calendar_text.push(Line::from(vec![
        Span::styled("Legend: ", Style::default().fg(Color::Gray)),
        Span::styled("Temperature Range", Style::default().fg(Color::White)),
        Span::raw(" | "),
        Span::styled("Rain %", Style::default().fg(Color::Blue)),
    ]));
    
    let calendar = Paragraph::new(calendar_text)
        .block(
            Block::default()
                .title("Weather Calendar")
                .borders(Borders::ALL)
                .style(Style::default().fg(Color::Cyan)),
        )
        .wrap(Wrap { trim: false });
    
    frame.render_widget(calendar, area);
}

impl Drop for WeatherTui {
    fn drop(&mut self) {
        // Restore terminal on drop
        let _ = disable_raw_mode();
        let _ = execute!(
            self.terminal.backend_mut(),
            LeaveAlternateScreen,
            DisableMouseCapture
        );
        let _ = self.terminal.show_cursor();
        
        // Print a newline to ensure the terminal is in a good state
        println!();
    }
}