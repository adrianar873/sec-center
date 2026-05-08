// Security Center - Main entry point
//
// A TUI application for system monitoring, network scanning,
// port scanning, and connection monitoring built with Rust and Ratatui.

mod app;
mod ui;
mod system;
mod network;
mod ports;
mod connections;
mod actions;

use crossterm::{
    event::{self, Event, KeyCode, KeyEventKind},
    terminal::{EnterAlternateScreen, LeaveAlternateScreen, disable_raw_mode, enable_raw_mode},
};
use ratatui::{Terminal, backend::CrosstermBackend};

use app::App;

/// Main entry point for the Security Center TUI application
///
/// This function:
/// 1. Enables raw mode for terminal control
/// 2. Enters the alternate screen buffer to preserve terminal contents
/// 3. Creates and runs the main event loop
/// 4. Restores the terminal state on exit
fn main() -> anyhow::Result<()> {
    // Setup terminal - switch to raw mode for direct key input
    enable_raw_mode()?;
    let mut stdout = std::io::stdout();
    crossterm::execute!(stdout, EnterAlternateScreen)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    // Create app state and run the main event loop
    let mut app = App::new();
    let result = run_app(&mut terminal, &mut app);

    // Restore terminal to original state
    disable_raw_mode()?;
    crossterm::execute!(terminal.backend_mut(), LeaveAlternateScreen)?;
    terminal.show_cursor()?;

    result
}

/// Main event loop for the application
///
/// # Arguments
/// * `terminal` - The ratatui terminal instance for rendering
/// * `app` - Mutable reference to the application state
///
/// This function:
/// 1. Refreshes system information (CPU, memory, connections)
/// 2. Draws the UI based on current tab selection
/// 3. Handles keyboard input with 250ms poll timeout
/// 4. Loops until user presses 'q' to quit
fn run_app<B: ratatui::backend::Backend>(
    terminal: &mut Terminal<B>,
    app: &mut App,
) -> anyhow::Result<()> {
    loop {
        // Refresh dynamic data
        app.refresh();
        
        // Auto-close popups after timeout
        app.update_popup();
        
        // Render UI
        terminal.draw(|f| ui::draw(f, app))?;

        // Check for keyboard events with timeout
        if event::poll(std::time::Duration::from_millis(250))? {
            if let Event::Key(key) = event::read()? {
                if key.kind == KeyEventKind::Press {
                    // If popup is active, any key dismisses it
                    if app.popup.is_some() {
                        app.popup = None;
                        continue;
                    }
                    
                    match key.code {
                        // Quit application
                        KeyCode::Char('q') => return Ok(()),
                        
                        // Tab navigation
                        KeyCode::Tab => app.next_tab(),
                        KeyCode::BackTab => app.prev_tab(),
                        KeyCode::Right => app.next_tab(),
                        KeyCode::Left => app.prev_tab(),
                        
                        // Network tab shortcuts (Tab 2)
                        KeyCode::Char('s') | KeyCode::Char('S') => {
                            if app.selected_tab == 2 {
                                let count = app.network_info.scan();
                                app.show_popup(format!("Scan finished: {} devices found", count));
                            }
                            if app.selected_tab == 3 {
                                let count = app.port_scan_info.scan();
                                app.show_popup(format!("Scan finished: {} ports found", count));
                            }
                        },
                        
                        // Set network baseline (Tab 2)
                        KeyCode::Char('b') | KeyCode::Char('B') => {
                            if app.selected_tab == 2 {
                                app.network_info.set_baseline();
                                let count = app.network_info.baseline.len();
                                app.show_popup(format!("Baseline saved: {} devices", count));
                            }
                        },
                        
                        // Actions tab navigation (Tab 5)
                        KeyCode::Up => {
                            if app.selected_tab == 5 {
                                match app.selected_action {
                                    Some(current) => {
                                        app.selected_action = Some(if current == 0 { 3 } else { current - 1 });
                                    }
                                    None => {
                                        app.selected_action = Some(0);
                                    }
                                }
                            }
                        },
                        
                        KeyCode::Down => {
                            if app.selected_tab == 5 {
                                match app.selected_action {
                                    Some(current) => {
                                        app.selected_action = Some(if current >= 3 { 0 } else { current + 1 });
                                    }
                                    None => {
                                        app.selected_action = Some(0);
                                    }
                                }
                            }
                        },
                        
                        // Execute selected action
                        KeyCode::Enter => {
                            if app.selected_tab == 5 {
                                app.execute_selected_action();
                                if let Some(ref result) = app.last_action_result {
                                    app.show_popup(result.message.clone());
                                }
                            }
                        },
                        
                        _ => {}
                    }
                }
            }
        }
    }
}
