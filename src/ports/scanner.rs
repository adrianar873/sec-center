use std::net::TcpStream;
use std::time::Duration;
use std::thread;
use std::sync::{Arc, Mutex};

/// Scan configuration
pub struct ScanConfig {
    pub target: String,
    pub start_port: u16,
    pub end_port: u16,
    pub timeout_ms: u64,
    pub threads: usize,
}

impl Default for ScanConfig {
    fn default() -> Self {
        Self {
            target: "127.0.0.1".to_string(),
            start_port: 1,
            end_port: 1024,
            timeout_ms: 200,
            threads: 100,
        }
    }
}

/// Result of a port scan
pub struct ScanResult {
    pub open_ports: Vec<u16>,
}

/// Scans ports using TCP connect with thread pool
pub fn scan(config: &ScanConfig) -> ScanResult {
    let target = Arc::new(config.target.clone());
    let results = Arc::new(Mutex::new(Vec::new()));
    let mut handles = Vec::new();
    
    let ports_per_thread = (config.end_port - config.start_port + 1) / config.threads as u16 + 1;
    let timeout_ms = config.timeout_ms;
    
    for thread_id in 0..config.threads {
        let target = Arc::clone(&target);
        let results = Arc::clone(&results);
        let start = config.start_port + (thread_id as u16 * ports_per_thread);
        let end = std::cmp::min(start + ports_per_thread - 1, config.end_port);
        
        if start > config.end_port {
            break;
        }
        
        let handle = thread::spawn(move || {
            for port in start..=end {
                if scan_port(&target, port, timeout_ms) {
                    results.lock().unwrap().push(port);
                }
            }
        });
        handles.push(handle);
    }
    
    for handle in handles {
        handle.join().unwrap();
    }
    
    let mut open_ports = results.lock().unwrap().clone();
    open_ports.sort();
    
    ScanResult { open_ports }
}

/// Scans a single port using TCP connect
fn scan_port(target: &str, port: u16, timeout_ms: u64) -> bool {
    let addr = format!("{}:{}", target, port);
    TcpStream::connect_timeout(
        &addr.parse().unwrap(),
        Duration::from_millis(timeout_ms),
    ).is_ok()
}
