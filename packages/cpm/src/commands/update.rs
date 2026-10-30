use anyhow::Result;
use comfy_table::Cell;
use indicatif::{ProgressBar, ProgressStyle};
use std::time::Duration;

use crate::backend::ApkBackend;
use crate::ui::{create_table, print_error, print_success};

pub fn run(backend: &ApkBackend) -> Result<()> {
    let pb = ProgressBar::new_spinner();
    pb.set_style(
        ProgressStyle::default_spinner()
            .tick_chars("⠋⠙⠹⠸⠼⠴⠦⠧⠇⠏ ")
            .template("{spinner:.green} {msg}")
            .expect("Invalid spinner template"),
    );
    pb.enable_steady_tick(Duration::from_millis(80));
    pb.set_message("Synchronizing package repositories...".to_string());

    let output = backend.execute_output(&["update"])?;
    pb.finish_and_clear();

    if !output.status.success() {
        let err = String::from_utf8_lossy(&output.stderr);
        print_error(&format!("Update failed: {}", err));
        return Ok(());
    }

    let stdout = String::from_utf8_lossy(&output.stdout);
    let mut table = create_table(vec!["Status", "Repository / URL"]);

    for line in stdout.lines() {
        let line = line.trim();
        if line.is_empty() {
            continue;
        }

        if line.starts_with("OK:") {
            print_success(line);
        } else if let Some((ver, url)) = line.split_once(' ') {
            table.add_row(vec![
                Cell::new(ver).fg(comfy_table::Color::Green),
                Cell::new(url.trim_matches(|c| c == '[' || c == ']')),
            ]);
        } else {
            table.add_row(vec![
                Cell::new("SYNC").fg(comfy_table::Color::Cyan),
                Cell::new(line),
            ]);
        }
    }

    if table.row_iter().len() > 0 {
        println!("{table}");
    }
    print_success("Package database is up to date!");
    Ok(())
}
