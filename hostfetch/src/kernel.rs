use std::fs;
use std::io;

pub fn get_kernel_version() -> Result<String, io::Error> {
    let version = fs::read_to_string("/proc/sys/kernel/osrelease")?;
    Ok(version.trim().to_string())
}