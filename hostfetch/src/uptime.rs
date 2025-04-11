use std::process::Command;
use std::io::{self, Error, ErrorKind};

pub fn get_uptime() -> io::Result<String> {
    let output = Command::new("uptime")
        .arg("-p")
        .output()?;

    if !output.status.success() {
        let error_msg = String::from_utf8_lossy(&output.stderr);
        return Err(Error::new(ErrorKind::Other, error_msg.into_owned()));
    }

    let uptime = String::from_utf8_lossy(&output.stdout)
        .trim()
        .to_string();

    Ok(uptime)
}