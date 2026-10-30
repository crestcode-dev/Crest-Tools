use anyhow::Result;
use colored::*;
use serde::Deserialize;

use crate::backend::ApkBackend;
use crate::ui::{confirm_prompt, human_bytes, print_error, print_info, print_success};

#[derive(Debug)]
struct RemovedPackage {
    name: String,
    version: String,
}

#[derive(Deserialize, Debug, Default)]
struct PkgSizeInfo {
    #[allow(dead_code)]
    name: Option<String>,
    #[serde(rename = "installed-size")]
    installed_size: Option<u64>,
}

pub fn run(backend: &ApkBackend, packages: &[String], simulate: bool, auto_yes: bool) -> Result<()> {
    if packages.is_empty() {
        print_error("No packages specified to remove.");
        return Ok(());
    }

    // Step 1: Run dry-run / simulation with verbose output
    let mut sim_args = vec!["del", "-v", "--simulate"];
    for p in packages {
        sim_args.push(p.as_str());
    }

    let sim_output = backend.execute_output(&sim_args)?;

    if !sim_output.status.success() {
        let err = String::from_utf8_lossy(&sim_output.stderr);
        let out = String::from_utf8_lossy(&sim_output.stdout);
        print_error("Unable to resolve removal dependencies:");
        if !err.is_empty() {
            eprintln!("{}", err.trim().red());
        } else {
            eprintln!("{}", out.trim().red());
        }
        return Ok(());
    }

    // Step 2: Parse packages to remove
    let stdout = String::from_utf8_lossy(&sim_output.stdout);
    let mut planned: Vec<RemovedPackage> = Vec::new();

    for line in stdout.lines() {
        let trimmed = line.trim();
        // Look for: (X/Y) Purging name (version)
        if trimmed.starts_with('(') {
            let parts: Vec<&str> = trimmed.split_whitespace().collect();
            if parts.len() >= 4 && parts[1] == "Purging" {
                let name = parts[2].to_string();
                let ver = parts[3].trim_matches(|c| c == '(' || c == ')').to_string();
                planned.push(RemovedPackage {
                    name,
                    version: ver,
                });
            }
        }
    }

    if planned.is_empty() {
        print_info("INFO", "None of the specified packages are currently installed.");
        return Ok(());
    }

    // Step 3: Query sizes
    let pkg_names: Vec<&str> = planned.iter().map(|p| p.name.as_str()).collect();
    let mut info_args = vec!["info", "--format", "json", "--fields", "name,installed-size"];
    for name in &pkg_names {
        info_args.push(name);
    }

    let mut total_freed: u64 = 0;
    if let Ok(info_out) = backend.execute_output(&info_args) {
        if info_out.status.success() {
            if let Ok(sizes) = serde_json::from_slice::<Vec<PkgSizeInfo>>(&info_out.stdout) {
                for s in sizes {
                    if let Some(is) = s.installed_size {
                        total_freed += is;
                    }
                }
            }
        }
    }

    // Step 4: Display pretty preview
    println!();
    println!("{}", "The following packages will be REMOVED:".bold());
    println!();

    let max_len = planned.iter().map(|p| p.name.len()).max().unwrap_or(12);

    for pkg in &planned {
        println!(
            "  {} {:<width$}  {}",
            "-".red().bold(),
            pkg.name.bright_white().bold(),
            pkg.version.yellow(),
            width = max_len + 2
        );
    }

    println!();
    println!(
        "{} {}",
        "Disk space to be freed:".dimmed(),
        human_bytes(total_freed).bright_red().bold()
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
    print_info("REMOVE", "Removing packages...");

    // Step 6: Execute actual removal
    let mut real_args = vec!["del"];
    for p in packages {
        real_args.push(p.as_str());
    }

    let status = backend.execute_interactive(&real_args)?;
    if status.success() {
        println!();
        print_success("Package(s) removed successfully!");
    } else {
        print_error(&format!("Removal exited with status: {}", status));
    }

    Ok(())
}
