# rcn

A copy-paste component library for [gpui](https://github.com/zed-industries/zed/tree/main/crates/gpui) Rust apps — shadcn, but for gpui.

## Vision

- **Storybook-style viewer** — a gpui app that renders every component in the library so you can browse, inspect, and interact with them.
- **Copy-paste components** — components live in this repo as source. You don't install them as a crate; you vendor them into your project and tailor them to your use case.
- **CLI** — resolve and copy components straight from GitHub into your project, adding any required dependency crates. See [CLI](#cli) below.

## Status

Early bootstrap. Currently a bare-bones gpui hello-world app verifying the toolchain and rendering pipeline.

## CLI

Install the CLI:

```sh
cargo install --git https://github.com/a1re1/rcn rcn-cli
```

Then, inside a cargo project:

```sh
rcn init                  # vendor core files, write rcn.toml, pin gpui deps
rcn add button            # vendor a component (and its registry deps)
rcn add combobox input    # multiple names; kebab or snake case
rcn add --all             # vendor every component in the registry
rcn list                  # show registry components and what's installed
rcn diff button           # unified diff of local button vs registry
rcn registry build        # maintainer: scan src/components → registry.json
```

Pass `--path <local-rcn-checkout>` to `init`/`add`/`list`/`diff` to work offline against a local clone instead of GitHub raw. Use `--overwrite` with `add` to replace already-vendored files.

## Development

```sh
cargo run
```

Requires macOS (gpui's platform layer is built via `gpui_platform` with the CoreText text system).
