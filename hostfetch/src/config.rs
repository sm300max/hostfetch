use serde::{Serialize, Deserialize};
use std::fs;
use dirs;

#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    pub position: Position,
    pub color: Color,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Position {
    pub hostname: u32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Color {
    pub main_color: String,
    pub info_color: String,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            position: Position::default(),
            color: Color::default(),
        }
    }
}

impl Default for Position {
    fn default() -> Self {
        Self {
            hostname: 1,
        }
    }
}

impl Default for Color {
    fn default() -> Self {
        Self {
            main_color: "none".into(),
            info_color: "blue".into(),
        }
    }
}

pub fn load_or_create() -> Result<Config, Box<dyn std::error::Error>> {
    let mut config_path = dirs::home_dir()
        .ok_or("Home directory not found")?
        .join(".config")
        .join("hostfetch")
        .join("config.toml");

    if let Some(parent) = config_path.parent() {
        fs::create_dir_all(parent)?;
    }

    if !config_path.exists() {
        let default_config = Config::default();
        let toml = toml::to_string_pretty(&default_config)?;
        fs::write(&config_path, toml)?;
    }

    let content = fs::read_to_string(config_path)?;
    Ok(toml::from_str(&content)?)
}