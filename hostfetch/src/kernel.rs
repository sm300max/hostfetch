use std::process::Command;
use std::string::FromUtf8Error;

#[derive(Debug)]
 pub struct UnameResults {
    pub uname: String,
    pub kernel_version: String,
}

pub fn get_uname_data() -> Result<UnameResults, String> {
    let uname = Command::new("uname")
        .arg("-s")
        .output()
        .map_err(|e| format!("uname error: {}", e))?;

    let kernel_version = Command::new("uname")
        .arg("-r")
        .output()
        .map_err(|e| format!("uname -r error: {}", e))?;

    if !uname.status.success() || !kernel_version.status.success() {
        return Err("Command execution failed".into());
    }

    let parse_output = |output: Vec<u8>| -> Result<String, FromUtf8Error> {
        String::from_utf8(output).map(|s| s.trim().to_string())
    };

    Ok(UnameResults {
        uname: parse_output(uname.stdout)
            .map_err(|e| format!("kernel name parsing error: {}", e))?,
        kernel_version: parse_output(kernel_version.stdout)
            .map_err(|e| format!("kernel version parsing error: {}", e))?,
    })
}