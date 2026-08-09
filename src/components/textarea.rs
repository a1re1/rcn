//! Textarea — port of shadcn base-vega `ui/textarea.tsx`.
//!
//! The shadcn textarea shell (`min-h-16 w-full rounded-md border px-3 py-2
//! text-sm shadow-xs`) around the [`Input`](crate::components::Input)
//! editing machinery. Editing is currently single-line — TODO(rcn):
//! wrapped multi-line shaping and cursor movement; Enter inserts no
//! newline yet.

use gpui::{
    App, Entity, Focusable as _, IntoElement, ParentElement, RenderOnce, Styled, Window, div,
    prelude::FluentBuilder as _, px,
};

use crate::components::input::Input;
use crate::theme::{Theme, alpha};

#[derive(IntoElement)]
pub struct Textarea {
    input: Entity<Input>,
    rows: u32,
}

impl Textarea {
    /// Wraps an [`Input`] entity owned by the caller.
    pub fn new(input: Entity<Input>) -> Self {
        Self { input, rows: 3 }
    }

    /// Minimum visible rows (drives the shell's min-height).
    pub fn rows(mut self, rows: u32) -> Self {
        self.rows = rows.max(1);
        self
    }
}

impl RenderOnce for Textarea {
    fn render(self, window: &mut Window, cx: &mut App) -> impl IntoElement {
        let theme = Theme::of(cx).clone();
        let focused = self.input.read(cx).focus_handle(cx).is_focused(window);
        let min_height = 20. * self.rows as f32 + 16.;

        div()
            .w_full()
            .min_h(px(min_height))
            .rounded(theme.radius_md())
            .border_1()
            .border_color(if focused { theme.ring } else { theme.input })
            .when(theme.dark, |el| el.bg(alpha(theme.input, 0.3)))
            .px(px(12.))
            .py(px(8.))
            .text_size(px(14.))
            .line_height(px(20.))
            .text_color(theme.foreground)
            .shadow_xs()
            .child(self.input)
    }
}
