use libc::{self, c_int, c_double};

#[cfg(target_os = "android")]
const PROC_PATH: &str = "/proc/loadavg";

pub fn get_loadavg_string() -> String {
    let (one, five, fifteen) = match try_getloadavg() {
        Ok(avg) => avg,
        Err(e) => return format!("Error: {}", e),
    };

    format!("{:.2}, {:.2}, {:.2}", one, five, fifteen)
}

fn try_getloadavg() -> Result<(f64, f64, f64), String> {
    #[cfg(not(target_os = "android"))]
    {
        let mut loads: [c_double; 3] = [0.0; 3];
        let result = unsafe {
            libc::getloadavg(loads.as_mut_ptr(), 3)
        };

        if result == 3 {
            Ok((loads[0], loads[1], loads[2]))
        } else {
            Err(format!("Error while getting Load Average (errno: {})", errno()))
        }
    }

    #[cfg(target_os = "android")]
    {
        use std::fs;
        use std::io;

        let content = fs::read_to_string(PROC_PATH)
            .map_err(|e| format!("Failed to read {}: {}", PROC_PATH, e))?;

        let parse_float = |s: &str| -> Result<f64, String> {
            s.parse().map_err(|_| format!("Invalid number: {}", s))
        };

        let mut parts = content.split_whitespace();
        let one = parts.next().ok_or("Missing 1m value")?;
        let five = parts.next().ok_or("Missing 5m value")?;
        let fifteen = parts.next().ok_or("Missing 15m value")?;

        Ok((
            parse_float(one)?,
            parse_float(five)?,
            parse_float(fifteen)?,
        ))
    }
}

#[cfg(not(target_os = "android"))]
fn errno() -> c_int {
    unsafe {
        *libc::__errno_location()
    }
}