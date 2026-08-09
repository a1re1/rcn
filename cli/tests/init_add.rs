//! Offline integration test: init + add against this repo checkout.
//!
//! Creates a temp cargo project, points `--path` at the rcn repo root
//! (`CARGO_MANIFEST_DIR/..`), and asserts vendored files, mod.rs, Cargo.toml
//! deps, and the scaffolded main.rs. No network.

use std::env;
use std::fs;
use std::path::{Path, PathBuf};
use std::sync::Mutex;

use rcn_cli::{Cli, Commands, run};
use tempfile::tempdir;

/// Serialize tests that mutate process-global cwd.
static CWD_LOCK: Mutex<()> = Mutex::new(());

fn rcn_repo_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .expect("cli/ has a parent")
        .to_path_buf()
}

fn write_hello_world_bin(project: &Path) {
    let src = project.join("src");
    fs::create_dir_all(&src).unwrap();
    fs::write(
        src.join("main.rs"),
        "fn main() {\n    println!(\"Hello, world!\");\n}\n",
    )
    .unwrap();
    fs::write(
        project.join("Cargo.toml"),
        r#"[package]
name = "demo"
version = "0.1.0"
edition = "2024"

[dependencies]
"#,
    )
    .unwrap();
}

#[test]
fn init_and_add_button_combobox_offline() {
    let _guard = CWD_LOCK.lock().unwrap_or_else(|e| e.into_inner());

    let repo = rcn_repo_root();
    assert!(
        repo.join("registry.json").is_file(),
        "registry.json missing at {}",
        repo.display()
    );

    let tmp = tempdir().unwrap();
    let project = tmp.path().join("demo");
    fs::create_dir_all(&project).unwrap();
    write_hello_world_bin(&project);

    let prev = env::current_dir().unwrap();
    env::set_current_dir(&project).unwrap();
    let result = (|| -> anyhow::Result<()> {
        run(Cli {
            command: Commands::Init {
                path: Some(repo.clone()),
                r#ref: None,
                force: false,
            },
        })?;

        run(Cli {
            command: Commands::Add {
                names: vec!["button".into(), "combobox".into()],
                all: false,
                overwrite: false,
                path: Some(repo.clone()),
            },
        })?;
        Ok(())
    })();
    let _ = env::set_current_dir(&prev);
    result.expect("init + add should succeed offline");

    // Core files vendored
    assert!(project.join("src/theme.rs").is_file());
    assert!(project.join("src/motion.rs").is_file());
    assert!(project.join("src/assets.rs").is_file());
    assert!(project.join("rcn.toml").is_file());

    // Components: button, combobox, and transitive input
    for name in ["button.rs", "combobox.rs", "input.rs"] {
        let p = project.join("src/components").join(name);
        assert!(p.is_file(), "missing vendored component {}", p.display());
    }

    // mod.rs has sorted pub mod lines
    let mod_rs = fs::read_to_string(project.join("src/components/mod.rs")).unwrap();
    let mods: Vec<&str> = mod_rs
        .lines()
        .filter(|l| l.starts_with("pub mod "))
        .collect();
    assert_eq!(
        mods,
        vec!["pub mod button;", "pub mod combobox;", "pub mod input;",],
        "mod.rs should list sorted, deduped modules:\n{mod_rs}"
    );

    // Cargo.toml gained gpui + unicode-segmentation
    let cargo = fs::read_to_string(project.join("Cargo.toml")).unwrap();
    assert!(cargo.contains("gpui"), "Cargo.toml missing gpui:\n{cargo}");
    assert!(
        cargo.contains("gpui_platform") || cargo.contains("gpui-platform"),
        "Cargo.toml missing gpui_platform:\n{cargo}"
    );
    assert!(
        cargo.contains("unicode-segmentation"),
        "Cargo.toml missing unicode-segmentation (via input):\n{cargo}"
    );

    // main.rs was scaffolded from hello-world
    let main_rs = fs::read_to_string(project.join("src/main.rs")).unwrap();
    assert!(
        main_rs.contains("mod theme")
            && main_rs.contains("mod motion")
            && main_rs.contains("mod assets")
            && main_rs.contains("mod components"),
        "main.rs should declare core mods:\n{main_rs}"
    );
    assert!(
        main_rs.contains("rcn is ready") || main_rs.contains("Application"),
        "main.rs should be a gpui scaffold, not hello-world:\n{main_rs}"
    );
    assert!(
        !main_rs.contains("Hello, world!"),
        "hello-world main should have been replaced:\n{main_rs}"
    );
}
