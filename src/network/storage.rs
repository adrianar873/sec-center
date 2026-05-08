use std::collections::HashMap;
use std::fs;
use std::path::PathBuf;
use crate::network::devices::NetworkDevice;
use anyhow::Result;

/// Gets the config directory: ~/.config/sec-center/
fn get_config_dir() -> PathBuf {
    let mut dir = dirs::home_dir().expect("Could not get home directory");
    dir.push(".config");
    dir.push("sec-center");
    
    if let Err(e) = fs::create_dir_all(&dir) {
        eprintln!("Error creating directory: {}", e);
    }
    
    dir
}

/// Saves the baseline to disk as JSON
pub fn save_baseline(devices: &HashMap<String, NetworkDevice>) -> Result<()> {
    let path = get_config_dir().join("baseline.json");
    let json = serde_json::to_string_pretty(devices)?;
    
    fs::write(&path, json)?;
    
    Ok(())
}

/// Loads the baseline from disk
pub fn load_baseline() -> Result<HashMap<String, NetworkDevice>> {
    let path = get_config_dir().join("baseline.json");
    
    if !path.exists() {
        return Ok(HashMap::new());
    }
    
    let content = fs::read_to_string(&path)?;
    let devices = serde_json::from_str(&content)?;
    
    Ok(devices)
}
