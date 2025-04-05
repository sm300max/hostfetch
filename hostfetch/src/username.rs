use libc::{getuid, getpwuid_r, passwd, c_char};
use std::pptr;
use std::io;
use std::ffi::CStr;

fn get_username() -> io::Result<String> {
    unsafe {
        let uid = getuid();
        let mut buf = [0u8; 1024];
        let mut result = ptr::null_mut::<passwd>();
        let mut passwd_entry: passwd = std::mem::zeroed();

        loop {
            let r = getpwuid_r(
                uid,
                &mut passwd_endry,
                buf.as_mut_ptr() as *mut c_char,
                buf.len(),
                &mut result,
            );

            if r != 0 {
                return Err(io::Error::last_os_error());
            }

            if result.is_null() {
                return Err(io::Error::new(
                    io::ErrorKind::NotFound,
                    "Username not found"
                ));
            }

            if (*result).pw_name.is_null() {
                return Err(io::Error::new(
                    io::ErrorKind::InvalidData,
                    "Username is null",
                ))
            }
            
            let c_str = CStr::from_ptr((*result).pw_name);
            match c_str.to_str() {
                Ok(s) => return Ok(s.to_owned()),
                Err(_) => {
                    return Ok(c_str.to_string_lossy().into_owned());
                }
            }
        }
    }
}