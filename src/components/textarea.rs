//! Textarea — port of shadcn base-nova `ui/textarea.tsx`.
//!
//! The shadcn textarea shell (`min-h-16 w-full rounded-lg border px-2.5 py-2`)
//! around the [`Input`](crate::components::Input) editing machinery. Editing
//! is currently single-line — TODO(rcn): wrapped multi-line shaping and
//! cursor movement; Enter inserts no newline yet. shadcn's
//! `field-sizing-content` auto-grow is also TODO(rcn).

use gpui::{
    App, Entity, Focusable as _, IntoElement, ParentElement, RenderOnce, Styled, Window, div,
    prelude::FluentBuilder as _, px,
};

use crate::components::input::Input;
use crate::theme::{Theme, alpha};

#[derive(IntoElement)]
pub struct Textarea {
    input: Entity<Input>,
    rows: Option<u32>,
    disabled: bool,
    invalid: bool,
}

impl Textarea {
    /// Wraps an [`Input`] entity owned by the caller.
    pub fn new(input: Entity<Input>) -> Self {
        Self {
            input,
            rows: None,
            disabled: false,
            invalid: false,
        }
    }

    /// Minimum visible rows (drives the shell's min-height).
    ///
    /// When unset, the shell uses shadcn's `min-h-16` (64px). When set, min
    /// height is `20 * rows + 16` px.
    pub fn rows(mut self, rows: u32) -> Self {
        self.rows = Some(rows.max(1));
        self
    }

    /// Disabled shell styling (opacity 0.5 + muted input background).
    ///
    /// shadcn's `cursor-not-allowed` is omitted (no gpui equivalent worth
    /// faking). The caller is responsible for also calling
    /// [`Input::set_disabled`]`(true)` on the wrapped entity so the field is
    /// unfocusable/uneditable.
    pub fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }

    /// Invalid shell styling (destructive border + always-on destructive ring).
    ///
    /// Wins over the focused ring/border: when invalid, the destructive
    /// chrome is shown whether focused or not.
    pub fn invalid(mut self, invalid: bool) -> Self {
        self.invalid = invalid;
        self
    }
}

impl RenderOnce for Textarea {
    fn render(self, window: &mut Window, cx: &mut App) -> impl IntoElement {
        let theme = Theme::of(cx).clone();
        let focused = self.input.read(cx).focus_handle(cx).is_focused(window);
        let min_height = match self.rows {
            Some(n) => 20. * n as f32 + 16.,
            None => 64.,
        };
        // Focus chrome is suppressed while disabled; invalid wins over focus.
        let show_focus = focused && !self.disabled && !self.invalid;

        div()
            .w_full()
            .min_h(px(min_height))
            .rounded(theme.radius_lg())
            .border_1()
            .border_color(if self.invalid {
                if theme.dark {
                    alpha(theme.destructive, 0.5)
                } else {
                    theme.destructive
                }
            } else if show_focus {
                theme.ring
            } else {
                theme.input
            })
            .when(self.invalid, |el| {
                el.shadow(crate::motion::focus_ring_destructive(&theme))
            })
            .when(show_focus, |el| {
                el.shadow(crate::motion::focus_ring(&theme))
            })
            .map(|el| {
                if self.disabled {
                    if theme.dark {
                        el.bg(alpha(theme.input, 0.8))
                    } else {
                        el.bg(alpha(theme.input, 0.5))
                    }
                } else if theme.dark {
                    el.bg(alpha(theme.input, 0.3))
                } else {
                    el
                }
            })
            .when(self.disabled, |el| el.opacity(0.5))
            .px(px(10.))
            .py(px(8.))
            .text_size(px(14.))
            .line_height(px(20.))
            .text_color(theme.foreground)
            .child(self.input)
    }
}
