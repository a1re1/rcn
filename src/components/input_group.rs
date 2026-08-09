//! InputGroup — port of shadcn base-vega `ui/input-group.tsx`.
//!
//! An input-styled shell that lays addons (icons, text, buttons) around a
//! bare [`Input`](crate::components::Input): the group draws the border,
//! focus ring, and padding; the input supplies the editing core. Block
//! (multi-row) alignments are omitted.

use gpui::{
    AnyElement, App, Entity, Focusable as _, IntoElement, ParentElement as _, RenderOnce, Styled,
    Window, div, prelude::FluentBuilder as _, px,
};

use crate::components::input::Input;
use crate::theme::{Theme, alpha};

/// A leading or trailing addon cluster (muted, non-editing).
#[derive(IntoElement)]
pub struct InputGroupAddon {
    children: Vec<AnyElement>,
}

impl InputGroupAddon {
    pub fn new() -> Self {
        Self {
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

impl RenderOnce for InputGroupAddon {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let theme = Theme::of(cx);
        div()
            .flex()
            .flex_row()
            .flex_shrink_0()
            .items_center()
            .gap(px(6.))
            .text_size(px(14.))
            .line_height(px(20.))
            .text_color(theme.muted_foreground)
            .children(self.children)
    }
}

#[derive(IntoElement)]
pub struct InputGroup {
    input: Entity<Input>,
    leading: Option<AnyElement>,
    trailing: Option<AnyElement>,
}

impl InputGroup {
    /// Wraps a bare [`Input`] entity (`input.set_bare(true)`).
    pub fn new(input: Entity<Input>) -> Self {
        Self {
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

impl RenderOnce for InputGroup {
    fn render(self, window: &mut Window, cx: &mut App) -> impl IntoElement {
        let theme = Theme::of(cx).clone();
        let focused = self.input.read(cx).focus_handle(cx).is_focused(window);

        // Shell: h-9 rounded-md border shadow-xs, ring border when the inner
        // input has focus.
        div()
            .flex()
            .flex_row()
            .items_center()
            .gap(px(8.))
            .h(px(36.))
            .w_full()
            .rounded(theme.radius_md())
            .border_1()
            .border_color(if focused { theme.ring } else { theme.input })
            .when(theme.dark, |el| el.bg(alpha(theme.input, 0.3)))
            .px(px(12.))
            .shadow_xs()
            .children(self.leading)
            .child(div().flex_1().child(self.input))
            .children(self.trailing)
    }
}
