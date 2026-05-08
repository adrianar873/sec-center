// UI module - contains all user interface components
// This module handles rendering of different views using Ratatui

pub mod layout;
pub mod help_view;
pub mod system_view;
pub mod network_view;
pub mod ports_view;
pub mod connections_view;
pub mod actions_view;
pub mod popup;

use ratatui::Frame;
use crate::app::App;

/// Main draw function that renders the entire UI
///
/// # Arguments
/// * `frame` - The ratatui frame to render to
/// * `app` - The application state
pub fn draw(frame: &mut Frame, app: &mut App) {
    layout::draw(frame, app);
    
    // Draw popup on top if exists
    if let Some(ref popup) = app.popup {
        popup::draw(frame, popup);
    }
}
