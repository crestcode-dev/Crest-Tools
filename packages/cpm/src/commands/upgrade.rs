use anyhow::Result;
use colored::*;
use serde::Deserialize;

use crate::backend::ApkBackend;
use crate::ui::{confirm_prompt, human_bytes, print_error, print_info, print_success};

#[derive(Debug)]
struct PlannedPackage {
    name: String,
    version: String,
}

#[derive(Deserialize, Debug, Default)]
struct PkgSizeInfo {
    #[allow(dead_code)]
    name: Option<String>,
    #[serde(rename = "installed-size")]
    installed_size: Option<u64>,
    #[serde(rename = "file-size")]
    file_size: Option<u64>,
}

pub fn run(backend: &ApkBackend, simulate: bool, auto_yes: bool) -> Result<()> {
    print_info("UPGRADE", "Checking for system package updates...");

    // Step 1: Run dry-run upgrade simulation
    let sim_output = backend.execute_output(&["upgrade", "-v", "--simulate"])?;

    if !sim_output.status.success() {
        let err = String::from_utf8_lossy(&sim_output.stderr);
        print_error(&format!("Upgrade check failed: {}", err));
        return Ok(());
    }

    // Step 2: Parse packages to upgrade
    let stdout = String::from_utf8_lossy(&sim_output.stdout);
    let mut planned: Vec<PlannedPackage> = Vec::new();

    for line in stdout.lines() {
        let trimmed = line.trim();
        // Look for: Upgrading name (old_ver -> new_ver)
        if trimmed.contains("Upgrading") {
            let parts: Vec<&str> = trimmed.split_whitespace().collect();
            if let Some(idx) = parts.iter().position(|&p| p == "Upgrading") {
                if parts.len() > idx + 2 {
                    let name = parts[idx + 1].to_string();
                    let old_ver = parts[idx + 2].trim_matches(|c| c == '(' || c == ')');
                    let new_ver = parts.last().unwrap_or(&"").trim_matches(|c| c == '(' || c == ')');
                    let display_ver = format!("{} -> {}", old_ver, new_ver);
                    planned.push(PlannedPackage {
                        name,
                        version: display_ver,
                    });
                }
            }
        }
    }

    if planned.is_empty() {
        println!("{}", "All packages are up to date.".bright_green().bold());
        return Ok(());
    }

    // Step 3: Query sizes
    let pkg_names: Vec<&str> = planned.iter().map(|p| p.name.as_str()).collect();
    let mut info_args = vec!["info", "--format", "json", "--fields", "name,file-size,installed-size"];
    for name in &pkg_names {
        info_args.push(name);
    }

    let mut total_download: u64 = 0;
    let mut total_installed: u64 = 0;

    if let Ok(info_out) = backend.execute_output(&info_args) {
        if info_out.status.success() {
            if let Ok(sizes) = serde_json::from_slice::<Vec<PkgSizeInfo>>(&info_out.stdout) {
                for s in sizes {
                    if let Some(fs) = s.file_size {
                        total_download += fs;
                    }
                    if let Some(is) = s.installed_size {
                        total_installed += is;
                    }
                }
            }
        }
    }

    // Step 4: Display pretty preview
    println!();
    println!("{}", "The following packages will be UPGRADED:".bold());
    println!();

    let max_len = planned.iter().map(|p| p.name.len()).max().unwrap_or(12);

    for pkg in &planned {
        println!(
            "  {} {:<width$}  {}",
            "^".cyan().bold(),
            pkg.name.bright_white().bold(),
            pkg.version.yellow(),
            width = max_len + 2
        );
    }

    println!();
    println!(
        "{} {} {} {}",
        "Download:".dimmed(),
        human_bytes(total_download).bright_cyan().bold(),
        "|".dimmed(),
        format!("Disk: +{}", human_bytes(total_installed)).bright_magenta().bold()
    );

    if simulate {
        println!();
        print_info("DRY-RUN", "Simulation complete. No changes were made.");
        return Ok(());
    }

    // Step 5: User confirmation
    if !auto_yes {
        let confirmed = confirm_prompt("Do you want to continue?")?;
        if !confirmed {
            println!("{}", "Operation aborted.".yellow());
            return Ok(());
        }
    }

    println!();
    print_info("UPGRADE", "Applying system updates...");

    let status = backend.execute_interactive(&["upgrade"])?;
    if status.success() {
        println!();
        print_success("System packages upgraded successfully!");
    } else {
        print_error(&format!("Upgrade failed with status: {}", status));
    }

    Ok(())
}
