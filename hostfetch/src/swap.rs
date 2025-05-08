use sysinfo::System;

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
    let percent = format!("{:.0}%", (used as f64 / total as f64) * 100.0);

    Some((ratio, percent))
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
        format!("{:.0}{}", size, UNITS[unit_index])
    } else {
        format!("{:.1}{}", size, UNITS[unit_index])
    }
}