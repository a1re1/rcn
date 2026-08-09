//! rcn-cli library — shadcn-style installer for the rcn gpui component library.

pub mod add;
pub mod cargo_edit;
pub mod config;
pub mod diff;
pub mod init;
pub mod list;
pub mod modfile;
pub mod registry;
pub mod source;

use anyhow::Result;
use clap::{Parser, Subcommand};
use std::path::PathBuf;

#[derive(Parser, Debug)]
#[command(name = "rcn", version, about = "Install rcn gpui components into your project")]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    /// Initialize rcn in a consumer cargo project
    Init {
        /// Path to a local rcn repo checkout (offline source)
        #[arg(long)]
        path: Option<PathBuf>,
        /// Git ref for GitHub raw fetches
        #[arg(long)]
        r#ref: Option<String>,
        /// Overwrite existing rcn.toml / core files
        #[arg(long)]
        force: bool,
    },
    /// Add one or more components (and their registry deps)
    Add {
        /// Component names (kebab-case or snake_case)
        names: Vec<String>,
        /// Add every component in the registry
        #[arg(long)]
        all: bool,
        /// Overwrite existing component files
        #[arg(long)]
        overwrite: bool,
        /// Path to a local rcn repo checkout (offline source)
        #[arg(long)]
        path: Option<PathBuf>,
    },
    /// Show a unified diff of an installed component against the registry
    Diff {
        /// Component name (kebab-case or snake_case)
        name: String,
        /// Path to a local rcn repo checkout (offline source)
        #[arg(long)]
        path: Option<PathBuf>,
    },
    /// List registry components (and which are installed)
    List {
        /// Path to a local rcn repo checkout (offline source)
        #[arg(long)]
        path: Option<PathBuf>,
    },
    /// Maintainer: scan the rcn repo and write registry.json
    Registry {
        #[command(subcommand)]
        command: RegistryCommands,
    },
}

#[derive(Subcommand, Debug)]
pub enum RegistryCommands {
    /// Build registry.json from src/components
    Build {
        /// Path to the rcn repo root (default: current directory)
        #[arg(long)]
        repo: Option<PathBuf>,
    },
}

/// Entry point shared by the binary and integration tests.
pub fn run(cli: Cli) -> Result<()> {
    match cli.command {
        Commands::Init { path, r#ref, force } => init::run(path, r#ref, force),
        Commands::Add {
            names,
            all,
            overwrite,
            path,
        } => add::run(names, all, overwrite, path),
        Commands::Diff { name, path } => diff::run(name, path),
        Commands::List { path } => list::run(path),
        Commands::Registry {
            command: RegistryCommands::Build { repo },
        } => registry::build(repo),
    }
}