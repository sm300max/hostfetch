use libc::{getuid, getpwuid_r, passwd, c_char};
use std::{
    ffi::CStr,
    io,
    mem::MaybeUninit,
    ptr,
};

pub fn get_username() -> io::Result<String> {
    unsafe {
        let uid = getuid();
        let mut buffer = [0u8; 1024];
        let mut passwd_entry = MaybeUninit::<passwd>::uninit();
        let mut result_ptr = ptr::null_mut::<passwd>();

        let status = getpwuid_r(
            uid,
            passwd_entry.as_mut_ptr(),
            buffer.as_mut_ptr() as *mut c_char,
            buffer.len(),
            &mut result_ptr,
        );

        if status != 0 {
            return Err(io::Error::last_os_error());
        }

        if result_ptr.is_null() {
            return Err(io::Error::new(
                io::ErrorKind::NotFound,
                "Username not found"
            ));
        }

        let passwd_entry = passwd_entry.assume_init();
        let cstr = CStr::from_ptr(passwd_entry.pw_name);

        Ok(cstr.to_string_lossy().into_owned())
    }
}