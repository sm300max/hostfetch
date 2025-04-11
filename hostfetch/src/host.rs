use std::fs;
use std::path::Path;
use std::process::Command;

pub fn get_device_info() -> String {
    let (vendor, product) = get_device_info_components();
    
    match (vendor, product) {
        (Some(v), Some(p)) => {
            if p.starts_with(&v) {
                p
            } else {
                format!("{} {}", v, p)
            }
        }
        (Some(v), None) => v,
        (None, Some(p)) => p,
        _ => "Unknown Device".to_string()
    }
}

fn get_device_info_components() -> (Option<String>, Option<String>) {
    let sources: [fn() -> (Option<String>, Option<String>); 6] = [
        || {
            let vendor = getprop("ro.product.manufacturer");
            let product = getprop("ro.product.model");
            (vendor, product)
        },
        
        || {
            let vendor = read_dmi("sys_vendor")
                .or_else(|| read_dmi("board_vendor"));
            
            let product = read_dmi("product_name")
                .or_else(|| read_dmi("product_version"));
            
            (vendor, product)
        },
        
        || {
            if let Some((v, p)) = read_device_tree() {
                (v, Some(p))
            } else {
                (None, None)
            }
        },
        
        || {
            let mut vendor = None;
            let mut product = None;
            
            for path in &["/system/build.prop", "/vendor/build.prop"] {
                if let Ok(content) = fs::read_to_string(path) {
                    for line in content.lines() {
                        if line.starts_with("ro.product.manufacturer=") && vendor.is_none() {
                            vendor = line.split('=').nth(1).map(cleanup_string);
                        }
                        if line.starts_with("ro.product.model=") && product.is_none() {
                            product = line.split('=').nth(1).map(cleanup_string);
                        }
                    }
                }
            }
            (vendor, product)
        },
        
        || {
            if let Ok(cpuinfo) = fs::read_to_string("/proc/cpuinfo") {
                let hardware = cpuinfo.lines()
                    .find(|l| l.starts_with("Hardware"))
                    .and_then(|l| l.split(':').nth(1))
                    .map(cleanup_string);
                
                let model = cpuinfo.lines()
                    .find(|l| l.starts_with("model name"))
                    .and_then(|l| l.split(':').nth(1))
                    .map(cleanup_string);
                
                (hardware, model)
            } else {
                (None, None)
            }
        },
        
        || {
            let board_name = read_sys_file("/sys/class/dmi/id/board_name");
            let product_name = read_sys_file("/sys/devices/virtual/dmi/id/product_name");
            (board_name, product_name)
        }
    ];

    for source in &sources {
        let (vendor, product) = source();
        if vendor.is_some() || product.is_some() {
            return (vendor, product);
        }
    }
    
    (None, None)
}

fn read_device_tree() -> Option<(Option<String>, String)> {
    let dt_model = read_sys_file("/proc/device-tree/model")
        .or_else(|| read_sys_file("/sys/firmware/devicetree/base/model"))?;

    let cleaned = dt_model
        .replace('\0', "")
        .trim()
        .to_string();

    if let Some((vendor, model)) = cleaned.split_once(' ') {
        let vendor_clean = vendor
            .trim_start_matches("Raspberry Pi")
            .trim()
            .to_string();
        
        return Some((
            Some(vendor_clean),
            model.trim().to_string()
        ));
    }
    
    Some((None, cleaned))
}

fn getprop(prop: &str) -> Option<String> {
    Command::new("getprop")
        .arg(prop)
        .output()
        .ok()
        .and_then(|o| String::from_utf8(o.stdout).ok())
        .map(|s| s.trim().to_string())
        .filter(|s| !s.is_empty())
        .map(|s| s.replace("_", " "))  // Нормализуем пробелы
}

fn read_dmi(field: &str) -> Option<String> {
    let path = format!("/sys/class/dmi/id/{}", field);
    read_sys_file(path)
        .or_else(|| read_sys_file(format!("/sys/devices/virtual/dmi/id/{}", field)))
}

fn read_sys_file<P: AsRef<Path>>(path: P) -> Option<String> {
    fs::read_to_string(path)
        .ok()
        .map(|s| cleanup_string(&s))
}

fn cleanup_string(s: &str) -> String {
    s.trim()
        .replace("\\n", "")
        .replace('\0', "")
        .trim()
        .to_string()
}
