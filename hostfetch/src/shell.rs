use std::env;

pub fn get_shell_name() -> String {
    env::var("SHELL")
        .unwrap_or_else(|_| "unknown".into())
        .split('/')
        .last()
        .unwrap_or("unknown")
        .to_string()
}