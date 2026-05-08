use sysinfo::System;

#[allow(dead_code)]
/// Process information for display
pub struct ProcessInfo {
    /// Process name
    pub name: String,
    /// CPU usage percentage
    pub cpu_usage: f32,
    /// Memory usage in bytes
    pub memory: u64,
    #[allow(dead_code)]
    /// Process ID
    pub pid: u32,
}

#[allow(dead_code)]
impl ProcessInfo {
    /// Collects and sorts processes by CPU usage
    ///
    /// # Arguments
    /// * `system` - Reference to the system instance
    /// * `limit` - Maximum number of processes to return
    pub fn collect_top(system: &System, limit: usize) -> Vec<Self> {
        let mut processes: Vec<Self> = system
            .processes()
            .values()
            .map(|p| Self {
                name: p.name().to_string_lossy().to_string(),
                cpu_usage: p.cpu_usage(),
                memory: p.memory(),
                pid: p.pid().as_u32(),
            })
            .collect();

        processes.sort_by(|a, b| b.cpu_usage.partial_cmp(&a.cpu_usage).unwrap());
        processes.truncate(limit);
        processes
    }

    /// Formats memory into human-readable string
    pub fn format_memory(bytes: u64) -> String {
        const MB: u64 = 1024 * 1024;
        const GB: u64 = MB * 1024;

        if bytes >= GB {
            format!("{:.2} GB", bytes as f64 / GB as f64)
        } else {
            format!("{} MB", bytes / MB)
        }
    }
}
