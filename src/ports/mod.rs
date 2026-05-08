// Ports module - handles TCP port scanning
//
// This module provides:
// - Multi-threaded TCP connect scanning using a thread pool
// - Port-to-service name mapping (e.g., 80 -> HTTP)
// - Configurable scan ranges and timeouts
// - Default scan: localhost ports 1-1024

pub mod scanner;
pub mod services;

/// Main port scanning state
///
/// Tracks scan progress, results, and timing information.
pub struct PortScanInfo {
    /// List of open ports with detected service names
    pub open_ports: Vec<(u16, Option<String>)>,  // (port, service)
    /// Whether a scan is currently in progress
    pub scan_in_progress: bool,
    /// Human-readable timestamp of last scan
    pub last_scan_time: Option<String>,
    /// Scan duration in milliseconds
    pub scan_duration_ms: Option<u128>,
}

impl PortScanInfo {
    /// Creates a new PortScanInfo instance with default values
    pub fn new() -> Self {
        Self {
            open_ports: Vec::new(),
            scan_in_progress: false,
            last_scan_time: None,
            scan_duration_ms: None,
        }
    }

    /// Performs a TCP port scan and returns the count of open ports
    ///
    /// Uses a 100-thread pool for parallel scanning.
    /// Default target: 127.0.0.1 (localhost)
    /// Default range: ports 1-1024
    /// Default timeout: 200ms per port
    pub fn scan(&mut self) -> usize {
        self.scan_in_progress = true;
        let start = std::time::Instant::now();
        
        let config = scanner::ScanConfig::default();
        let result = scanner::scan(&config);
        
        // Map port numbers to service names
        self.open_ports = result.open_ports
            .iter()
            .map(|&port| {
                let service = services::detect_service(port);
                (port, service)
            })
            .collect();
        
        self.scan_duration_ms = Some(start.elapsed().as_millis());
        self.last_scan_time = Some("Just now".to_string());
        self.scan_in_progress = false;
        
        self.open_ports.len()
    }
}
