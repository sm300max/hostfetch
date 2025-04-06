use std::fs;
use std::io;
use std::path::Path;

pub fn get_os_info() -> io::Result<String> {
    try_read_file("/etc/os-release")
        .or_else(|_| try_read_file("/usr/lib/os-release"))
        .or_else(|_| try_read_file("/etc/lsb-release"))
        .or_else(|_| try_read_legacy_debian())
        .or(Ok("Unknown Linux".to_string()))
}

fn try_read_file(path: &str) -> io::Result<String> {
    if !Path::new(path).exists() {
        return Err(io::Error::new(io::ErrorKind::NotFound, "File not found"));
    }

    fs::read_to_string(path)
        .and_then(|content| {
            parse_os_release(&content)
                .ok_or_else(|| io::Error::new(io::ErrorKind::InvalidData, "Failed to parse OS info"))
        })
}

fn try_read_legacy_debian() -> io::Result<String> {
    fs::read_to_string("/etc/debian_version")
        .map(|ver| format!("Debian {}", ver.trim()))
}

fn parse_os_release(content: &str) -> Option<String> {
    let mut pretty_name = None;
    let mut name = None;
    let mut version_id = None;

    for line in content.lines() {
        if let Some((key, value)) = line.split_once('=') {
            let value = trim_quotes(value).to_string();
            match key {
                "PRETTY_NAME" => pretty_name = Some(value),
                "NAME" if name.is_none() => name = Some(value),
                "VERSION_ID" if version_id.is_none() => version_id = Some(value),
                _ => {}
            }
        }
    }

    if let Some(pname) = pretty_name {
        return Some(pname);
    }

    match (&name, &version_id) {
        (Some(n), Some(v)) => Some(format!("{} {}", n, v)),
        (Some(n), None) => Some(n.clone()),
        (None, Some(v)) => Some(format!("Unknown OS {}", v)),
        (None, None) => None,
    }
}

fn trim_quotes(s: &str) -> &str {
    s.trim_matches('"')
}