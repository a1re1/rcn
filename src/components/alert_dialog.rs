//! AlertDialog — port of shadcn base-vega `ui/alert-dialog.tsx`.
//!
//! A modal that interrupts the user and expects a response: unlike
//! [`Dialog`](crate::components::Dialog), there is no close button and the
//! backdrop does not dismiss — only the Action/Cancel buttons close it.
//! Header/Title/Description/Footer share the Dialog shapes.
//!
//! Sizing and shape overrides come from the caller via [`Styled`] and apply
//! to the floating panel (the element carrying background, border, and shadow),
//! not the full-viewport backdrop.

use gpui::{
    AnyElement, App, ElementId, InteractiveElement as _, IntoElement, ParentElement,
    Refineable as _, RenderOnce, StyleRefinement, Styled, Window, anchored, deferred, div, point,
    px,
};

pub use crate::components::dialog::{
    DialogDescription as AlertDialogDescription, DialogFooter as AlertDialogFooter,
    DialogHeader as AlertDialogHeader, DialogTitle as AlertDialogTitle,
};
use crate::theme::Theme;

/// Modal alert surface. Sizing and shape overrides via [`Styled`] target the
/// floating panel root (bg/border/shadow), not the backdrop.
#[derive(IntoElement)]
pub struct AlertDialog {
    id: ElementId,
    open: bool,
    children: Vec<AnyElement>,
    style: StyleRefinement,
}

impl AlertDialog {
    pub fn new(id: impl Into<ElementId>) -> Self {
        Self {
            id: id.into(),
            open: false,
            children: Vec::new(),
            style: StyleRefinement::default(),
        }
    }

    pub fn open(mut self, open: bool) -> Self {
        self.open = open;
        self
    }
}

impl ParentElement for AlertDialog {
    fn extend(&mut self, elements: impl IntoIterator<Item = AnyElement>) {
        self.children.extend(elements);
    }
}

impl Styled for AlertDialog {
    fn style(&mut self) -> &mut StyleRefinement {
        &mut self.style
    }
}

impl RenderOnce for AlertDialog {
    fn render(self, window: &mut Window, cx: &mut App) -> impl IntoElement {
        if !self.open {
            return div().into_any_element();
        }
        let theme = Theme::of(cx).clone();
        let viewport = window.viewport_size();

        let mut panel = div()
            .occlude()
            .flex()
            .flex_col()
            .gap(px(16.))
            .w(px(512.).min(viewport.width - px(32.)))
            .rounded(theme.radius_lg())
            .border_1()
            .border_color(theme.border)
            .bg(theme.background)
            .p(px(24.))
            .shadow_lg()
            .text_size(px(14.))
            .line_height(px(20.))
            .text_color(theme.foreground)
            .children(self.children);
        panel.style().refine(&self.style);

        deferred(
            anchored().position(point(px(0.), px(0.))).child(
                div()
                    .id(self.id)
                    .occlude()
                    .w(viewport.width)
                    .h(viewport.height)
                    .flex()
                    .items_center()
                    .justify_center()
                    .bg(gpui::hsla(0., 0., 0., 0.5))
                    .child(crate::motion::dialog_in("alert-dialog-in", panel)),
            ),
        )
        .into_any_element()
    }
}
