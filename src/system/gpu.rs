use std::process::Command;

/// GPU information structure
pub struct GpuInfo {
    /// GPU name/model
    pub name: String,
    #[allow(dead_code)]
    /// GPU usage percentage (if available)
    pub usage: Option<f32>,
    #[allow(dead_code)]
    /// GPU memory total in bytes (if available)
    pub memory_total: Option<u64>,
    #[allow(dead_code)]
    /// GPU memory used in bytes (if available)
    pub memory_used: Option<u64>,
    #[allow(dead_code)]
    /// GPU temperature in Celsius (if available)
    pub temperature: Option<f32>,
    /// Whether a GPU was detected
    pub detected: bool,
}

impl GpuInfo {
    /// Creates a new GpuInfo instance by detecting available GPUs
    pub fn detect() -> Self {
        // Try NVIDIA first
        if let Some(mut info) = Self::detect_nvidia() {
            info.detected = true;
            return info;
        }

        // Try AMD
        if let Some(mut info) = Self::detect_amd() {
            info.detected = true;
            return info;
        }

        // Fallback: no GPU detected
        Self {
            name: "No GPU detected".to_string(),
            usage: None,
            memory_total: None,
            memory_used: None,
            temperature: None,
            detected: false,
        }
    }

    /// Attempts to detect NVIDIA GPU using nvidia-smi
    fn detect_nvidia() -> Option<Self> {
        let output = Command::new("nvidia-smi")
            .args(&[
                "--query-gpu=name,utilization.gpu,memory.total,memory.used,temperature.gpu",
                "--format=csv,noheader,nounits",
            ])
            .output()
            .ok()?;

        if !output.status.success() {
            return None;
        }

        let stdout = String::from_utf8_lossy(&output.stdout);
        let line = stdout.lines().next()?;
        let parts: Vec<&str> = line.split(',').map(|s| s.trim()).collect();

        if parts.len() >= 5 {
            let name = parts[0].to_string();
            let usage = parts[1].parse::<f32>().ok();
            let memory_total = parts[2].parse::<u64>().ok().map(|mb| mb * 1024 * 1024);
            let memory_used = parts[3].parse::<u64>().ok().map(|mb| mb * 1024 * 1024);
            let temperature = parts[4].parse::<f32>().ok();

            Some(Self {
                name,
                usage,
                memory_total,
                memory_used,
                temperature,
                detected: true,
            })
        } else {
            None
        }
    }

    /// Attempts to detect AMD GPU (basic implementation)
    fn detect_amd() -> Option<Self> {
        let output = Command::new("lspci").args(&["-k"]).output().ok()?;

        let stdout = String::from_utf8_lossy(&output.stdout);

        for line in stdout.lines() {
            if line.to_lowercase().contains("vga") && line.to_lowercase().contains("amd") {
                return Some(Self {
                    name: "AMD GPU (basic info)".to_string(),
                    usage: None,
                    memory_total: None,
                    memory_used: None,
                    temperature: None,
                    detected: true,
                });
            }
        }

        None
    }
}
