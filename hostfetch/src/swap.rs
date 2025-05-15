use sysinfo::System;
use owo_colors::{OwoColorize, Style};

pub fn get_swap_info() -> Option<(String, String)> {
    let mut system = System::new();
    system.refresh_memory();

    let total = system.total_swap();
    let free = system.free_swap();

    if total == 0 {
        return None;
    }

    let used = total - free;
    let ratio = format!("{} / {}", format_size(used), format_size(total));
    
    let percent = (used as f64 / total as f64) * 100.0;
    let percent_value = format!("{:.0}%", percent);
    
    let bold_style = Style::new().bold();
    
    let colored_percent = if percent <= 50.0 {
        percent_value
            .style(bold_style
                .green())
            .to_string()
    } else if percent <= 75.0 {
        percent_value
            .style(bold_style
                .yellow())
            .to_string()
    } else {
        percent_value
            .style(bold_style
                .red())
            .to_string()
    };

    Some((ratio, colored_percent))
}

fn format_size(bytes: u64) -> String {
    const UNITS: [&str; 5] = ["B", "KB", "MB", "GB", "TB"];
    let mut size = bytes as f64;
    let mut unit_index = 0;

    while size >= 1024.0 && unit_index < UNITS.len() - 1 {
        size /= 1024.0;
        unit_index += 1;
    }

    if size.fract() == 0.0 || unit_index == 0 {
        format!("{:.0} {}", size, UNITS[unit_index])
    } else {
        format!("{:.1} {}", size, UNITS[unit_index])
    }
}