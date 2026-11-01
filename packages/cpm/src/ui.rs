use anyhow::Result;
use colored::*;
use comfy_table::{presets::UTF8_FULL, Cell, Color as TColor, ContentArrangement, Table};
use std::io::{self, Write};

pub fn print_banner() {
    println!(
        "{}",
        "┌──────────────────────────────────────────────┐"
            .cyan()
            .bold()
    );
    println!(
        "{}   {}    {}",
        "│".cyan().bold(),
        "CPM - CrestEditor Package Manager".bright_green().bold(),
        "│".cyan().bold()
    );
    println!(
        "{}",
        "└──────────────────────────────────────────────┘"
            .cyan()
            .bold()
    );
}

pub fn create_table(headers: Vec<&str>) -> Table {
    let mut table = Table::new();
    table
        .load_style(UTF8_FULL.with_rounded_corners())
        .set_content_arrangement(ContentArrangement::Dynamic);

    let header_cells: Vec<Cell> = headers
        .into_iter()
        .map(|h| Cell::new(h).fg(TColor::Cyan).add_attribute(comfy_table::Attribute::Bold))
        .collect();

    table.set_header(header_cells);
    table
}

pub fn print_info(tag: &str, msg: &str) {
    println!("{} {}", format!("[{}]", tag).cyan().bold(), msg);
}

pub fn print_success(msg: &str) {
    println!("{} {}", "[✓]".green().bold(), msg.bright_green());
}

pub fn print_error(msg: &str) {
    eprintln!("{} {}", "[✗]".red().bold(), msg.bright_red());
}

#[allow(dead_code)]
pub fn print_warning(msg: &str) {
    println!("{} {}", "[!]".yellow().bold(), msg.bright_yellow());
}

pub fn human_bytes(bytes: u64) -> String {
    const KIB: f64 = 1024.0;
    const MIB: f64 = 1024.0 * 1024.0;
    const GIB: f64 = 1024.0 * 1024.0 * 1024.0;

    let b = bytes as f64;
    if b >= GIB {
        format!("{:.1} GiB", b / GIB)
    } else if b >= MIB {
        format!("{:.1} MiB", b / MIB)
    } else if b >= KIB {
        format!("{:.1} KiB", b / KIB)
    } else {
        format!("{} B", bytes)
    }
}

pub fn confirm_prompt(prompt: &str) -> Result<bool> {
    print!("\n{} [Y/n] ", prompt.bold());
    io::stdout().flush()?;

    let mut input = String::new();
    io::stdin().read_line(&mut input)?;
    let trimmed = input.trim().to_lowercase();
    Ok(trimmed.is_empty() || trimmed == "y" || trimmed == "yes")
}
