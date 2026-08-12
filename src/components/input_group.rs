//! InputGroup — port of shadcn `ui/input-group.tsx` (shell chrome tracks
//! base-nova: h-8 rounded-lg border, no resting shadow).
//!
//! An input-styled shell that lays addons (icons, text, buttons) around a
//! bare [`Input`](crate::components::Input): the group draws the border,
//! focus ring, and padding; the input supplies the editing core. Block
//! (multi-row) alignments are omitted.
//!
//! Sizing and shape overrides come from the caller via [`Styled`].

use gpui::{
    AnyElement, App, Entity, Focusable as _, IntoElement, ParentElement as _, Refineable as _,
    RenderOnce, StyleRefinement, Styled, Window, div, prelude::FluentBuilder as _, px,
};

use crate::components::input::Input;
use crate::theme::{Theme, alpha};

/// A leading or trailing addon cluster (muted, non-editing).
///
/// Sizing and shape overrides come from the caller via [`Styled`].
#[derive(IntoElement)]
pub struct InputGroupAddon {
    style: StyleRefinement,
    children: Vec<AnyElement>,
}

impl InputGroupAddon {
    pub fn new() -> Self {
        Self {
            style: StyleRefinement::default(),
            children: Vec::new(),
        }
    }

    pub fn child(mut self, child: impl IntoElement) -> Self {
        self.children.push(child.into_any_element());
        self
    }
}

impl Default for InputGroupAddon {
    fn default() -> Self {
        Self::new()
    }
}

impl Styled for InputGroupAddon {
    fn style(&mut self) -> &mut StyleRefinement {
        &mut self.style
    }
}

impl RenderOnce for InputGroupAddon {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let theme = Theme::of(cx);
        let mut root = div()
            .flex()
            .flex_row()
            .flex_shrink_0()
            .items_center()
            .gap(px(6.))
            .text_size(px(14.))
            .line_height(px(20.))
            .text_color(theme.muted_foreground)
            .children(self.children);
        root.style().refine(&self.style);
        root
    }
}

/// Input-styled shell around a bare [`Input`].
///
/// Sizing and shape overrides come from the caller via [`Styled`].
#[derive(IntoElement)]
pub struct InputGroup {
    style: StyleRefinement,
    input: Entity<Input>,
    leading: Option<AnyElement>,
    trailing: Option<AnyElement>,
}

impl InputGroup {
    /// Wraps a bare [`Input`] entity (`input.set_bare(true)`).
    pub fn new(input: Entity<Input>) -> Self {
        Self {
            style: StyleRefinement::default(),
            input,
            leading: None,
            trailing: None,
        }
    }

    pub fn leading(mut self, addon: impl IntoElement) -> Self {
        self.leading = Some(addon.into_any_element());
        self
    }

    pub fn trailing(mut self, addon: impl IntoElement) -> Self {
        self.trailing = Some(addon.into_any_element());
        self
    }
}

impl Styled for InputGroup {
    fn style(&mut self) -> &mut StyleRefinement {
        &mut self.style
    }
}

impl RenderOnce for InputGroup {
    fn render(self, window: &mut Window, cx: &mut App) -> impl IntoElement {
        let theme = Theme::of(cx).clone();
        let focused = self.input.read(cx).focus_handle(cx).is_focused(window);

        // Shell: base-nova h-8 rounded-lg border (no resting shadow), ring
        // border while the inner input has focus. The ring is a border
        // overlay, not a box shadow — gpui paints shadows behind the quad,
        // so they show through transparent backgrounds as a fill (see
        // motion::focus_ring_overlay).
        let mut root = div()
            .flex()
            .flex_row()
            .items_center()
            .gap(px(8.))
            .h(px(32.))
            .w_full()
            .rounded(theme.radius_lg())
            .border_1()
            .border_color(if focused { theme.ring } else { theme.input })
            .when(theme.dark, |el| el.bg(alpha(theme.input, 0.3)))
            .px(px(12.))
            .children(self.leading)
            .child(div().flex_1().child(self.input))
            .children(self.trailing)
            .when(focused, |el| {
                el.child(crate::motion::focus_ring_overlay(&theme, theme.radius_lg()))
            });
        root.style().refine(&self.style);
        root
    }
}
