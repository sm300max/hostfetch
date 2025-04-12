use libc;
use std::io;

pub fn get_loadavg_string() -> String {
    match try_get_loadavg() {
        Ok(avg) => format!("{:.2}, {:.2}, {:.2}", avg[0], avg[1], avg[2]),
        Err(e) => format!("Error: {}", e)
    }
}

fn try_get_loadavg() -> Result<[f64; 3], Box<dyn std::error::Error>> {
    let mut loads = [0.0f64; 3];

    unsafe {
        let result = libc::getloadavg(loads.as_mut_ptr(), 3);

        match result {
            3 => Ok(loads),
            x if x < 0 => Err(Box::new(io::Error::last_os_error())),
            _ => Err(Box::new(io::Error::new(
                io::ErrorKind::InvalidData,
                "Error while getting load average"
            )))
        }
    }
}