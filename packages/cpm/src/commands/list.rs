use anyhow::Result;
use colored::*;
use comfy_table::Cell;

use crate::backend::ApkBackend;
use crate::ui::{create_table, print_error, print_info};

pub fn run(backend: &ApkBackend) -> Result<()> {
    print_info("LIST", "Fetching installed packages...");

    let output = backend.execute_output(&["info", "-v"])?;
    if !output.status.success() {
        let err = String::from_utf8_lossy(&output.stderr);
        print_error(&format!("Failed to retrieve packages: {}", err));
        return Ok(());
    }

    let stdout = String::from_utf8_lossy(&output.stdout);
    let lines: Vec<&str> = stdout.lines().filter(|l| !l.trim().is_empty()).collect();

    let mut table = create_table(vec!["Package Name", "Installed Version"]);

    for line in &lines {
        let (pkg, ver) = split_pkg_version(line.trim());
        table.add_row(vec![
            Cell::new(pkg).fg(comfy_table::Color::Green),
            Cell::new(ver).fg(comfy_table::Color::Yellow),
        ]);
    }

    println!("{table}");
    println!(
        "{} {}",
        "Total installed packages:".dimmed(),
        lines.len().to_string().bright_cyan().bold()
    );
    Ok(())
}

fn split_pkg_version(raw: &str) -> (&str, &str) {
    let bytes = raw.as_bytes();
    for i in (1..bytes.len()).rev() {
        if bytes[i - 1] == b'-' && bytes[i].is_ascii_digit() {
            return (&raw[..i - 1], &raw[i..]);
        }
    }
    (raw, "unknown")
}
