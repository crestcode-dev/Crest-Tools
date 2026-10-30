use anyhow::Result;
use colored::*;
use serde::Deserialize;

use crate::backend::ApkBackend;
use crate::ui::{confirm_prompt, human_bytes, print_error, print_info, print_success};

#[derive(Debug)]
struct PlannedPackage {
    action: String, // Installing, Upgrading
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

pub fn run(backend: &ApkBackend, packages: &[String], simulate: bool, auto_yes: bool) -> Result<()> {
    if packages.is_empty() {
        print_error("No packages specified to install.");
        return Ok(());
    }

    // Step 1: Run dry-run with -u (upgrade allowed) and verbose output
    let mut sim_args = vec!["add", "-u", "-v", "--simulate"];
    for p in packages {
        sim_args.push(p.as_str());
    }

    let sim_output = backend.execute_output(&sim_args)?;

    if !sim_output.status.success() {
        let err = String::from_utf8_lossy(&sim_output.stderr);
        let out = String::from_utf8_lossy(&sim_output.stdout);
        print_error("Unable to resolve package dependencies:");
        if !err.is_empty() {
            eprintln!("{}", err.trim().red());
        } else {
            eprintln!("{}", out.trim().red());
        }
        return Ok(());
    }

    // Step 2: Parse planned operations
    let stdout = String::from_utf8_lossy(&sim_output.stdout);
    let mut planned: Vec<PlannedPackage> = Vec::new();

    for line in stdout.lines() {
        let trimmed = line.trim();
        let parts: Vec<&str> = trimmed.split_whitespace().collect();
        if let Some(idx) = parts.iter().position(|&p| p == "Installing" || p == "Upgrading") {
            let action = parts[idx];
            if parts.len() > idx + 2 {
                let name = parts[idx + 1].to_string();
                if action == "Installing" {
                    let ver = parts[idx + 2].trim_matches(|c| c == '(' || c == ')').to_string();
                    planned.push(PlannedPackage {
                        action: action.to_string(),
                        name,
                        version: ver,
                    });
                } else if action == "Upgrading" {
                    let old_ver = parts[idx + 2].trim_matches(|c| c == '(' || c == ')');
                    let new_ver = parts.last().unwrap_or(&"").trim_matches(|c| c == '(' || c == ')');
                    let display_ver = format!("{} -> {}", old_ver, new_ver);
                    planned.push(PlannedPackage {
                        action: action.to_string(),
                        name,
                        version: display_ver,
                    });
                }
            }
        }
    }

    if planned.is_empty() {
        print_info("INFO", "All requested packages are already installed and up to date.");
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

    // Step 4: Display pretty interactive preview
    println!();
    println!("{}", "The following packages will be INSTALLED / UPGRADED:".bold());
    println!();

    let max_len = planned.iter().map(|p| p.name.len()).max().unwrap_or(12);

    for pkg in &planned {
        let symbol = if pkg.action == "Upgrading" {
            "^".cyan().bold()
        } else {
            "+".green().bold()
        };

        println!(
            "  {} {:<width$}  {}",
            symbol,
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

    // If dry run requested, stop here
    if simulate {
        println!();
        print_info("DRY-RUN", "Simulation complete. No changes were made.");
        return Ok(());
    }

    // Step 5: User confirmation prompt
    if !auto_yes {
        let confirmed = confirm_prompt("Do you want to continue?")?;
        if !confirmed {
            println!("{}", "Operation aborted.".yellow());
            return Ok(());
        }
    }

    println!();
    print_info("INSTALL", "Applying changes...");

    // Step 6: Execute actual installation with -u
    let mut real_args = vec!["add", "-u"];
    for p in packages {
        real_args.push(p.as_str());
    }

    let status = backend.execute_interactive(&real_args)?;
    if status.success() {
        println!();
        print_success("Package(s) installed/upgraded successfully!");
    } else {
        print_error(&format!("Installation exited with status: {}", status));
    }

    Ok(())
}
