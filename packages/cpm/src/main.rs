use anyhow::Result;
use clap::{Args, Parser, Subcommand};

mod backend;
mod commands;
mod ui;

use backend::ApkBackend;

#[derive(Parser)]
#[command(
    name = "cpm",
    author = "CrestCode Team",
    version = "0.1.0",
    about = "CrestCode Package Manager - Modern APK frontend & Extension Manager",
    long_about = "A high-performance package manager for Alpine Linux & CrestCode with APT-style syntax, beautiful tables, and extension management."
)]
struct Cli {
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

fn main() -> Result<()> {
    let cli = Cli::parse();
    ui::print_banner();

    let backend = ApkBackend::detect()?;

    match cli.command {
        Commands::Install(args) => {
            commands::install::run(&backend, &args.packages, args.simulate, args.yes)?;
        }
        Commands::Remove(args) => {
            commands::remove::run(&backend, &args.packages, args.simulate, args.yes)?;
        }
        Commands::Update => {
            commands::update::run(&backend)?;
        }
        Commands::Upgrade(args) => {
            commands::upgrade::run(&backend, args.simulate, args.yes)?;
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
    }

    Ok(())
}
