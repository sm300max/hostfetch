use std::io;
use std::fs;
use libc;

pub fn capture_hostname(hostname: &mut String) -> io::Result<()> {
    if try_libc_hostname(hostname).is_ok() {
        return Ok(());
    }

    if let Ok(proc_hostname) = fs::read_to_string("/proc/sys/kernel/hostname") {
        *hostname = proc_hostname.trim_end_matches('\n').to_string();
        return Ok(());
    }

    if let Ok(env_host) = std::env::var("HOSTNAME") {
        *hostname = env_host;
        return Ok(());
    }

    Err(io::Error::new(io::ErrorKind::PermissionDenied, "Failed to get hostname"))
}

fn try_libc_hostname(hostname :&mut String) -> io::Result<()> {
    let mut buf = [0u8; 64];
    let result = unsafe {
        libc::gethostname(buf.as_mut_ptr() as *mut libc::c_char, buf.len())
    };

    if result != 0 {
        return Err(io::Error::last_os_error());
    }

    let end = buf.iter().position(|&b| b == 0).unwrap_or(buf.len());
    *hostname = String::from_utf8_lossy(&buf[..end]).into_owned();
    Ok(())
}