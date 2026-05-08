use std::fs;

/// Motherboard/Baseboard information
pub struct MotherboardInfo {
    /// Manufacturer name
    pub manufacturer: String,
    /// Product/model name
    pub product: String,
    #[allow(dead_code)]
    /// BIOS version (if available)
    pub bios_version: Option<String>,
}

#[allow(dead_code)]
impl MotherboardInfo {
    /// Detects motherboard information on Linux systems
    pub fn detect() -> Self {
        let manufacturer = Self::read_dmi_info("board_vendor");
        let product = Self::read_dmi_info("board_name");
        let bios_version = Some(Self::read_dmi_info("bios_version")).filter(|s| !s.is_empty());

        Self {
            manufacturer: if manufacturer.is_empty() { "Unknown".to_string() } else { manufacturer },
            product: if product.is_empty() { "Unknown".to_string() } else { product },
            bios_version,
        }
    }

    /// Reads DMI information from /sys/class/dmi/id/
    fn read_dmi_info(field: &str) -> String {
        let path = format!("/sys/class/dmi/id/{}", field);
        fs::read_to_string(&path)
            .ok()
            .map(|s| s.trim().to_string())
            .unwrap_or_default()
    }
}
