use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Style},
    text::{Line, Span},
    widgets::Paragraph,
    Frame,
};

use crate::app::App;
use crate::system::{
    cpu::CpuInfo,
    memory::MemoryInfo,
    disk::DiskInfo,
    temperature::TemperatureInfo,
    gpu::GpuInfo,
    motherboard::MotherboardInfo,
};
use super::layout::{create_block, color_for_percentage, progress_bar};

/// Draws the system view with hardware info, usage graphs, and temperatures
///
/// # Arguments
/// * `frame` - The ratatui frame to render to
/// * `area` - The rectangular area to render within
/// * `app` - The application state containing system information
pub fn draw(frame: &mut Frame, area: Rect, app: &App) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(12),  // Hardware info
            Constraint::Length(6),    // Usage graphs
            Constraint::Min(0),       // Temperatures
        ])
        .split(area);

    draw_hardware_info(frame, app, chunks[0]);
    draw_usage_graphs(frame, app, chunks[1]);
    draw_temperatures(frame, app, chunks[2]);
}

/// Draws hardware information (CPU, RAM, Disks, GPU, Motherboard)
fn draw_hardware_info(frame: &mut Frame, app: &App, area: Rect) {
    let cpu_info = CpuInfo::collect(&app.system_info.system);
    let mem_info = MemoryInfo::collect(&app.system_info.system);
    let gpu_info = GpuInfo::detect();
    let mb_info = MotherboardInfo::detect();

    let info = Paragraph::new(vec![
        Line::from(vec![
            Span::raw("CPU Model: "),
            Span::styled(&cpu_info.brand, Style::default().fg(Color::Cyan)),
        ]),
        Line::from(vec![
            Span::raw("Motherboard: "),
            Span::styled(format!("{} {}", mb_info.manufacturer, mb_info.product), 
                       Style::default().fg(Color::Cyan)),
        ]),
        Line::from(vec![
            Span::raw("RAM Total: "),
            Span::styled(MemoryInfo::format_bytes(mem_info.total), Style::default().fg(Color::Green)),
        ]),
        Line::from(vec![
            Span::raw("GPU Model: "),
            if gpu_info.detected {
                Span::styled(&gpu_info.name, Style::default().fg(Color::Cyan))
            } else {
                Span::styled("No GPU detected", Style::default().fg(Color::Yellow))
            },
        ]),
        Line::from(""),
        Line::from(format!("Disks: {} detected", app.system_info.disks.list().len())),
    ])
    .block(create_block("Hardware Information"));

    frame.render_widget(info, area);
}

/// Draws usage graphs (CPU, RAM, Disk speeds)
fn draw_usage_graphs(frame: &mut Frame, app: &App, area: Rect) {
    let cpu_info = CpuInfo::collect(&app.system_info.system);
    let mem_info = MemoryInfo::collect(&app.system_info.system);
    let disk_info = &app.system_info.disk_info;

    let cpu_color = color_for_percentage(cpu_info.usage);
    let mem_color = color_for_percentage(mem_info.usage_percent());

    let cpu_bar = progress_bar(cpu_info.usage, 30);
    let mem_bar = progress_bar(mem_info.usage_percent(), 30);

    let disk_read = DiskInfo::format_speed(disk_info.read_speed);
    let disk_write = DiskInfo::format_speed(disk_info.write_speed);

    let info = Paragraph::new(vec![
        Line::from(vec![
            Span::raw("CPU Usage: "),
            Span::styled(cpu_bar, Style::default().fg(cpu_color)),
            Span::raw(format!(" {:.1}%", cpu_info.usage)),
        ]),
        Line::from(vec![
            Span::raw("RAM Usage: "),
            Span::styled(mem_bar, Style::default().fg(mem_color)),
            Span::raw(format!(" {:.1}%", mem_info.usage_percent())),
        ]),
        Line::from(format!("Disk Speed: Read {} | Write {}", disk_read, disk_write)),
    ])
    .block(create_block("Usage Graphs"));

    frame.render_widget(info, area);
}

/// Draws temperature information (separate section)
fn draw_temperatures(frame: &mut Frame, app: &App, area: Rect) {
    let temp_info = TemperatureInfo::collect(&app.system_info.components);
    let gpu_info = GpuInfo::detect();

    let mut temp_lines: Vec<Line> = Vec::new();

    // CPU temperature
    match temp_info.cpu_temperature() {
        Some(cpu_temp) => {
            let temp_color = if cpu_temp < 50.0 {
                Color::Green
            } else if cpu_temp < 70.0 {
                Color::Yellow
            } else {
                Color::Red
            };
            let bar = progress_bar((cpu_temp / 100.0) * 100.0, 20);
            temp_lines.push(Line::from(vec![
                Span::raw("CPU Temp: "),
                Span::styled(bar, Style::default().fg(temp_color)),
                Span::raw(format!(" {:.1}C", cpu_temp)),
            ]));
        },
        None => {
            temp_lines.push(Line::from("CPU Temp: N/A"));
        }
    }

    // GPU temperature
    if gpu_info.detected {
        match temp_info.gpu_temperature() {
            Some(gpu_temp) => {
                let temp_color = if gpu_temp < 50.0 {
                    Color::Green
                } else if gpu_temp < 70.0 {
                    Color::Yellow
                } else {
                    Color::Red
                };
                let bar = progress_bar((gpu_temp / 100.0) * 100.0, 20);
                temp_lines.push(Line::from(vec![
                    Span::raw("GPU Temp: "),
                    Span::styled(bar, Style::default().fg(temp_color)),
                    Span::raw(format!(" {:.1}C", gpu_temp)),
                ]));
            },
            None => {}
        }
    }

    // Other temperature sensors
    for sensor in &temp_info.sensors {
        if !sensor.label.to_lowercase().contains("cpu") && 
           !sensor.label.to_lowercase().contains("gpu") {
            let temp_color = if sensor.temperature < 50.0 {
                Color::Green
            } else if sensor.temperature < 70.0 {
                Color::Yellow
            } else {
                Color::Red
            };
            temp_lines.push(Line::from(vec![
                Span::raw(format!("{}: ", sensor.label)),
                Span::styled(format!("{:.1}C", sensor.temperature), Style::default().fg(temp_color)),
            ]));
        }
    }

    let widget = Paragraph::new(temp_lines)
        .block(create_block("Temperatures"));

    frame.render_widget(widget, area);
}
