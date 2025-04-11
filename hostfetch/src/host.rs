use std::fs;
use std::path::Path;
use std::process::Command;

pub fn get_device_name() -> String {
    let (vendor, model) = get_device_info();
    
    match (vendor, model) {
        (Some(v), Some(m)) if !v.is_empty() && !m.is_empty() => format!("{} {} ({})", v, m, m),
        (_, Some(m)) => format!("{} ({})", m, m),
        (Some(v), _) => format!("{} (Unknown Model)", v),
        _ => "Unknown Device".to_string()
    }
}

fn get_device_info() -> (Option<String>, Option<String>) {
    let methods: [fn() -> (Option<String>, Option<String>); 5] = [
        get_android_info,
        get_dmi_info,
        get_device_tree_info,
        get_cpu_hardware_info,
        get_build_prop_info,
    ];

    methods.iter()
        .find_map(|method| {
            let (v, m) = method();
            if v.is_some() || m.is_some() { Some((v, m)) } else { None }
        })
        .unwrap_or((None, None))
}

fn get_android_info() -> (Option<String>, Option<String>) {
    let vendor = Command::new("getprop")
        .arg("ro.product.manufacturer")
        .output()
        .ok()
        .and_then(|o| String::from_utf8(o.stdout).ok())
        .map(|s| s.trim().to_string())
        .filter(|s| !s.is_empty());

    let model = Command::new("getprop")
        .arg("ro.product.model")
        .output()
        .ok()
        .and_then(|o| String::from_utf8(o.stdout).ok())
        .map(|s| s.trim().to_string())
        .filter(|s| !s.is_empty());

    (vendor, model)
}

fn get_dmi_info() -> (Option<String>, Option<String>) {
    let vendor = read_sys_file("/sys/class/dmi/id/sys_vendor")
        .or_else(|| read_sys_file("/sys/class/dmi/id/board_vendor"));
    
    let model = read_sys_file("/sys/class/dmi/id/product_name")
        .or_else(|| read_sys_file("/sys/class/dmi/id/product_version"));

    (vendor, model)
}

fn get_device_tree_info() -> (Option<String>, Option<String>) {
    let full_info = read_sys_file("/proc/device-tree/model")
        .or_else(|| read_sys_file("/sys/firmware/devicetree/base/model"))
        .map(|s| s.replace('\0', "").trim().to_string())
        .filter(|s| !s.is_empty());

    if let Some(info) = full_info {
        if let Some((vendor, model)) = info.split_once(' ') {
            return (
                Some(vendor.trim().to_string()),
                Some(model.trim().to_string())
            );
        }
        return (None, Some(info));
    }
    (None, None)
}

fn get_cpu_hardware_info() -> (Option<String>, Option<String>) {
    fs::read_to_string("/proc/cpuinfo")
        .ok()
        .and_then(|contents| {
            let hardware = contents.lines()
                .find(|l| l.starts_with("Hardware"))
                .and_then(|l| l.split(':').nth(1))
                .map(|s| s.trim().to_string());

            let model_name = contents.lines()
                .find(|l| l.starts_with("model name"))
                .and_then(|l| l.split(':').nth(1))
                .map(|s| s.trim().to_string());

            Some((hardware, model_name))
        })
        .unwrap_or((None, None))
}

fn get_build_prop_info() -> (Option<String>, Option<String>) {
    let paths = [
        "/system/build.prop",
        "/vendor/build.prop",
        "/product/build.prop",
    ];

    let (mut vendor, mut model) = (None, None);

    for path in &paths {
        if let Ok(contents) = fs::read_to_string(path) {
            for line in contents.lines() {
                if line.starts_with("ro.product.manufacturer=") && vendor.is_none() {
                    vendor = line.splitn(2, '=')
                        .nth(1)
                        .map(|s| s.trim().to_string())
                        .filter(|s| !s.is_empty());
                }
                if line.starts_with("ro.product.model=") && model.is_none() {
                    model = line.splitn(2, '=')
                        .nth(1)
                        .map(|s| s.trim().to_string())
                        .filter(|s| !s.is_empty());
                }
            }
        }
    }

    (vendor, model)
}

fn read_sys_file<P: AsRef<Path>>(path: P) -> Option<String> {
    fs::read_to_string(path)
        .ok()
        .map(|s| s.trim().to_string())
        .filter(|s| !s.is_empty())
}
