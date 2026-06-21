use serde::Deserialize;
use std::fs;

#[derive(Debug, Deserialize, Clone)]
pub struct ShellConfig {
    pub width: u32,
    pub height: u32,
    pub background: String,
    pub frameless: bool,
}

#[derive(Debug, Deserialize, Clone)]
pub struct ThemeConfig {
    pub accent: String,
    pub font: String,
    pub radius: u32,
}

#[derive(Debug, Deserialize, Clone)]
pub struct FragmentConfig {
    pub shell: ShellConfig,
    pub theme: ThemeConfig,
}

impl FragmentConfig {
    pub fn load() -> Self {
        let data = fs::read_to_string("config.yaml")
            .expect("Failed to read config.yaml");

        serde_yaml::from_str(&data)
            .expect("Failed to parse config.yaml")
    }
}
