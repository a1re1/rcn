//! Dialog — port of shadcn base-vega `ui/dialog.tsx`.
//!
//! A modal window over a dimmed backdrop, centered in the viewport.
//! Controlled: the caller owns `open` and receives changes via
//! `on_open_change` (backdrop click and the close button both close).
//! Open/close animations are omitted.

use std::rc::Rc;

use gpui::{
    AnyElement, App, ElementId, FontWeight, InteractiveElement as _, IntoElement, ParentElement,
    RenderOnce, StatefulInteractiveElement as _, Styled, Window, anchored, deferred, div, point,
    prelude::FluentBuilder as _, px, svg,
};

use crate::theme::{Theme, alpha};

pub type OpenChangeHandler = Rc<dyn Fn(&bool, &mut Window, &mut App) + 'static>;

/// The modal root: renders nothing until `open`; then a dimmed backdrop
/// with the content panel centered.
#[derive(IntoElement)]
pub struct Dialog {
    id: ElementId,
    open: bool,
    on_open_change: Option<OpenChangeHandler>,
    children: Vec<AnyElement>,
}

impl Dialog {
    pub fn new(id: impl Into<ElementId>) -> Self {
        Self {
            id: id.into(),
            open: false,
            on_open_change: None,
            children: Vec::new(),
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
        self.on_open_change = Some(Rc::new(handler));
        self
    }
}

impl ParentElement for Dialog {
    fn extend(&mut self, elements: impl IntoIterator<Item = AnyElement>) {
        self.children.extend(elements);
    }
}

impl RenderOnce for Dialog {
    fn render(self, window: &mut Window, cx: &mut App) -> impl IntoElement {
        if !self.open {
            return div().into_any_element();
        }
        let theme = Theme::of(cx).clone();
        let viewport = window.viewport_size();
        let close = self.on_open_change.clone();
        let close_button = self.on_open_change;

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
                    // Overlay: bg-black/50
                    .bg(gpui::hsla(0., 0., 0., 0.5))
                    .when_some(close, |el, close| {
                        el.on_click(move |_, window, cx| close(&false, window, cx))
                    })
                    .child(crate::motion::dialog_in(
                        "dialog-in",
                        // Content: bg-background rounded-lg border p-6 shadow-lg,
                        // w-full max-w-[calc(100%-2rem)] sm:max-w-lg
                        div()
                            .id("dialog-content")
                            .occlude()
                            .relative()
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
                            .children(self.children)
                            // Close: absolute top-4 right-4, muted x
                            .when_some(close_button, |el, close| {
                                el.child(
                                    div()
                                        .id("dialog-close")
                                        .absolute()
                                        .top(px(16.))
                                        .right(px(16.))
                                        .rounded(theme.radius_sm())
                                        .p(px(2.))
                                        .hover(|s| s.bg(alpha(theme.muted, 0.8)))
                                        .on_click(move |_, window, cx| close(&false, window, cx))
                                        .child(
                                            svg()
                                                .path(theme.icons.x())
                                                .size(px(16.))
                                                .text_color(theme.muted_foreground),
                                        ),
                                )
                            }),
                    )),
            ),
        )
        .into_any_element()
    }
}

/// flex flex-col gap-2 text-center sm:text-left.
#[derive(IntoElement)]
pub struct DialogHeader {
    children: Vec<AnyElement>,
}

impl DialogHeader {
    pub fn new() -> Self {
        Self {
            children: Vec::new(),
        }
    }
}

impl Default for DialogHeader {
    fn default() -> Self {
        Self::new()
    }
}

impl ParentElement for DialogHeader {
    fn extend(&mut self, elements: impl IntoIterator<Item = AnyElement>) {
        self.children.extend(elements);
    }
}

impl RenderOnce for DialogHeader {
    fn render(self, _window: &mut Window, _cx: &mut App) -> impl IntoElement {
        div().flex().flex_col().gap(px(8.)).children(self.children)
    }
}

/// text-lg leading-none font-semibold (heading font).
#[derive(IntoElement)]
pub struct DialogTitle {
    children: Vec<AnyElement>,
}

impl DialogTitle {
    pub fn new() -> Self {
        Self {
            children: Vec::new(),
        }
    }
}

impl Default for DialogTitle {
    fn default() -> Self {
        Self::new()
    }
}

impl ParentElement for DialogTitle {
    fn extend(&mut self, elements: impl IntoIterator<Item = AnyElement>) {
        self.children.extend(elements);
    }
}

impl RenderOnce for DialogTitle {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let theme = Theme::of(cx);
        div()
            .text_size(px(18.))
            .line_height(px(22.))
            .font_weight(FontWeight::SEMIBOLD)
            .when_some(theme.heading_font(), |el, font| el.font_family(font))
            .children(self.children)
    }
}

/// text-sm text-muted-foreground.
#[derive(IntoElement)]
pub struct DialogDescription {
    children: Vec<AnyElement>,
}

impl DialogDescription {
    pub fn new() -> Self {
        Self {
            children: Vec::new(),
        }
    }
}

impl Default for DialogDescription {
    fn default() -> Self {
        Self::new()
    }
}

impl ParentElement for DialogDescription {
    fn extend(&mut self, elements: impl IntoIterator<Item = AnyElement>) {
        self.children.extend(elements);
    }
}

impl RenderOnce for DialogDescription {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let theme = Theme::of(cx);
        div()
            .text_size(px(14.))
            .line_height(px(20.))
            .text_color(theme.muted_foreground)
            .children(self.children)
    }
}

/// flex justify-end gap-2 (row on desktop).
#[derive(IntoElement)]
pub struct DialogFooter {
    children: Vec<AnyElement>,
}

impl DialogFooter {
    pub fn new() -> Self {
        Self {
            children: Vec::new(),
        }
    }
}

impl Default for DialogFooter {
    fn default() -> Self {
        Self::new()
    }
}

impl ParentElement for DialogFooter {
    fn extend(&mut self, elements: impl IntoIterator<Item = AnyElement>) {
        self.children.extend(elements);
    }
}

impl RenderOnce for DialogFooter {
    fn render(self, _window: &mut Window, _cx: &mut App) -> impl IntoElement {
        div()
            .flex()
            .flex_row()
            .justify_end()
            .gap(px(8.))
            .children(self.children)
    }
}
