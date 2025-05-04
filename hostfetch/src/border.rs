use lazy_static::lazy_static;
use regex::Regex;

lazy_static! {
    static ref ANSI_ESCAPE: Regex = Regex::new(r"\x1B\[[0-9;]*[a-zA-Z]").unwrap();
}

fn visible_length(s: &str) -> usize {
    ANSI_ESCAPE.replace_all(s, "").chars().count() 
}

pub fn draw_border(lines: &[String]) {
    if lines.is_empty() {
        return;
    }

    let max_length = lines
        .iter()
        .map(|line| visible_length(line))
        .max()
        .unwrap_or(0);

    let top_border = format!("╭─{}─╮", "─".repeat(max_length));
    let bottom_border = format!("╰─{}─╯", "─".repeat(max_length));

    println!("{}", top_border);
    for line in lines {
        let visible_len = visible_length(line);
        let padding = max_length - visible_len;
        println!("│ {}{} │", line, " ".repeat(padding));
    }
    println!("{}", bottom_border);
}