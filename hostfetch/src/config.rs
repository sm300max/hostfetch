use serde::{Serialize, Deserialize};
use std::fs;
use dirs;
use colored::{Color, ColoredString, Colorize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    pub host: ColorConfig,
    pub position: Position,
    pub color: ColorForInfo,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ColorConfig {
    pub host_color: String,
    #[serde(default)]
    pub style: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Position {
    pub hostname: u32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ColorForInfo {
    pub main_color: String,
    #[serde(default)]
    pub main_style: Vec<String>,
    pub info_color: String,
    #[serde(default)]
    pub info_style: Vec<String>,
}

impl Config {
    pub fn format_host(&self, text: &str) -> ColoredString {
        let color = self.get_color("host_color");
        self.apply_style(text.color(color), &self.host.style)
    }

    pub fn format_main_info(&self, text: &str) -> ColoredString {
        let color = self.get_color("main");
        self.apply_style(text.color(color), &self.color.main_style)
    }

    pub fn format_secondary_info(&self, text: &str) -> ColoredString {
        let color = self.get_color("info");
        self.apply_style(text.color(color), &self.color.info_style)
    }

    fn apply_style(&self, text: ColoredString, styles: &[String]) -> ColoredString {
        styles.iter().fold(text, |acc, style| {
            match style.to_lowercase().as_str() {
                "bold" => acc.bold(),
                "italic" => acc.italic(),
                "underline" => acc.underline(),
                "dimmed" => acc.dimmed(),
                _ => acc,
            }
        })
    }

    pub fn get_color(&self, color_type: &str) -> Color {
        let color_str = match color_type {
            "host_color" => self.host.host_color.as_str(),
            "main" => self.color.main_color.as_str(),
            "info" => self.color.info_color.as_str(),
            _ => "white",
        };

        if let Some(rgb) = Self::parse_hex(color_str) {
            return Color::TrueColor {
                r: rgb.0,
                g: rgb.1,
                b: rgb.2
            };
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
            style: Vec::new(),
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
            main_style: Vec::new(),
            info_color: "blue".into(),
            info_style: Vec::new(),
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

# Available styles: bold, italic, underline, dimmed
# HEX colors: #RGB or #RRGGBB

[host]
host_color = "magenta"  # Host color
style = ["bold"]        # Host styles

[position]
hostname = 1 

[color]
main_color = "none"     # Main info color
main_style = ["italic"] # Main info styles
info_color = "blue"     # Secondary info color
info_style = ["bold"] # Secondary info styles
"#;

        fs::write(&config_path, toml_content)?;
    }

    let content = fs::read_to_string(config_path)?;
    Ok(toml::from_str(&content)?)
}