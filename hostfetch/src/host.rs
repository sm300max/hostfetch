use std::fs;
use std::path::Path;
use std::process::Command;

pub fn get_device_info() -> String {
    let mut result = String::new();

    let detectors: [&dyn Fn() -> Option<String>; 7] = [
        &detect_android_marketing_name,
        &detect_dmi_product_name,
        &detect_device_tree_model,
        &detect_android_model,
        &detect_board_name,
        &detect_cpu_hardware,
        &detect_product_version,
    ];

    for detector in &detectors {
        if let Some(name) = detector() {
            result = normalize_device_name(&name);
            break;
        }
    }

    if result.is_empty() {
        result = "Unknown Device".to_string();
    }

    result
}

fn normalize_device_name(name: &str) -> String {
    name.trim()
        .replace('\0', "")
        .replace("Not Specified", "")
        .replace("Not Applicable", "")
        .replace("Default string", "")
        .replace("System Product Name", "")
        .replace("To be filled by O.E.M.", "")
        .trim()
        .to_string()
}

fn detect_android_marketing_name() -> Option<String> {
    get_property("ro.product.marketname")
        .or_else(|| get_property("ro.product.vendor.marketname"))
        .or_else(|| get_property("ro.product.odm.marketname"))
}

fn detect_dmi_product_name() -> Option<String> {
    read_dmi_field("product_name")
        .or_else(|| read_dmi_field("product_version"))
}

fn detect_device_tree_model() -> Option<String> {
    read_sys_file("/proc/device-tree/model")
        .or_else(|| read_sys_file("/sys/firmware/devicetree/base/model"))
}

fn detect_android_model() -> Option<String> {
    get_property("ro.product.model")
        .or_else(|| get_property("ro.product.vendor.model"))
        .or_else(|| get_property("ro.product.odm.model"))
}

fn detect_board_name() -> Option<String> {
    read_dmi_field("board_name")
}

fn detect_cpu_hardware() -> Option<String> {
    read_sys_file("/proc/cpuinfo")
        .and_then(|content| {
            content.lines()
                .find(|l| l.starts_with("Hardware") || l.starts_with("model name"))
                .and_then(|l| l.split(':').nth(1))
                .map(str::trim)
                .map(|s| s.to_string()) // Исправлено здесь
        })
}

fn detect_product_version() -> Option<String> {
    read_dmi_field("product_version")
}

fn read_dmi_field(field: &str) -> Option<String> {
    let paths = [
        format!("/sys/class/dmi/id/{}", field),
        format!("/sys/devices/virtual/dmi/id/{}", field),
    ];

    paths.iter()
        .find_map(|path| read_sys_file(path))
}

fn read_sys_file<P: AsRef<Path>>(path: P) -> Option<String> {
    fs::read_to_string(path)
        .ok()
        .map(|s| s.trim().to_string())
        .filter(|s| !s.is_empty())
}

fn get_property(prop: &str) -> Option<String> {
    Command::new("getprop")
        .arg(prop)
        .output()
        .ok()
        .and_then(|o| String::from_utf8(o.stdout).ok())
        .map(|s| s.trim().to_string())
        .filter(|s| !s.is_empty())
}
