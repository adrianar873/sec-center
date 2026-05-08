// Network module - handles network scanning and device tracking
//
// This module provides:
// - Network device discovery via ARP scan or /proc/net/arp fallback
// - Device tracking with IP, MAC, and hostname
// - Baseline comparison to detect new/gone devices
// - Persistent storage of baseline to ~/.config/sec-center/baseline.json

pub mod scanner;
pub mod storage;
pub mod devices;

use scanner::ScanConfig;
use storage::{save_baseline, load_baseline};
use devices::NetworkDevice;
use std::collections::HashMap;

/// Network information including scanned devices and baseline comparison
///
/// This struct tracks network devices found via scanning and
/// provides baseline comparison to detect network changes.
pub struct NetworkInfo {
    /// Devices discovered in the most recent scan
    pub current_devices: Vec<NetworkDevice>,
    /// Baseline devices for comparison (loaded from disk or set by user)
    pub baseline: HashMap<String, NetworkDevice>,
    /// Whether a baseline has been established
    pub baseline_set: bool,
    /// Devices present now but not in baseline
    pub new_devices: Vec<NetworkDevice>,
    /// Devices in baseline but not currently visible
    pub gone_devices: Vec<NetworkDevice>,
    /// Timestamp of the last network scan (human-readable)
    pub last_scan_time: Option<String>,
}

impl NetworkInfo {
    /// Creates a new NetworkInfo instance
    ///
    /// Loads any existing baseline from ~/.config/sec-center/baseline.json
    pub fn new() -> Self {
        let baseline = load_baseline().unwrap_or_default();
        let baseline_set = !baseline.is_empty();
        
        Self {
            current_devices: Vec::new(),
            baseline,
            baseline_set,
            new_devices: Vec::new(),
            gone_devices: Vec::new(),
            last_scan_time: None,
        }
    }
    
    /// Performs a network scan and returns device count
    ///
    /// Uses arp-scan if available, falls back to /proc/net/arp.
    /// Compares results with baseline if one is set.
    pub fn scan(&mut self) -> usize {
        let config = ScanConfig::default();
        self.current_devices = scanner::scan(&config);
        self.last_scan_time = Some("Just now".to_string());
        
        // Compare with baseline if set
        if self.baseline_set {
            self.compare_with_baseline();
        }
        
        self.current_devices.len()
    }
    
    /// Compares current devices with baseline to find changes
    fn compare_with_baseline(&mut self) {
        let current_ips: std::collections::HashSet<String> = 
            self.current_devices.iter().map(|d| d.ip.clone()).collect();
        let baseline_ips: std::collections::HashSet<String> = 
            self.baseline.keys().cloned().collect();
        
        // Find new devices (in current, not in baseline)
        self.new_devices = self.current_devices
            .iter()
            .filter(|d| !baseline_ips.contains(&d.ip))
            .cloned()
            .collect();
        
        // Find gone devices (in baseline, not in current)
        self.gone_devices = self.baseline
            .values()
            .filter(|d| !current_ips.contains(&d.ip))
            .cloned()
            .collect();
    }
    
    /// Sets current devices as the baseline
    ///
    /// Saves baseline to ~/.config/sec-center/baseline.json for persistence.
    /// Clears new/gone device lists.
    pub fn set_baseline(&mut self) {
        self.baseline = HashMap::new();
        for device in &self.current_devices {
            self.baseline.insert(device.ip.clone(), device.clone());
        }
        self.baseline_set = true;
        self.new_devices.clear();
        self.gone_devices.clear();
        
        if let Err(e) = save_baseline(&self.baseline) {
            eprintln!("Failed to save baseline: {}", e);
        }
    }
}
