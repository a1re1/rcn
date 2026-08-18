//! Separator — port of shadcn base-vega `ui/separator.tsx`.
//!
//! A 1px border-colored rule; horizontal fills the row, vertical stretches
//! to its flex container's height. Sizing and shape overrides come from the
//! caller via [`Styled`].

use gpui::{
    App, IntoElement, Refineable as _, RenderOnce, StyleRefinement, Styled, Window, div, px,
};

use crate::theme::Theme;

#[derive(Clone, Copy, PartialEq, Eq, Default, Debug)]
pub enum SeparatorOrientation {
    #[default]
    Horizontal,
    Vertical,
}

#[derive(IntoElement)]
pub struct Separator {
    orientation: SeparatorOrientation,
    style: StyleRefinement,
}

impl Separator {
    pub fn new() -> Self {
        Self {
            orientation: SeparatorOrientation::default(),
            style: StyleRefinement::default(),
        }
    }

    pub fn orientation(mut self, orientation: SeparatorOrientation) -> Self {
        self.orientation = orientation;
        self
    }

    pub fn vertical() -> Self {
        Self::new().orientation(SeparatorOrientation::Vertical)
    }
}

impl Default for Separator {
    fn default() -> Self {
        Self::new()
    }
}

impl Styled for Separator {
    fn style(&mut self) -> &mut StyleRefinement {
        &mut self.style
    }
}

impl RenderOnce for Separator {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let theme = Theme::of(cx);
        // shrink-0 bg-border; data-horizontal:h-px w-full,
        // data-vertical:w-px self-stretch
        let base = div().flex_shrink_0().bg(theme.border);
        let mut root = match self.orientation {
            SeparatorOrientation::Horizontal => base.h(px(1.)).w_full(),
            SeparatorOrientation::Vertical => base.w(px(1.)).self_stretch(),
        };
        root.style().refine(&self.style);
        root
    }
}
