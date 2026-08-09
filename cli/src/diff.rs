//! `rcn diff` subcommand.

use anyhow::{Context, Result, bail};
use similar::{ChangeTag, TextDiff};
use std::path::{Path, PathBuf};

use crate::config;
use crate::source;

/// `rcn diff <name> [--path <local-rcn-repo>]`
pub fn run(name: String, path: Option<PathBuf>) -> Result<()> {
    let project = std::env::current_dir().context("failed to resolve current directory")?;
    let cfg = config::load(&project)?;

    let src = source::resolve(path, Some(&cfg.source.repo), Some(&cfg.source.r#ref))?;
    let registry = src.fetch_registry()?;

    let entry = registry.find(&name).ok_or_else(|| {
        anyhow::anyhow!("unknown component `{name}` — run `rcn list` to see available components")
    })?;

    let components_dir = project.join(&cfg.paths.components);
    let mut any_diff = false;
    let mut missing = Vec::new();

    for rel in &entry.files {
        let file_name = Path::new(rel)
            .file_name()
            .and_then(|s| s.to_str())
            .ok_or_else(|| anyhow::anyhow!("invalid component file path `{rel}`"))?;
        let local_path = components_dir.join(file_name);

        if !local_path.exists() {
            missing.push(rel_display(&project, &local_path));
            continue;
        }

        let local = std::fs::read_to_string(&local_path)
            .with_context(|| format!("failed to read {}", local_path.display()))?;
        let remote = src
            .fetch_text(rel)
            .with_context(|| format!("failed to fetch registry version of `{rel}`"))?;

        if local == remote {
            continue;
        }

        any_diff = true;
        print_unified_diff(&rel_display(&project, &local_path), rel, &local, &remote);
    }

    if !missing.is_empty() {
        bail!(
            "component `{}` is not installed (missing: {})",
            entry.name,
            missing.join(", ")
        );
    }

    if !any_diff {
        println!("no changes");
    }

    Ok(())
}

fn print_unified_diff(local_label: &str, remote_label: &str, local: &str, remote: &str) {
    let diff = TextDiff::from_lines(local, remote);
    println!("--- {local_label}");
    println!("+++ {remote_label}");
    for op in diff.ops() {
        for change in diff.iter_changes(op) {
            let sign = match change.tag() {
                ChangeTag::Delete => "-",
                ChangeTag::Insert => "+",
                ChangeTag::Equal => " ",
            };
            // change.to_string() keeps the trailing newline from the line.
            print!("{sign}{change}");
        }
    }
}

fn rel_display(project: &Path, path: &Path) -> String {
    path.strip_prefix(project)
        .map(|p| p.display().to_string())
        .unwrap_or_else(|_| path.display().to_string())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::config::{self, Config};
    use crate::registry::{ComponentEntry, REGISTRY_FILE, Registry};
    use std::collections::BTreeMap;
    use tempfile::tempdir;

    fn write_mini_registry(root: &Path) {
        let reg = Registry {
            version: 1,
            gpui_deps: BTreeMap::new(),
            core: vec![],
            components: vec![ComponentEntry {
                name: "button".into(),
                module: "button".into(),
                description: "A button".into(),
                files: vec!["src/components/button.rs".into()],
                registry_deps: vec![],
                crate_deps: vec![],
            }],
        };
        std::fs::write(
            root.join(REGISTRY_FILE),
            serde_json::to_string_pretty(&reg).unwrap(),
        )
        .unwrap();
        let comp_dir = root.join("src/components");
        std::fs::create_dir_all(&comp_dir).unwrap();
        std::fs::write(comp_dir.join("button.rs"), "pub fn button() {}\n").unwrap();
    }

    #[test]
    fn diff_reports_no_changes_when_identical() {
        let repo = tempdir().unwrap();
        write_mini_registry(repo.path());

        let project = tempdir().unwrap();
        config::save(project.path(), &Config::default()).unwrap();
        let dest = project.path().join("src/components");
        std::fs::create_dir_all(&dest).unwrap();
        std::fs::write(dest.join("button.rs"), "pub fn button() {}\n").unwrap();

        let src = source::Source::local(repo.path());
        let registry = src.fetch_registry().unwrap();
        let entry = registry.find("button").unwrap();
        let local = std::fs::read_to_string(dest.join("button.rs")).unwrap();
        let remote = src.fetch_text(&entry.files[0]).unwrap();
        assert_eq!(local, remote);
    }

    #[test]
    fn diff_detects_content_change() {
        let a = "line one\n";
        let b = "line two\n";
        let diff = TextDiff::from_lines(a, b);
        let mut has_change = false;
        for op in diff.ops() {
            for change in diff.iter_changes(op) {
                if change.tag() != ChangeTag::Equal {
                    has_change = true;
                }
            }
        }
        assert!(has_change);
    }
}
