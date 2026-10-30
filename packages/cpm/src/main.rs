use anyhow::Result;
use clap::{Args, Parser, Subcommand};

mod backend;
mod commands;
mod config;
mod ui;

use backend::ApkBackend;
use config::Config;

#[derive(Parser)]
#[command(
    name = "cpm",
    author = "CrestCode Team",
    version = "0.1.1",
    about = "CrestCode Package Manager - Modern APK frontend & Extension Manager",
    long_about = "A high-performance package manager for Alpine Linux & CrestCode with APT-style syntax, beautiful tables, and extension management."
)]
struct Cli {
    /// Suppress banner output
    #[arg(long, global = true)]
    no_banner: bool,

    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Install packages (apt-style syntax, wraps apk add)
    Install(PackageArgs),

    /// Remove packages (apt-style syntax, wraps apk del)
    Remove(PackageArgs),

    /// Update package indexes (wraps apk update)
    Update,

    /// Upgrade installed packages (wraps apk upgrade)
    Upgrade(UpgradeArgs),

    /// Search for packages with pretty output (wraps apk search)
    Search {
        /// Package name or keyword to search
        query: String,
    },

    /// List installed packages
    List,

    /// Manage CrestCode extensions
    #[command(subcommand)]
    Ext(ExtCommands),

    /// Manage CPM configuration settings
    #[command(subcommand)]
    Config(ConfigCommands),
}

#[derive(Args)]
pub struct PackageArgs {
    /// Packages to install or remove
    #[arg(required = true)]
    pub packages: Vec<String>,

    /// Simulate action without making changes
    #[arg(short, long)]
    pub simulate: bool,

    /// Automatic yes to prompts
    #[arg(short, long)]
    pub yes: bool,
}

#[derive(Args)]
pub struct UpgradeArgs {
    /// Simulate upgrade without making changes
    #[arg(short, long)]
    pub simulate: bool,

    /// Automatic yes to prompts
    #[arg(short, long)]
    pub yes: bool,
}

#[derive(Subcommand)]
pub enum ExtCommands {
    /// List installed CrestCode extensions
    List,

    /// Scaffold a new CrestCode extension template
    New {
        /// Extension unique identifier (e.g. my-theme, python-tools)
        id: String,

        /// Optional friendly name
        #[arg(short, long)]
        name: Option<String>,
    },

    /// Remove an installed CrestCode extension
    Remove {
        /// Extension identifier to remove
        id: String,
    },
}

#[derive(Subcommand)]
pub enum ConfigCommands {
    /// List all configuration settings
    List,

    /// Get value of a configuration setting
    Get {
        /// Configuration key (e.g. show_banner, auto_yes, color, show_hints)
        key: Option<String>,
    },

    /// Set a configuration setting (e.g. cpm config set show_banner false)
    Set {
        /// Configuration key to set (e.g. show_banner, auto_yes, color, show_hints)
        key: String,

        /// Configuration value (e.g. true/false, yes/no, 1/0)
        value: String,
    },

    /// Show the path to the configuration file
    Path,

    /// Reset configuration to default values
    Reset,
}

fn main() -> Result<()> {
    let cli = Cli::parse();
    let mut cfg = Config::load();

    if !cfg.color {
        colored::control::set_override(false);
    }

    if cfg.show_banner && !cli.no_banner {
        ui::print_banner();
    }

    // Config commands do not require an APK backend
    if let Commands::Config(ref config_cmd) = cli.command {
        match config_cmd {
            ConfigCommands::List => {
                config::list_config(&cfg)?;
            }
            ConfigCommands::Get { key } => {
                if let Some(k) = key {
                    match cfg.get_value(k) {
                        Ok(v) => println!("{}: {}", k, v),
                        Err(e) => ui::print_error(&e.to_string()),
                    }
                } else {
                    config::list_config(&cfg)?;
                }
            }
            ConfigCommands::Set { key, value } => {
                match cfg.set_value(key, value) {
                    Ok(_) => ui::print_success(&format!(
                        "Configuration updated: {} = {}",
                        key, value
                    )),
                    Err(e) => ui::print_error(&e.to_string()),
                }
            }
            ConfigCommands::Path => {
                config::print_path()?;
            }
            ConfigCommands::Reset => {
                config::reset_config()?;
            }
        }
        return Ok(());
    }

    let backend = ApkBackend::detect()?;

    match cli.command {
        Commands::Install(args) => {
            let auto_yes = args.yes || cfg.auto_yes;
            commands::install::run(&backend, &args.packages, args.simulate, auto_yes)?;
        }
        Commands::Remove(args) => {
            let auto_yes = args.yes || cfg.auto_yes;
            commands::remove::run(&backend, &args.packages, args.simulate, auto_yes)?;
        }
        Commands::Update => {
            commands::update::run(&backend)?;
        }
        Commands::Upgrade(args) => {
            let auto_yes = args.yes || cfg.auto_yes;
            commands::upgrade::run(&backend, args.simulate, auto_yes)?;
        }
        Commands::Search { query } => {
            commands::search::run(&backend, &query)?;
        }
        Commands::List => {
            commands::list::run(&backend)?;
        }
        Commands::Ext(ext_cmd) => match ext_cmd {
            ExtCommands::List => {
                commands::extension::list()?;
            }
            ExtCommands::New { id, name } => {
                commands::extension::new_extension(&id, name)?;
            }
            ExtCommands::Remove { id } => {
                commands::extension::remove(&id)?;
            }
        },
        Commands::Config(_) => unreachable!(),
    }

    Ok(())
}
