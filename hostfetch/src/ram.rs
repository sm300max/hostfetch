use sysinfo::{MemoryRefreshKind, RefreshKind, System};

const BOLD: &str = "\x1b[1m";
const GREEN: &str = "\x1b[32m";
const YELLOW: &str = "\x1b[33m";
const RED: &str = "\x1b[31m";
const RESET: &str = "\x1b[0m";

pub struct MemoryData {
    used_bytes: u64,
    total_bytes: u64,
    percent: f64,
}

impl MemoryData {
    pub fn new() -> Self {
        let mut sys = System::new_with_specifics(
            RefreshKind::new().with_memory(MemoryRefreshKind::new().with_ram())
        );
        sys.refresh_memory();
        
        let total = sys.total_memory();
        let used = sys.used_memory();
        let percent = if total > 0 {
            (used as f64 / total as f64) * 100.0
        } else {
            0.0
        };

        MemoryData {
            used_bytes: used,
            total_bytes: total,
            percent,
        }
    }

    pub fn formatted_usage(&self) -> String {
        format!(
            "{:.1} GB / {:.1} GB",
            self.used_bytes as f64 / 1e9,
            self.total_bytes as f64 / 1e9
        )
    }

    pub fn formatted_percent(&self) -> String {
        let color = match self.percent {
            p if p < 50.0 => GREEN,
            p if p < 75.0 => YELLOW,
            _ => RED,
        };
        format!("{}{}{BOLD}{:.0}%{}", color, BOLD, self.percent, RESET)
    }
}