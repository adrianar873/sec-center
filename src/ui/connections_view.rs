use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Style},
    text::{Line, Span},
    widgets::{Paragraph, Row, Table, Cell},
    Frame,
};

use crate::app::App;
use super::layout::create_block;

/// Draws the connections view with active connections
pub fn draw(frame: &mut Frame, area: Rect, app: &mut App) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(5),   // Status
            Constraint::Min(3),       // Connection list
        ])
        .split(area);

    draw_status(frame, app, chunks[0]);
    draw_connection_list(frame, app, chunks[1]);
}

/// Draws connection status and controls
fn draw_status(frame: &mut Frame, app: &App, area: Rect) {
    let tcp_count = app.connection_info.tcp_count;
    let udp_count = app.connection_info.udp_count;
    let last_update = app.connection_info.last_update;
    let elapsed = last_update.elapsed().as_secs();
    
    let info = Paragraph::new(vec![
        Line::from(vec![
            Span::raw("TCP: "),
            Span::styled(format!("{}", tcp_count), Style::default().fg(Color::Green)),
            Span::raw(" active | UDP: "),
            Span::styled(format!("{}", udp_count), Style::default().fg(Color::Cyan)),
        ]),
        Line::from(format!("Last update: {} seconds ago", elapsed)),
        Line::from("Auto-refresh: every 5 seconds"),
    ])
    .block(create_block("Active Connections"));
    
    frame.render_widget(info, area);
}

/// Draws the list of active connections
fn draw_connection_list(frame: &mut Frame, app: &App, area: Rect) {
    let connections = &app.connection_info.connections;
    
    if connections.is_empty() {
        let placeholder = Paragraph::new("No active connections found.")
            .block(create_block("Connections"));
        frame.render_widget(placeholder, area);
        return;
    }
    
    let rows: Vec<Row> = connections
        .iter()
        .map(|conn| {
            let state_str = conn.state.as_deref().unwrap_or("");
            let state_color = match state_str {
                "ESTABLISHED" => Color::Green,
                "LISTEN" => Color::Yellow,
                "TIME_WAIT" | "CLOSE_WAIT" => Color::Red,
                _ => Color::White,
            };
            
            Row::new(vec![
                Cell::from(conn.local_addr.to_string()),
                Cell::from(conn.remote_addr.map(|a| a.to_string()).unwrap_or("*".to_string())),
                Cell::from(conn.protocol.clone()),
                Cell::from(Span::styled(state_str.to_string(), Style::default().fg(state_color))),
            ])
        })
        .collect();
    
    let table = Table::new(rows, &[
        Constraint::Length(25),
        Constraint::Length(25),
        Constraint::Length(8),
        Constraint::Length(15),
    ])
    .block(create_block("Active Connections"))
    .header(
        Row::new(vec!["Local Addr", "Remote Addr", "Proto", "State"])
            .style(Style::default().fg(Color::Cyan)),
    )
    .column_spacing(1);
    
    frame.render_widget(table, area);
}
