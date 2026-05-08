use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Style},
    text::{Line, Span},
    widgets::{Paragraph, Row, Table, Cell},
    Frame,
};

use crate::app::App;
use super::layout::create_block;

/// Draws the ports view with port scan results
pub fn draw(frame: &mut Frame, area: Rect, app: &mut App) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(5),   // Status
            Constraint::Min(3),       // Port list
        ])
        .split(area);

    draw_status(frame, app, chunks[0]);
    draw_port_list(frame, app, chunks[1]);
}

/// Draws port scan status and controls
fn draw_status(frame: &mut Frame, app: &App, area: Rect) {
    let port_count = app.port_scan_info.open_ports.len();
    let status = if app.port_scan_info.scan_in_progress {
        "Scanning...".to_string()
    } else {
        "Idle".to_string()
    };
    let status_color = if app.port_scan_info.scan_in_progress {
        Color::Yellow
    } else {
        Color::Green
    };
    
    let duration = app.port_scan_info.scan_duration_ms
        .map(|ms| format!(" (took {}ms)", ms))
        .unwrap_or_default();
    
    let last_scan = app.port_scan_info.last_scan_time.as_deref().unwrap_or("Never");

    let info = Paragraph::new(vec![
        Line::from(vec![
            Span::raw("Status: "),
            Span::styled(status, Style::default().fg(status_color)),
        ]),
        Line::from(format!("Open ports: {}", port_count)),
        Line::from(format!("Last scan: {}{}", last_scan, duration)),
        Line::from("Press [S] to scan ports 1-1024"),
    ])
    .block(create_block("Port Scanner [S] Scan"));
    
    frame.render_widget(info, area);
}

/// Draws the list of open ports
fn draw_port_list(frame: &mut Frame, app: &App, area: Rect) {
    let ports = &app.port_scan_info.open_ports;
    
    if ports.is_empty() {
        let placeholder = Paragraph::new("No open ports found. Press [S] to scan.")
            .block(create_block("Open Ports"));
        frame.render_widget(placeholder, area);
        return;
    }
    
    let rows: Vec<Row> = ports
        .iter()
        .map(|(port, service)| {
            let service_str = service.as_deref().unwrap_or("Unknown");
            Row::new(vec![
                Cell::from(port.to_string()),
                Cell::from("TCP"),
                Cell::from(service_str),
                Cell::from(""),  // Banner (future use)
            ])
        })
        .collect();
    
    let table = Table::new(rows, &[
            Constraint::Length(8),
            Constraint::Length(6),
            Constraint::Length(15),
            Constraint::Length(30),
        ])
        .block(create_block("Open Ports"))
        .header(
            Row::new(vec!["Port", "Proto", "Service", "Banner"])
                .style(Style::default().fg(Color::Cyan)),
        )
        .column_spacing(1);
    
    frame.render_widget(table, area);
}
