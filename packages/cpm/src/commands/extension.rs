use anyhow::{Context, Result};
use colored::*;
use comfy_table::Cell;
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::{Path, PathBuf};

use crate::ui::{create_table, print_error, print_info, print_success};

#[derive(Debug, Serialize, Deserialize)]
pub struct ExtensionManifest {
    pub id: String,
    pub name: String,
    pub version: String,
    pub author: Option<String>,
    pub description: Option<String>,
}

fn get_extensions_dir() -> PathBuf {
    let home = std::env::var("HOME").unwrap_or_else(|_| "/root".to_string());
    Path::new(&home).join(".crestcode").join("extensions")
}

pub fn list() -> Result<()> {
    print_info("CREST-EXT", "Listing installed CrestEditor extensions...");
    let ext_dir = get_extensions_dir();

    if !ext_dir.exists() {
        println!("{}", "No extensions installed yet.".dimmed());
        return Ok(());
    }

    let mut table = create_table(vec!["Extension ID", "Name", "Version", "Description"]);
    let mut count = 0;

    for entry in fs::read_dir(&ext_dir)? {
        let entry = entry?;
        let path = entry.path();
        if path.is_dir() {
            let manifest_path = path.join("extension.json");
            if manifest_path.exists() {
                if let Ok(content) = fs::read_to_string(&manifest_path) {
                    if let Ok(manifest) = serde_json::from_str::<ExtensionManifest>(&content) {
                        count += 1;
                        table.add_row(vec![
                            Cell::new(manifest.id).fg(comfy_table::Color::Magenta),
                            Cell::new(manifest.name).fg(comfy_table::Color::Green),
                            Cell::new(manifest.version).fg(comfy_table::Color::Yellow),
                            Cell::new(manifest.description.unwrap_or_else(|| "-".to_string())),
                        ]);
                    }
                }
            }
        }
    }

    if count == 0 {
        println!("{}", "No valid CrestEditor extensions found in ~/.crestcode/extensions/".dimmed());
    } else {
        println!("{table}");
        println!(
            "{} {}",
            "Total CrestEditor extensions:".dimmed(),
            count.to_string().bright_magenta().bold()
        );
    }

    Ok(())
}

pub fn new_extension(id: &str, name: Option<String>) -> Result<()> {
    let ext_dir = get_extensions_dir().join(id);
    if ext_dir.exists() {
        print_error(&format!("Extension '{}' already exists at {:?}", id, ext_dir));
        return Ok(());
    }

    fs::create_dir_all(&ext_dir).context("Failed to create extension directory")?;

    let ext_name = name.unwrap_or_else(|| id.to_string());
    let manifest = ExtensionManifest {
        id: id.to_string(),
        name: ext_name,
        version: "0.1.0".to_string(),
        author: Some("CrestEditor Developer".to_string()),
        description: Some(format!("CrestEditor extension for {}", id)),
    };

    let manifest_json = serde_json::to_string_pretty(&manifest)?;
    fs::write(ext_dir.join("extension.json"), manifest_json)?;

    // Create an entry point file
    fs::write(
        ext_dir.join("index.js"),
        "// CrestEditor Extension Entry Point\nexport function activate(context) {\n    console.log('Extension activated!');\n}\n",
    )?;

    print_success(&format!(
        "Created new CrestEditor extension '{}' at {}",
        id.bright_magenta(),
        ext_dir.display().to_string().cyan()
    ));

    Ok(())
}

pub fn remove(id: &str) -> Result<()> {
    let ext_dir = get_extensions_dir().join(id);
    if !ext_dir.exists() {
        print_error(&format!("Extension '{}' not found.", id));
        return Ok(());
    }

    fs::remove_dir_all(&ext_dir)?;
    print_success(&format!("Extension '{}' removed successfully.", id.bright_magenta()));
    Ok(())
}
