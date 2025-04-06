use std::fs;
use std::io;

pub fn get_os_info() -> io::Result<String> {

    let content = fs::read_to_string("/etc/os-release")?;

    let mut os_name = String::new();
    let mut os_version = String::new();

    for line in content.lines() {
        if line.starts_with("NAME=") && os_name.is_empty() {
            os_name = line["NAME=".len()..].trim_matches('"').to_string();
        } else if line.starts_with("VERSION=") && os_version.is_empty() {
            os_version = line["VERSION=".len()..].trim_matches('"').to_string();
        }
    }

    if os_name.is_empty() {
        os_name = "Unknown OS".to_string();
    }

    if os_version.is_empty() {
        os_version = "Unknown Version".to_string();
    }

    Ok(format!("{} {}", os_name, os_version))

}