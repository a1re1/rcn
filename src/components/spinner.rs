//! Spinner — port of shadcn base-vega `ui/spinner.tsx`.
//!
//! A spinning loader arc (`animate-spin size-4`), colored by `.color(..)`
//! (defaults to the foreground token, standing in for `text-current`).

use std::time::Duration;

use gpui::{
    Animation, AnimationExt as _, App, Hsla, IntoElement, Pixels, RenderOnce, Styled,
    Transformation, Window, percentage, px, svg,
};

use crate::assets::ICON_LOADER;
use crate::theme::Theme;

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
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let theme = Theme::of(cx);
        let color = self.color.unwrap_or(theme.foreground);
        svg()
            .path(ICON_LOADER)
            .size(self.size)
            .flex_shrink_0()
            .text_color(color)
            .with_animation(
                "spinner",
                Animation::new(Duration::from_secs(1)).repeat(),
                |el, delta| el.with_transformation(Transformation::rotate(percentage(delta))),
            )
    }
}
