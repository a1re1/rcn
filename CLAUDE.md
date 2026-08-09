# CLAUDE.md

rcn — a copy-paste component library for gpui (Zed's UI framework): shadcn, but
for gpui Rust apps. Early bootstrap; currently a bare-bones gpui hello-world app
verifying the toolchain and rendering pipeline. macOS only (the platform layer
comes from `gpui_platform` with the CoreText text system).

## Commands

- `cargo run` — build and launch the app.
- `cargo build` — compile without launching.

Builds compile through **sccache** (`.cargo/config.toml` sets `rustc-wrapper`),
so a fresh worktree's first build pulls the gpui dependency tree from cache
instead of recompiling it. sccache must be installed (`brew install sccache`)
or cargo fails with "could not execute process `sccache`".
