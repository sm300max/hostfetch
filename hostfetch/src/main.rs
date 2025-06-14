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
mod terminal;
mod shell;
mod locale;

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

fn draw_centered_border(content: &str, color: colored::Color, max_width: usize) {
    let content_len = visible_length(content);
    let padding = (max_width.saturating_sub(content_len)) / 2;
    let line = format!(
        "{}{}{}",
        " ".repeat(padding),
        content,
        " ".repeat(max_width - content_len - padding)
    );
    draw_border(&[line], color);
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cfg = load_or_create()?;
    let mut all_lines = Vec::new();

    //icons

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

    let terminal_icon = if cfg.icons_enabled() {
        "\u{f489} "
    } else {
        ""
    };

    let shell_icon = if cfg.icons_enabled() {
        "\u{e691} "
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

    let locale_icon = if cfg.icons_enabled() {
        "\u{f274} "
    } else {
        ""
    };

    //names

    let os_name = if cfg.names_enabled() {
        "OS:"
    } else {
        ":"
    };

    let host_name = if cfg.names_enabled() {
        "Host:"
    } else {
        ":"
    };

    let terminal_name = if cfg.names_enabled() {
        "Terminal:"
    } else {
        ":"
    };

    let shell_name = if cfg.names_enabled() {
        "Shell:"
    } else {
        ":"
    };

    let kernel_name = if cfg.names_enabled() {
        "Kernel:"
    } else {
        ":"
    };

    let uptime_name = if cfg.names_enabled() {
        "Uptime:"
    } else {
        ":"
    };

    let load_average_name = if cfg.names_enabled() {
        "Load Average:"
    } else {
        ":"
    };

    let ram_name = if cfg.names_enabled() {
        "RAM:"
    } else {
        ":"
    };

    let swap_name = if cfg.names_enabled() {
        "Swap:"
    } else {
        ":"
    };

    let locale_name = if cfg.names_enabled() {
        "Locale:"
    } else {
        ":"
    };

    //space

    let os_space = if cfg.names_enabled() {
        "              "
    } else {
        " "
    };

    let host_space = if cfg.names_enabled() {
        "            "
    } else {
        " "
    };

    let terminal_space = if cfg.names_enabled() {
        "        "
    } else {
        " "
    };

    let shell_space = if cfg.names_enabled() {
        "           "
    } else {
        " "
    };

    let kernel_space = if cfg.names_enabled() {
        "          "
    } else {
        " "
    };

    let uptime_space = if cfg.names_enabled() {
        "          "
    } else {
        " "
    };

    let load_average_space = if cfg.names_enabled() {
        "    "
    } else {
        " "
    };

    let ram_space = if cfg.names_enabled() {
        "             "
    } else {
        " "
    };

    let swap_space = if cfg.names_enabled() {
        "            "
    } else {
        " "
    };

    let locale_space = if cfg.names_enabled() {
        "          "
    } else {
        " "
    };

    let host = get_device_info();
    let mut my_host = String::new();

    let name_color = cfg.get_name_color();
    let name_style = cfg.get_name_styles();
    let info_color = cfg.get_info_color();
    let info_style = cfg.get_info_styles();
    let host_color = cfg.get_host_color();
    let host_style = cfg.get_host_styles();
    let icon_color = cfg.get_icon_color();

    let os_info: String = oschecker::get_os_info()?;
    let uptime_result = uptime::get_uptime();
    let load_info = load_average::get_loadavg();
    let mem = ram::MemoryData::new();
    let ram_usage = mem.formatted_usage();
    let ram_percent = mem.formatted_percent();
    let swap_data = swap::get_swap_info();
    let terminal = terminal::detect_terminal();
    let shell_info = shell::get_shell_name();
    let locale_result = locale::get_locale();

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

    let locale = match locale_result {
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
    };

    match hostname::get_hostname(&mut my_host) {
        Ok(()) => {
            let user_host = format!(
                "{}@{}",
                username.color(host_color).style(host_style),
                my_host.color(host_color).style(host_style)
            );
            all_lines.push(user_host);
        },
        Err(e) => eprintln!("Error getting hostname: {}", e),
    }

    let os_line = format!(
        "{}{}{}{}",
        os_icon.color(icon_color),
        os_name.color(name_color).style(name_style),
        os_space,
        os_info.color(info_color).style(info_style)
    );            

    let host_line = format!(
        "{}{}{}{}",
        host_icon.color(icon_color),
        host_name.color(name_color).style(name_style),
        host_space,
        host.color(info_color).style(info_style)
    );

    let terminal_line = format!(
        "{}{}{}{}",
        terminal_icon.color(icon_color),
        terminal_name.color(name_color).style(name_style),
        terminal_space,
        terminal.color(info_color).style(info_style)
    );

    let shell_line = format!(
        "{}{}{}{}",
        shell_icon.color(icon_color),
        shell_name.color(name_color).style(name_style),
        shell_space,
        shell_info.color(info_color).style(info_style),
    );

    let kernel_line = format!(
        "{}{}{}{} {}",
        kernel_icon.color(icon_color),
        kernel_name.color(name_color).style(name_style),
        kernel_space,
        uname_data.color(info_color).style(info_style),
        kernel_data.color(info_color).style(info_style)
    );          

    let uptime_line = format!(
        "{}{}{}{}",
        uptime_icon.color(icon_color),
        uptime_name.color(name_color).style(name_style),
        uptime_space,
        uptime.color(info_color).style(info_style)
    );

    let load_average_line = format!(
        "{}{}{}{}",
        load_average_icon.color(icon_color),
        load_average_name.color(name_color).style(name_style),
        load_average_space,
        load_info.color(info_color).style(info_style)
    );

    let ram_line = format!(
        "{}{}{}{} ({})",
        ram_icon.color(icon_color),
        ram_name.color(name_color).style(name_style),
        ram_space,
        ram_usage.color(info_color).style(info_style),
        ram_percent
    );

    let swap_line = format!(
        "{}{}{}{} ({})",
        swap_icon.color(icon_color),
        swap_name.color(name_color).style(name_style),
        swap_space,
        swap_usage.color(info_color).style(info_style),
        swap_percent
    );

    let locale_line = format!(
        "{}{}{}{}",
        locale_icon.color(icon_color),
        locale_name.color(name_color).style(name_style),
        locale_space,
        locale.color(info_color).style(info_style)
    );

    let mut items = vec![
        (cfg.position.host_order, host_line),
        (cfg.position.os_order, os_line),
        (cfg.position.terminal_order, terminal_line),
        (cfg.position.shell_order, shell_line),
        (cfg.position.kernel_order, kernel_line),
        (cfg.position.uptime_order, uptime_line),
        (cfg.position.load_average_order, load_average_line),
        (cfg.position.ram_order, ram_line),
        (cfg.position.swap_order, swap_line),
        (cfg.position.locale_order, locale_line),
    ];

    items.retain(|(order, _)| *order > 0);
    items.sort_by_key(|(order, _)| *order);

    for (_, line) in items {
        all_lines.push(line);
    }

    let max_length = all_lines
        .iter()
        .map(|line| visible_length(line))
        .max()
        .unwrap_or(0);

    let mut system_lines = Vec::new();
    for line in all_lines.iter().skip(1) {
        system_lines.push(line.clone());
    }

    let border_color = cfg.border_color();

    if !all_lines.is_empty() {
        let user_host = all_lines[0].clone();
        draw_centered_border(&user_host, host_color, max_length);
    }

    if !system_lines.is_empty() {
        draw_border(&system_lines, border_color);
    }


    Ok(())
}
