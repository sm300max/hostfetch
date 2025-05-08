mod config;
mod hostname;
mod username;
mod oschecker;
mod host;
mod kernel;
mod uptime;
mod load_average;
mod ram;
mod swap;

use colored::Colorize;
use config::{load_or_create, Stylize};
use host::get_device_info;
use lazy_static::lazy_static;
use regex::Regex;

lazy_static! {
    static ref ANSI_ESCAPE: Regex = Regex::new(r"\x1B\[[0-9;]*[a-zA-Z]").unwrap();
}

fn visible_length(s: &str) -> usize {
    ANSI_ESCAPE.replace_all(s, "").chars().count()
}

fn draw_border(lines: &[String], color: colored::Color) {
    if lines.is_empty() {
        return;
    }

    let max_length = lines
        .iter()
        .map(|line| visible_length(line))
        .max()
        .unwrap_or(0);

    let top = format!("╭─{}─╮", "─".repeat(max_length)).color(color);
    let bottom = format!("╰─{}─╯", "─".repeat(max_length)).color(color);

    println!("{}", top);
    for line in lines {
        let padding = max_length - visible_length(line);
        println!(
            "{} {}{} {}",
            "│".color(color),
            line,
            " ".repeat(padding),
            "│".color(color)
        );
    }
    println!("{}", bottom);
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cfg = load_or_create()?;
    let mut output_lines = Vec::new();

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

    let uptime_icon = if cfg.icons_enabled() {
        "\u{f43a} "
    } else {
        ""
    };

    let load_average_icon = if cfg.icons_enabled() {
        "\u{23f2} "
    } else {
        ""
    };

    let ram_icon = if cfg.icons_enabled() {
        "\u{f035b} "
    } else {
        ""
    };

    let swap_icon = if cfg.icons_enabled() {
        "\u{ebcb} "
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
    let uptime_result = uptime::get_uptime();
    let load_info = load_average::get_loadavg();
    let mem = ram::MemoryData::new();
    let ram_usage = mem.formatted_usage();
    let ram_percent = mem.formatted_percent();
    let swap_data = swap::get_swap_info();

    let (swap_usage, swap_percent) = match swap_data {
        Some(data) => data,
        None => {
            ("No swap".to_string(), "(0%)".to_string())
        }
    };

    let uptime = match uptime_result {
        Ok(value) => value,
        Err(e) => {
            eprintln!("Error: {}", e);
            return Err(e.into());
        }
    };

    let (uname_data, kernel_data) = match kernel::get_uname_data() {
        Ok(data) => (data.uname, data.kernel_version),
        Err(e) => {
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
    };    println!("{}", swap_percent);

    match hostname::get_hostname(&mut my_host) {
        Ok(()) => output_lines.push(format!(
            "{}@{}",
            username.color(host_color).style(host_styles),
            my_host.color(host_color).style(host_styles)
        )),
        Err(e) => eprintln!("Error getting hostname: {}", e),
    }

    let separator = "-".repeat(visible_length(&format!("{}@{}", username, my_host)));
    output_lines.push(format!("{}", separator.color(host_color)));

    let os_line = format!(
        "{}{}              {}",
        os_icon.color(icon_color),
        "OS:".color(main_color).style(main_style),
        os_info.color(info_color).style(info_style)
    );

    let host_line = format!(
        "{}{}            {}",
        host_icon.color(icon_color),
        "Host:".color(main_color).style(main_style),
        host.color(info_color).style(info_style)
    );

    let kernel_line = format!(
        "{}{}          {} {}",
        kernel_icon.color(icon_color),
        "Kernel:".color(main_color).style(main_style),
        uname_data.color(info_color).style(info_style),
        kernel_data.color(info_color).style(info_style)
    );
    println!("{}", swap_percent);
    let uptime_line = format!(
        "{}{}          {}",
        uptime_icon.color(icon_color),
        "Uptime:".color(main_color).style(main_style),
        uptime.color(info_color).style(info_style)
    );

    let load_average_line = format!(
        "{}{}    {}",
        load_average_icon.color(icon_color),
        "Load Average:".color(main_color).style(main_style),
        load_info.color(info_color).style(info_style)
    );

    let ram_line = format!(
        "{}{}             {} ({})", 
        ram_icon.color(icon_color), 
        "RAM:".color(main_color).style(main_style), 
        ram_usage.color(info_color).style(info_style), 
        ram_percent
    );

    let swap_line = format!(
        "{}{}            {} ({})",
        swap_icon.color(icon_color),
        "Swap:".color(main_color).style(main_style),
        swap_usage.color(info_color).style(info_style),
        swap_percent
    );

    let mut items = vec![
        (cfg.position.host_order, host_line),
        (cfg.position.os_order, os_line),
        (cfg.position.kernel_order, kernel_line),
        (cfg.position.uptime_order, uptime_line),
        (cfg.position.load_average_order, load_average_line),
        (cfg.position.ram_order, ram_line),
        (cfg.position.swap_order, swap_line),
    ];

    items.retain(|(order, _)| *order > 0);
    items.sort_by_key(|(order, _)| *order);

    for (_, line) in items {
        output_lines.push(line);
    }

    draw_border(&output_lines, cfg.border_color());

    Ok(())
}