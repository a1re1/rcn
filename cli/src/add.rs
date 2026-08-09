//! `rcn add` subcommand.

use anyhow::{Context, Result, bail};
use std::collections::BTreeSet;
use std::path::{Path, PathBuf};

use crate::cargo_edit::ensure_crate_deps;
use crate::config;
use crate::modfile;
use crate::registry::CrateDep;
use crate::source;

/// `rcn add <names...> [--all] [--overwrite] [--path <local-rcn-repo>]`
pub fn run(names: Vec<String>, all: bool, overwrite: bool, path: Option<PathBuf>) -> Result<()> {
    if !all && names.is_empty() {
        bail!("specify at least one component name, or pass --all");
    }

    let project = std::env::current_dir().context("failed to resolve current directory")?;
    source::require_cargo_toml(&project)?;
    let cfg = config::load(&project)?;

    let src = source::resolve(path, Some(&cfg.source.repo), Some(&cfg.source.r#ref))?;
    println!("→ fetching registry from {}", src.label());
    let registry = src.fetch_registry()?;

    let install_set = registry.resolve(&names, all)?;
    if install_set.is_empty() {
        println!("· nothing to install");
        return Ok(());
    }

    println!(
        "→ installing {} component{}: {}",
        install_set.len(),
        if install_set.len() == 1 { "" } else { "s" },
        install_set.join(", ")
    );

    let components_dir = project.join(&cfg.paths.components);
    std::fs::create_dir_all(&components_dir)
        .with_context(|| format!("failed to create {}", components_dir.display()))?;

    let mut modules: Vec<String> = Vec::new();
    let mut crate_deps: Vec<CrateDep> = Vec::new();
    let mut seen_crates: BTreeSet<String> = BTreeSet::new();
    let mut added_files = 0usize;
    let mut skipped_files = 0usize;

    for name in &install_set {
        let entry = registry
            .find(name)
            .ok_or_else(|| anyhow::anyhow!("unknown component `{name}`"))?;

        modules.push(entry.module.clone());

        for dep in &entry.crate_deps {
            if seen_crates.insert(dep.name.clone()) {
                crate_deps.push(dep.clone());
            }
        }

        for rel in &entry.files {
            let file_name = Path::new(rel)
                .file_name()
                .and_then(|s| s.to_str())
                .ok_or_else(|| anyhow::anyhow!("invalid component file path `{rel}`"))?;
            let dest = components_dir.join(file_name);

            if dest.exists() && !overwrite {
                println!(
                    "· {} already exists — skipped (pass --overwrite)",
                    rel_display(&project, &dest)
                );
                skipped_files += 1;
                continue;
            }

            let contents = src
                .fetch_text(rel)
                .with_context(|| format!("failed to fetch `{rel}` for component `{name}`"))?;
            if let Some(parent) = dest.parent() {
                std::fs::create_dir_all(parent)
                    .with_context(|| format!("failed to create {}", parent.display()))?;
            }
            std::fs::write(&dest, contents)
                .with_context(|| format!("failed to write {}", dest.display()))?;
            println!("✓ {}", rel_display(&project, &dest));
            added_files += 1;
        }
    }

    // Keep modules sorted for a stable mod.rs (ensure_pub_mods also sorts).
    modules.sort();
    modules.dedup();

    let mod_path = components_dir.join("mod.rs");
    let mod_changed = modfile::update_components_mod(&mod_path, &modules)?;
    if mod_changed {
        println!("✓ updated {}", rel_display(&project, &mod_path));
    } else {
        println!("· {} already up to date", rel_display(&project, &mod_path));
    }

    if !crate_deps.is_empty() {
        let cargo_path = project.join("Cargo.toml");
        let cargo_text = std::fs::read_to_string(&cargo_path)
            .with_context(|| format!("failed to read {}", cargo_path.display()))?;
        let (next_cargo, added) = ensure_crate_deps(&cargo_text, &crate_deps)?;
        if next_cargo != cargo_text {
            std::fs::write(&cargo_path, next_cargo)
                .with_context(|| format!("failed to write {}", cargo_path.display()))?;
        }
        if added.is_empty() {
            println!("· Cargo.toml already has required crate deps — left unchanged");
        } else {
            println!("✓ added to Cargo.toml: {}", added.join(", "));
        }
    }

    println!();
    println!(
        "Done. {} file{} written, {} skipped.",
        added_files,
        if added_files == 1 { "" } else { "s" },
        skipped_files,
    );
    Ok(())
}

fn rel_display(project: &Path, path: &Path) -> String {
    path.strip_prefix(project)
        .map(|p| p.display().to_string())
        .unwrap_or_else(|_| path.display().to_string())
}
