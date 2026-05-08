use sysinfo::{Cpu, System};

/// CPU information and usage statistics
pub struct CpuInfo {
    /// Global CPU usage percentage
    pub usage: f32,
    /// Per-core usage percentages
    pub core_usages: Vec<f32>,
    /// CPU brand/model name
    pub brand: String,
}

impl CpuInfo {
    /// Collects current CPU information from the system
    ///
    /// # Arguments
    /// * `system` - Reference to the system instance
    pub fn collect(system: &System) -> Self {
        let usage = system.global_cpu_usage();
        let cpus: Vec<&Cpu> = system.cpus().iter().collect();
        let core_usages: Vec<f32> = cpus.iter().map(|cpu| cpu.cpu_usage()).collect();

        let brand = if let Some(first_cpu) = cpus.first() {
            first_cpu.brand().to_string()
        } else {
            "Unknown".to_string()
        };

        Self {
            usage,
            core_usages,
            brand,
        }
    }

    #[allow(dead_code)]
    /// Returns the number of CPU cores
    pub fn core_count(&self) -> usize {
        self.core_usages.len()
    }
}
