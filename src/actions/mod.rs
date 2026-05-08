// Actions module - handles system actions and tasks
//
// This module provides system administration actions:
// - Flush network cache (ARP + DNS)
// - Export system state report to JSON
// - Check internet connectivity
// - Update system packages (apt update & upgrade)

use std::fs;
use std::path::PathBuf;
use std::process::Command;
use std::net::TcpStream;
use std::time::Duration;

/// Available actions in the Actions tab
pub enum Action {
    FlushNetworkCache,
    ExportReport,
    CheckInternet,
    SystemUpdate,
}

/// Result of an action execution
pub struct ActionResult {
    pub success: bool,
    pub message: String,
}

/// Executes the specified action
pub fn execute_action(action: &Action, app_state: &AppState) -> ActionResult {
    match action {
        Action::FlushNetworkCache => flush_network_cache(),
        Action::ExportReport => export_report(app_state),
        Action::CheckInternet => check_internet_connection(),
        Action::SystemUpdate => system_update(),
    }
}

/// Flushes network cache (ARP + DNS)
///
/// Note: ARP cache flush requires root privileges.
/// For DNS flush, tries multiple methods without sudo.
fn flush_network_cache() -> ActionResult {
    let mut messages = Vec::new();
    let mut success = true;
    
    // Try to flush ARP cache (best effort, may fail without root)
    let arp_result = Command::new("ip")
        .args(["-s", "-s", "neigh", "flush", "all"])
        .output();
    
    match arp_result {
        Ok(output) => {
            if output.status.success() {
                messages.push("ARP cache cleared".to_string());
            } else {
                let stderr = String::from_utf8_lossy(&output.stderr);
                if stderr.to_lowercase().contains("permission") || stderr.to_lowercase().contains("not permitted") {
                    messages.push("ARP flush: requires root (run with sudo)".to_string());
                    success = false;
                } else {
                    messages.push(format!("ARP flush: {}", stderr.trim()));
                }
            }
        }
        Err(e) => {
            messages.push(format!("Failed to run ip command: {}", e));
            success = false;
        }
    }
    
    // Flush DNS cache - try different methods
    let dns_methods = vec![
        ("resolvectl", vec!["flush-caches"]),
        ("systemd-resolve", vec!["--flush-caches"]),
    ];
    
    let mut dns_cleared = false;
    for (cmd, args) in dns_methods {
        if let Ok(output) = Command::new(cmd).args(&args).output() {
            if output.status.success() {
                messages.push(format!("DNS cache cleared ({})", cmd));
                dns_cleared = true;
                break;
            }
        }
    }
    
    if !dns_cleared {
        messages.push("DNS cache: no flush method available".to_string());
    }
    
    ActionResult {
        success,
        message: messages.join(", "),
    }
}

/// Exports current state to a JSON report
fn export_report(app_state: &AppState) -> ActionResult {
    let report = serde_json::json!({
        "timestamp": chrono::Utc::now().to_rfc3339(),
        "system": {
            "cpu_count": app_state.system_info.system.cpus().len(),
            "memory_total": app_state.system_info.system.total_memory(),
            "memory_used": app_state.system_info.system.used_memory(),
            "disk_read_speed": app_state.system_info.disk_info.read_speed,
            "disk_write_speed": app_state.system_info.disk_info.write_speed,
        },
        "network": {
            "devices_count": app_state.network_info.current_devices.len(),
            "baseline_count": app_state.network_info.baseline.len(),
        },
        "ports": {
            "scanned_count": app_state.port_scan_info.open_ports.len(),
        },
        "connections": {
            "tcp_count": app_state.connection_info.tcp_count,
            "udp_count": app_state.connection_info.udp_count,
        }
    });
    
    let home = dirs::home_dir().unwrap_or_else(|| PathBuf::from("."));
    let report_path = home.join("sec-center-reporte.json");
    
    match fs::write(&report_path, serde_json::to_string_pretty(&report).unwrap_or_default()) {
        Ok(_) => ActionResult {
            success: true,
            message: format!("Report saved to: {}", report_path.display()),
        },
        Err(e) => ActionResult {
            success: false,
            message: format!("Failed to save report: {}", e),
        }
    }
}

/// Checks internet connectivity by trying to connect to common DNS servers
fn check_internet_connection() -> ActionResult {
    let test_hosts = vec![
        ("8.8.8.8", 53),      // Google DNS
        ("1.1.1.1", 53),      // Cloudflare DNS
        ("208.67.222.222", 53), // OpenDNS
    ];
    
    let timeout = Duration::from_secs(3);
    let mut connected = false;
    let mut messages = Vec::new();
    
    for (host, port) in test_hosts {
        match TcpStream::connect_timeout(&format!("{}:{}", host, port).parse().unwrap(), timeout) {
            Ok(_) => {
                connected = true;
                messages.push(format!("Connected to {}", host));
                break;
            }
            Err(e) => {
                messages.push(format!("Failed {}: {}", host, e));
            }
        }
    }
    
    if connected {
        ActionResult {
            success: true,
            message: "Internet connection: OK".to_string(),
        }
    } else {
        ActionResult {
            success: false,
            message: format!("No internet: {}", messages.join("; ")),
        }
    }
}

/// Runs system update and upgrade (apt update && apt upgrade)
///
/// Note: Requires sudo privileges. If sudo requires password, this will fail
/// in the TUI. Run the app with sudo or configure passwordless sudo for apt.
fn system_update() -> ActionResult {
    // First run apt update
    let update_result = Command::new("apt")
        .args(["update"])
        .output();
    
    match update_result {
        Ok(output) => {
            if output.status.success() {
                // Then run apt upgrade
                let upgrade_result = Command::new("apt")
                    .args(["upgrade", "-y"])
                    .output();
                
                match upgrade_result {
                    Ok(up_out) => {
                        if up_out.status.success() {
                            ActionResult {
                                success: true,
                                message: "System updated and upgraded successfully".to_string(),
                            }
                        } else {
                            // Check if it's a permission error
                            let stderr = String::from_utf8_lossy(&up_out.stderr);
                            if stderr.to_lowercase().contains("permission denied") {
                                ActionResult {
                                    success: false,
                                    message: "Permission denied: run with sudo or use: sudo ./sec-center".to_string(),
                                }
                            } else {
                                ActionResult {
                                    success: false,
                                    message: format!("Upgrade failed: {}", stderr),
                                }
                            }
                        }
                    }
                    Err(e) => ActionResult {
                        success: false,
                        message: format!("Failed to run upgrade: {}", e),
                    }
                }
            } else {
                let stderr = String::from_utf8_lossy(&output.stderr);
                if stderr.to_lowercase().contains("permission denied") {
                    ActionResult {
                        success: false,
                        message: "Permission denied: run with sudo or use: sudo ./sec-center".to_string(),
                    }
                } else {
                    ActionResult {
                        success: false,
                        message: format!("Update failed: {}", stderr),
                    }
                }
            }
        }
        Err(e) => ActionResult {
            success: false,
            message: format!("Failed to run update: {}", e),
        }
    }
}

/// Wrapper struct to pass app state to actions
pub struct AppState<'a> {
    pub system_info: &'a crate::system::SystemInfo,
    pub network_info: &'a crate::network::NetworkInfo,
    pub port_scan_info: &'a crate::ports::PortScanInfo,
    pub connection_info: &'a crate::connections::ConnectionInfo,
}
