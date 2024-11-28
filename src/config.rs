use serde::{Deserialize, Serialize};
use std::fs;
use std::path::Path;

pub const MEMORY_WARNING_THRESHOLD: f64 = 80.0;
pub const MEMORY_CRITICAL_THRESHOLD: f64 = 90.0;
pub const BYTES_TO_GB: f64 = 1024.0 * 1024.0 * 1024.0;
pub const UPDATE_INTERVAL: u64 = 1;

#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    pub update_interval: u64,
    pub log_level: String,
    pub log_dir: String,
    pub max_log_size: u64,
    pub max_backup_files: u32,
    pub display: DisplayConfig,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DisplayConfig {
    pub bar_width: usize,
    pub show_disk_info: bool,
    pub show_system_load: bool,
    pub refresh_rate: u64,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            update_interval: 1,
            log_level: "info".to_string(),
            log_dir: "logs".to_string(),
            max_log_size: 10 * 1024 * 1024,
            max_backup_files: 5,
            display: DisplayConfig::default(),
        }
    }
}

impl Config {
    pub fn load() -> Result<Self, Box<dyn std::error::Error>> {
        let config_path = "config.json";
        if !Path::new(config_path).exists() {
            let config = Config::default();
            let json = serde_json::to_string_pretty(&config)?;
            fs::write(config_path, json)?;
            return Ok(config);
        }
        let content = fs::read_to_string(config_path)?;
        Ok(serde_json::from_str(&content)?)
    }
}

impl Default for DisplayConfig {
    fn default() -> Self {
        Self {
            bar_width: 50,
            show_disk_info: true,
            show_system_load: true,
            refresh_rate: 1,
        }
    }
}
