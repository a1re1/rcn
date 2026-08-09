//! InputOtp — port of shadcn base-vega `ui/input-otp.tsx`.
//!
//! One-time-password slots bound to a hidden bare [`Input`]: typing fills
//! the boxes left to right; clicking any slot focuses the hidden field.
//! The active slot (next to fill) shows the ring border while focused.
//! Paste works through the input; slot-level caret animation is omitted.

use gpui::{
    App, Entity, Focusable as _, InteractiveElement as _, IntoElement, ParentElement as _,
    RenderOnce, StatefulInteractiveElement as _, Styled, Window, div, prelude::FluentBuilder as _,
    px,
};

use crate::components::input::Input;
use crate::components::separator::Separator;
use crate::theme::{Theme, alpha};

#[derive(IntoElement)]
pub struct InputOtp {
    input: Entity<Input>,
    slots: usize,
    /// Slot count per group; a separator renders between groups.
    group: usize,
}

impl InputOtp {
    /// Wraps a bare [`Input`] entity holding the OTP value.
    pub fn new(input: Entity<Input>, slots: usize) -> Self {
        Self {
            input,
            slots: slots.max(1),
            group: usize::MAX,
        }
    }

    /// Splits slots into groups of `size` with a separator between them.
    pub fn group(mut self, size: usize) -> Self {
        self.group = size.max(1);
        self
    }
}

impl RenderOnce for InputOtp {
    fn render(self, window: &mut Window, cx: &mut App) -> impl IntoElement {
        let theme = Theme::of(cx).clone();
        let input = self.input.read(cx);
        let value: Vec<char> = input.text().chars().take(self.slots).collect();
        let focused = input.focus_handle(cx).is_focused(window);
        let active = value.len().min(self.slots - 1);
        let focus_handle = input.focus_handle(cx);

        div()
            .id("input-otp")
            .flex()
            .flex_row()
            .items_center()
            .gap(px(8.))
            .on_click(move |_, window, cx| window.focus(&focus_handle, cx))
            // The hidden editing core: zero-sized but rendered so it keeps
            // receiving input while focused.
            .child(
                div()
                    .w(px(0.))
                    .h(px(0.))
                    .overflow_hidden()
                    .child(self.input.clone()),
            )
            .children((0..self.slots).flat_map(|index| {
                let first_in_group = index % self.group == 0;
                let is_active = focused && index == active && value.len() < self.slots;
                let mut parts: Vec<gpui::AnyElement> = Vec::new();
                if index > 0 && first_in_group {
                    parts.push(
                        div()
                            .w(px(12.))
                            .flex()
                            .justify_center()
                            .child(Separator::new())
                            .into_any_element(),
                    );
                }
                // Slot: size-9 border, joined corners inside a group.
                let last_in_group = (index + 1) % self.group == 0 || index + 1 == self.slots;
                parts.push(
                    div()
                        .flex()
                        .size(px(36.))
                        .items_center()
                        .justify_center()
                        .text_size(px(14.))
                        .line_height(px(20.))
                        .text_color(theme.foreground)
                        .border_t_1()
                        .border_b_1()
                        .border_r_1()
                        .when(first_in_group, |el| el.border_l_1())
                        .border_color(if is_active { theme.ring } else { theme.input })
                        .when(theme.dark, |el| el.bg(alpha(theme.input, 0.3)))
                        .when(first_in_group, |el| el.rounded_l(theme.radius_md()))
                        .when(last_in_group, |el| el.rounded_r(theme.radius_md()))
                        .child(value.get(index).map(|c| c.to_string()).unwrap_or_default())
                        .into_any_element(),
                );
                parts
            }))
    }
}
