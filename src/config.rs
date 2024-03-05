use std::{path::PathBuf, str::FromStr};

use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct ConfigFile {
    pub slack_channel_id: String,
    pub slack_app_token: String,
    pub disk_threshold: u8,
}

impl Default for ConfigFile {
    fn default() -> Self {
        Self {
            slack_channel_id: "".to_string(),
            slack_app_token: "".to_string(),
            disk_threshold: 80,
        }
    }
}

impl ConfigFile {
    pub fn save(&self) {
        let config = serde_yaml::to_string(&self).expect("Failed to serialize config file");
        std::fs::write(CONFIG_PATH, config).expect("Failed to write config file");
    }
}

const CONFIG_PATH: &str = if cfg!(linux) { "/etc/nosyman.yaml" } else { "" };

pub fn load_config() -> ConfigFile {
    let config = std::fs::read_to_string(CONFIG_PATH).expect("Failed to read config file");
    serde_yaml::from_str(&config).expect("Failed to parse config file")
}

pub fn exists_config() -> bool {
    PathBuf::from_str(CONFIG_PATH).unwrap().exists()
}
