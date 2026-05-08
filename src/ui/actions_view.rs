use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Style, Modifier},
    text::{Line, Span},
    widgets::{Block, Borders, List, ListItem, Paragraph, Wrap},
    Frame,
};

use crate::app::App;
use super::layout::create_block;

/// Available actions with descriptions
const ACTIONS: &[(&str, &str)] = &[
    ("Flush Network Cache", "Clears ARP and DNS cache (ip neigh flush, systemd-resolve --flush-caches)"),
    ("Export Report", "Saves current state to ~/sec-center-report.json"),
    ("Check Internet Connection", "Verifies connectivity by testing public DNS servers"),
    ("Update System", "Runs sudo apt update && sudo apt upgrade -y"),
];

/// Draws the actions view
pub fn draw(frame: &mut Frame, area: Rect, app: &mut App) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3),   // Title/help
            Constraint::Min(5),      // Action list
            Constraint::Length(3),   // Status message
        ])
        .split(area);

    draw_help(frame, chunks[0]);
    draw_action_list(frame, app, chunks[1]);
    draw_status(frame, app, chunks[2]);
}

/// Draws help text
fn draw_help(frame: &mut Frame, area: Rect) {
    let help = Paragraph::new("Select an action and press Enter to execute")
        .style(Style::default().fg(Color::Gray))
        .block(create_block("Actions - System Tasks"));
    frame.render_widget(help, area);
}

/// Draws the list of available actions
fn draw_action_list(frame: &mut Frame, app: &mut App, area: Rect) {
    let items: Vec<ListItem> = ACTIONS
        .iter()
        .enumerate()
        .map(|(i, (name, desc))| {
            let style = if Some(i) == app.selected_action {
                Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD)
            } else {
                Style::default()
            };
            
            let content = vec![
                Line::from(vec![
                    Span::styled(format!("{} ", if Some(i) == app.selected_action { ">" } else { " " }), style),
                    Span::styled(*name, style),
                ]),
                Line::from(vec![
                    Span::raw("    "),
                    Span::styled(*desc, Style::default().fg(Color::Gray)),
                ]),
                Line::from(""),
            ];
            
            ListItem::new(content)
        })
        .collect();

    let list = List::new(items)
        .block(Block::default().borders(Borders::NONE));
    
    frame.render_widget(list, area);
}

/// Draws status message from last action
fn draw_status(frame: &mut Frame, app: &App, area: Rect) {
    if let Some(ref result) = app.last_action_result {
        let (color, prefix) = if result.success {
            (Color::Green, "SUCCESS: ")
        } else {
            (Color::Red, "ERROR: ")
        };
        
        let message = Paragraph::new(format!("{}{}", prefix, result.message))
            .style(Style::default().fg(color))
            .block(create_block("Last Action Result"))
            .wrap(Wrap { trim: true });
        frame.render_widget(message, area);
    } else {
        let placeholder = Paragraph::new("No actions executed yet")
            .style(Style::default().fg(Color::DarkGray))
            .block(create_block("Last Action Result"));
        frame.render_widget(placeholder, area);
    }
}
