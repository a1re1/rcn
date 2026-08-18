//! Spinner — port of shadcn base-vega `ui/spinner.tsx`.
//!
//! A spinning loader arc (`animate-spin size-4`), drawn as an [`Icon`] so it
//! picks up the ambient text color (`text-current`) — visible inside filled
//! buttons, badges, etc.; `.color(..)` overrides. Sizing and other style
//! overrides also come from the caller via [`Styled`].

use std::time::Duration;

use gpui::prelude::FluentBuilder;
use gpui::{
    Animation, AnimationExt as _, App, Hsla, IntoElement, Pixels, Refineable as _, RenderOnce,
    StyleRefinement, Styled, Window, percentage, px,
};

use crate::assets::ICON_LOADER;
use crate::components::Icon;

#[derive(IntoElement)]
pub struct Spinner {
    size: Pixels,
    color: Option<Hsla>,
    style: StyleRefinement,
}

impl Spinner {
    pub fn new() -> Self {
        Self {
            size: px(16.),
            color: None,
            style: StyleRefinement::default(),
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

impl Styled for Spinner {
    fn style(&mut self) -> &mut StyleRefinement {
        &mut self.style
    }
}

impl RenderOnce for Spinner {
    fn render(self, _window: &mut Window, _cx: &mut App) -> impl IntoElement {
        let color = self.color;
        let mut root = Icon::new(ICON_LOADER)
            .size(self.size)
            .flex_shrink_0()
            .when_some(color, |icon, color| icon.text_color(color));
        root.style().refine(&self.style);
        root.with_animation(
            "spinner",
            Animation::new(Duration::from_secs(1)).repeat(),
            |icon, delta| icon.rotate(percentage(delta)),
        )
    }
}
