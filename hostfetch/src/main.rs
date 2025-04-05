mod config;
mod hostname;
mod username;

use colored::Colorize;
use config::{load_or_create, Stylize};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut my_host = String::new();
    let cfg = load_or_create()?;

	let host_color = cfg.get_host_color();
    let host_styles = cfg.get_host_styles();

    let username = match username::get_username() {
        Ok(name) => name,
        Err(e) => {
            eprintln!("Error getting username: {}", e);
            "unknown".to_string()
        }
    };

    match hostname::get_hostname(&mut my_host) {
        Ok(()) => println!(
            "{}@{}", 
            username.color(host_color).style(host_styles),
            my_host.color(host_color).style(host_styles)
        ),
        Err(e) => eprintln!("Error getting hostname: {}", e),
    }

    let separator = "-".repeat(username.len() + my_host.len() + 1);
    println!("{}", separator.color(host_color));

    Ok(())
}