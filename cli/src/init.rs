//! `rcn init` subcommand.

use anyhow::{bail, Context, Result};
use std::path::{Path, PathBuf};

use crate::cargo_edit::{
    inject_gpui_deps, is_hello_world_main, scaffold_main_rs, suggested_mod_lines,
};
use crate::config::{self, Config, CONFIG_FILE};
use crate::modfile::COMPONENTS_MOD_HEADER;
use crate::registry::Registry;
use crate::source::{self, Source};

/// `rcn init [--path <local-rcn-repo>] [--ref <git-ref>] [--force]`
pub fn run(path: Option<PathBuf>, r#ref: Option<String>, force: bool) -> Result<()> {
    let project = std::env::current_dir().context("failed to resolve current directory")?;
    source::require_cargo_toml(&project)?;

    let config_path = project.join(CONFIG_FILE);
    if config_path.exists() && !force {
        bail!("{CONFIG_FILE} already exists — pass --force to reinitialize");
    }

    // Build source: --path wins (offline); else GitHub at --ref / default main.
    let src = source::resolve(path, Some("a1re1/rcn"), r#ref.as_deref())?;
    println!("→ fetching registry from {}", src.label());
    let registry = src.fetch_registry()?;

    let mut cfg = Config::default();
    if let Some(r) = &r#ref {
        cfg.source.r#ref = r.clone();
    }
    // When using a local path we still record the default GitHub source so
    // later `add`/`diff` without --path can fetch from the network.
    if let Source::GitHub { repo, r#ref } = &src {
        cfg.source.repo = repo.clone();
        cfg.source.r#ref = r#ref.clone();
    }

    config::save(&project, &cfg)?;
    println!("✓ wrote {CONFIG_FILE}");

    vendor_core(&project, &cfg, &src, &registry)?;

    // Inject gpui + gpui_platform.
    let cargo_path = project.join("Cargo.toml");
    let cargo_text = std::fs::read_to_string(&cargo_path)
        .with_context(|| format!("failed to read {}", cargo_path.display()))?;
    let (next_cargo, added) = inject_gpui_deps(&cargo_text, &registry.gpui_deps)?;
    if next_cargo != cargo_text {
        std::fs::write(&cargo_path, next_cargo)
            .with_context(|| format!("failed to write {}", cargo_path.display()))?;
    }
    if added.is_empty() {
        println!("· Cargo.toml already has gpui dependencies — left unchanged");
    } else {
        println!("✓ added to Cargo.toml: {}", added.join(", "));
    }

    wire_modules(&project)?;

    print_next_steps(&cfg);
    Ok(())
}

fn vendor_core(project: &Path, cfg: &Config, src: &Source, registry: &Registry) -> Result<()> {
    // Map logical core names → consumer destination paths.
    let dest_for = |name: &str| -> Option<PathBuf> {
        match name {
            "theme" => Some(project.join(&cfg.paths.theme)),
            "motion" => Some(project.join(&cfg.paths.motion)),
            "assets" => Some(project.join(&cfg.paths.assets)),
            "components_mod" => {
                // Always create a fresh header-only components/mod.rs.
                Some(project.join(&cfg.paths.components).join("mod.rs"))
            }
            _ => None,
        }
    };

    for core in &registry.core {
        let Some(dest) = dest_for(&core.name) else {
            continue;
        };
        if let Some(parent) = dest.parent() {
            std::fs::create_dir_all(parent)
                .with_context(|| format!("failed to create {}", parent.display()))?;
        }

        if core.name == "components_mod" {
            // Intentionally do NOT copy the storybook's full mod.rs — start empty.
            if dest.exists() {
                println!("· {} already exists — left unchanged", rel_display(project, &dest));
            } else {
                std::fs::write(&dest, COMPONENTS_MOD_HEADER).with_context(|| {
                    format!("failed to write {}", dest.display())
                })?;
                println!("✓ wrote {}", rel_display(project, &dest));
            }
            continue;
        }

        let text = src.fetch_text(&core.path)?;
        std::fs::write(&dest, text)
            .with_context(|| format!("failed to write {}", dest.display()))?;
        println!("✓ vendored {}", rel_display(project, &dest));
    }

    // Ensure the components directory exists even if components_mod was skipped.
    let components_dir = project.join(&cfg.paths.components);
    std::fs::create_dir_all(&components_dir)
        .with_context(|| format!("failed to create {}", components_dir.display()))?;
    let mod_rs = components_dir.join("mod.rs");
    if !mod_rs.exists() {
        std::fs::write(&mod_rs, COMPONENTS_MOD_HEADER)?;
        println!("✓ wrote {}", rel_display(project, &mod_rs));
    }

    Ok(())
}

fn wire_modules(project: &Path) -> Result<()> {
    let main_rs = project.join("src/main.rs");
    let lib_rs = project.join("src/lib.rs");

    if main_rs.exists() {
        let text = std::fs::read_to_string(&main_rs)
            .with_context(|| format!("failed to read {}", main_rs.display()))?;
        if is_hello_world_main(&text) {
            std::fs::write(&main_rs, scaffold_main_rs())
                .with_context(|| format!("failed to write {}", main_rs.display()))?;
            println!("✓ replaced src/main.rs with a minimal gpui app");
        } else {
            println!("· src/main.rs looks customized — left unchanged");
            println!("  add these module declarations near the top of src/main.rs:");
            for line in suggested_mod_lines().lines() {
                println!("    {line}");
            }
        }
        return Ok(());
    }

    if lib_rs.exists() {
        println!("· lib-only crate detected — left src/lib.rs unchanged");
        println!("  add these module declarations near the top of src/lib.rs:");
        for line in suggested_mod_lines().lines() {
            println!("    {line}");
        }
        return Ok(());
    }

    // No main.rs and no lib.rs — create src/main.rs scaffold.
    let src_dir = project.join("src");
    std::fs::create_dir_all(&src_dir)?;
    std::fs::write(&main_rs, scaffold_main_rs())?;
    println!("✓ created src/main.rs with a minimal gpui app");
    Ok(())
}

fn print_next_steps(cfg: &Config) {
    println!();
    println!("rcn is initialized. Next steps:");
    println!("  • Add components:    rcn add button");
    println!("  • Add everything:    rcn add --all");
    println!("  • Browse registry:   rcn list");
    println!();
    println!(
        "Components install into {components}/; theme/motion/assets live at the paths in {CONFIG_FILE}.",
        components = cfg.paths.components,
    );
    println!(
        "Tip: this repo builds gpui via sccache — `brew install sccache` speeds consumer builds too (optional)."
    );
}

fn rel_display(project: &Path, path: &Path) -> String {
    path.strip_prefix(project)
        .map(|p| p.display().to_string())
        .unwrap_or_else(|_| path.display().to_string())
}
