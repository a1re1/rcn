//! `rcn list` subcommand.

use anyhow::{Context, Result};
use std::path::{Path, PathBuf};

use crate::config;
use crate::source;

/// `rcn list [--path <local-rcn-repo>]`
///
/// Works inside an initialized consumer project, and also with `--path` alone
/// (no `rcn.toml`) — in that case nothing is marked installed.
pub fn run(path: Option<PathBuf>) -> Result<()> {
    let project = std::env::current_dir().context("failed to resolve current directory")?;

    let (src, components_dir) = resolve_list_context(&project, path)?;
    println!("→ registry from {}", src.label());
    let registry = src.fetch_registry()?;

    if registry.components.is_empty() {
        println!("(no components in registry)");
        return Ok(());
    }

    // Sort by name for stable output (registry is usually already sorted).
    let mut components = registry.components.clone();
    components.sort_by(|a, b| a.name.cmp(&b.name));

    let name_width = components
        .iter()
        .map(|c| c.name.len())
        .max()
        .unwrap_or(0)
        .max(4);

    println!();
    for entry in &components {
        let installed = components_dir
            .as_ref()
            .map(|dir| {
                entry.files.iter().any(|rel| {
                    let file_name = Path::new(rel).file_name().and_then(|s| s.to_str());
                    match file_name {
                        Some(name) => dir.join(name).exists(),
                        None => false,
                    }
                })
            })
            .unwrap_or(false);

        let marker = if installed { "[installed]" } else { "[ ]" };
        let desc = if entry.description.is_empty() {
            String::new()
        } else {
            format!("  {}", entry.description)
        };
        println!(
            "  {marker:<11}  {:<width$}{desc}",
            entry.name,
            width = name_width
        );
    }
    println!();
    println!("{} components", components.len());
    Ok(())
}

/// Resolve source + optional local components dir for installed checks.
///
/// - With `rcn.toml`: use config for source (unless `--path` overrides) and
///   the configured components path for installed markers.
/// - Without `rcn.toml` but with `--path`: offline registry only, nothing installed.
/// - Without either: friendly error pointing at `rcn init` / `--path`.
fn resolve_list_context(
    project: &Path,
    path: Option<PathBuf>,
) -> Result<(source::Source, Option<PathBuf>)> {
    let cfg_path = project.join(config::CONFIG_FILE);
    if cfg_path.exists() {
        let cfg = config::load(project)?;
        let src = source::resolve(
            path,
            Some(&cfg.source.repo),
            Some(&cfg.source.r#ref),
        )?;
        let components_dir = Some(project.join(&cfg.paths.components));
        return Ok((src, components_dir));
    }

    if let Some(p) = path {
        let src = source::resolve(Some(p), None, None)?;
        return Ok((src, None));
    }

    // No config and no --path: still allow default GitHub, but no installed marks.
    // Spec: "Must work both inside an initialized consumer project and (with
    // --path) without rcn.toml". Without either, give the init hint.
    anyhow::bail!("no {} found — run `rcn init` first (or pass --path to a local rcn checkout)", config::CONFIG_FILE);
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::config::Config;
    use crate::registry::{ComponentEntry, Registry, REGISTRY_FILE};
    use std::collections::BTreeMap;
    use tempfile::tempdir;

    fn write_mini_registry(root: &Path) {
        let reg = Registry {
            version: 1,
            gpui_deps: BTreeMap::new(),
            core: vec![],
            components: vec![
                ComponentEntry {
                    name: "button".into(),
                    module: "button".into(),
                    description: "A button".into(),
                    files: vec!["src/components/button.rs".into()],
                    registry_deps: vec![],
                    crate_deps: vec![],
                },
                ComponentEntry {
                    name: "input".into(),
                    module: "input".into(),
                    description: "Text input".into(),
                    files: vec!["src/components/input.rs".into()],
                    registry_deps: vec![],
                    crate_deps: vec![],
                },
            ],
        };
        std::fs::write(
            root.join(REGISTRY_FILE),
            serde_json::to_string_pretty(&reg).unwrap(),
        )
        .unwrap();
    }

    #[test]
    fn list_without_config_requires_path_or_init() {
        let project = tempdir().unwrap();
        let err = resolve_list_context(project.path(), None)
            .unwrap_err()
            .to_string();
        assert!(err.contains("rcn init"), "{err}");
    }

    #[test]
    fn list_with_path_only_has_no_installed_dir() {
        let repo = tempdir().unwrap();
        write_mini_registry(repo.path());
        let project = tempdir().unwrap();
        let (src, components_dir) =
            resolve_list_context(project.path(), Some(repo.path().to_path_buf())).unwrap();
        assert!(matches!(src, source::Source::Local(_)));
        assert!(components_dir.is_none());
    }

    #[test]
    fn list_with_config_reports_components_dir() {
        let repo = tempdir().unwrap();
        write_mini_registry(repo.path());
        let project = tempdir().unwrap();
        config::save(project.path(), &Config::default()).unwrap();
        let (src, components_dir) =
            resolve_list_context(project.path(), Some(repo.path().to_path_buf())).unwrap();
        assert!(matches!(src, source::Source::Local(_)));
        assert_eq!(
            components_dir.unwrap(),
            project.path().join("src/components")
        );
    }
}
