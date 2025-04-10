use std::fs;
use std::path::Path;

pub fn get_device_name() -> String {
    get_dmi_product_name()
        .or_else(|| get_device_tree_model())
        .or_else(|| get_product_name_alternative())
        .unwrap_or_else(|| "Unknown Device".to_string())
}

fn get_dmi_product_name() -> Option<String> {
    read_sys_file("/sys/class/dmi/id/product_name")
}

fn get_device_tree_model() -> Option<String> {
    read_sys_file("/proc/device_tree/model")
        .map(|s| s.trim_matches('\0').trim().to_string())
}

fn get_product_name_alternative() -> Option<String> {
    read_sys_file("/sys/firmware/devicetree/base/model")
        .or_else(|| read_sys_file("/sys/devices/virtual/dmi/id/product_name"))
}

fn read_sys_file<P: AsRef<Path>>(path :P) -> Option<String> {
    fs::read_to_string(path)
        .ok()
        .map(|s| s.trim().to_string())
        .filter(|s| !s.is_empty())
}