// System module - handles system monitoring components
//
// This module collects and refreshes system information including:
// - CPU brand, model, and usage
// - RAM total and used memory
// - Disk information and I/O speeds
// - GPU detection
// - Motherboard information
// - Temperature sensors

pub mod cpu;
pub mod memory;
pub mod disk;
pub mod gpu;
pub mod temperature;
pub mod processes;
pub mod motherboard;

use sysinfo::{System, Disks, Components};
use std::time::Instant;

/// Holds all system information collected from various sources
///
/// This struct aggregates data from sysinfo and other system APIs.
/// It is refreshed periodically via the `refresh()` method.
pub struct SystemInfo {
    /// Main system struct from sysinfo crate (CPU, memory, etc.)
    pub system: System,
    /// List of disks with usage information
    pub disks: Disks,
    /// List of hardware components (temperature sensors, etc.)
    pub components: Components,
    /// Calculated disk I/O speeds
    pub disk_info: disk::DiskInfo,
    /// Timestamp of last refresh (for calculating speeds)
    pub last_update: Instant,
}

impl SystemInfo {
    /// Creates a new SystemInfo instance and initializes all components
    ///
    /// This performs initial refresh of all system data.
    /// Note: This may take some time on first call due to sysinfo initialization.
    pub fn new() -> Self {
        let mut system = System::new_all();
        system.refresh_all();

        let disks = Disks::new_with_refreshed_list();
        let components = Components::new_with_refreshed_list();

        let disk_info = disk::DiskInfo::new(&disks);

        Self {
            system,
            disks,
            components,
            disk_info,
            last_update: Instant::now(),
        }
    }

    /// Refreshes all system information
    ///
    /// Updates CPU, memory, disk, and component data.
    /// Also recalculates disk I/O speeds based on elapsed time.
    pub fn refresh(&mut self) {
        self.system.refresh_all();
        self.disks.refresh();
        self.components.refresh();
        self.disk_info.update(&self.disks, self.last_update.elapsed());
        self.last_update = Instant::now();
    }
}
