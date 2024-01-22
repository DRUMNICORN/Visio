// config.rs
use serde::{Deserialize, Serialize};
use std::fs;
use tauri::api::path::home_dir;

#[derive(Serialize, Deserialize)]
pub struct Config {
    key: String,
    value: String,
}

pub fn set_config(key: String, value: String) {
    let config = Config { key, value };
    let serialized = serde_json::to_string(&config).unwrap();

    // Get the path to the config file
    let config_path = home_dir().unwrap().join(".my_app").join("config.json");

    // Create the directory if it doesn't exist
    if let Some(parent) = config_path.parent() {
        fs::create_dir_all(parent).unwrap();
    }

    // Write the config to the file
    fs::write(config_path, serialized).unwrap();
}

pub fn get_config(key: String) -> Option<String> {
    // Get the path to the config file
    let config_path = home_dir().unwrap().join(".my_app").join("config.json");

    // Read the config from the file
    if let Ok(serialized) = fs::read_to_string(config_path) {
        if let Ok(config) = serde_json::from_str::<Config>(&serialized) {
            if config.key == key {
                return Some(config.value);
            }
        }
    }

    None
}

pub fn get_or_default(key: &str, default: &str) -> String {
    match get_config(key.to_string()) {
        Some(value) => value,
        None => default.to_string(),
    }
}