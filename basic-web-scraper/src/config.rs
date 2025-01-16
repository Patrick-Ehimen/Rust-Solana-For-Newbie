use serde::{Deserialize, Serialize};
use std::fs;

#[derive(Serialize, Deserialize)]
pub struct Config {
    pub urls: Vec<String>,
    pub output_format: String,
}

impl Config {
    pub fn new(config_file: &str) -> Self {
        let config_data = fs::read_to_string(config_file).expect("Unable to read config file");
        serde_json::from_str(&config_data).expect("Unable to parse config file")
    }
}
