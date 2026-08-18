//! Drawer — port of shadcn base-vega `ui/drawer.tsx` (vaul upstream).
//!
//! A bottom panel with rounded top corners and a grab handle, over a
//! dimmed backdrop. Controlled like Dialog; the drag-to-dismiss gesture
//! and snap points are omitted — backdrop click dismisses.
//!
//! Sizing and shape overrides come from the caller via [`Styled`] and apply
//! to the floating panel (the element carrying background, border, and shadow),
//! not the full-viewport backdrop.

use gpui::{
    AnyElement, App, ElementId, InteractiveElement as _, IntoElement, ParentElement,
    Refineable as _, RenderOnce, StatefulInteractiveElement as _, StyleRefinement, Styled, Window,
    anchored, deferred, div, point, prelude::FluentBuilder as _, px,
};

use crate::components::dialog::OpenChangeHandler;
pub use crate::components::dialog::{
    DialogDescription as DrawerDescription, DialogFooter as DrawerFooter,
    DialogTitle as DrawerTitle,
};
use crate::theme::Theme;

/// Bottom drawer surface. Sizing and shape overrides via [`Styled`] target the
/// floating panel root (bg/border/shadow), not the backdrop.
#[derive(IntoElement)]
pub struct Drawer {
    id: ElementId,
    open: bool,
    on_open_change: Option<OpenChangeHandler>,
    children: Vec<AnyElement>,
    style: StyleRefinement,
}

impl Drawer {
    pub fn new(id: impl Into<ElementId>) -> Self {
        Self {
            id: id.into(),
            open: false,
            on_open_change: None,
            children: Vec::new(),
            style: StyleRefinement::default(),
        }
    }

    pub fn open(mut self, open: bool) -> Self {
        self.open = open;
        self
    }

    pub fn on_open_change(
        mut self,
        handler: impl Fn(&bool, &mut Window, &mut App) + 'static,
    ) -> Self {
        self.on_open_change = Some(std::rc::Rc::new(handler));
        self
    }
}

impl ParentElement for Drawer {
    fn extend(&mut self, elements: impl IntoIterator<Item = AnyElement>) {
        self.children.extend(elements);
    }
}

impl Styled for Drawer {
    fn style(&mut self) -> &mut StyleRefinement {
        &mut self.style
    }
}

impl RenderOnce for Drawer {
    fn render(self, window: &mut Window, cx: &mut App) -> impl IntoElement {
        if !self.open {
            return div().into_any_element();
        }
        let theme = Theme::of(cx).clone();
        let viewport = window.viewport_size();
        let close = self.on_open_change;

        // Content: rounded-t-lg border-t bg-background, with the
        // vaul grab handle centered on top.
        let mut panel = div()
            .occlude()
            .flex()
            .flex_col()
            .gap(px(16.))
            .w_full()
            .max_h(viewport.height * 0.8)
            .rounded_t(theme.radius_lg())
            .border_t_1()
            .border_color(theme.border)
            .bg(theme.background)
            .p(px(16.))
            .text_size(px(14.))
            .line_height(px(20.))
            .text_color(theme.foreground)
            .child(
                div()
                    .mx_auto()
                    .mt(px(4.))
                    .h(px(8.))
                    .w(px(100.))
                    .rounded_full()
                    .bg(theme.muted),
            )
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
                    .flex_col()
                    .justify_end()
                    .bg(gpui::hsla(0., 0., 0., 0.5))
                    .when_some(close, |el, close| {
                        el.on_click(move |_, window, cx| close(&false, window, cx))
                    })
                    .child(crate::motion::slide_in("drawer-in", false, 320., panel)),
            ),
        )
        .into_any_element()
    }
}

/// flex flex-col gap-0.5 p-4 text-center — the drawer header stack.
///
/// Sizing and shape overrides come from the caller via [`Styled`].
#[derive(IntoElement)]
pub struct DrawerHeader {
    children: Vec<AnyElement>,
    style: StyleRefinement,
}

impl DrawerHeader {
    pub fn new() -> Self {
        Self {
            children: Vec::new(),
            style: StyleRefinement::default(),
        }
    }
}

impl Default for DrawerHeader {
    fn default() -> Self {
        Self::new()
    }
}

impl ParentElement for DrawerHeader {
    fn extend(&mut self, elements: impl IntoIterator<Item = AnyElement>) {
        self.children.extend(elements);
    }
}

impl Styled for DrawerHeader {
    fn style(&mut self) -> &mut StyleRefinement {
        &mut self.style
    }
}

impl RenderOnce for DrawerHeader {
    fn render(self, _window: &mut Window, _cx: &mut App) -> impl IntoElement {
        let mut root = div()
            .flex()
            .flex_col()
            .items_center()
            .gap(px(2.))
            .p(px(16.))
            .children(self.children);
        root.style().refine(&self.style);
        root
    }
}
