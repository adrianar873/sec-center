use sysinfo::System;

/// Memory information including RAM usage
pub struct MemoryInfo {
    /// Total physical memory in bytes
    pub total: u64,
    /// Used physical memory in bytes
    pub used: u64,
    #[allow(dead_code)]
    /// Total swap memory in bytes
    pub swap_total: u64,
    #[allow(dead_code)]
    /// Used swap memory in bytes
    pub swap_used: u64,
}

impl MemoryInfo {
    /// Collects current memory information from the system
    ///
    /// # Arguments
    /// * `system` - Reference to the system instance
    pub fn collect(system: &System) -> Self {
        Self {
            total: system.total_memory(),
            used: system.used_memory(),
            swap_total: system.total_swap(),
            swap_used: system.used_swap(),
        }
    }

    /// Returns memory usage as a percentage
    pub fn usage_percent(&self) -> f32 {
        if self.total > 0 {
            (self.used as f32 / self.total as f32) * 100.0
        } else {
            0.0
        }
    }

    /// Formats bytes into human-readable string (GB or MB)
    pub fn format_bytes(bytes: u64) -> String {
        const GB: u64 = 1024 * 1024 * 1024;
        const MB: u64 = 1024 * 1024;

        if bytes >= GB {
            format!("{:.2} GB", bytes as f32 / GB as f32)
        } else {
            format!("{:.0} MB", bytes / MB)
        }
    }
}
