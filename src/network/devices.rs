use serde::{Deserialize, Serialize};
use std::time::{SystemTime, UNIX_EPOCH};

/// Represents a discovered network device with IP, MAC, and hostname
///
/// Devices are tracked with first seen and last seen timestamps
/// for baseline comparison and device management.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct NetworkDevice {
    /// IP address of the device
    pub ip: String,
    /// MAC address (if available)
    pub mac: Option<String>,
    /// Resolved hostname (if available)
    pub hostname: Option<String>,
    /// Unix timestamp when device was first seen
    pub first_seen: u64,
    /// Unix timestamp when device was last seen
    pub last_seen: u64,
}

impl NetworkDevice {
    /// Creates a new device with current timestamp
    pub fn new(ip: String, mac: Option<String>, hostname: Option<String>) -> Self {
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs();
        
        Self {
            ip,
            mac,
            hostname,
            first_seen: now,
            last_seen: now,
        }
    }

    /// Formats time elapsed since last seen
    pub fn last_seen_ago(&self) -> String {
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs();
        let diff = now - self.last_seen;
        
        if diff < 60 {
            format!("{} seconds ago", diff)
        } else if diff < 3600 {
            format!("{} minutes ago", diff / 60)
        } else {
            format!("{} hours ago", diff / 3600)
        }
    }
}
