use std::fs;
use std::path::Path;
use std::process::Command;

pub fn get_device_name() -> String {
    let methods = [
        get_android_properties(),
        get_device_tree_model(),
        get_dmi_product_name(),
        get_cpuinfo_hardware(),
        get_android_build_prop(),
        get_product_name_alternative(),
    ];

    methods.into_iter()
        .find_map(|m| m)
        .unwrap_or_else(|| "Unknown Device".to_string())
}

fn get_android_properties() -> Option<String> {
    Command::new("getprop")
        .arg("ro.product.model")
        .output()
        .ok()
        .and_then(|output| String::from_utf8(output.stdout).ok())
        .map(|s| s.trim().to_string())
        .filter(|s| !s.is_empty())
}

fn get_android_build_prop() -> Option<String> {
    let paths = [
        "/system/build.prop",
        "/vendor/build.prop",
        "/product/build.prop",
    ];
    
    paths.iter()
        .find_map(|path| fs::read_to_string(path).ok())
        .and_then(|contents| {
            contents.lines()
                .find(|l| l.starts_with("ro.product.model="))
                .and_then(|l| l.split('=').nth(1))
                .map(|s| s.trim().to_string()) 
        })
        .filter(|s| !s.is_empty())
}

fn get_device_tree_model() -> Option<String> {
    read_sys_file("/proc/device-tree/model")
        .or_else(|| read_sys_file("/sys/firmware/devicetree/base/model"))
        .map(|s| s.replace('\0', "").trim().to_string())
        .filter(|s| !s.is_empty())
}

fn get_dmi_product_name() -> Option<String> {
    read_sys_file("/sys/class/dmi/id/product_name")
}

fn get_cpuinfo_hardware() -> Option<String> {
    fs::read_to_string("/proc/cpuinfo")
        .ok()
        .and_then(|contents| {
            contents.lines()
                .find(|l| l.starts_with("Hardware"))
                .and_then(|l| l.split(':').nth(1))
                .map(|s| s.trim().to_string()) 
        })
        .filter(|s| !s.is_empty())
}

fn get_product_name_alternative() -> Option<String> {
    read_sys_file("/sys/firmware/devicetree/base/model")
        .or_else(|| read_sys_file("/sys/class/dmi/id/board_vendor"))
        .or_else(|| read_sys_file("/sys/class/dmi/id/board_name"))
}

fn read_sys_file<P: AsRef<Path>>(path: P) -> Option<String> {
    fs::read_to_string(path)
        .ok()
        .map(|s| s.trim().to_string())
        .filter(|s| !s.is_empty())
}

  
