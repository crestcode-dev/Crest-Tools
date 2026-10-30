use anyhow::Result;
use colored::*;
use comfy_table::Cell;

use crate::backend::ApkBackend;
use crate::ui::{create_table, print_error, print_info};

pub fn run(backend: &ApkBackend, query: &str) -> Result<()> {
    print_info("SEARCH", &format!("Searching for '{}'...", query.bright_yellow()));

    let output = backend.execute_output(&["search", "-v", query])?;

    if !output.status.success() {
        let err = String::from_utf8_lossy(&output.stderr);
        print_error(&format!("Search failed: {}", err));
        return Ok(());
    }

    let stdout = String::from_utf8_lossy(&output.stdout);
    let lines: Vec<&str> = stdout.lines().filter(|l| !l.trim().is_empty()).collect();

    if lines.is_empty() {
        println!("{}", "No packages found matching your query.".dimmed());
        return Ok(());
    }

    let mut table = create_table(vec!["Package", "Version", "Description"]);

    for line in lines {
        // Alpine format: name-version - description
        // Example: htop-3.5.3-r0 - Interactive process viewer
        if let Some((pkg_ver, desc)) = line.split_once(" - ") {
            let (pkg_name, version) = split_pkg_version(pkg_ver);
            table.add_row(vec![
                Cell::new(pkg_name).fg(comfy_table::Color::Green),
                Cell::new(version).fg(comfy_table::Color::Yellow),
                Cell::new(desc),
            ]);
        } else {
            let (pkg_name, version) = split_pkg_version(line.trim());
            table.add_row(vec![
                Cell::new(pkg_name).fg(comfy_table::Color::Green),
                Cell::new(version).fg(comfy_table::Color::Yellow),
                Cell::new("-"),
            ]);
        }
    }

    println!("{table}");
    Ok(())
}

fn split_pkg_version(raw: &str) -> (&str, &str) {
    // Look for hyphen followed by a digit
    // e.g. "htop-3.5.3-r0" -> ("htop", "3.5.3-r0")
    // or "ca-certificates-bundle-20260611-r0" -> ("ca-certificates-bundle", "20260611-r0")
    let bytes = raw.as_bytes();
    for i in (1..bytes.len()).rev() {
        if bytes[i - 1] == b'-' && bytes[i].is_ascii_digit() {
            return (&raw[..i - 1], &raw[i..]);
        }
    }
    (raw, "latest")
}
