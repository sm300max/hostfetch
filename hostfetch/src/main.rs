mod config;
mod hostname;
mod username;
mod oschecker;

use colored::Colorize;
use config::{load_or_create, Stylize};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut my_host = String::new();
    let cfg = load_or_create()?;

    let main_color = cfg.get_main_color();
    let main_style = cfg.get_main_styles();

    let info_color = cfg.get_secondary_color();
    let info_style = cfg.get_secondary_styles();

	let host_color = cfg.get_host_color();
    let host_styles = cfg.get_host_styles();

    let os_info: String = oschecker::get_os_info()?;

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

    println!("{}: {}", "OS".color(main_color).style(main_style), os_info.color(info_color).style(info_style));

    Ok(())
}