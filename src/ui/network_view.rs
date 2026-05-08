use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Style},
    text::{Line, Span},
    widgets::{Paragraph, Row, Table, Cell},
    Frame,
};

use crate::app::App;
use super::layout::create_block;

/// Draws the network view with device list and change detection
pub fn draw(frame: &mut Frame, area: Rect, app: &mut App) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(5),   // Status
            Constraint::Min(3),       // Device list
            Constraint::Length(5),    // Changes (new/gone)
        ])
        .split(area);

    draw_status(frame, app, chunks[0]);
    draw_device_list(frame, app, chunks[1]);
    draw_changes(frame, app, chunks[2]);
}

/// Draws network status and control buttons
fn draw_status(frame: &mut Frame, app: &App, area: Rect) {
    let scan_time = app.network_info.last_scan_time.as_deref().unwrap_or("Never");
    let device_count = app.network_info.current_devices.len();
    let baseline_status = if app.network_info.baseline_set {
        format!("Saved ({} devices)", app.network_info.baseline.len())
    } else {
        "Not set".to_string()
    };

    let info = Paragraph::new(vec![
        Line::from(vec![
            Span::raw("Status: "),
            Span::styled("Active", Style::default().fg(Color::Green)),
        ]),
        Line::from(format!("Last scan: {}", scan_time)),
        Line::from(format!("Devices found: {}", device_count)),
        Line::from(format!("Baseline: {}", baseline_status)),
    ])
    .block(create_block("Network Status [S] Scan | [B] Set Baseline"));
    
    frame.render_widget(info, area);
}

/// Draws the list of discovered devices
fn draw_device_list(frame: &mut Frame, app: &App, area: Rect) {
    let devices = &app.network_info.current_devices;
    
    if devices.is_empty() {
        let placeholder = Paragraph::new("No devices found. Press [S] to scan.")
            .block(create_block("Discovered Devices"));
        frame.render_widget(placeholder, area);
        return;
    }
    
    let rows: Vec<Row> = devices
        .iter()
        .map(|d| {
            let is_new = app.network_info.new_devices.iter().any(|n| n.ip == d.ip);
            let status = if is_new {
                Span::styled("NEW!", Style::default().fg(Color::Yellow))
            } else {
                Span::raw("")
            };
            
            Row::new(vec![
                Cell::from(d.ip.as_str()),
                Cell::from(d.mac.as_deref().unwrap_or("N/A")),
                Cell::from(d.hostname.as_deref().unwrap_or("N/A")),
                Cell::from(status),
            ])
        })
        .collect();
    
    let table = Table::new(rows, &[Constraint::Length(15), Constraint::Length(20), Constraint::Length(25), Constraint::Length(15)])
        .block(create_block("Discovered Devices"))
        .header(
            Row::new(vec!["IP", "MAC", "Hostname", "Status"])
                .style(Style::default().fg(Color::Cyan)),
        )
        .column_spacing(1);
    
    frame.render_widget(table, area);
}

/// Draws the changes section (new and gone devices)
fn draw_changes(frame: &mut Frame, app: &App, area: Rect) {
    let mut lines = Vec::new();
    
    // New devices
    if !app.network_info.new_devices.is_empty() {
        lines.push(Line::from(vec![
            Span::styled("NEW: ", Style::default().fg(Color::Yellow)),
        ]));
        for device in &app.network_info.new_devices {
            lines.push(Line::from(format!(
                "  {} ({})",
                device.ip,
                device.hostname.as_deref().unwrap_or("Unknown")
            )));
        }
    }
    
    // Gone devices
    if !app.network_info.gone_devices.is_empty() {
        lines.push(Line::from(vec![
            Span::styled("GONE: ", Style::default().fg(Color::Red)),
        ]));
        for device in &app.network_info.gone_devices {
            lines.push(Line::from(format!(
                "  {} (last seen {})",
                device.ip,
                device.last_seen_ago()
            )));
        }
    }
    
    if lines.is_empty() {
        lines.push(Line::from(vec![
            Span::styled("No changes detected from baseline", Style::default().fg(Color::Green)),
        ]));
    }
    
    let changes = Paragraph::new(lines)
        .block(create_block("Detected Changes"));
    
    frame.render_widget(changes, area);
}
