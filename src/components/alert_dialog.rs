//! AlertDialog — port of shadcn base-vega `ui/alert-dialog.tsx`.
//!
//! A modal that interrupts the user and expects a response: unlike
//! [`Dialog`](crate::components::Dialog), there is no close button and the
//! backdrop does not dismiss — only the Action/Cancel buttons close it.
//! Header/Title/Description/Footer share the Dialog shapes.

use gpui::{
    AnyElement, App, ElementId, InteractiveElement as _, IntoElement, ParentElement, RenderOnce,
    Styled, Window, anchored, deferred, div, point, px,
};

pub use crate::components::dialog::{
    DialogDescription as AlertDialogDescription, DialogFooter as AlertDialogFooter,
    DialogHeader as AlertDialogHeader, DialogTitle as AlertDialogTitle,
};
use crate::theme::Theme;

#[derive(IntoElement)]
pub struct AlertDialog {
    id: ElementId,
    open: bool,
    children: Vec<AnyElement>,
}

impl AlertDialog {
    pub fn new(id: impl Into<ElementId>) -> Self {
        Self {
            id: id.into(),
            open: false,
            children: Vec::new(),
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

impl RenderOnce for AlertDialog {
    fn render(self, window: &mut Window, cx: &mut App) -> impl IntoElement {
        if !self.open {
            return div().into_any_element();
        }
        let theme = Theme::of(cx).clone();
        let viewport = window.viewport_size();

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
                    .child(crate::motion::dialog_in(
                        "alert-dialog-in",
                        div()
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
                            .children(self.children),
                    )),
            ),
        )
        .into_any_element()
    }
}
