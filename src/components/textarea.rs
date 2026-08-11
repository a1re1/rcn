//! Textarea — port of shadcn base-nova `ui/textarea.tsx`.
//!
//! The shadcn textarea shell (`min-h-16 w-full rounded-lg border px-2.5 py-2`)
//! around the [`Input`](crate::components::Input) editing machinery, with the
//! native textarea's corner resize grip (vertical-only, on by default; opt
//! out with [`Textarea::resizable`]`(false)` — Tailwind `resize-none`). Editing
//! is currently single-line — TODO(rcn): wrapped multi-line shaping and
//! cursor movement; Enter inserts no newline yet. shadcn's
//! `field-sizing-content` auto-grow is also TODO(rcn).

use gpui::{
    App, AppContext as _, Context, CursorStyle, DragMoveEvent, Entity, EntityId, Focusable as _,
    InteractiveElement as _, IntoElement, ParentElement, Pixels, Render, RenderOnce,
    StatefulInteractiveElement as _, Styled, Window, div, prelude::FluentBuilder as _, px,
};

use crate::components::Icon;
use crate::components::input::Input;
use crate::theme::{Theme, alpha};

/// Drag payload for the corner resize grip; the entity id scopes the
/// group-level `on_drag_move` to the textarea that started the drag.
struct ResizeDrag {
    textarea: EntityId,
}

struct ResizeDragPreview;

impl Render for ResizeDragPreview {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        gpui::Empty
    }
}

#[derive(IntoElement)]
pub struct Textarea {
    input: Entity<Input>,
    rows: Option<u32>,
    disabled: bool,
    invalid: bool,
    resizable: bool,
}

impl Textarea {
    /// Wraps an [`Input`] entity owned by the caller.
    pub fn new(input: Entity<Input>) -> Self {
        Self {
            input,
            rows: None,
            disabled: false,
            invalid: false,
            resizable: true,
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

    /// The corner drag-to-resize grip (CSS `resize: vertical` — dragging
    /// grows the textarea's height only; width always fills the container).
    /// On by default; pass `false` for Tailwind's `resize-none`.
    // Not yet exercised by the storybook (every docs example keeps the grip).
    #[allow(dead_code)]
    pub fn resizable(mut self, resizable: bool) -> Self {
        self.resizable = resizable;
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

        // Like the native vertical resizer, a drag fixes the shell's height
        // (width always fills the container); keyed off the wrapped entity so
        // each textarea keeps its own dragged height.
        let entity_id = self.input.entity_id();
        let resize_state: Entity<Option<Pixels>> =
            window.use_keyed_state(("textarea-size", entity_id), cx, |_, _| None);
        let dragged_height = *resize_state.read(cx);

        div()
            .w_full()
            .map(|el| match dragged_height {
                Some(h) => el.h(h),
                None => el.min_h(px(min_height)),
            })
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
            // Rings are border overlays, not box shadows: gpui paints shadows
            // behind the quad, so they'd show through the transparent bg as a
            // fill (see motion::focus_ring_overlay).
            .when(self.invalid, |el| {
                el.child(crate::motion::focus_ring_overlay_destructive(
                    &theme,
                    theme.radius_lg(),
                ))
            })
            .when(show_focus, |el| {
                el.child(crate::motion::focus_ring_overlay(&theme, theme.radius_lg()))
            })
            .when(self.resizable, |el| {
                let state = resize_state.clone();
                el.on_drag_move(move |event: &DragMoveEvent<ResizeDrag>, _window, cx| {
                    if event.drag(cx).textarea != entity_id {
                        return;
                    }
                    // Vertical-only: the bottom edge tracks the pointer,
                    // clamped to the configured min-height.
                    let h =
                        f32::from(event.event.position.y - event.bounds.origin.y).max(min_height);
                    state.update(cx, |s, cx| {
                        *s = Some(px(h));
                        cx.notify();
                    });
                })
                .child(
                    div()
                        .id(("textarea-resize", entity_id))
                        .absolute()
                        .bottom(px(0.))
                        .right(px(0.))
                        .size(px(12.))
                        .cursor(CursorStyle::ResizeUpDown)
                        .child(
                            Icon::new(crate::assets::ICON_RESIZE_GRIP)
                                .size(px(12.))
                                .text_color(theme.muted_foreground),
                        )
                        .on_drag(
                            ResizeDrag {
                                textarea: entity_id,
                            },
                            |_, _, _, cx| cx.new(|_| ResizeDragPreview),
                        ),
                )
            })
    }
}
