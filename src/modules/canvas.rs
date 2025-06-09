use crate::modules::types::{DailyForecast, HourlyForecast, WeatherCondition};
use ratatui::{
    layout::Rect,
    style::Color,
    widgets::canvas::{Canvas, Circle, Context, Line, Points, Rectangle},
    widgets::{Block, Borders},
    Frame,
};
use std::f64::consts::PI;
use std::time::{SystemTime, UNIX_EPOCH};

/// Renders a stunning weather canvas with highly detailed, professional-quality visuals
pub fn render_weather_canvas<B: ratatui::backend::Backend>(
    condition: &WeatherCondition,
    temperature: f64,
    humidity: u8,
    wind_speed: f64,
    is_day: bool,
    frame: &mut Frame<B>,
    area: Rect,
) {
    let canvas = Canvas::default()
        .block(
            Block::default()
                .title("🌤️ Weather Visualization")
                .borders(Borders::ALL)
                .style(ratatui::style::Style::default().fg(Color::Cyan)),
        )
        .x_bounds([0.0, 400.0])
        .y_bounds([0.0, 200.0])
        .paint(|ctx| {
            // Draw atmospheric background
            draw_sky_gradient(ctx, is_day, temperature, condition);
            draw_ground_terrain(ctx, condition);

            // Draw main weather elements based on condition
            match condition {
                WeatherCondition::Clear => {
                    if is_day {
                        draw_magnificent_sun(ctx, 320.0, 160.0, temperature);
                    } else {
                        draw_beautiful_moon(ctx, 320.0, 160.0);
                        draw_stellar_field(ctx);
                    }
                }
                WeatherCondition::Clouds => {
                    draw_cloud_formations(ctx, humidity, is_day, false);
                    if is_day {
                        draw_sun_through_clouds(ctx, 340.0, 150.0);
                    }
                }
                WeatherCondition::Rain | WeatherCondition::Drizzle => {
                    draw_rain_system(ctx, condition == &WeatherCondition::Rain, wind_speed);
                }
                WeatherCondition::Thunderstorm => {
                    draw_storm_system(ctx, wind_speed);
                }
                WeatherCondition::Snow => {
                    draw_snow_system(ctx, temperature, wind_speed);
                }
                WeatherCondition::Fog | WeatherCondition::Mist => {
                    draw_fog_system(ctx, condition == &WeatherCondition::Fog, wind_speed);
                }
                _ => {
                    draw_cloud_formations(ctx, 50, is_day, false);
                }
            }

            // Add atmospheric effects
            if wind_speed > 8.0 {
                draw_wind_patterns(ctx, wind_speed);
            }

            // Weather data visualization
            draw_weather_indicators(ctx, temperature, humidity, wind_speed);
        });

    frame.render_widget(canvas, area);
}

/// Draw realistic sky gradient with atmospheric effects
fn draw_sky_gradient(
    ctx: &mut Context,
    is_day: bool,
    temperature: f64,
    condition: &WeatherCondition,
) {
    let layers = 25;

    for layer in 0..layers {
        let y_start = 100.0 + (layer as f64 * 4.0);
        let y_end = y_start + 4.0;
        let intensity = layer as f64 / layers as f64;

        let color = if is_day {
            match condition {
                WeatherCondition::Thunderstorm => {
                    if intensity < 0.3 {
                        Color::Black
                    } else if intensity < 0.7 {
                        Color::DarkGray
                    } else {
                        Color::Gray
                    }
                }
                WeatherCondition::Fog | WeatherCondition::Mist => {
                    if intensity < 0.5 {
                        Color::Gray
                    } else {
                        Color::White
                    }
                }
                _ => {
                    if temperature > 35.0 {
                        if intensity < 0.2 {
                            Color::Yellow
                        } else if intensity < 0.5 {
                            Color::LightYellow
                        } else if intensity < 0.8 {
                            Color::LightBlue
                        } else {
                            Color::Blue
                        }
                    } else if temperature < 5.0 {
                        if intensity < 0.3 {
                            Color::White
                        } else if intensity < 0.7 {
                            Color::LightBlue
                        } else {
                            Color::Blue
                        }
                    } else {
                        if intensity < 0.4 {
                            Color::LightBlue
                        } else if intensity < 0.8 {
                            Color::Blue
                        } else {
                            Color::Blue
                        }
                    }
                }
            }
        } else {
            match condition {
                WeatherCondition::Clear => {
                    if intensity < 0.6 {
                        Color::Black
                    } else if intensity < 0.9 {
                        Color::Blue
                    } else {
                        Color::Blue
                    }
                }
                _ => {
                    if intensity < 0.8 {
                        Color::Black
                    } else {
                        Color::DarkGray
                    }
                }
            }
        };

        for y in (y_start as u32)..=(y_end as u32) {
            ctx.draw(&Line {
                x1: 0.0,
                y1: y as f64,
                x2: 400.0,
                y2: y as f64,
                color,
            });
        }
    }
}

/// Draw detailed ground terrain with environmental adaptation
fn draw_ground_terrain(ctx: &mut Context, condition: &WeatherCondition) {
    // Main horizon line
    ctx.draw(&Line {
        x1: 0.0,
        y1: 50.0,
        x2: 400.0,
        y2: 50.0,
        color: Color::Green,
    });

    // Ground surface with condition-specific details
    for y in 0..50 {
        let base_color = match condition {
            WeatherCondition::Snow => Color::White,
            WeatherCondition::Rain | WeatherCondition::Drizzle => Color::DarkGray,
            WeatherCondition::Fog | WeatherCondition::Mist => Color::Gray,
            _ => {
                if y > 30 {
                    Color::Green
                } else {
                    Color::DarkGray
                }
            }
        };

        let density = match y {
            0..=15 => 30,
            16..=35 => 20,
            _ => 12,
        };

        for x in (0..400).step_by(density) {
            let offset_x = x as f64 + ((y as f64 * 0.5).sin() * 3.0);
            if offset_x >= 0.0 && offset_x < 400.0 {
                ctx.draw(&Points {
                    coords: &[(offset_x, y as f64)],
                    color: base_color,
                });
            }
        }
    }

    // Add ground features based on weather
    match condition {
        WeatherCondition::Rain | WeatherCondition::Drizzle => {
            draw_puddles(ctx);
        }
        WeatherCondition::Snow => {
            draw_snow_drifts(ctx);
        }
        _ => {
            draw_grass_details(ctx);
        }
    }
}

/// Draw a magnificent, realistic sun
fn draw_magnificent_sun(ctx: &mut Context, x: f64, y: f64, temperature: f64) {
    let sun_color = match temperature as i32 {
        t if t > 40 => Color::Red,
        t if t > 30 => Color::LightRed,
        t if t > 20 => Color::Yellow,
        t if t > 10 => Color::LightYellow,
        _ => Color::White,
    };

    // Sun corona (outermost glow)
    for radius in (25..35).step_by(2) {
        ctx.draw(&Circle {
            x,
            y,
            radius: radius as f64,
            color: Color::LightYellow,
        });
    }

    // Main sun body with layered effect
    for radius in (8..18).rev().step_by(2) {
        let layer_color = if radius > 14 {
            sun_color
        } else if radius > 10 {
            Color::Yellow
        } else {
            Color::LightYellow
        };

        ctx.draw(&Circle {
            x,
            y,
            radius: radius as f64,
            color: layer_color,
        });
    }

    // Brilliant sun rays with varying lengths and thickness
    for ray in 0..24 {
        let angle = (ray as f64) * PI / 12.0;
        let is_major_ray = ray % 3 == 0;
        let ray_length = if is_major_ray { 45.0 } else { 30.0 };
        let thickness = if is_major_ray { 4 } else { 2 };

        let start_radius = 20.0;
        let start_x = x + start_radius * angle.cos();
        let start_y = y + start_radius * angle.sin();
        let end_x = x + (start_radius + ray_length) * angle.cos();
        let end_y = y + (start_radius + ray_length) * angle.sin();

        for t in 0..thickness {
            let offset = (t as f64 - thickness as f64 / 2.0) * 0.5;
            let perp_angle = angle + PI / 2.0;

            ctx.draw(&Line {
                x1: start_x + offset * perp_angle.cos(),
                y1: start_y + offset * perp_angle.sin(),
                x2: end_x + offset * perp_angle.cos(),
                y2: end_y + offset * perp_angle.sin(),
                color: if t < thickness / 2 {
                    sun_color
                } else {
                    Color::Yellow
                },
            });
        }
    }

    // Add sun spots for realism
    let spots = [(x - 5.0, y + 3.0), (x + 4.0, y - 2.0), (x - 2.0, y - 6.0)];
    for (sx, sy) in spots.iter() {
        ctx.draw(&Circle {
            x: *sx,
            y: *sy,
            radius: 1.5,
            color: Color::Yellow,
        });
    }
}

/// Draw a beautiful, detailed moon with phases
fn draw_beautiful_moon(ctx: &mut Context, x: f64, y: f64) {
    // Moon glow/halo
    for radius in (20..28).step_by(2) {
        ctx.draw(&Circle {
            x,
            y,
            radius: radius as f64,
            color: Color::Gray,
        });
    }

    // Main moon body
    ctx.draw(&Circle {
        x,
        y,
        radius: 15.0,
        color: Color::White,
    });

    // Detailed crater system
    let craters = [
        (x - 6.0, y + 4.0, 3.0, Color::Gray),
        (x + 5.0, y - 3.0, 2.5, Color::DarkGray),
        (x - 2.0, y - 7.0, 2.0, Color::Gray),
        (x + 8.0, y + 2.0, 1.5, Color::DarkGray),
        (x - 4.0, y - 2.0, 1.8, Color::Gray),
        (x + 2.0, y + 6.0, 2.2, Color::DarkGray),
    ];

    for (cx, cy, cr, color) in craters.iter() {
        ctx.draw(&Circle {
            x: *cx,
            y: *cy,
            radius: *cr,
            color: *color,
        });

        // Crater rim highlights
        ctx.draw(&Circle {
            x: cx - 0.5,
            y: cy - 0.5,
            radius: cr + 0.8,
            color: Color::White,
        });
    }

    // Mare (dark areas)
    let mare_areas = [(x - 3.0, y + 8.0, 4.0), (x + 6.0, y - 5.0, 3.5)];

    for (mx, my, mr) in mare_areas.iter() {
        ctx.draw(&Circle {
            x: *mx,
            y: *my,
            radius: *mr,
            color: Color::DarkGray,
        });
    }
}

/// Draw a brilliant stellar field
fn draw_stellar_field(ctx: &mut Context) {
    let constellations = [
        // Big Dipper pattern
        [
            (80.0, 170.0),
            (90.0, 175.0),
            (100.0, 172.0),
            (115.0, 170.0),
            (125.0, 165.0),
            (130.0, 160.0),
            (120.0, 155.0),
        ],
        // Orion pattern
        [
            (300.0, 180.0),
            (310.0, 175.0),
            (320.0, 170.0),
            (315.0, 160.0),
            (305.0, 165.0),
            (325.0, 165.0),
            (330.0, 155.0),
        ],
    ];

    // Draw constellation patterns
    for constellation in constellations.iter() {
        // Connect stars with faint lines
        for i in 0..constellation.len() - 1 {
            ctx.draw(&Line {
                x1: constellation[i].0,
                y1: constellation[i].1,
                x2: constellation[i + 1].0,
                y2: constellation[i + 1].1,
                color: Color::DarkGray,
            });
        }

        // Draw bright stars
        for (sx, sy) in constellation.iter() {
            draw_twinkling_star(ctx, *sx, *sy, 3.0);
        }
    }

    // Additional scattered stars
    let scattered_stars = [
        (50.0, 185.0, 2.0),
        (150.0, 180.0, 2.5),
        (200.0, 190.0, 2.0),
        (250.0, 185.0, 3.0),
        (350.0, 175.0, 2.5),
        (370.0, 190.0, 2.0),
        (20.0, 165.0, 1.5),
        (180.0, 160.0, 2.0),
        (280.0, 195.0, 2.5),
    ];

    for (sx, sy, size) in scattered_stars.iter() {
        draw_twinkling_star(ctx, *sx, *sy, *size);
    }
}

/// Draw a twinkling star with cross pattern
fn draw_twinkling_star(ctx: &mut Context, x: f64, y: f64, size: f64) {
    let time = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_millis();
    let twinkle = ((time as f64 * 0.01).sin() * 0.3 + 0.7).max(0.4);
    let brightness = (size * twinkle) as u8;

    let star_color = match brightness {
        b if b > 2 => Color::White,
        b if b > 1 => Color::LightBlue,
        _ => Color::Gray,
    };

    // Star center
    ctx.draw(&Points {
        coords: &[(x, y)],
        color: star_color,
    });

    // Star rays (cross pattern)
    let ray_points = [
        (x - size, y),
        (x + size, y),
        (x, y - size),
        (x, y + size),
        (x - size * 0.7, y - size * 0.7),
        (x + size * 0.7, y + size * 0.7),
        (x - size * 0.7, y + size * 0.7),
        (x + size * 0.7, y - size * 0.7),
    ];

    ctx.draw(&Points {
        coords: &ray_points,
        color: star_color,
    });
}

/// Draw realistic cloud formations with depth and detail
fn draw_cloud_formations(ctx: &mut Context, humidity: u8, is_day: bool, is_storm: bool) {
    let base_color = if is_storm {
        Color::Black
    } else if is_day {
        Color::White
    } else {
        Color::Gray
    };

    let num_clouds = ((humidity / 15).max(2).min(8)) as usize;
    let cloud_data = [
        (60.0, 140.0, 32.0, 0.8),
        (140.0, 155.0, 38.0, 1.0),
        (220.0, 135.0, 35.0, 0.9),
        (300.0, 150.0, 30.0, 0.7),
        (380.0, 140.0, 28.0, 0.8),
        (100.0, 125.0, 25.0, 0.6),
        (260.0, 165.0, 33.0, 0.9),
        (340.0, 130.0, 29.0, 0.7),
    ];

    for (_i, (x, y, size, opacity)) in cloud_data.iter().take(num_clouds).enumerate() {
        draw_realistic_cloud(ctx, *x, *y, *size, base_color, *opacity, is_storm);

        // Add cloud shadows on ground if it's day
        if is_day && !is_storm {
            draw_cloud_shadow(ctx, *x, *size);
        }
    }
}

/// Draw a single realistic cloud with multiple layers and detail
fn draw_realistic_cloud(
    ctx: &mut Context,
    x: f64,
    y: f64,
    size: f64,
    base_color: Color,
    _opacity: f64,
    is_storm: bool,
) {
    let num_puffs = if is_storm { 8 } else { 6 };

    // Cloud puffs arranged in natural formation
    let puff_positions = [
        (x - size * 0.7, y + size * 0.2, size * 0.8),
        (x - size * 0.3, y + size * 0.6, size * 1.0),
        (x + size * 0.1, y + size * 0.3, size * 0.9),
        (x + size * 0.5, y + size * 0.1, size * 0.7),
        (x + size * 0.8, y - size * 0.2, size * 0.6),
        (x + size * 0.2, y - size * 0.4, size * 0.8),
        (x - size * 0.1, y - size * 0.1, size * 0.5),
        (x - size * 0.4, y - size * 0.3, size * 0.6),
    ];

    // Draw cloud layers for depth
    for layer in 0..3 {
        let layer_offset = layer as f64 * 2.0;
        let layer_color = match (layer, is_storm) {
            (0, true) => Color::Black,
            (1, true) => Color::DarkGray,
            (2, true) => Color::Gray,
            (0, false) => base_color,
            (1, false) => Color::White,
            (2, false) => Color::LightBlue,
            _ => base_color,
        };

        for (_i, (px, py, psize)) in puff_positions.iter().take(num_puffs).enumerate() {
            let adjusted_size = psize * (1.0 - layer as f64 * 0.1);
            ctx.draw(&Circle {
                x: px + layer_offset,
                y: py + layer_offset,
                radius: adjusted_size,
                color: layer_color,
            });
        }
    }

    // Add storm cloud specific effects
    if is_storm {
        // Dark underbelly
        ctx.draw(&Rectangle {
            x: x - size,
            y: y - size * 0.5,
            width: size * 2.0,
            height: 12.0,
            color: Color::Black,
        });

        // Menacing wisps
        for wisp in 0..5 {
            let wisp_x = x - size + (wisp as f64 * size * 0.5);
            let wisp_y = y - size * 0.8;
            ctx.draw(&Line {
                x1: wisp_x,
                y1: wisp_y,
                x2: wisp_x + 8.0,
                y2: wisp_y - 15.0,
                color: Color::DarkGray,
            });
        }
    }
}

/// Draw cloud shadow on ground
fn draw_cloud_shadow(ctx: &mut Context, cloud_x: f64, cloud_size: f64) {
    let shadow_y = 45.0;
    let shadow_width = cloud_size * 1.5;

    for x in 0..(shadow_width as u32) {
        let shadow_x = cloud_x - shadow_width / 2.0 + x as f64;
        if shadow_x >= 0.0 && shadow_x < 400.0 {
            let opacity_factor = 1.0 - (x as f64 - shadow_width / 2.0).abs() / (shadow_width / 2.0);
            if opacity_factor > 0.3 {
                ctx.draw(&Points {
                    coords: &[(shadow_x, shadow_y)],
                    color: Color::DarkGray,
                });
            }
        }
    }
}

/// Draw sun partially visible through clouds
fn draw_sun_through_clouds(ctx: &mut Context, x: f64, y: f64) {
    // Muted sun disc
    ctx.draw(&Circle {
        x,
        y,
        radius: 12.0,
        color: Color::LightYellow,
    });

    // Sun rays breaking through clouds
    for ray in 0..12 {
        let angle = (ray as f64) * PI / 6.0;
        let ray_length = 30.0 + (ray % 3) as f64 * 10.0;

        let _end_x = x + ray_length * angle.cos();
        let _end_y = y + ray_length * angle.sin();

        // Gradient ray effect
        for segment in 0..6 {
            let seg_factor = segment as f64 / 6.0;
            let seg_x = x + (ray_length * seg_factor) * angle.cos();
            let seg_y = y + (ray_length * seg_factor) * angle.sin();

            let seg_color = if seg_factor < 0.5 {
                Color::LightYellow
            } else {
                Color::Yellow
            };

            ctx.draw(&Points {
                coords: &[(seg_x, seg_y)],
                color: seg_color,
            });
        }
    }
}

/// Draw detailed rain system with varying intensity
fn draw_rain_system(ctx: &mut Context, heavy_rain: bool, wind_speed: f64) {
    // Rain clouds
    draw_cloud_formations(ctx, 90, true, false);

    // Animate rain drops
    let time = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_millis();
    let animation_offset = (time / 120) % 80;

    let drop_density = if heavy_rain { 70 } else { 45 };
    let drop_length = if heavy_rain { 18.0 } else { 12.0 };
    let wind_lean = (wind_speed * 0.8).min(8.0);

    for i in 0..drop_density {
        for layer in 0..25 {
            let base_x = (i * 6) as f64;
            let fall_speed = if heavy_rain { 10 } else { 8 };
            let y_pos =
                ((layer * fall_speed + animation_offset as usize + i * 2) % 140 + 60) as f64;

            // Wind effect on rain angle
            let wind_offset = (y_pos - 60.0) * wind_lean * 0.02;
            let final_x = base_x + wind_offset;

            if final_x >= 0.0 && final_x < 400.0 && y_pos > 50.0 {
                let drop_bottom = y_pos - drop_length;

                // Main raindrop
                ctx.draw(&Line {
                    x1: final_x,
                    y1: y_pos,
                    x2: final_x + wind_lean * 0.3,
                    y2: drop_bottom,
                    color: Color::Blue,
                });

                // Raindrop highlight
                ctx.draw(&Line {
                    x1: final_x + 0.5,
                    y1: y_pos,
                    x2: final_x + 0.5 + wind_lean * 0.3,
                    y2: drop_bottom,
                    color: Color::LightBlue,
                });

                // Ground impact splash
                if y_pos < 65.0 {
                    let splash_points = [
                        (final_x - 3.0, 50.0),
                        (final_x + 3.0, 50.0),
                        (final_x - 1.5, 48.0),
                        (final_x + 1.5, 48.0),
                        (final_x, 46.0),
                    ];
                    ctx.draw(&Points {
                        coords: &splash_points,
                        color: Color::LightBlue,
                    });
                }
            }
        }
    }

    // Puddle formation
    draw_puddles(ctx);
}

/// Draw puddles on the ground
fn draw_puddles(ctx: &mut Context) {
    let puddles = [
        (70.0, 42.0, 35.0),
        (180.0, 45.0, 40.0),
        (290.0, 44.0, 30.0),
        (360.0, 43.0, 25.0),
    ];

    for (px, py, width) in puddles.iter() {
        // Puddle reflection
        for w in 0..(*width as usize / 2) {
            let ripple_x = px - (width / 2.0) + (w as f64 * 2.0);
            let ripple_y = py + ((w as f64 * 0.3).sin() * 1.5);

            ctx.draw(&Points {
                coords: &[(ripple_x, ripple_y), (ripple_x + 1.0, ripple_y)],
                color: Color::LightBlue,
            });
        }

        // Puddle edges
        ctx.draw(&Points {
            coords: &[(px - width / 2.0, *py), (px + width / 2.0, *py)],
            color: Color::Blue,
        });
    }
}

/// Draw dramatic thunderstorm system
fn draw_storm_system(ctx: &mut Context, wind_speed: f64) {
    // Massive storm clouds
    draw_cloud_formations(ctx, 95, true, true);

    // Lightning system
    let time = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_millis();
    let lightning_cycle = time % 4000;

    if lightning_cycle < 150 || (lightning_cycle > 2000 && lightning_cycle < 2100) {
        draw_lightning_bolt(ctx, 180.0, 160.0);
    }
    if lightning_cycle > 500 && lightning_cycle < 600 {
        draw_lightning_bolt(ctx, 280.0, 150.0);
    }

    // Heavy storm rain
    draw_torrential_rain(ctx, wind_speed);

    // Ground effects
    draw_storm_ground_effects(ctx);
}

/// Draw realistic lightning bolt with branching
fn draw_lightning_bolt(ctx: &mut Context, start_x: f64, start_y: f64) {
    let main_path = [
        (start_x, start_y),
        (start_x + 12.0, start_y - 25.0),
        (start_x - 8.0, start_y - 45.0),
        (start_x + 18.0, start_y - 70.0),
        (start_x - 5.0, start_y - 95.0),
        (start_x + 15.0, start_y - 120.0),
        (start_x + 2.0, start_y - 140.0),
    ];

    // Main lightning channel
    for i in 0..main_path.len() - 1 {
        let (x1, y1) = main_path[i];
        let (x2, y2) = main_path[i + 1];

        // Multi-stroke lightning effect
        for stroke in 0..6 {
            let offset = (stroke as f64 - 2.5) * 0.8;
            ctx.draw(&Line {
                x1: x1 + offset,
                y1,
                x2: x2 + offset,
                y2,
                color: match stroke {
                    0..=1 => Color::White,
                    2..=3 => Color::LightYellow,
                    _ => Color::Yellow,
                },
            });
        }
    }

    // Lightning branches
    for (i, (x, y)) in main_path.iter().enumerate().skip(1).step_by(2) {
        // Left branches
        let branch_end_x = x - 20.0 - (i as f64 * 3.0);
        let branch_end_y = y - 15.0;

        ctx.draw(&Line {
            x1: *x,
            y1: *y,
            x2: branch_end_x,
            y2: branch_end_y,
            color: Color::LightYellow,
        });

        // Right branches
        let branch_end_x = x + 15.0 + (i as f64 * 2.0);
        let branch_end_y = y - 12.0;

        ctx.draw(&Line {
            x1: *x,
            y1: *y,
            x2: branch_end_x,
            y2: branch_end_y,
            color: Color::Yellow,
        });

        // Sub-branches
        ctx.draw(&Line {
            x1: branch_end_x,
            y1: branch_end_y,
            x2: branch_end_x + 8.0,
            y2: branch_end_y - 8.0,
            color: Color::Yellow,
        });
    }

    // Ground strike flash
    ctx.draw(&Circle {
        x: start_x + 2.0,
        y: 50.0,
        radius: 8.0,
        color: Color::White,
    });
}

/// Draw torrential rain for storm systems
fn draw_torrential_rain(ctx: &mut Context, wind_speed: f64) {
    let time = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_millis();
    let rain_offset = (time / 80) % 60;
    let wind_lean = (wind_speed * 1.2).min(12.0);

    for i in 0..90 {
        for layer in 0..30 {
            let base_x = (i * 5) as f64;
            let y_pos = ((layer * 12 + rain_offset as usize + i * 3) % 120 + 60) as f64;
            let wind_offset = (y_pos - 60.0) * wind_lean * 0.03;
            let final_x = base_x + wind_offset;

            if final_x >= 0.0 && final_x < 400.0 && y_pos > 50.0 {
                let drop_length = 25.0;
                let drop_bottom = y_pos - drop_length;

                // Heavy rain strokes
                for thickness in 0..4 {
                    ctx.draw(&Line {
                        x1: final_x + thickness as f64 * 0.5,
                        y1: y_pos,
                        x2: final_x + thickness as f64 * 0.5 + wind_lean * 0.4,
                        y2: drop_bottom,
                        color: if thickness < 2 {
                            Color::Blue
                        } else {
                            Color::LightBlue
                        },
                    });
                }
            }
        }
    }
}

/// Draw storm ground effects
fn draw_storm_ground_effects(ctx: &mut Context) {
    // Large puddles with ripples
    let storm_puddles = [(80.0, 40.0, 50.0), (200.0, 43.0, 60.0), (320.0, 41.0, 45.0)];

    let time = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_millis();
    let ripple_phase = (time / 200) % 20;

    for (px, py, width) in storm_puddles.iter() {
        // Puddle base
        for w in 0..(*width as usize / 3) {
            let puddle_x = px - (width / 2.0) + (w as f64 * 3.0);
            ctx.draw(&Points {
                coords: &[(puddle_x, *py), (puddle_x + 1.0, *py)],
                color: Color::Blue,
            });
        }

        // Animated ripples
        for ripple in 0..3 {
            let ripple_radius = ((ripple_phase + ripple * 7) % 20) as f64 * 2.0;
            if ripple_radius < width / 2.0 {
                for angle in (0..360).step_by(30) {
                    let angle_rad = (angle as f64) * PI / 180.0;
                    let ripple_x = px + ripple_radius * angle_rad.cos();
                    let ripple_y = py + (ripple_radius * 0.3) * angle_rad.sin();

                    if ripple_x >= px - width / 2.0 && ripple_x <= px + width / 2.0 {
                        ctx.draw(&Points {
                            coords: &[(ripple_x, ripple_y)],
                            color: Color::LightBlue,
                        });
                    }
                }
            }
        }
    }
}

/// Draw beautiful snow system with different flake types
fn draw_snow_system(ctx: &mut Context, temperature: f64, wind_speed: f64) {
    // Snow clouds
    draw_cloud_formations(ctx, 80, true, false);

    let time = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_millis();
    let snow_frame = (time / 500) % 60;
    let wind_drift = wind_speed * 0.8;

    let flake_count = if temperature < -5.0 { 45 } else { 35 };

    for i in 0..flake_count {
        for layer in 0..20 {
            let base_x = (i * 9) as f64;
            let base_drift = 12.0 * ((layer as f64 * 0.15).sin());
            let wind_drift_effect = layer as f64 * wind_drift * 0.02;
            let final_x = base_x + base_drift + wind_drift_effect;

            let fall_speed = if temperature < -10.0 { 8 } else { 6 };
            let y_pos = ((layer * fall_speed + snow_frame as usize + i * 4) % 140 + 60) as f64;

            if final_x >= 0.0 && final_x < 400.0 && y_pos < 180.0 {
                let flake_type = (i + layer) % 6;
                draw_detailed_snowflake(ctx, final_x, y_pos, flake_type, temperature);
            }
        }
    }

    // Snow accumulation on ground
    draw_snow_drifts(ctx);
}

/// Draw detailed snowflakes with various patterns
fn draw_detailed_snowflake(ctx: &mut Context, x: f64, y: f64, flake_type: usize, temperature: f64) {
    let size = if temperature < -10.0 { 4.0 } else { 3.0 };

    match flake_type {
        0 => {
            // Classic six-pointed star
            for arm in 0..6 {
                let angle = (arm as f64) * PI / 3.0;
                let end_x = x + size * angle.cos();
                let end_y = y + size * angle.sin();

                ctx.draw(&Line {
                    x1: x,
                    y1: y,
                    x2: end_x,
                    y2: end_y,
                    color: Color::White,
                });

                // Cross branches
                let mid_x = x + (size * 0.6) * angle.cos();
                let mid_y = y + (size * 0.6) * angle.sin();
                let perp_angle = angle + PI / 2.0;

                ctx.draw(&Line {
                    x1: mid_x + 1.5 * perp_angle.cos(),
                    y1: mid_y + 1.5 * perp_angle.sin(),
                    x2: mid_x - 1.5 * perp_angle.cos(),
                    y2: mid_y - 1.5 * perp_angle.sin(),
                    color: Color::White,
                });
            }
        }
        1 => {
            // Hexagonal plate
            let hex_points = (0..6)
                .map(|i| {
                    let angle = (i as f64) * PI / 3.0;
                    (x + size * angle.cos(), y + size * angle.sin())
                })
                .collect::<Vec<_>>();

            for i in 0..6 {
                let next_i = (i + 1) % 6;
                ctx.draw(&Line {
                    x1: hex_points[i].0,
                    y1: hex_points[i].1,
                    x2: hex_points[next_i].0,
                    y2: hex_points[next_i].1,
                    color: Color::White,
                });
            }

            // Center dot
            ctx.draw(&Points {
                coords: &[(x, y)],
                color: Color::White,
            });
        }
        2 => {
            // Dendrite pattern
            let main_arms = [(x, y - size), (x, y + size), (x - size, y), (x + size, y)];

            for &(end_x, end_y) in main_arms.iter() {
                ctx.draw(&Line {
                    x1: x,
                    y1: y,
                    x2: end_x,
                    y2: end_y,
                    color: Color::White,
                });

                // Branching
                for branch in 1..3 {
                    let branch_factor = branch as f64 / 3.0;
                    let branch_x = x + (end_x - x) * branch_factor;
                    let branch_y = y + (end_y - y) * branch_factor;

                    ctx.draw(&Points {
                        coords: &[
                            (branch_x + 1.0, branch_y + 1.0),
                            (branch_x - 1.0, branch_y - 1.0),
                        ],
                        color: Color::White,
                    });
                }
            }
        }
        3 => {
            // Stellar dendrite
            for arm in 0..8 {
                let angle = (arm as f64) * PI / 4.0;
                let arm_length = if arm % 2 == 0 { size } else { size * 0.7 };
                let end_x = x + arm_length * angle.cos();
                let end_y = y + arm_length * angle.sin();

                ctx.draw(&Line {
                    x1: x,
                    y1: y,
                    x2: end_x,
                    y2: end_y,
                    color: Color::White,
                });
            }
        }
        4 => {
            // Column crystal
            ctx.draw(&Rectangle {
                x: x - 1.0,
                y: y - size,
                width: 2.0,
                height: size * 2.0,
                color: Color::White,
            });

            // End caps
            ctx.draw(&Line {
                x1: x - 2.0,
                y1: y - size,
                x2: x + 2.0,
                y2: y - size,
                color: Color::White,
            });
            ctx.draw(&Line {
                x1: x - 2.0,
                y1: y + size,
                x2: x + 2.0,
                y2: y + size,
                color: Color::White,
            });
        }
        _ => {
            // Simple cross
            let cross_points = [
                (x, y),
                (x - size, y),
                (x + size, y),
                (x, y - size),
                (x, y + size),
            ];
            ctx.draw(&Points {
                coords: &cross_points,
                color: Color::White,
            });
        }
    }
}

/// Draw snow drifts on the ground
fn draw_snow_drifts(ctx: &mut Context) {
    // Variable snow depth creating natural drifts
    for x in 0..400 {
        let drift_height = 8.0 + 6.0 * ((x as f64 * 0.02).sin()) + 3.0 * ((x as f64 * 0.05).cos());
        let snow_depth = drift_height.max(2.0).min(15.0);

        for y in 0..(snow_depth as u32) {
            let ground_y = 50.0 - y as f64;
            if ground_y >= 35.0 {
                let snow_density = if y < (snow_depth as u32 / 2) { 8 } else { 4 };
                if x % snow_density == 0 {
                    ctx.draw(&Points {
                        coords: &[(x as f64, ground_y)],
                        color: Color::White,
                    });
                }
            }
        }
    }

    // Snow mounds and texture
    let mounds = [(120.0, 48.0, 25.0), (280.0, 47.0, 30.0)];
    for (mx, my, mw) in mounds.iter() {
        for w in 0..(*mw as usize / 2) {
            let mound_x = mx - (mw / 2.0) + (w as f64 * 2.0);
            let mound_height = 3.0 * (1.0 - (w as f64 - mw / 2.0).abs() / (mw / 2.0));

            for h in 0..(mound_height as u32) {
                ctx.draw(&Points {
                    coords: &[(mound_x, my + h as f64)],
                    color: Color::White,
                });
            }
        }
    }
}

/// Draw atmospheric fog system
fn draw_fog_system(ctx: &mut Context, thick_fog: bool, wind_speed: f64) {
    let layers = if thick_fog { 18 } else { 12 };
    let time = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_millis();
    let fog_drift = (time as f64 * 0.02).sin() * wind_speed * 0.5;

    // Multi-layered fog with realistic movement
    for layer in 0..layers {
        let base_y = 55.0 + (layer as f64 * 7.0);
        let layer_density = if thick_fog { 6 } else { 8 };
        let fog_opacity = 1.0 - (layer as f64 / layers as f64) * 0.6;

        for x in (0..400).step_by(layer_density) {
            let wave1 = 8.0 * ((x as f64 * 0.015 + fog_drift).sin());
            let wave2 = 4.0 * ((x as f64 * 0.03 + layer as f64 * 0.5).cos());
            let final_y = base_y + wave1 + wave2;

            let fog_color = match (layer % 4, thick_fog) {
                (0, true) => Color::White,
                (1, true) => Color::Gray,
                (2, true) => Color::DarkGray,
                (3, true) => Color::Black,
                (0, false) => Color::White,
                (1, false) => Color::Gray,
                _ => Color::DarkGray,
            };

            if fog_opacity > 0.3 {
                // Horizontal fog streaks
                ctx.draw(&Line {
                    x1: x as f64,
                    y1: final_y,
                    x2: (x + layer_density) as f64,
                    y2: final_y,
                    color: fog_color,
                });

                // Vertical fog wisps
                if x % (layer_density * 2) == 0 {
                    ctx.draw(&Line {
                        x1: x as f64,
                        y1: final_y,
                        x2: x as f64,
                        y2: final_y + 5.0,
                        color: fog_color,
                    });
                }
            }
        }
    }

    // Fog tendrils and swirls
    draw_fog_tendrils(ctx, wind_speed, thick_fog);
}

/// Draw realistic fog tendrils
fn draw_fog_tendrils(ctx: &mut Context, wind_speed: f64, thick_fog: bool) {
    let tendril_count = if thick_fog { 12 } else { 8 };
    let time = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_millis();
    let motion = (time as f64 * 0.01).sin() * wind_speed * 0.3;

    for tendril in 0..tendril_count {
        let start_x = (tendril * 35) as f64 + motion;
        let start_y = 80.0 + (tendril as f64 * 3.0);

        // Curved tendril path
        let tendril_points = [
            (start_x, start_y),
            (start_x + 20.0 + motion * 0.5, start_y + 12.0),
            (start_x + 35.0 + motion, start_y + 8.0),
            (start_x + 50.0 + motion * 1.5, start_y + 15.0),
            (start_x + 65.0 + motion * 0.8, start_y + 5.0),
        ];

        // Draw smooth tendril curves
        for i in 0..tendril_points.len() - 1 {
            let (x1, y1) = tendril_points[i];
            let (x2, y2) = tendril_points[i + 1];

            if x1 >= 0.0 && x1 < 400.0 && x2 >= 0.0 && x2 < 400.0 {
                // Main tendril
                ctx.draw(&Line {
                    x1,
                    y1,
                    x2,
                    y2,
                    color: Color::Gray,
                });

                // Tendril thickness
                ctx.draw(&Line {
                    x1,
                    y1: y1 + 1.0,
                    x2,
                    y2: y2 + 1.0,
                    color: Color::White,
                });

                // Wispy edges
                if thick_fog {
                    ctx.draw(&Line {
                        x1,
                        y1: y1 - 1.0,
                        x2,
                        y2: y2 - 1.0,
                        color: Color::DarkGray,
                    });
                }
            }
        }
    }
}

/// Draw dynamic wind patterns
fn draw_wind_patterns(ctx: &mut Context, wind_speed: f64) {
    let time = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_millis();
    let motion_offset = (time / 150) % 200;
    let num_streams = ((wind_speed / 6.0).min(10.0).max(3.0)) as usize;

    for stream in 0..num_streams {
        let base_y = 70.0 + (stream as f64 * 18.0);
        let wave_amplitude = 8.0 + wind_speed * 0.3;
        let stream_speed = 3.0 + wind_speed * 0.2;

        // Wind stream lines
        for segment in 0..25 {
            let x = (segment * 18) as f64;
            let moving_x = (x + motion_offset as f64 * stream_speed) % 450.0;

            if moving_x < 400.0 {
                let wave_y = base_y + wave_amplitude * ((moving_x * 0.02 + stream as f64).sin());
                let end_x = moving_x + 25.0;
                let end_y = wave_y + 2.0 * ((end_x * 0.02).sin());

                // Wind streak colors based on intensity
                let wind_color = match wind_speed as u32 {
                    s if s > 20 => Color::Red,    // Severe winds
                    s if s > 15 => Color::Yellow, // Strong winds
                    s if s > 10 => Color::White,  // Moderate winds
                    _ => Color::Gray,             // Light winds
                };

                // Main wind line
                ctx.draw(&Line {
                    x1: moving_x,
                    y1: wave_y,
                    x2: end_x.min(400.0),
                    y2: end_y,
                    color: wind_color,
                });

                // Wind particles
                if segment % 3 == 0 {
                    let particle_points = [
                        (moving_x + 8.0, wave_y - 2.0),
                        (moving_x + 12.0, wave_y + 1.0),
                        (moving_x + 16.0, wave_y - 1.0),
                    ];

                    ctx.draw(&Points {
                        coords: &particle_points,
                        color: wind_color,
                    });
                }
            }
        }
    }
}

/// Draw grass details for clear weather
fn draw_grass_details(ctx: &mut Context) {
    let time = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_millis();
    let sway = ((time as f64 * 0.001).sin() * 2.0) as i32;

    for x in (0..400).step_by(12) {
        let grass_height = 3 + (x % 8);
        for h in 0..grass_height {
            let grass_x = x as f64 + sway as f64 * (h as f64 * 0.3);
            let grass_y = 50.0 + h as f64;

            if grass_x >= 0.0 && grass_x < 400.0 {
                ctx.draw(&Points {
                    coords: &[(grass_x, grass_y)],
                    color: if h > grass_height / 2 {
                        Color::Green
                    } else {
                        Color::DarkGray
                    },
                });
            }
        }
    }
}

/// Draw comprehensive weather data indicators
fn draw_weather_indicators(ctx: &mut Context, temperature: f64, humidity: u8, wind_speed: f64) {
    let panel_x = 15.0;
    let panel_y = 185.0;

    // Temperature thermometer
    let temp_height = (temperature.abs() * 1.5).min(35.0);
    let temp_color = match temperature as i32 {
        t if t > 35 => Color::Red,
        t if t > 25 => Color::LightRed,
        t if t > 15 => Color::Yellow,
        t if t > 5 => Color::Green,
        t if t > -5 => Color::LightBlue,
        _ => Color::Blue,
    };

    // Thermometer bulb
    ctx.draw(&Circle {
        x: panel_x,
        y: panel_y - 40.0,
        radius: 3.0,
        color: temp_color,
    });

    // Thermometer tube
    for h in 0..(temp_height as u32) {
        ctx.draw(&Points {
            coords: &[(panel_x, panel_y - 37.0 + h as f64)],
            color: temp_color,
        });
    }

    // Humidity indicator (water drops)
    let humidity_drops = (humidity / 20).min(5);
    for drop in 0..humidity_drops {
        let drop_x = panel_x + 12.0;
        let drop_y = panel_y - 35.0 + (drop as f64 * 8.0);

        // Water drop shape
        ctx.draw(&Circle {
            x: drop_x,
            y: drop_y,
            radius: 2.0,
            color: Color::Blue,
        });
        ctx.draw(&Points {
            coords: &[(drop_x, drop_y - 3.0)],
            color: Color::LightBlue,
        });
    }

    // Wind speed indicator (flag)
    let wind_strength = (wind_speed / 5.0).min(6.0) as usize;
    let flag_x = panel_x + 25.0;
    let flag_y = panel_y - 35.0;

    // Flag pole
    ctx.draw(&Line {
        x1: flag_x,
        y1: flag_y,
        x2: flag_x,
        y2: flag_y + 30.0,
        color: Color::DarkGray,
    });

    // Wind flag segments
    for segment in 0..wind_strength {
        let segment_y = flag_y + (segment as f64 * 4.0);
        let flag_length = 8.0 - (segment as f64 * 1.0);

        ctx.draw(&Line {
            x1: flag_x,
            y1: segment_y,
            x2: flag_x + flag_length,
            y2: segment_y,
            color: Color::Red,
        });
    }

    // Panel frame
    ctx.draw(&Rectangle {
        x: panel_x - 5.0,
        y: panel_y - 45.0,
        width: 45.0,
        height: 50.0,
        color: Color::White,
    });
}

/// Render current weather canvas with improved error handling
pub fn render_current_weather_canvas<B: ratatui::backend::Backend>(
    hourly_data: &[HourlyForecast],
    frame: &mut Frame<B>,
    area: Rect,
) {
    if let Some(current) = hourly_data.first() {
        let is_day = is_daytime(&current.timestamp);

        render_weather_canvas(
            &current.main_condition,
            current.temperature,
            current.humidity,
            current.wind_speed,
            is_day,
            frame,
            area,
        );
    } else {
        // Enhanced fallback display for no data
        let canvas = Canvas::default()
            .block(
                Block::default()
                    .title("⚠️ No Weather Data Available")
                    .borders(Borders::ALL)
                    .style(ratatui::style::Style::default().fg(Color::Red)),
            )
            .x_bounds([0.0, 400.0])
            .y_bounds([0.0, 200.0])
            .paint(|ctx| {
                // Error background
                ctx.draw(&Rectangle {
                    x: 100.0,
                    y: 80.0,
                    width: 200.0,
                    height: 40.0,
                    color: Color::DarkGray,
                });

                // Error icon (warning triangle)
                let warning_points = [(200.0, 90.0), (190.0, 110.0), (210.0, 110.0)];
                for i in 0..warning_points.len() {
                    let next_i = (i + 1) % warning_points.len();
                    ctx.draw(&Line {
                        x1: warning_points[i].0,
                        y1: warning_points[i].1,
                        x2: warning_points[next_i].0,
                        y2: warning_points[next_i].1,
                        color: Color::Red,
                    });
                }

                // Exclamation point
                ctx.draw(&Line {
                    x1: 200.0,
                    y1: 95.0,
                    x2: 200.0,
                    y2: 105.0,
                    color: Color::Red,
                });
                ctx.draw(&Points {
                    coords: &[(200.0, 108.0)],
                    color: Color::Red,
                });
            });

        frame.render_widget(canvas, area);
    }
}

/// Enhanced daytime detection
fn is_daytime(timestamp: &chrono::DateTime<chrono::Utc>) -> bool {
    use chrono::Timelike;
    let hour = timestamp.hour();
    hour >= 6 && hour < 18
}

/// Render enhanced forecast canvas with detailed mini weather scenes
pub fn render_forecast_canvas<B: ratatui::backend::Backend>(
    daily_data: &[DailyForecast],
    frame: &mut Frame<B>,
    area: Rect,
) {
    let canvas = Canvas::default()
        .block(
            Block::default()
                .title("📅 7-Day Detailed Forecast")
                .borders(Borders::ALL)
                .style(ratatui::style::Style::default().fg(Color::Cyan)),
        )
        .x_bounds([0.0, 500.0])
        .y_bounds([0.0, 100.0])
        .paint(|ctx| {
            // Enhanced ground line with texture
            ctx.draw(&Line {
                x1: 0.0,
                y1: 20.0,
                x2: 500.0,
                y2: 20.0,
                color: Color::Green,
            });

            // Ground texture
            for x in (0..500).step_by(15) {
                ctx.draw(&Points {
                    coords: &[(x as f64, 18.0), (x as f64, 19.0)],
                    color: Color::DarkGray,
                });
            }

            // Draw each day with enhanced weather representations
            for (i, day) in daily_data.iter().take(7).enumerate() {
                let x_offset = i as f64 * 70.0 + 10.0;

                // Enhanced weather scene for each day
                match day.main_condition {
                    WeatherCondition::Clear => {
                        // Detailed mini sun
                        ctx.draw(&Circle {
                            x: x_offset + 30.0,
                            y: 65.0,
                            radius: 10.0,
                            color: Color::Yellow,
                        });

                        // Sun rays
                        for ray in 0..12 {
                            let angle = (ray as f64) * PI / 6.0;
                            let ray_length = if ray % 2 == 0 { 18.0 } else { 14.0 };
                            let end_x = x_offset + 30.0 + ray_length * angle.cos();
                            let end_y = 65.0 + ray_length * angle.sin();

                            ctx.draw(&Line {
                                x1: x_offset + 30.0,
                                y1: 65.0,
                                x2: end_x,
                                y2: end_y,
                                color: Color::LightYellow,
                            });
                        }
                    }
                    WeatherCondition::Clouds => {
                        // Detailed mini clouds
                        draw_realistic_cloud(
                            ctx,
                            x_offset + 30.0,
                            65.0,
                            8.0,
                            Color::White,
                            1.0,
                            false,
                        );
                    }
                    WeatherCondition::Rain | WeatherCondition::Drizzle => {
                        // Rain cloud with animation
                        draw_realistic_cloud(
                            ctx,
                            x_offset + 30.0,
                            70.0,
                            7.0,
                            Color::Gray,
                            1.0,
                            false,
                        );

                        // Mini rain drops
                        for drop in 0..8 {
                            let drop_x = x_offset + 25.0 + (drop as f64 * 2.0);
                            ctx.draw(&Line {
                                x1: drop_x,
                                y1: 60.0,
                                x2: drop_x,
                                y2: 45.0,
                                color: Color::Blue,
                            });
                        }
                    }
                    WeatherCondition::Thunderstorm => {
                        // Storm cloud with lightning
                        draw_realistic_cloud(
                            ctx,
                            x_offset + 30.0,
                            70.0,
                            8.0,
                            Color::DarkGray,
                            1.0,
                            true,
                        );

                        // Mini lightning bolt
                        let lightning_points = [
                            (x_offset + 30.0, 60.0),
                            (x_offset + 35.0, 50.0),
                            (x_offset + 28.0, 40.0),
                            (x_offset + 33.0, 30.0),
                        ];

                        for i in 0..lightning_points.len() - 1 {
                            ctx.draw(&Line {
                                x1: lightning_points[i].0,
                                y1: lightning_points[i].1,
                                x2: lightning_points[i + 1].0,
                                y2: lightning_points[i + 1].1,
                                color: Color::LightYellow,
                            });
                        }
                    }
                    WeatherCondition::Snow => {
                        // Snow cloud
                        draw_realistic_cloud(
                            ctx,
                            x_offset + 30.0,
                            70.0,
                            7.0,
                            Color::White,
                            1.0,
                            false,
                        );

                        // Mini snowflakes
                        let snow_positions = [
                            (x_offset + 25.0, 55.0),
                            (x_offset + 28.0, 50.0),
                            (x_offset + 32.0, 58.0),
                            (x_offset + 35.0, 52.0),
                            (x_offset + 30.0, 45.0),
                            (x_offset + 26.0, 48.0),
                        ];

                        for (sx, sy) in snow_positions.iter() {
                            // Mini snowflake pattern
                            let flake_arms = [
                                (*sx, *sy),
                                (*sx - 2.0, *sy),
                                (*sx + 2.0, *sy),
                                (*sx, *sy - 2.0),
                                (*sx, *sy + 2.0),
                            ];
                            ctx.draw(&Points {
                                coords: &flake_arms,
                                color: Color::White,
                            });
                        }
                    }
                    WeatherCondition::Fog | WeatherCondition::Mist => {
                        // Layered fog effect
                        for layer in 0..5 {
                            let fog_y = 45.0 + (layer as f64 * 4.0);
                            ctx.draw(&Line {
                                x1: x_offset + 20.0,
                                y1: fog_y,
                                x2: x_offset + 50.0,
                                y2: fog_y,
                                color: if layer % 2 == 0 {
                                    Color::Gray
                                } else {
                                    Color::White
                                },
                            });
                        }
                    }
                    _ => {
                        // Default weather symbol
                        ctx.draw(&Circle {
                            x: x_offset + 30.0,
                            y: 65.0,
                            radius: 8.0,
                            color: Color::Gray,
                        });

                        // Question mark for unknown weather
                        ctx.draw(&Line {
                            x1: x_offset + 27.0,
                            y1: 62.0,
                            x2: x_offset + 30.0,
                            y2: 60.0,
                            color: Color::White,
                        });
                        ctx.draw(&Line {
                            x1: x_offset + 30.0,
                            y1: 60.0,
                            x2: x_offset + 33.0,
                            y2: 62.0,
                            color: Color::White,
                        });
                        ctx.draw(&Line {
                            x1: x_offset + 33.0,
                            y1: 62.0,
                            x2: x_offset + 30.0,
                            y2: 65.0,
                            color: Color::White,
                        });
                        ctx.draw(&Points {
                            coords: &[(x_offset + 30.0, 68.0)],
                            color: Color::White,
                        });
                    }
                }

                // Enhanced temperature visualization
                let temp_height = (day.temp_max * 0.8).min(25.0);
                let temp_color = match day.temp_max as i32 {
                    t if t > 35 => Color::Red,
                    t if t > 25 => Color::LightRed,
                    t if t > 15 => Color::Yellow,
                    t if t > 5 => Color::Green,
                    t if t > -5 => Color::LightBlue,
                    _ => Color::Blue,
                };

                // Temperature bar with gradient effect
                for h in 0..(temp_height as u32) {
                    let bar_color = if h > (temp_height as u32 * 3 / 4) {
                        temp_color
                    } else if h > (temp_height as u32 / 2) {
                        Color::Yellow
                    } else {
                        Color::Green
                    };

                    ctx.draw(&Points {
                        coords: &[(x_offset + 55.0, 20.0 + h as f64)],
                        color: bar_color,
                    });
                }

                // Day separator with style
                if i < 6 {
                    ctx.draw(&Line {
                        x1: x_offset + 70.0,
                        y1: 0.0,
                        x2: x_offset + 70.0,
                        y2: 100.0,
                        color: Color::DarkGray,
                    });

                    // Decorative separator dots
                    for dot in 0..5 {
                        ctx.draw(&Points {
                            coords: &[(x_offset + 70.0, 20.0 + dot as f64 * 15.0)],
                            color: Color::Gray,
                        });
                    }
                }
            }
        });

    frame.render_widget(canvas, area);
}
