use std::io;
use libc;

pub fn capture_hostname(hostname: &mut String) -> io::Result<()> {
    let mut buf = [0u8; 64];

    let result = unsafe {
        libc::gethostname(buf.as_mut_ptr() as *mut libc::c_char, buf.len())
    };

    if result != 0 {
        return Err(io::Error::last_os_error())
    }

    let end = buf.iter().position(|&b| b == 0).unwrap_or(buf.len());
    *hostname = String::from_utf8_lossy(&buf[..end]).into_owned();

    Ok(())
}