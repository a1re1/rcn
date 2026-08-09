//! Consumer `rcn.toml` load/save.

use anyhow::{Context, Result, bail};
use serde::{Deserialize, Serialize};
use std::path::{Path, PathBuf};

pub const CONFIG_FILE: &str = "rcn.toml";

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct Config {
    pub source: SourceSection,
    pub paths: Paths,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct SourceSection {
    /// GitHub owner/repo, e.g. "a1re1/rcn"
    pub repo: String,
    /// Git ref for raw fetches
    #[serde(rename = "ref")]
    pub r#ref: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct Paths {
    pub components: String,
    pub theme: String,
    pub motion: String,
    pub assets: String,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            source: SourceSection {
                repo: "a1re1/rcn".into(),
                r#ref: "main".into(),
            },
            paths: Paths {
                components: "src/components".into(),
                theme: "src/theme.rs".into(),
                motion: "src/motion.rs".into(),
                assets: "src/assets.rs".into(),
            },
        }
    }
}

impl Config {
    /// Build a default config, optionally overriding the git ref.
    pub fn with_ref(r#ref: impl Into<String>) -> Self {
        let mut cfg = Self::default();
        cfg.source.r#ref = r#ref.into();
        cfg
    }
}

/// Load `rcn.toml` from `dir`, or error with a friendly init hint.
pub fn load(dir: &Path) -> Result<Config> {
    let path = dir.join(CONFIG_FILE);
    if !path.exists() {
        bail!("no {CONFIG_FILE} found — run `rcn init` first");
    }
    let text = std::fs::read_to_string(&path)
        .with_context(|| format!("failed to read {}", path.display()))?;
    let cfg: Config = toml_edit::de::from_str(&text)
        .map_err(|e| anyhow::anyhow!("failed to parse {CONFIG_FILE}: {e}"))?;
    Ok(cfg)
}

/// Write `rcn.toml` into `dir` with stable, hand-editable formatting.
pub fn save(dir: &Path, cfg: &Config) -> Result<PathBuf> {
    let path = dir.join(CONFIG_FILE);
    let text = format_config(cfg);
    std::fs::write(&path, text).with_context(|| format!("failed to write {}", path.display()))?;
    Ok(path)
}

/// Render `rcn.toml` as a stable TOML document matching the documented shape.
pub fn format_config(cfg: &Config) -> String {
    format!(
        r#"[source]
repo = "{repo}"
ref = "{ref}"

[paths]
components = "{components}"
theme = "{theme}"
motion = "{motion}"
assets = "{assets}"
"#,
        repo = cfg.source.repo,
        ref = cfg.source.r#ref,
        components = cfg.paths.components,
        theme = cfg.paths.theme,
        motion = cfg.paths.motion,
        assets = cfg.paths.assets,
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::tempdir;

    #[test]
    fn load_missing_hints_init() {
        let dir = tempdir().unwrap();
        let err = load(dir.path()).unwrap_err().to_string();
        assert!(err.contains("rcn init"), "{err}");
    }

    #[test]
    fn save_roundtrip() {
        let dir = tempdir().unwrap();
        let cfg = Config::with_ref("v1");
        save(dir.path(), &cfg).unwrap();
        let loaded = load(dir.path()).unwrap();
        assert_eq!(loaded, cfg);
        let text = std::fs::read_to_string(dir.path().join(CONFIG_FILE)).unwrap();
        assert!(text.contains("[source]"));
        assert!(text.contains("repo = \"a1re1/rcn\""));
        assert!(text.contains("ref = \"v1\""));
        assert!(text.contains("components = \"src/components\""));
    }
}
