mod config;
mod hostname;
mod username;
mod oschecker;
mod host;
mod kernel;

use colored::Colorize;
use config::{load_or_create, Stylize};
use host::get_device_info;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cfg = load_or_create()?;

    let os_icon = if cfg.icons_enabled() {
        "\u{f31a} "
    } else {
        ""
    };

    let host_icon = if cfg.icons_enabled() {
        "\u{f109} "
    } else {
        ""
    };

    let kernel_icon = if cfg.icons_enabled() {
        "\u{f013} "
    } else {
        ""
    };

    let host = get_device_info();

    let mut my_host = String::new();

    let main_color = cfg.get_main_color();
    let main_style = cfg.get_main_styles();
    let info_color = cfg.get_secondary_color();
    let info_style = cfg.get_secondary_styles();
	let host_color = cfg.get_host_color();
    let host_styles = cfg.get_host_styles();
    let icon_color = cfg.get_icon_color();

    let os_info: String = oschecker::get_os_info()?;

    let (uname_data, kernel_data) = match kernel::get_uname_data() {
        Ok(data) => (data.uname, data.kernel_version),
        Err(e) =>{
            eprintln!("Error: {}", e);
            ("Unknown".into(), "Unknown".into())
        }
    };

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

    let os_line = format!("{}{}:      {}",os_icon.color(icon_color), "OS".color(main_color).style(main_style), os_info.color(info_color).style(info_style));

    let host_line = format!("{}{}:    {}", host_icon.color(icon_color), "Host".color(main_color).style(main_style), host.color(info_color).style(info_style));

    let kernel_line = format!("{}{}:  {} {}", kernel_icon.color(icon_color), "Kernel".color(main_color).style(main_style), uname_data.color(info_color).style(info_style), kernel_data.color(info_color).style(info_style));

    let mut items = vec![
        (cfg.position.host_order, host_line),
        (cfg.position.os_order, os_line),
        (cfg.position.kernel_order, kernel_line),
    ];

    items.retain(|(order, _)| *order > 0);
    items.sort_by_key(|(order, _)| *order);

    for (_, line) in &items {
        println!("{}", line);
    }

    Ok(())
}
