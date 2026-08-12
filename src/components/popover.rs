//! Popover — port of shadcn base-vega `ui/popover.tsx`.
//!
//! Controlled: the caller owns `open` and receives the next value in
//! `on_open_change` (trigger clicks toggle it; clicking outside the panel
//! closes it). The panel renders through `deferred(anchored(...))` so it
//! paints above surrounding content and snaps inside the window, centered
//! below the trigger with a 4px offset like the source's default
//! `side="bottom" align="center" sideOffset={4}`.
//! Open/close animations are omitted.

use std::rc::Rc;

use gpui::{
    Anchor, AnyElement, App, ElementId, FontWeight, InteractiveElement as _, IntoElement,
    ParentElement, RenderOnce, StatefulInteractiveElement as _, Styled, Window, anchored, deferred,
    div, point, prelude::FluentBuilder as _, px,
};

use crate::theme::{Theme, alpha};

type OpenChangeHandler = Rc<dyn Fn(&bool, &mut Window, &mut App) + 'static>;

#[derive(IntoElement)]
pub struct Popover {
    id: ElementId,
    trigger: Option<AnyElement>,
    content: Option<AnyElement>,
    open: bool,
    on_open_change: Option<OpenChangeHandler>,
}

impl Popover {
    pub fn new(id: impl Into<ElementId>) -> Self {
        Self {
            id: id.into(),
            trigger: None,
            content: None,
            open: false,
            on_open_change: None,
        }
    }

    /// The element that toggles the popover (any element; wrap interactivity
    /// is added here, so a plain `Button` without `on_click` works).
    pub fn trigger(mut self, trigger: impl IntoElement) -> Self {
        self.trigger = Some(trigger.into_any_element());
        self
    }

    /// The panel content shown when open.
    pub fn content(mut self, content: impl IntoElement) -> Self {
        self.content = Some(content.into_any_element());
        self
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

impl RenderOnce for Popover {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let theme = Theme::of(cx).clone();
        let open = self.open;
        let toggle = self.on_open_change.clone();
        let close = self.on_open_change;

        div()
            .relative()
            .flex()
            .flex_col()
            .items_start()
            .child(
                div()
                    .id(self.id)
                    .when_some(toggle, |el, toggle| {
                        el.on_click(move |_, window, cx| toggle(&!open, window, cx))
                    })
                    .children(self.trigger),
            )
            .when(open, |el| {
                // side="bottom" align="center" sideOffset={4}: pin a zero-size
                // wrapper at the trigger's bottom-center, then anchor the
                // panel's top-center there (same pattern as tooltip);
                // `anchored` keeps the panel inside the window on overflow.
                el.child(
                    div()
                        .absolute()
                        .left(gpui::relative(0.5))
                        .top(gpui::relative(1.))
                        .child(deferred(
                            anchored()
                                .anchor(Anchor::TopCenter)
                                .offset(point(px(0.), px(4.)))
                                .snap_to_window_with_margin(px(8.))
                                .child(crate::motion::pop_in(
                                    "popover-in",
                                    // w-72 rounded-3xl bg-popover p-4 text-sm
                                    // text-popover-foreground shadow-md ring-1
                                    // ring-foreground/10, flex-col gap-4
                                    div()
                                        .occlude()
                                        .flex()
                                        .flex_col()
                                        .gap(px(16.))
                                        .w(px(288.))
                                        .rounded(theme.radius_3xl())
                                        .bg(theme.popover)
                                        .text_color(theme.popover_foreground)
                                        .text_size(px(14.))
                                        .line_height(px(20.))
                                        .p(px(16.))
                                        .shadow_md()
                                        .border_1()
                                        .border_color(alpha(theme.foreground, 0.1))
                                        .when_some(close, |el, close| {
                                            el.on_mouse_down_out(move |_, window, cx| {
                                                close(&false, window, cx)
                                            })
                                        })
                                        .children(self.content),
                                )),
                        )),
                )
            })
    }
}

/// flex flex-col gap-1 text-sm
#[derive(IntoElement)]
pub struct PopoverHeader {
    children: Vec<AnyElement>,
}

impl PopoverHeader {
    pub fn new() -> Self {
        Self {
            children: Vec::new(),
        }
    }
}

impl Default for PopoverHeader {
    fn default() -> Self {
        Self::new()
    }
}

impl ParentElement for PopoverHeader {
    fn extend(&mut self, elements: impl IntoIterator<Item = AnyElement>) {
        self.children.extend(elements);
    }
}

impl RenderOnce for PopoverHeader {
    fn render(self, _window: &mut Window, _cx: &mut App) -> impl IntoElement {
        div().flex().flex_col().gap(px(4.)).children(self.children)
    }
}

/// font-medium
#[derive(IntoElement)]
pub struct PopoverTitle {
    children: Vec<AnyElement>,
}

impl PopoverTitle {
    pub fn new() -> Self {
        Self {
            children: Vec::new(),
        }
    }
}

impl Default for PopoverTitle {
    fn default() -> Self {
        Self::new()
    }
}

impl ParentElement for PopoverTitle {
    fn extend(&mut self, elements: impl IntoIterator<Item = AnyElement>) {
        self.children.extend(elements);
    }
}

impl RenderOnce for PopoverTitle {
    fn render(self, _window: &mut Window, _cx: &mut App) -> impl IntoElement {
        div()
            .font_weight(FontWeight::MEDIUM)
            .children(self.children)
    }
}

/// text-muted-foreground
#[derive(IntoElement)]
pub struct PopoverDescription {
    children: Vec<AnyElement>,
}

impl PopoverDescription {
    pub fn new() -> Self {
        Self {
            children: Vec::new(),
        }
    }
}

impl Default for PopoverDescription {
    fn default() -> Self {
        Self::new()
    }
}

impl ParentElement for PopoverDescription {
    fn extend(&mut self, elements: impl IntoIterator<Item = AnyElement>) {
        self.children.extend(elements);
    }
}

impl RenderOnce for PopoverDescription {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let theme = Theme::of(cx);
        div()
            .text_color(theme.muted_foreground)
            .children(self.children)
    }
}
