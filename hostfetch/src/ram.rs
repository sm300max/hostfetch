use sysinfo::{RefreshKind, System, MemoryRefreshKind};

pub fn get_memory_usage() -> String {
    let mut sys = System::new_with_specifics(
        RefreshKind::new().with_memory(
            MemoryRefreshKind::new().with_ram() // Явно запрашиваем обновление RAM
        )
    );
    
    sys.refresh_memory();
    
    let total = sys.total_memory();
    let used = sys.used_memory();

    format_memory(used, total)
}

fn format_memory(used: u64, total: u64) -> String {
    const UNIT_GB: u64 = 1024 * 1024 * 1024;
    const UNIT_MB: u64 = 1024 * 1024;

    let (total_value, unit) = if total >= UNIT_GB {
        (total as f64 / UNIT_GB as f64, "GB")
    } else {
        (total as f64 / UNIT_MB as f64, "MB")
    };

    let used_value = if total >= UNIT_GB {
        used as f64 / UNIT_GB as f64
    } else {
        used as f64 / UNIT_MB as f64
    };

    format!("{:.1} {} / {:.1} {}", used_value, unit, total_value, unit)
}