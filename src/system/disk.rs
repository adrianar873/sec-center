use std::time::Duration;
use sysinfo::Disks;

/// Disk information including speed calculations
pub struct DiskInfo {
    /// Disk read speed in bytes per second (currently not available in sysinfo 0.31)
    pub read_speed: f64,
    /// Disk write speed in bytes per second (currently not available in sysinfo 0.31)
    pub write_speed: f64,
}

impl DiskInfo {
    /// Creates a new DiskInfo instance with initial values
    ///
    /// # Arguments
    /// * `_disks` - Reference to the disks list (reserved for future use)
    pub fn new(_disks: &Disks) -> Self {
        Self {
            read_speed: 0.0,
            write_speed: 0.0,
        }
    }

    /// Updates disk speed calculations (no-op for now due to API limitations)
    ///
    /// # Arguments
    /// * `_disks` - Reference to the updated disks list
    /// * `_elapsed` - Time elapsed since last update
    pub fn update(&mut self, _disks: &Disks, _elapsed: Duration) {
        // Disk I/O stats not available in sysinfo 0.31 via the Disks API
        // This would require reading from /proc/diskstats directly
    }

    /// Formats bytes per second into human-readable string
    pub fn format_speed(bytes_per_sec: f64) -> String {
        const KB: f64 = 1024.0;
        const MB: f64 = KB * 1024.0;
        const GB: f64 = MB * 1024.0;

        if bytes_per_sec >= GB {
            format!("{:.2} GB/s", bytes_per_sec / GB)
        } else if bytes_per_sec >= MB {
            format!("{:.2} MB/s", bytes_per_sec / MB)
        } else if bytes_per_sec >= KB {
            format!("{:.2} KB/s", bytes_per_sec / KB)
        } else {
            format!("{:.0} B/s", bytes_per_sec)
        }
    }
}
