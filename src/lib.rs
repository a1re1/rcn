//! rcn — a copy-paste component library for gpui. The library crate exposes
//! the components and the storybook shell so multiple entry points can boot
//! it: `src/main.rs` (native macOS) and `web/` (wasm32 via gpui_web).

pub mod assets;
pub mod components;
pub mod container_query;
pub mod motion;
pub mod storybook;
pub mod storybook_docs;
pub mod theme;
