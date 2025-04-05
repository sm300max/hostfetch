use serde::{Deserialize, Serialize};
use std::fs;
use dirs;
use colored::{Color, ColoredString, Colorize};

pub trait Stylize {
    fn style(self, styles: &[String]) -> ColoredString;
}

impl Stylize for ColoredString {
    fn style(mut self, styles: &[String]) -> ColoredString {
        for style in styles {
            self = match style.to_lowercase().as_str() {
                "bold" => self.bold(),
                "italic" => self.italic(),
                "underline" => self.underline(),
                "dimmed" => self.dimmed(),
                "blink" => self.blink(),
                "reverse" => self.reversed(),
                _ => self,
            };
        }
        self
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    pub host: HostStyle,
    pub position: Position,
    pub info: InfoStyle,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct HostStyle {
    pub color: String,
    #[serde(default)]
    pub styles: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Position {
    pub hostname_order: u32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct InfoStyle {
    pub main_color: String,
    #[serde(default)]
    pub main_styles: Vec<String>,
    pub secondary_color: String,
    #[serde(default)]
    pub secondary_styles: Vec<String>,
}

impl Config {
    pub fn get_host_color(&self) -> Color {
        self.parse_color(&self.host.color)
    }

    pub fn get_host_styles(&self) -> &Vec<String> {
        &self.host.styles
    }

    fn parse_color(&self, color_str: &str) -> Color {
        if let Some(rgb) = Self::parse_hex(color_str) {
            return Color::TrueColor { r: rgb.0, g: rgb.1, b: rgb.2 };
        }

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

    fn parse_hex(hex: &str) -> Option<(u8, u8, u8)> {
        let hex = hex.trim_start_matches('#');
        
        match hex.len() {
            3 => {
                let r = u8::from_str_radix(&hex[0..1], 16).ok()?;
                let g = u8::from_str_radix(&hex[1..2], 16).ok()?;
                let b = u8::from_str_radix(&hex[2..3], 16).ok()?;
                Some((r * 17, g * 17, b * 17))
            }
            6 => {
                let r = u8::from_str_radix(&hex[0..2], 16).ok()?;
                let g = u8::from_str_radix(&hex[2..4], 16).ok()?;
                let b = u8::from_str_radix(&hex[4..6], 16).ok()?;
                Some((r, g, b))
            }
            _ => None,
        }
    }
}

impl Default for Config {
    fn default() -> Self {
        Self {
            host: HostStyle {
                color: "magenta".into(),
                styles: vec!["bold".into()],
            },
            position: Position {
                hostname_order: 1,
            },
            info: InfoStyle {
                main_color: "cyan".into(),
                main_styles: vec!["italic".into()],
                secondary_color: "blue".into(),
                secondary_styles: vec!["bold".into()],
            },
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
        let toml_content = r#"
[host]
color = "magenta"
styles = ["bold"]

[position]
hostname_order = 1

[info]
main_color = "cyan"
main_styles = ["italic"]
secondary_color = "blue"
secondary_styles = ["bold"]
        "#;

        fs::write(&config_path, toml_content)?;
    }

    let content = fs::read_to_string(config_path)?;
    Ok(toml::from_str(&content)?)
}