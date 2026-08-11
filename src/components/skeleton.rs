//! Skeleton — port of shadcn base-nova `ui/skeleton.tsx`.
//!
//! A rounded-md bg-muted placeholder block with animate-pulse, shown while
//! content is loading. Sizing and shape come from the caller via [`Styled`]
//! (gpui's equivalent of shadcn's `className` passthrough):
//!
//! ```ignore
//! Skeleton::new().h(px(20.)).w(px(100.)).rounded_full()
//! ```
//!
//! TODO(rcn): RTL docs example omitted — no direction/translation infra.

use gpui::{App, IntoElement, Refineable as _, RenderOnce, StyleRefinement, Styled, Window, div};

use crate::motion;
use crate::theme::Theme;

#[derive(IntoElement)]
pub struct Skeleton {
    style: StyleRefinement,
}

impl Skeleton {
    pub fn new() -> Self {
        Self {
            style: StyleRefinement::default(),
        }
    }
}

impl Default for Skeleton {
    fn default() -> Self {
        Self::new()
    }
}

impl Styled for Skeleton {
    fn style(&mut self) -> &mut StyleRefinement {
        &mut self.style
    }
}

impl RenderOnce for Skeleton {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let theme = Theme::of(cx);
        // rounded-md bg-muted; caller refinement applied on top so e.g.
        // .rounded_full() wins over the default radius_md.
        let mut base = div().bg(theme.muted).rounded(theme.radius_md());
        base.style().refine(&self.style);
        motion::pulse("skeleton-pulse", base)
    }
}
