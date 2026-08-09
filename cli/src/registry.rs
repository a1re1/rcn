//! Registry scan/build and types.

use anyhow::{Context, Result, bail};
use serde::{Deserialize, Serialize};
use std::collections::{BTreeMap, BTreeSet, HashMap, HashSet};
use std::fs;
use std::path::{Path, PathBuf};

/// Schema version written into registry.json.
pub const REGISTRY_VERSION: u32 = 1;

/// Relative path of the generated registry file at the repo root.
pub const REGISTRY_FILE: &str = "registry.json";

/// Core source files that every consumer needs, keyed by logical name.
const CORE_FILES: &[(&str, &str)] = &[
    ("theme", "src/theme.rs"),
    ("motion", "src/motion.rs"),
    ("assets", "src/assets.rs"),
    ("components_mod", "src/components/mod.rs"),
];

/// A git dependency pin (gpui / gpui_platform).
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct GitDep {
    pub git: String,
    pub rev: String,
    pub package: String,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub features: Vec<String>,
}

/// An external crates.io-style dependency declared by a component.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct CrateDep {
    pub name: String,
    pub version: String,
}

/// One installable component in the registry.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct ComponentEntry {
    /// Kebab-case public name (e.g. `alert-dialog`).
    pub name: String,
    /// Snake_case Rust module / file stem (e.g. `alert_dialog`).
    pub module: String,
    /// First line of the file's `//!` doc comment, or empty.
    pub description: String,
    /// Repo-relative source paths that make up this component.
    pub files: Vec<String>,
    /// Other component names this one imports via `use crate::components::…`.
    pub registry_deps: Vec<String>,
    /// External crate deps (name uses crates.io hyphen form).
    pub crate_deps: Vec<CrateDep>,
}

/// A core file entry (theme / motion / assets / components mod).
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct CoreFile {
    /// Logical name: `theme`, `motion`, `assets`, or `components_mod`.
    pub name: String,
    /// Repo-relative path.
    pub path: String,
}

/// Full `registry.json` document.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct Registry {
    pub version: u32,
    pub gpui_deps: BTreeMap<String, GitDep>,
    pub core: Vec<CoreFile>,
    pub components: Vec<ComponentEntry>,
}

impl Registry {
    /// Look up a component by kebab or snake name.
    pub fn find(&self, name: &str) -> Option<&ComponentEntry> {
        let key = normalize_name(name);
        self.components
            .iter()
            .find(|c| c.name == key || c.module == name.replace('-', "_"))
    }

    /// Expand `names` (or every component when `all`) into a cycle-safe
    /// install set including transitive `registry_deps`. Order is a stable
    /// topological order (deps before dependents) when acyclic; if a cycle is
    /// present the remaining cyclic nodes are appended in name order.
    pub fn resolve(&self, names: &[String], all: bool) -> Result<Vec<String>> {
        let by_name: HashMap<&str, &ComponentEntry> = self
            .components
            .iter()
            .map(|c| (c.name.as_str(), c))
            .collect();

        let mut requested: Vec<String> = if all {
            self.components.iter().map(|c| c.name.clone()).collect()
        } else {
            let mut out = Vec::new();
            for raw in names {
                let key = normalize_name(raw);
                if !by_name.contains_key(key.as_str()) {
                    // Also accept snake_case module name.
                    if let Some(c) = self
                        .components
                        .iter()
                        .find(|c| c.module == raw.replace('-', "_"))
                    {
                        out.push(c.name.clone());
                    } else {
                        bail!("unknown component `{raw}`");
                    }
                } else {
                    out.push(key);
                }
            }
            out
        };

        // Dedup requested while preserving order.
        let mut seen = HashSet::new();
        requested.retain(|n| seen.insert(n.clone()));

        // Collect transitive closure.
        let mut needed: BTreeSet<String> = BTreeSet::new();
        let mut stack = requested;
        while let Some(name) = stack.pop() {
            if !needed.insert(name.clone()) {
                continue;
            }
            let entry = by_name
                .get(name.as_str())
                .copied()
                .ok_or_else(|| anyhow::anyhow!("unknown component `{name}`"))?;
            for dep in &entry.registry_deps {
                if !needed.contains(dep) {
                    stack.push(dep.clone());
                }
            }
        }

        // Kahn topological sort for a stable install order.
        let mut indegree: HashMap<&str, usize> = HashMap::new();
        let mut dependents: HashMap<&str, Vec<&str>> = HashMap::new();
        for name in &needed {
            indegree.entry(name.as_str()).or_insert(0);
            let entry = by_name[name.as_str()];
            for dep in &entry.registry_deps {
                if needed.contains(dep) {
                    *indegree.entry(name.as_str()).or_insert(0) += 1;
                    dependents
                        .entry(dep.as_str())
                        .or_default()
                        .push(name.as_str());
                }
            }
        }

        let mut ready: BTreeSet<&str> = indegree
            .iter()
            .filter(|(_, d)| **d == 0)
            .map(|(&n, _)| n)
            .collect();
        let mut ordered = Vec::new();
        while let Some(n) = ready.iter().next().copied() {
            ready.remove(n);
            ordered.push(n.to_string());
            if let Some(children) = dependents.get(n) {
                for &child in children {
                    if let Some(d) = indegree.get_mut(child) {
                        *d = d.saturating_sub(1);
                        if *d == 0 {
                            ready.insert(child);
                        }
                    }
                }
            }
        }

        // Cycle remainder (if any) — append in sorted name order so the
        // install set is still complete and deterministic.
        if ordered.len() < needed.len() {
            let mut rest: Vec<String> = needed
                .into_iter()
                .filter(|n| !ordered.iter().any(|o| o == n))
                .collect();
            rest.sort();
            ordered.extend(rest);
        }

        Ok(ordered)
    }
}

/// `rcn registry build [--repo <path>]`
pub fn build(repo: Option<PathBuf>) -> Result<()> {
    let repo = repo.unwrap_or_else(|| PathBuf::from("."));
    let registry = scan_repo(&repo)?;
    let out = repo.join(REGISTRY_FILE);
    let json = serde_json::to_string_pretty(&registry).context("serialize registry.json")? + "\n";
    fs::write(&out, json).with_context(|| format!("write {}", out.display()))?;
    eprintln!(
        "wrote {} ({} components)",
        out.display(),
        registry.components.len()
    );
    Ok(())
}

/// Scan an rcn checkout and build a [`Registry`] value (does not write).
pub fn scan_repo(repo: &Path) -> Result<Registry> {
    let cargo_toml = fs::read_to_string(repo.join("Cargo.toml"))
        .with_context(|| format!("read {}/Cargo.toml", repo.display()))?;
    let gpui_deps = parse_gpui_deps(&cargo_toml)?;
    let crate_versions = parse_crate_versions(&cargo_toml);

    let mut core = Vec::new();
    for &(name, rel) in CORE_FILES {
        let path = repo.join(rel);
        if !path.is_file() {
            bail!("missing core file {rel}");
        }
        core.push(CoreFile {
            name: name.to_string(),
            path: rel.to_string(),
        });
    }

    let components_dir = repo.join("src/components");
    if !components_dir.is_dir() {
        bail!(
            "components directory not found: {}",
            components_dir.display()
        );
    }

    let mut components = Vec::new();
    let mut entries: Vec<PathBuf> = fs::read_dir(&components_dir)
        .with_context(|| format!("read {}", components_dir.display()))?
        .filter_map(|e| e.ok())
        .map(|e| e.path())
        .filter(|p| {
            p.extension().and_then(|e| e.to_str()) == Some("rs")
                && p.file_stem().and_then(|s| s.to_str()) != Some("mod")
        })
        .collect();
    entries.sort();

    for path in entries {
        let stem = path
            .file_stem()
            .and_then(|s| s.to_str())
            .ok_or_else(|| anyhow::anyhow!("bad component path {}", path.display()))?
            .to_string();
        let source =
            fs::read_to_string(&path).with_context(|| format!("read {}", path.display()))?;
        let rel = format!("src/components/{stem}.rs");
        let description = first_doc_line(&source);
        let registry_deps = parse_registry_deps(&source);
        let crate_deps = parse_crate_deps(&source, &crate_versions);
        components.push(ComponentEntry {
            name: stem.replace('_', "-"),
            module: stem,
            description,
            files: vec![rel],
            registry_deps,
            crate_deps,
        });
    }

    Ok(Registry {
        version: REGISTRY_VERSION,
        gpui_deps,
        core,
        components,
    })
}

/// Normalize a user-supplied component name to kebab-case registry form.
pub fn normalize_name(name: &str) -> String {
    name.replace('_', "-")
}

/// First non-empty `//!` doc line, stripped of the prefix.
pub fn first_doc_line(source: &str) -> String {
    for line in source.lines() {
        let trimmed = line.trim();
        if let Some(rest) = trimmed.strip_prefix("//!") {
            let text = rest.trim();
            if !text.is_empty() {
                return text.to_string();
            }
            // keep scanning past empty `//!` lines
            continue;
        }
        // Stop once we leave the leading doc-comment block / blank lines.
        if trimmed.is_empty() {
            continue;
        }
        if trimmed.starts_with("//") {
            continue;
        }
        break;
    }
    String::new()
}

/// Component names referenced by `use crate::components::<module>` lines.
/// Returns kebab-case names, sorted and deduped.
pub fn parse_registry_deps(source: &str) -> Vec<String> {
    let mut deps = BTreeSet::new();
    for line in source.lines() {
        let trimmed = line.trim();
        // Match `use crate::components::foo` and `pub use crate::components::foo`.
        let rest = if let Some(r) = trimmed.strip_prefix("pub use crate::components::") {
            r
        } else if let Some(r) = trimmed.strip_prefix("use crate::components::") {
            r
        } else {
            continue;
        };
        let module = rest
            .split(|c: char| c == ':' || c == ';' || c == '{' || c == ' ' || c == '/')
            .next()
            .unwrap_or("")
            .trim();
        if !module.is_empty() {
            deps.insert(module.replace('_', "-"));
        }
    }
    deps.into_iter().collect()
}

/// External crates from top-level `use <crate>::…` lines (excluding
/// std/core/alloc/gpui/crate/super/self), mapped to versions via `versions`.
pub fn parse_crate_deps(source: &str, versions: &HashMap<String, String>) -> Vec<CrateDep> {
    const SKIP: &[&str] = &[
        "std",
        "core",
        "alloc",
        "gpui",
        "gpui_platform",
        "crate",
        "super",
        "self",
    ];
    let mut found: BTreeMap<String, String> = BTreeMap::new();
    for line in source.lines() {
        let trimmed = line.trim();
        // Only top-level `use foo::…` (not `pub use`, not attributes).
        let rest = match trimmed.strip_prefix("use ") {
            Some(r) => r,
            None => continue,
        };
        // Skip `use crate::`, `use super::`, re-exports already handled above.
        let crate_root = rest
            .split(|c: char| c == ':' || c == ';' || c == '{' || c == ' ' || c == '/')
            .next()
            .unwrap_or("")
            .trim();
        if crate_root.is_empty() || SKIP.contains(&crate_root) {
            continue;
        }
        // crates.io names use hyphens; Rust imports use underscores.
        let crate_name = crate_root.replace('_', "-");
        if let Some(version) = versions
            .get(&crate_name)
            .or_else(|| versions.get(crate_root))
        {
            found.entry(crate_name).or_insert_with(|| version.clone());
        } else {
            // Still record the dep with an empty version so callers can see it;
            // build will prefer known versions from Cargo.toml.
            found.entry(crate_name).or_insert_with(String::new);
        }
    }
    found
        .into_iter()
        .filter(|(_, v)| !v.is_empty())
        .map(|(name, version)| CrateDep { name, version })
        .collect()
}

/// Parse `gpui` and `gpui_platform` git dependency specs from a Cargo.toml.
pub fn parse_gpui_deps(cargo_toml: &str) -> Result<BTreeMap<String, GitDep>> {
    let value: toml_edit::DocumentMut = cargo_toml
        .parse()
        .context("parse Cargo.toml for gpui deps")?;
    let deps = value
        .get("dependencies")
        .and_then(|i| i.as_table())
        .ok_or_else(|| anyhow::anyhow!("Cargo.toml has no [dependencies]"))?;

    let mut out = BTreeMap::new();
    for name in ["gpui", "gpui_platform"] {
        let item = deps
            .get(name)
            .ok_or_else(|| anyhow::anyhow!("Cargo.toml missing dependencies.{name}"))?;
        let table = item
            .as_inline_table()
            .map(|t| t as &dyn TomlTableLike)
            .or_else(|| item.as_table().map(|t| t as &dyn TomlTableLike))
            .ok_or_else(|| anyhow::anyhow!("dependencies.{name} is not a table"))?;

        let git = table
            .get_str("git")
            .ok_or_else(|| anyhow::anyhow!("dependencies.{name} missing git"))?
            .to_string();
        let rev = table
            .get_str("rev")
            .ok_or_else(|| anyhow::anyhow!("dependencies.{name} missing rev"))?
            .to_string();
        let package = table.get_str("package").unwrap_or(name).to_string();
        let features = table.get_str_array("features").unwrap_or_default();

        out.insert(
            name.to_string(),
            GitDep {
                git,
                rev,
                package,
                features,
            },
        );
    }
    Ok(out)
}

/// Collect simple versioned crate deps from `[dependencies]` (string or
/// `{ version = "…" }` forms). Keys are crates.io names (hyphenated).
pub fn parse_crate_versions(cargo_toml: &str) -> HashMap<String, String> {
    let mut out = HashMap::new();
    let Ok(value) = cargo_toml.parse::<toml_edit::DocumentMut>() else {
        return out;
    };
    let Some(deps) = value.get("dependencies").and_then(|i| i.as_table()) else {
        return out;
    };
    for (key, item) in deps.iter() {
        let name = key.to_string();
        if name == "gpui" || name == "gpui_platform" {
            continue;
        }
        if let Some(s) = item.as_str() {
            out.insert(name, s.to_string());
            continue;
        }
        let version = item
            .as_inline_table()
            .and_then(|t| t.get("version"))
            .and_then(|v| v.as_str())
            .or_else(|| {
                item.as_table()
                    .and_then(|t| t.get("version"))
                    .and_then(|v| v.as_str())
            });
        if let Some(v) = version {
            out.insert(name, v.to_string());
        }
    }
    out
}

/// Tiny helper so we can read both inline and full tables the same way.
trait TomlTableLike {
    fn get_str(&self, key: &str) -> Option<&str>;
    fn get_str_array(&self, key: &str) -> Option<Vec<String>>;
}

impl TomlTableLike for toml_edit::Table {
    fn get_str(&self, key: &str) -> Option<&str> {
        self.get(key).and_then(|v| v.as_str())
    }
    fn get_str_array(&self, key: &str) -> Option<Vec<String>> {
        self.get(key).and_then(|v| v.as_array()).map(|arr| {
            arr.iter()
                .filter_map(|i| i.as_str().map(|s| s.to_string()))
                .collect()
        })
    }
}

impl TomlTableLike for toml_edit::InlineTable {
    fn get_str(&self, key: &str) -> Option<&str> {
        self.get(key).and_then(|v| v.as_str())
    }
    fn get_str_array(&self, key: &str) -> Option<Vec<String>> {
        self.get(key).and_then(|v| v.as_array()).map(|arr| {
            arr.iter()
                .filter_map(|i| i.as_str().map(|s| s.to_string()))
                .collect()
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn first_doc_line_reads_leading_module_docs() {
        let src = "//! Button — port of shadcn.\n//!\n//! More detail.\n\nuse gpui::div;\n";
        assert_eq!(first_doc_line(src), "Button — port of shadcn.");
    }

    #[test]
    fn first_doc_line_empty_without_docs() {
        assert_eq!(first_doc_line("use gpui::div;\n"), "");
    }

    #[test]
    fn parse_registry_deps_handles_use_and_pub_use() {
        let src = r#"
use crate::components::button::{Button, GroupPosition};
pub use crate::components::dialog::{Dialog, DialogResult};
use crate::theme::Theme;
use crate::components::separator::Separator;
"#;
        assert_eq!(
            parse_registry_deps(src),
            vec![
                "button".to_string(),
                "dialog".to_string(),
                "separator".to_string()
            ]
        );
    }

    #[test]
    fn parse_crate_deps_maps_underscore_to_hyphen() {
        let src = "use unicode_segmentation::UnicodeSegmentation as _;\nuse gpui::div;\nuse std::ops::Range;\n";
        let mut versions = HashMap::new();
        versions.insert("unicode-segmentation".into(), "1".into());
        let deps = parse_crate_deps(src, &versions);
        assert_eq!(
            deps,
            vec![CrateDep {
                name: "unicode-segmentation".into(),
                version: "1".into(),
            }]
        );
    }

    #[test]
    fn parse_gpui_deps_from_inline_tables() {
        let toml = r#"
[dependencies]
gpui = { git = "https://github.com/zed-industries/zed", rev = "abc123", package = "gpui" }
gpui_platform = { git = "https://github.com/zed-industries/zed", rev = "abc123", package = "gpui_platform", features = ["font-kit"] }
unicode-segmentation = "1"
"#;
        let deps = parse_gpui_deps(toml).unwrap();
        assert_eq!(deps["gpui"].rev, "abc123");
        assert_eq!(deps["gpui"].package, "gpui");
        assert!(deps["gpui"].features.is_empty());
        assert_eq!(deps["gpui_platform"].features, vec!["font-kit".to_string()]);
    }

    #[test]
    fn resolve_transitive_and_cycle_safe() {
        let registry = Registry {
            version: 1,
            gpui_deps: BTreeMap::new(),
            core: vec![],
            components: vec![
                ComponentEntry {
                    name: "a".into(),
                    module: "a".into(),
                    description: String::new(),
                    files: vec![],
                    registry_deps: vec!["b".into()],
                    crate_deps: vec![],
                },
                ComponentEntry {
                    name: "b".into(),
                    module: "b".into(),
                    description: String::new(),
                    files: vec![],
                    registry_deps: vec!["c".into()],
                    crate_deps: vec![],
                },
                ComponentEntry {
                    name: "c".into(),
                    module: "c".into(),
                    description: String::new(),
                    files: vec![],
                    registry_deps: vec![],
                    crate_deps: vec![],
                },
                ComponentEntry {
                    name: "cycle-x".into(),
                    module: "cycle_x".into(),
                    description: String::new(),
                    files: vec![],
                    registry_deps: vec!["cycle-y".into()],
                    crate_deps: vec![],
                },
                ComponentEntry {
                    name: "cycle-y".into(),
                    module: "cycle_y".into(),
                    description: String::new(),
                    files: vec![],
                    registry_deps: vec!["cycle-x".into()],
                    crate_deps: vec![],
                },
            ],
        };

        let set = registry.resolve(&["a".into()], false).unwrap();
        assert_eq!(set, vec!["c", "b", "a"]);

        let cyclic = registry.resolve(&["cycle-x".into()], false).unwrap();
        assert_eq!(cyclic.len(), 2);
        assert!(cyclic.contains(&"cycle-x".to_string()));
        assert!(cyclic.contains(&"cycle-y".to_string()));
    }

    #[test]
    fn resolve_unknown_component_errors() {
        let registry = Registry {
            version: 1,
            gpui_deps: BTreeMap::new(),
            core: vec![],
            components: vec![],
        };
        let err = registry.resolve(&["nope".into()], false).unwrap_err();
        assert!(err.to_string().contains("unknown component"));
    }

    #[test]
    fn scan_inline_fixture_directory() {
        let tmp = tempfile::tempdir().unwrap();
        let root = tmp.path();
        fs::create_dir_all(root.join("src/components")).unwrap();
        fs::write(
            root.join("Cargo.toml"),
            r#"
[package]
name = "fixture"
version = "0.1.0"

[dependencies]
gpui = { git = "https://example.com/zed", rev = "deadbeef", package = "gpui" }
gpui_platform = { git = "https://example.com/zed", rev = "deadbeef", package = "gpui_platform", features = ["font-kit"] }
unicode-segmentation = "1"
"#,
        )
        .unwrap();
        for (name, body) in [
            ("theme.rs", "//! theme\n"),
            ("motion.rs", "//! motion\n"),
            ("assets.rs", "//! assets\n"),
        ] {
            fs::write(root.join("src").join(name), body).unwrap();
        }
        fs::write(root.join("src/components/mod.rs"), "// mods\n").unwrap();
        fs::write(
            root.join("src/components/button.rs"),
            "//! Button — a button.\n\nuse gpui::div;\nuse crate::theme::Theme;\n",
        )
        .unwrap();
        fs::write(
            root.join("src/components/button_group.rs"),
            "//! Button group.\n\nuse crate::components::button::Button;\nuse crate::components::separator::Separator;\n",
        )
        .unwrap();
        fs::write(
            root.join("src/components/separator.rs"),
            "//! Separator.\n\nuse gpui::div;\n",
        )
        .unwrap();
        fs::write(
            root.join("src/components/input.rs"),
            "//! Input.\n\nuse unicode_segmentation::UnicodeSegmentation as _;\nuse gpui::div;\n",
        )
        .unwrap();

        let reg = scan_repo(root).unwrap();
        assert_eq!(reg.version, 1);
        assert_eq!(reg.gpui_deps["gpui"].rev, "deadbeef");
        assert_eq!(reg.core.len(), 4);
        assert_eq!(reg.components.len(), 4);

        let bg = reg.find("button-group").unwrap();
        assert_eq!(bg.module, "button_group");
        assert_eq!(bg.description, "Button group.");
        assert_eq!(bg.registry_deps, vec!["button", "separator"]);
        assert!(bg.crate_deps.is_empty());

        let input = reg.find("input").unwrap();
        assert_eq!(
            input.crate_deps,
            vec![CrateDep {
                name: "unicode-segmentation".into(),
                version: "1".into(),
            }]
        );
    }
}
