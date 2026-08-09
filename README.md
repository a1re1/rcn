# rcn

A copy-paste component library for [gpui](https://github.com/zed-industries/zed/tree/main/crates/gpui) Rust apps — shadcn, but for gpui.

## Vision

- **Storybook-style viewer** — a gpui app that renders every component in the library so you can browse, inspect, and interact with them.
- **Copy-paste components** — components live in this repo as source. You don't install them as a crate; you vendor them into your project and tailor them to your use case.
- **CLI** — resolve and copy components straight from GitHub into your project, adding any required dependency crates:

  ```sh
  rcn add button
  ```

## Status

Early bootstrap. Currently a bare-bones gpui hello-world app verifying the toolchain and rendering pipeline.

## Development

```sh
cargo run
```

Requires macOS (gpui's platform layer is built via `gpui_platform` with the CoreText text system).
