use anyhow::{bail, Context, Result};
use colored::*;
use comfy_table::Cell;
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::{Path, PathBuf};

use crate::ui::{create_table, print_info, print_success};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Config {
    #[serde(default = "default_true")]
    pub show_banner: bool,

    #[serde(default = "default_false")]
    pub auto_yes: bool,

    #[serde(default = "default_true")]
    pub color: bool,

    #[serde(default = "default_true")]
    pub show_hints: bool,
}

fn default_true() -> bool {
    true
}

fn default_false() -> bool {
    false
}

impl Default for Config {
    fn default() -> Self {
        Self {
            show_banner: true,
            auto_yes: false,
            color: true,
            show_hints: true,
        }
    }
}

pub fn get_config_path() -> PathBuf {
    if let Ok(custom) = std::env::var("CPM_CONFIG") {
        return PathBuf::from(custom);
    }
    let home = std::env::var("HOME").unwrap_or_else(|_| "/root".to_string());
    Path::new(&home).join(".config").join("cpm").join("config.toml")
}

impl Config {
    pub fn load() -> Self {
        let path = get_config_path();
        if path.exists() {
            if let Ok(content) = fs::read_to_string(&path) {
                if let Ok(cfg) = toml::from_str::<Config>(&content) {
                    return cfg;
                }
            }
        }
        Config::default()
    }

    pub fn save(&self) -> Result<()> {
        let path = get_config_path();
        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent).context("Failed to create config directory")?;
        }
        let serialized = toml::to_string_pretty(self).context("Failed to serialize config")?;
        fs::write(&path, serialized).context("Failed to write config file")?;
        Ok(())
    }

    pub fn set_value(&mut self, key: &str, val_str: &str) -> Result<()> {
        let lower = val_str.trim().to_lowercase();
        let parsed_bool = match lower.as_str() {
            "true" | "1" | "yes" | "on" => true,
            "false" | "0" | "no" | "off" => false,
            _ => bail!(
                "Invalid boolean value '{}'. Please use true/false, yes/no, or 1/0.",
                val_str
            ),
        };

        match key.trim().to_lowercase().as_str() {
            "show_banner" | "banner" => self.show_banner = parsed_bool,
            "auto_yes" | "yes" => self.auto_yes = parsed_bool,
            "color" | "colors" => self.color = parsed_bool,
            "show_hints" | "hints" => self.show_hints = parsed_bool,
            _ => bail!(
                "Unknown configuration key '{}'. Run 'cpm config list' to see available keys.",
                key
            ),
        }

        self.save()?;
        Ok(())
    }

    pub fn get_value(&self, key: &str) -> Result<String> {
        match key.trim().to_lowercase().as_str() {
            "show_banner" | "banner" => Ok(self.show_banner.to_string()),
            "auto_yes" | "yes" => Ok(self.auto_yes.to_string()),
            "color" | "colors" => Ok(self.color.to_string()),
            "show_hints" | "hints" => Ok(self.show_hints.to_string()),
            _ => bail!(
                "Unknown configuration key '{}'. Run 'cpm config list' to see available keys.",
                key
            ),
        }
    }
}

pub fn list_config(cfg: &Config) -> Result<()> {
    print_info("CONFIG", "Current CPM configuration settings:");
    println!();

    let mut table = create_table(vec!["Key", "Value", "Default", "Description"]);

    table.add_row(vec![
        Cell::new("show_banner").fg(comfy_table::Color::Cyan),
        Cell::new(cfg.show_banner).fg(if cfg.show_banner {
            comfy_table::Color::Green
        } else {
            comfy_table::Color::Red
        }),
        Cell::new("true").fg(comfy_table::Color::DarkGrey),
        Cell::new("Display ASCII banner header on command execution"),
    ]);

    table.add_row(vec![
        Cell::new("auto_yes").fg(comfy_table::Color::Cyan),
        Cell::new(cfg.auto_yes).fg(if cfg.auto_yes {
            comfy_table::Color::Green
        } else {
            comfy_table::Color::Red
        }),
        Cell::new("false").fg(comfy_table::Color::DarkGrey),
        Cell::new("Automatically answer Yes to interactive install/remove prompts"),
    ]);

    table.add_row(vec![
        Cell::new("color").fg(comfy_table::Color::Cyan),
        Cell::new(cfg.color).fg(if cfg.color {
            comfy_table::Color::Green
        } else {
            comfy_table::Color::Red
        }),
        Cell::new("true").fg(comfy_table::Color::DarkGrey),
        Cell::new("Enable colored terminal output"),
    ]);

    table.add_row(vec![
        Cell::new("show_hints").fg(comfy_table::Color::Cyan),
        Cell::new(cfg.show_hints).fg(if cfg.show_hints {
            comfy_table::Color::Green
        } else {
            comfy_table::Color::Red
        }),
        Cell::new("true").fg(comfy_table::Color::DarkGrey),
        Cell::new("Show helpful hints and operational suggestions"),
    ]);

    println!("{table}");
    println!(
        "{} {}",
        "Config file:".dimmed(),
        get_config_path().display().to_string().cyan()
    );
    Ok(())
}

pub fn print_path() -> Result<()> {
    let path = get_config_path();
    println!("{}", path.display());
    Ok(())
}

pub fn reset_config() -> Result<()> {
    let default_cfg = Config::default();
    default_cfg.save()?;
    print_success("Configuration reset to default settings.");
    Ok(())
}
