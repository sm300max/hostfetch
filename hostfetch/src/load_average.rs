use std::process::Command;
use std::fs;

pub fn get_loadavg() -> String {
    let output = Command::new("sh")
        .arg("-c")
        .arg(r#"uptime 2>/dev/null | awk -F 'load average: ' '{print $2}' | tr -d ','"#)
        .output();

    if let Ok(output) = output {
        if let Ok(mut output_str) = String::from_utf8(output.stdout) {
            output_str = output_str.trim().to_string();
            let parts: Vec<&str> = output_str.split_whitespace().take(3).collect();
            
            if parts.len() == 3 {
                return format_load(&parts);
            }
        }
    }

    if let Ok(content) = fs::read_to_string("/proc/loadavg") {
        let parts: Vec<&str> = content.split_whitespace().take(3).collect();
        if parts.len() == 3 {
            return format_load(&parts);
        }
    }

    "N/A".to_string()
}

fn format_load(parts: &[&str]) -> String {
    let parse = |s: &str| s.trim().parse().unwrap_or_else(|_| {
        s.chars().filter(|c| c.is_ascii_digit() || *c == '.').collect::<String>().parse().unwrap_or(0.0)
    });

    let one = parse(parts[0]);
    let five = parse(parts[1]);
    let fifteen = parse(parts[2]);

    format!("{:.2}, {:.2}, {:.2}", one, five, fifteen)
}
