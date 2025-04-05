use serde::{Serialize, Deserialize};
use std::fs;
use dirs;
use colored::Color;

#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    pub host: ColorConfig,
    pub position: Position,
    pub color: ColorForInfo,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ColorConfig {
    pub host_color: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Position {
    pub hostname: u32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ColorForInfo {
    pub main_color: String,
    pub info_color: String,
}

impl Config {
    pub fn get_color(&self, color_type: &str) -> Color {
        let color_str = match color_type {
            "host_color" => self.host.host_color.as_str(),
            "main" => self.color.main_color.as_str(),
            "info" => self.color.info_color.as_str(),
            _ => "white",
        };

        match color_str.to_lowercase().as_str() {
            "black" => Color::Black,
            "red" => Color::Red,
            "green" => Color::Green,
            "yellow" => Color::Yellow,
            "blue" => Color::Blue,
            "magenta" => Color::Magenta,
            "cyan" => Color::Cyan,
            "white" => Color::White,
            "bright_black" => Color::BrightBlack,
            "bright_red" => Color::BrightRed,
            "bright_green" => Color::BrightGreen,
            "bright_yellow" => Color::BrightYellow,
            "bright_blue" => Color::BrightBlue,
            "bright_magenta" => Color::BrightMagenta,
            "bright_cyan" => Color::BrightCyan,
            "bright_white" => Color::BrightWhite,
            _ => Color::White,
        }
    }
}

impl Default for Config {
    fn default() -> Self {
        Self {
            host: ColorConfig::default(),
            position: Position::default(),
            color: ColorForInfo::default(),
        }
    }
}

impl Default for ColorConfig {
    fn default() -> Self {
        Self {
            host_color: "magenta".into(),
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

impl Default for ColorForInfo {
    fn default() -> Self {
        Self {
            main_color: "none".into(),
            info_color: "blue".into(),
        }
    }
}

pub fn load_or_create() -> Result<Config, Box<dyn std::error::Error>> {
    let config_path = dirs::home_dir()
        .ok_or("Home directory not found")?
        .join(".config")
        .join("hostfetch")
        .join("config.toml");

    if let Some(parent) = config_path.parent() {
        fs::create_dir_all(parent)?;
    }

    if !config_path.exists() {
        let toml_content = r#"# All available colors: black, red, green, yellow, blue, magenta, cyan, white, 
# bright_black, bright_red, bright_green, bright_yellow, bright_blue, 
# bright_magenta, bright_cyan, bright_white. 

[host]
host_color = "magenta"  # Color for hostname display

[position]
hostname = 1 

[color]
main_color = "none" 
info_color = "blue"
"#;

        fs::write(&config_path, toml_content)?;
    }

    let content = fs::read_to_string(config_path)?;
    Ok(toml::from_str(&content)?)
}