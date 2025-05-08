use std::env;
use std::process::Command;

pub fn detect_terminal() -> String {
    let detectors = [
        ("KONSOLE_VERSION", "Konsole"),
        ("VTE_VERSION", "GNOME Terminal"),
        ("ALACRITTY_LOG", "Alacritty"),
        ("KITTY_PID", "Kitty"),
        ("TERMUX_VERSION", "Termux"),
        ("WEZTERM_EXECUTABLE", "WezTerm"),
        ("TERM_PROGRAM", ""),
        ("TERM", ""),  
    ];

    for (var, name) in &detectors {
        if let Ok(_) = env::var(var) {
            return if name.is_empty() {
                env::var(var).unwrap_or_else(|_| "unknown".into())
            } else {
                name.to_string()
            };
        }
    }

    if let Ok(ppid) = get_parent_pid() {
        let output = Command::new("ps")
            .args(["-o", "comm=", "-p", &ppid.to_string()])
            .output()
            .ok()
            .and_then(|o| String::from_utf8(o.stdout).ok());

        if let Some(cmd) = output {
            let proc_name = cmd.trim().to_lowercase();
            let known_terminals = [
                ("gnome-terminal", "GNOME Terminal"),
                ("xterm", "XTerm"),
                ("tilix", "Tilix"),
                ("terminator", "Terminator"),
                ("xfce4-terminal", "XFCE Terminal"),
                ("urxvt", "URxvt"),
                ("st", "ST"),
                ("kitty", "Kitty"),
                ("alacritty", "Alacritty"),
                ("termux", "Termux"),
                ("wezterm", "WezTerm"),
                ("lxterminal", "LXTerminal"),
            ];

            for (pattern, name) in &known_terminals {
                if proc_name.contains(pattern) {
                    return name.to_string();
                }
            }
        }
    }

    env::var("TERM").unwrap_or_else(|_| "unknown".into())
}

fn get_parent_pid() -> Result<u32, ()> {
    let stat = std::fs::read_to_string("/proc/self/stat").map_err(|_| ())?;
    stat.split(' ')
        .nth(3)
        .and_then(|s| s.parse().ok())
        .ok_or(())
}