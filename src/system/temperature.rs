use sysinfo::Components;

/// Temperature sensor information
pub struct TemperatureInfo {
    /// List of temperature sensors with their values
    pub sensors: Vec<TempSensor>,
}

/// Individual temperature sensor data
pub struct TempSensor {
    /// Sensor label (e.g., "CPU", "GPU")
    pub label: String,
    /// Temperature in Celsius
    pub temperature: f32,
    #[allow(dead_code)]
    /// Maximum temperature (if available)
    pub max: Option<f32>,
}

impl TemperatureInfo {
    /// Collects temperature information from system components
    ///
    /// # Arguments
    /// * `components` - Reference to the system components list
    pub fn collect(components: &Components) -> Self {
        let sensors: Vec<TempSensor> = components
            .list()
            .iter()
            .filter_map(|component| {
                let temp = component.temperature();
                if temp > 0.0 {
                    return Some(TempSensor {
                        label: component.label().to_string(),
                        temperature: temp,
                        max: Some(component.max()),
                    });
                }
                None
            })
            .collect();

        Self { sensors }
    }

    /// Finds CPU temperature sensor (if available)
    pub fn cpu_temperature(&self) -> Option<f32> {
        self.sensors
            .iter()
            .find(|s| s.label.to_lowercase().contains("cpu") || s.label.to_lowercase().contains("core"))
            .map(|s| s.temperature)
    }

    /// Finds GPU temperature sensor (if available)
    pub fn gpu_temperature(&self) -> Option<f32> {
        self.sensors
            .iter()
            .find(|s| s.label.to_lowercase().contains("gpu"))
            .map(|s| s.temperature)
    }
}
