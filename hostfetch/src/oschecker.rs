use std::fs;
use std::io;
use std::path::Path;
use std::process::Command;

pub fn get_os_info() -> io::Result<String> {
    if let Some(android_info) = detect_android() {
        return Ok(android_info);
    }

    try_read_standard_release()
        .or_else(|_| try_read_legacy_release())
        .or_else(|_| try_read_debian_version())
        .or(Ok("Unknown Linux".to_string()))
}

fn detect_android() -> Option<String> {
    if !is_android() {
        return None;
    }

    let mut info = String::from("Android");

    let version = get_property("ro.build.version.release")
        .or_else(|| get_property("ro.system.build.version.release"))
        .or_else(|| get_property("ro.build.version.sdk"))
        .unwrap_or_else(|| "Unknown Version".to_string());

    info.push_str(&format!(" {}", version));

    Some(info)
}

fn is_android() -> bool {
    Path::new("/system/build.prop").exists()
        || Path::new("/system/bin/dalvikvm").exists()
        || cfg!(target_os = "android")
}

fn get_property(key: &str) -> Option<String> {
    let prop = Command::new("getprop")
        .arg(key)
        .output()
        .ok()
        .and_then(|o| String::from_utf8(o.stdout).ok())
        .map(|s| s.trim().to_string())
        .filter(|s| !s.is_empty());

    if prop.is_some() {
        return prop;
    }

    let paths = [
        "/system/build.prop",
        "/vendor/build.prop",
        "/product/build.prop",
        "/system/etc/prop.default",
        "/default.prop",
    ];

    for path in &paths {
        if let Ok(content) = fs::read_to_string(path) {
            if let Some(value) = parse_prop(&content, key) {
                return Some(value);
            }
        }
    }
    None
}


fn try_read_standard_release() -> io::Result<String> {
    let paths = [
        "/etc/os-release",
        "/usr/lib/os-release",
    ];

    for path in &paths {
        if let Ok(content) = fs::read_to_string(path) {
            if let Some(info) = parse_standard_release(&content) {
                return Ok(info);
            }
        }
    }
    Err(io::Error::new(io::ErrorKind::NotFound, "No standard release file found"))
}

fn parse_standard_release(content: &str) -> Option<String> {
    let mut pretty_name = None;
    let mut name = None;
    let mut version_id = None;

    for line in content.lines() {
        let parts: Vec<&str> = line.splitn(2, '=').collect();
        if parts.len() != 2 {
            continue;
        }

        let key = parts[0];
        let value = parts[1].trim_matches('"');

        match key {
            "PRETTY_NAME" => pretty_name = Some(value.to_string()),
            "NAME" => name = Some(value.to_string()),
            "VERSION_ID" => version_id = Some(value.to_string()),
            _ => {}
        }
    }

    pretty_name
        .or_else(|| {
            name.map(|n| 
                version_id.map(|v| format!("{} {}", n, v))
                    .unwrap_or(n)
            )
        })
        .map(|s| s.replace(r#"\n"#, " ").replace(r#"\""#, ""))
}

fn try_read_legacy_release() -> io::Result<String> {
    let paths = [
        "/etc/lsb-release",
        "/etc/redhat-release",
        "/etc/gentoo-release",
        "/etc/altlinux-release",
    ];

    for path in &paths {
        if let Ok(content) = fs::read_to_string(path) {
            if let Some(info) = parse_legacy_release(&content) {
                return Ok(info);
            }
        }
    }
    Err(io::Error::new(io::ErrorKind::NotFound, "No legacy release file found"))
}

fn parse_legacy_release(content: &str) -> Option<String> {
    content.lines()
        .find(|line| line.starts_with("DISTRIB_DESCRIPTION"))
        .and_then(|line| line.splitn(2, '=').nth(1))
        .map(|s| s.trim_matches('"').to_string())
        .or_else(|| content.lines().next().map(|s| s.to_string()))
}

fn try_read_debian_version() -> io::Result<String> {
    fs::read_to_string("/etc/debian_version")
        .map(|ver| format!("Debian {}", ver.trim()))
}

fn parse_prop(content: &str, key: &str) -> Option<String> {
    content.lines()
        .find(|line| line.starts_with(key))
        .and_then(|line| line.splitn(2, '=').nth(1))
        .map(|value| value.trim().trim_matches('"').to_string())
        .filter(|s| !s.is_empty())
}
