// frame.rs
use crossterm::Result;
use std::io::Write;

pub fn draw_rounded_frame<W: Write>(stdout: &mut W, text: &str) -> Result<()> {
    let width = text.len() + 4; 
    let height = 5; 

    writeln!(stdout, "╭{}╮", "─".repeat(width - 2))?;

    writeln!(stdout, "│ {} │", text)?;

    for _ in 0..height - 3 {
        writeln!(stdout, "│{}│", " ".repeat(width - 2))?;
    }

    writeln!(stdout, "╰{}╯", "─".repeat(width - 2))?;

    stdout.flush()?;
    Ok(())
}
