#[cfg(not(target_os = "android"))]
use libc;

pub fn get_loadavg() -> String {
    #[cfg(not(target_os = "android"))] {
        let mut loads: [f64; 3] = [0.0; 3];
        unsafe {
            let result = libc::getloadavg(loads.as_mut_ptr(), 3);
            if result == 3 {
                return format!("{:.2}, {:.2}, {:.2}", loads[0], loads[1], loads[2]);
            }
        }
    }

    #[cfg(target_os = "android")] {
        if let Ok(content) = std::fs::read_to_string("/proc/loadavg") {
            let parts: Vec<&str> = content.split_whitespace().take(3).collect();
            if parts.len() == 3 {
                return format!("{}, {}, {}", parts[0], parts[1], parts[2]);
            }
        }
    }

    "N/A".to_string()
}
