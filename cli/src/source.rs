//! Fetch registry.json and component files from a local path or GitHub raw.

use anyhow::{bail, Context, Result};
use std::path::{Path, PathBuf};

use crate::registry::{Registry, REGISTRY_FILE};

/// Where component/registry files are loaded from.
#[derive(Debug, Clone)]
pub enum Source {
    Local(PathBuf),
    GitHub { repo: String, r#ref: String },
}

impl Source {
    pub fn local(path: impl Into<PathBuf>) -> Self {
        Self::Local(path.into())
    }

    pub fn github(repo: impl Into<String>, r#ref: impl Into<String>) -> Self {
        Self::GitHub {
            repo: repo.into(),
            r#ref: r#ref.into(),
        }
    }

    /// Human-readable label for logs.
    pub fn label(&self) -> String {
        match self {
            Self::Local(p) => format!("local {}", p.display()),
            Self::GitHub { repo, r#ref } => format!("github:{repo}@{ref}", ref = r#ref),
        }
    }

    /// Fetch a repo-relative path as UTF-8 text.
    pub fn fetch_text(&self, rel: &str) -> Result<String> {
        let rel = rel.trim_start_matches("./");
        match self {
            Self::Local(root) => {
                let path = root.join(rel);
                std::fs::read_to_string(&path)
                    .with_context(|| format!("failed to read {}", path.display()))
            }
            Self::GitHub { repo, r#ref } => {
                let url = format!(
                    "https://raw.githubusercontent.com/{repo}/{ref}/{rel}",
                    ref = r#ref,
                );
                let response = ureq::get(&url)
                    .call()
                    .with_context(|| format!("failed to fetch {url}"))?;
                response
                    .into_string()
                    .with_context(|| format!("failed to read body from {url}"))
            }
        }
    }

    /// Fetch and parse `registry.json`.
    pub fn fetch_registry(&self) -> Result<Registry> {
        let text = self
            .fetch_text(REGISTRY_FILE)
            .with_context(|| format!("failed to load {REGISTRY_FILE} from {}", self.label()))?;
        let reg: Registry = serde_json::from_str(&text)
            .with_context(|| format!("failed to parse {REGISTRY_FILE} from {}", self.label()))?;
        Ok(reg)
    }
}

/// Build a Source from optional `--path` and config / CLI values.
///
/// Prefer `--path` (offline local checkout). Otherwise use GitHub raw with the
/// given repo/ref, defaulting to `a1re1/rcn` @ `main`.
pub fn resolve(
    path: Option<PathBuf>,
    repo: Option<&str>,
    r#ref: Option<&str>,
) -> Result<Source> {
    if let Some(p) = path {
        if !p.exists() {
            bail!("local path does not exist: {}", p.display());
        }
        return Ok(Source::local(p));
    }
    let repo = repo.unwrap_or("a1re1/rcn");
    let r#ref = r#ref.unwrap_or("main");
    Ok(Source::github(repo, r#ref))
}

/// Convenience: require that `dir` looks like a cargo project.
pub fn require_cargo_toml(dir: &Path) -> Result<()> {
    if !dir.join("Cargo.toml").exists() {
        bail!(
            "no Cargo.toml in {} — run this inside a cargo project",
            dir.display()
        );
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::tempdir;

    #[test]
    fn local_fetch_reads_file() {
        let dir = tempdir().unwrap();
        std::fs::write(dir.path().join("hello.txt"), "hi").unwrap();
        let src = Source::local(dir.path());
        assert_eq!(src.fetch_text("hello.txt").unwrap(), "hi");
    }

    #[test]
    fn resolve_prefers_path() {
        let dir = tempdir().unwrap();
        let src = resolve(Some(dir.path().to_path_buf()), Some("x/y"), Some("z")).unwrap();
        match src {
            Source::Local(p) => assert_eq!(p, dir.path()),
            _ => panic!("expected local"),
        }
    }

    #[test]
    fn resolve_github_defaults() {
        let src = resolve(None, None, None).unwrap();
        match src {
            Source::GitHub { repo, r#ref } => {
                assert_eq!(repo, "a1re1/rcn");
                assert_eq!(r#ref, "main");
            }
            _ => panic!("expected github"),
        }
    }

    #[test]
    fn require_cargo_toml_errors() {
        let dir = tempdir().unwrap();
        let err = require_cargo_toml(dir.path()).unwrap_err().to_string();
        assert!(err.contains("Cargo.toml"), "{err}");
    }
}
