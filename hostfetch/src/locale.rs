use std::process::Command;
use std::io::{self, Error, ErrorKind};

pub fn get_locale() -> io::Result<String> {
    let output = Command::new("sh")
        .arg("-c")
        .arg(r#"echo $LANG"#)
        .output()?;

    if !output.status.success() {
        let error_msg = String::from_utf8_lossy(&output.stderr);
        return Err(Error::new(ErrorKind::Other, error_msg.into_owned()));
    }

    let locale = String::from_utf8_lossy(&output.stdout)
        .trim()
        .to_string();

    Ok(locale)
}