//! Spinner — port of shadcn base-vega `ui/spinner.tsx`.
//!
//! A spinning loader arc (`animate-spin size-4`). With no `.color(..)`, the
//! svg inherits ambient text color (`text-current`); `.color(..)` overrides.

use std::time::Duration;

use gpui::prelude::FluentBuilder;
use gpui::{
    Animation, AnimationExt as _, App, Hsla, IntoElement, Pixels, RenderOnce, Styled,
    Transformation, Window, percentage, px, svg,
};

use crate::assets::ICON_LOADER;

#[derive(IntoElement)]
pub struct Spinner {
    size: Pixels,
    color: Option<Hsla>,
}

impl Spinner {
    pub fn new() -> Self {
        Self {
            size: px(16.),
            color: None,
        }
    }

    pub fn size(mut self, size: Pixels) -> Self {
        self.size = size;
        self
    }

    pub fn color(mut self, color: Hsla) -> Self {
        self.color = Some(color);
        self
    }
}

impl Default for Spinner {
    fn default() -> Self {
        Self::new()
    }
}

impl RenderOnce for Spinner {
    fn render(self, _window: &mut Window, _cx: &mut App) -> impl IntoElement {
        // text-current: inherit ambient text color unless `.color(..)` overrides.
        svg()
            .path(ICON_LOADER)
            .size(self.size)
            .flex_shrink_0()
            .when_some(self.color, |this, color| this.text_color(color))
            .with_animation(
                "spinner",
                Animation::new(Duration::from_secs(1)).repeat(),
                |el, delta| el.with_transformation(Transformation::rotate(percentage(delta))),
            )
    }
}
