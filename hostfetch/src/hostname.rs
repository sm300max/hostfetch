use std::ffi::CStr;
use std::io;
use libc;

const HOSTNAME_MAX_LEN: usize = 256;

fn try_posix_hostname(hostname: &mut String) -> io::Result<()> {
    let mut buf = [0u8; HOSTNAME_MAX_LEN];

    unsafe {
        if libc::gethostname(buf.as_mut_ptr() as *mut libc::c_char , buf.len()) != 0 {
            return Err(io::Error::last_os_error());
        }
    }

    let cstr = CStr::from_bytes_until_nul(&buf)
        .map_err(|_| io::Error::new(io::ErrorKind::InvalidData, "Hostname contains null in the middle"))?;

    *hostname = cstr.to_string_lossy().into_owned();
    Ok(())
}

fn try_proc_hostname(hostname: &mut String) -> io::Result<()> {
    *hostname = std::fs::read_to_string("/proc/sys/kernel/hostname")?
        .trim_end_matches('\n')
        .to_string();
    Ok(())
}

fn try_hostname_command(hostname: &mut String) -> io::Result<()> {
    let output = std::process::Command::new("hostname")
        .output()?;

    if !output.status.success() {
        return Err(io::Error::new(
            io::ErrorKind::Other,
            "Hostname command failed"
        ))
    }

    *hostname = String::from_utf8(output.stdout)
        .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))?
        .trim_end_matches('\n')
        .to_string();
    Ok(())
}

fn try_env_hostname(hostname :&mut String) -> bool {
    ["HOSTNAME", "DHCP_HOSTNAME"]
        .iter()
        .find_map(|var| std::env::var(var).ok())
        .map(|val| {
            *hostname = val;
            true
        })
        .unwrap_or(false)
}

pub fn get_hostname(hostname: &mut String) -> io::Result<()> {
    if try_posix_hostname(hostname).is_ok() {
        return Ok(());
    }

    if try_proc_hostname(hostname).is_ok() {
        return Ok(());
    }

    if try_hostname_command(hostname).is_ok() {
        return Ok(());
    }

    if try_env_hostname(hostname) {
        return Ok(());
    }

    Err(io::Error::new(
        io::ErrorKind::NotFound,
        "Error while getting hostname"
    ))
}