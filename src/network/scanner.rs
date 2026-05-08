use std::process::Command;
use crate::network::devices::NetworkDevice;

/// Configuration for network scanning
pub struct ScanConfig {
    pub interface: Option<String>,
}

impl Default for ScanConfig {
    fn default() -> Self {
        Self { interface: None }
    }
}

/// Performs a network scan and returns discovered devices
pub fn scan(config: &ScanConfig) -> Vec<NetworkDevice> {
    // Try arp-scan first
    if let Some(devices) = scan_with_arp_scan(config) {
        if !devices.is_empty() {
            return devices;
        }
    }
    
    // Fallback: read /proc/net/arp
    scan_from_arp_table()
}

/// Uses arp-scan to discover devices
fn scan_with_arp_scan(config: &ScanConfig) -> Option<Vec<NetworkDevice>> {
    let mut cmd = Command::new("arp-scan");
    
    if let Some(iface) = &config.interface {
        cmd.arg("--interface").arg(iface);
    }
    
    cmd.arg("--localnet");
    
    let output = cmd.output().ok()?;
    let stdout = String::from_utf8_lossy(&output.stdout);
    
    parse_arp_scan_output(&stdout)
}

/// Parses arp-scan output
fn parse_arp_scan_output(output: &str) -> Option<Vec<NetworkDevice>> {
    let mut devices = Vec::new();
    
    for line in output.lines() {
        let parts: Vec<&str> = line.split_whitespace().collect();
        if parts.len() >= 2 {
            let ip = parts[0].to_string();
            let mac = if parts[1] != "(incomplete)" {
                Some(parts[1].to_string())
            } else {
                None
            };
            
            // Try to resolve hostname
            let hostname = resolve_hostname(&ip);
            
            devices.push(NetworkDevice::new(ip, mac, hostname));
        }
    }
    
    if devices.is_empty() {
        None
    } else {
        Some(devices)
    }
}

/// Reads the kernel ARP table from /proc/net/arp
fn scan_from_arp_table() -> Vec<NetworkDevice> {
    let mut devices = Vec::new();
    
    if let Ok(content) = std::fs::read_to_string("/proc/net/arp") {
        for (i, line) in content.lines().enumerate() {
            if i == 0 { continue; } // Skip header
            
            let parts: Vec<&str> = line.split_whitespace().collect();
            if parts.len() >= 4 {
                let ip = parts[0].to_string();
                let mac = if parts[3] != "00:00:00:00:00:00" {
                    Some(parts[3].to_string())
                } else {
                    None
                };
                
                let hostname = resolve_hostname(&ip);
                devices.push(NetworkDevice::new(ip, mac, hostname));
            }
        }
    }
    
    devices
}

/// Tries to resolve IP to hostname via reverse DNS
fn resolve_hostname(ip: &str) -> Option<String> {
    // Use nslookup simple approach
    if let Ok(output) = Command::new("nslookup").arg(ip).output() {
        let stdout = String::from_utf8_lossy(&output.stdout);
        for line in stdout.lines() {
            if line.to_lowercase().contains("name =") {
                if let Some(name) = line.split('=').nth(1) {
                    return Some(name.trim().trim_end_matches('.').to_string());
                }
            }
        }
    }
    None
}
