use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Style, Modifier},
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph, Wrap},
    Frame,
};

use super::layout::create_block;

/// Draws the help view with application documentation
pub fn draw(frame: &mut Frame, area: Rect) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3),   // Title
            Constraint::Min(10),     // Content
            Constraint::Length(3),   // Footer
        ])
        .split(area);

    draw_title(frame, chunks[0]);
    draw_content(frame, chunks[1]);
    draw_footer(frame, chunks[2]);
}

/// Draws the title section
fn draw_title(frame: &mut Frame, area: Rect) {
    let title = Paragraph::new("Security Center - Help & Documentation")
        .style(Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD))
        .block(create_block("Help"));
    frame.render_widget(title, area);
}

/// Draws the main content with key bindings and usage
fn draw_content(frame: &mut Frame, area: Rect) {
    let text = vec![
        Line::from(vec![
            Span::styled("KEY BINDINGS", Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD)),
        ]),
        Line::from(""),
        Line::from(vec![
            Span::styled("Tab / Right Arrow", Style::default().fg(Color::Green)),
            Span::raw(" - Switch to next tab"),
        ]),
        Line::from(vec![
            Span::styled("Shift+Tab / Left Arrow", Style::default().fg(Color::Green)),
            Span::raw(" - Switch to previous tab"),
        ]),
        Line::from(vec![
            Span::styled("Up / Down Arrows", Style::default().fg(Color::Green)),
            Span::raw(" - Navigate within tabs (Actions, etc.)"),
        ]),
        Line::from(vec![
            Span::styled("Enter", Style::default().fg(Color::Green)),
            Span::raw(" - Execute selected action (Actions tab)"),
        ]),
        Line::from(vec![
            Span::styled("S", Style::default().fg(Color::Green)),
            Span::raw(" - Start scan (Network, Ports tabs)"),
        ]),
        Line::from(vec![
            Span::styled("B", Style::default().fg(Color::Green)),
            Span::raw(" - Set network baseline (Network tab)"),
        ]),
        Line::from(vec![
            Span::styled("Q", Style::default().fg(Color::Green)),
            Span::raw(" - Quit application"),
        ]),
        Line::from(vec![
            Span::styled("Any key", Style::default().fg(Color::Green)),
            Span::raw(" - Dismiss popup notifications"),
        ]),
        Line::from(""),
        Line::from(vec![
            Span::styled("TAB OVERVIEW", Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD)),
        ]),
        Line::from(""),
        Line::from("1. Help - This documentation screen"),
        Line::from("2. System - CPU, RAM, Disk, GPU, Temperatures"),
        Line::from("3. Network - Device scanning, baseline comparison"),
        Line::from("4. Ports - TCP port scanning with thread pool"),
        Line::from("5. Connections - Active TCP/UDP connections monitor"),
        Line::from("6. Actions - System tasks (flush cache, update, etc.)"),
        Line::from(""),
        Line::from(vec![
            Span::styled("NOTES", Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD)),
        ]),
        Line::from(""),
        Line::from("• Popup notifications auto-close after 2 seconds"),
        Line::from("• Network baseline stored at ~/.config/sec-center/baseline.json"),
        Line::from("• Reports exported to ~/sec-center-report.json"),
        Line::from("• Some actions (flush cache, update) require sudo privileges"),
    ];

    let paragraph = Paragraph::new(text)
        .block(create_block("Usage Guide"))
        .wrap(Wrap { trim: true })
        .style(Style::default().fg(Color::White));
    
    frame.render_widget(paragraph, area);
}

/// Draws the footer with version info
fn draw_footer(frame: &mut Frame, area: Rect) {
    let footer = Paragraph::new("Security Center v0.1.0 | Press Q to quit")
        .style(Style::default().fg(Color::DarkGray))
        .block(Block::default().borders(Borders::NONE));
    frame.render_widget(footer, area);
}
