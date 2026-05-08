use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Tabs},
    Frame,
};

use crate::app::App;

/// Draws the main layout with tabs and content area
///
/// # Arguments
/// * `frame` - The ratatui frame to render to
/// * `app` - The application state
pub fn draw(frame: &mut Frame, app: &mut App) {
    let size = frame.size();

    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3),
            Constraint::Min(0),
        ])
        .split(size);

    draw_tabs(frame, app, chunks[0]);
    draw_content(frame, app, chunks[1]);
}

/// Draws the tab bar at the top of the screen
fn draw_tabs(frame: &mut Frame, app: &App, area: Rect) {
    let titles: Vec<Line> = app
        .tabs
        .iter()
        .map(|t| Line::from(vec![Span::styled(*t, Style::default().fg(Color::Green))]))
        .collect();

    let tabs = Tabs::new(titles)
        .block(Block::default().borders(Borders::ALL).title("SECURITY CENTER"))
        .select(app.selected_tab)
        .style(Style::default().fg(Color::White))
        .highlight_style(
            Style::default()
                .fg(Color::Yellow)
                .add_modifier(Modifier::BOLD),
        );

    frame.render_widget(tabs, area);
}

/// Routes to the appropriate content view based on selected tab
fn draw_content(frame: &mut Frame, app: &mut App, area: Rect) {
    match app.selected_tab {
        0 => super::help_view::draw(frame, area),
        1 => super::system_view::draw(frame, area, app),
        2 => super::network_view::draw(frame, area, app),
        3 => super::ports_view::draw(frame, area, app),
        4 => super::connections_view::draw(frame, area, app),
        5 => super::actions_view::draw(frame, area, app),
        _ => {}
    }
}

/// Helper function to create a bordered block with a title
///
/// # Arguments
/// * `title` - The title text for the block
pub fn create_block(title: &str) -> Block<'_> {
    Block::default().borders(Borders::ALL).title(title)
}

/// Creates a color based on percentage thresholds
///
/// # Arguments
/// * `percentage` - The percentage value (0-100)
pub fn color_for_percentage(percentage: f32) -> Color {
    if percentage < 50.0 {
        Color::Green
    } else if percentage < 80.0 {
        Color::Yellow
    } else {
        Color::Red
    }
}

/// Creates a progress bar string using Unicode block characters
///
/// # Arguments
/// * `percentage` - The percentage value (0-100)
/// * `width` - The width of the progress bar in characters
pub fn progress_bar(percentage: f32, width: u16) -> String {
    let filled = (percentage / 100.0 * width as f32) as usize;
    let empty = width as usize - filled;

    let mut bar = String::with_capacity(width as usize);
    for _ in 0..filled {
        bar.push('\u{2588}'); // Full block
    }
    for _ in 0..empty {
        bar.push('\u{2591}'); // Light shade
    }

    bar
}
