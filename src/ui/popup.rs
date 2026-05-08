use std::time::{Duration, Instant};
use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    style::Style,
    text::Line,
    widgets::{Block, Borders, Paragraph},
    Frame,
};

/// Popup notification that auto-closes after 2 seconds
pub struct Popup {
    pub message: String,
    pub created_at: Instant,
}

impl Popup {
    /// Creates a new popup with current timestamp
    pub fn new(message: String) -> Self {
        Self {
            message,
            created_at: Instant::now(),
        }
    }

    /// Checks if the popup should close (after 2 seconds)
    pub fn should_close(&self) -> bool {
        self.created_at.elapsed() > Duration::from_secs(2)
    }
}

/// Draws a centered popup on top of the current UI
pub fn draw(frame: &mut Frame, popup: &Popup) {
    let area = frame.size();
    
    // Calculate centered rectangle (40% width, 15% height)
    let popup_area = centered_rect(40, 15, area);
    
    // Clear the area first (black background)
    frame.render_widget(ratatui::widgets::Clear, popup_area);
    
    // Create popup widget
    let popup_widget = Paragraph::new(vec![
        Line::from(""),
        Line::from(popup.message.clone()),
    ])
    .block(Block::default().borders(Borders::ALL).title("Notification"))
    .style(Style::default().fg(ratatui::style::Color::Yellow))
    .alignment(ratatui::layout::Alignment::Center);
    
    frame.render_widget(popup_widget, popup_area);
}

/// Calculates a centered rectangle
fn centered_rect(percent_x: u16, percent_y: u16, r: Rect) -> Rect {
    let popup_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Percentage((100 - percent_y) / 2),
            Constraint::Percentage(percent_y),
            Constraint::Percentage((100 - percent_y) / 2),
        ])
        .split(r);

    Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage((100 - percent_x) / 2),
            Constraint::Percentage(percent_x),
            Constraint::Percentage((100 - percent_x) / 2),
        ])
        .split(popup_layout[1])[1]
}
