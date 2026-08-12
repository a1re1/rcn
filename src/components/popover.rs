//! Popover — port of shadcn base-vega `ui/popover.tsx`.
//!
//! Controlled: the caller owns `open` and receives the next value in
//! `on_open_change` (trigger clicks toggle it; clicking outside the panel
//! closes it). The panel renders through `deferred(anchored(...))` so it
//! paints above surrounding content and snaps inside the window, centered
//! below the trigger with a 4px offset like the source's default
//! `side="bottom" align="center" sideOffset={4}`.
//! Open/close animations are omitted.
//!
//! Sizing and shape overrides come from the caller via [`Styled`] and apply
//! to the floating popover panel (the element carrying background, border, and
//! shadow), not the trigger wrapper.

use std::rc::Rc;

use gpui::{
    Anchor, AnyElement, App, ElementId, FontWeight, InteractiveElement as _, IntoElement,
    ParentElement, Refineable as _, RenderOnce, StatefulInteractiveElement as _, StyleRefinement,
    Styled, Window, anchored, deferred, div, point, prelude::FluentBuilder as _, px,
};

use crate::theme::{Theme, alpha};

type OpenChangeHandler = Rc<dyn Fn(&bool, &mut Window, &mut App) + 'static>;

/// Floating popover anchored below a trigger.
///
/// Sizing and shape overrides via [`Styled`] target the floating panel root
/// (bg/border/shadow), not the trigger.
#[derive(IntoElement)]
pub struct Popover {
    id: ElementId,
    trigger: Option<AnyElement>,
    content: Option<AnyElement>,
    open: bool,
    on_open_change: Option<OpenChangeHandler>,
    style: StyleRefinement,
}

impl Popover {
    pub fn new(id: impl Into<ElementId>) -> Self {
        Self {
            id: id.into(),
            trigger: None,
            content: None,
            open: false,
            on_open_change: None,
            style: StyleRefinement::default(),
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

impl Styled for Popover {
    fn style(&mut self) -> &mut StyleRefinement {
        &mut self.style
    }
}

impl RenderOnce for Popover {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let theme = Theme::of(cx).clone();
        let open = self.open;
        let toggle = self.on_open_change.clone();
        let close = self.on_open_change;
        let style = self.style;

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
                //
                // w-72 rounded-3xl bg-popover p-4 text-sm text-popover-foreground
                // shadow-md ring-1 ring-foreground/10, flex-col gap-4
                let mut panel = div()
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
                        el.on_mouse_down_out(move |_, window, cx| close(&false, window, cx))
                    })
                    .children(self.content);
                // Caller styles win over panel defaults (floating panel, not trigger).
                panel.style().refine(&style);
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
                                .child(crate::motion::pop_in("popover-in", panel)),
                        )),
                )
            })
    }
}

/// flex flex-col gap-1 text-sm
///
/// Sizing and shape overrides come from the caller via [`Styled`].
#[derive(IntoElement)]
pub struct PopoverHeader {
    children: Vec<AnyElement>,
    style: StyleRefinement,
}

impl PopoverHeader {
    pub fn new() -> Self {
        Self {
            children: Vec::new(),
            style: StyleRefinement::default(),
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

impl Styled for PopoverHeader {
    fn style(&mut self) -> &mut StyleRefinement {
        &mut self.style
    }
}

impl RenderOnce for PopoverHeader {
    fn render(self, _window: &mut Window, _cx: &mut App) -> impl IntoElement {
        let mut root = div().flex().flex_col().gap(px(4.)).children(self.children);
        root.style().refine(&self.style);
        root
    }
}

/// font-medium
///
/// Sizing and shape overrides come from the caller via [`Styled`].
#[derive(IntoElement)]
pub struct PopoverTitle {
    children: Vec<AnyElement>,
    style: StyleRefinement,
}

impl PopoverTitle {
    pub fn new() -> Self {
        Self {
            children: Vec::new(),
            style: StyleRefinement::default(),
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

impl Styled for PopoverTitle {
    fn style(&mut self) -> &mut StyleRefinement {
        &mut self.style
    }
}

impl RenderOnce for PopoverTitle {
    fn render(self, _window: &mut Window, _cx: &mut App) -> impl IntoElement {
        let mut root = div()
            .font_weight(FontWeight::MEDIUM)
            .children(self.children);
        root.style().refine(&self.style);
        root
    }
}

/// text-muted-foreground
///
/// Sizing and shape overrides come from the caller via [`Styled`].
#[derive(IntoElement)]
pub struct PopoverDescription {
    children: Vec<AnyElement>,
    style: StyleRefinement,
}

impl PopoverDescription {
    pub fn new() -> Self {
        Self {
            children: Vec::new(),
            style: StyleRefinement::default(),
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

impl Styled for PopoverDescription {
    fn style(&mut self) -> &mut StyleRefinement {
        &mut self.style
    }
}

impl RenderOnce for PopoverDescription {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let theme = Theme::of(cx);
        let mut root = div()
            .text_color(theme.muted_foreground)
            .children(self.children);
        root.style().refine(&self.style);
        root
    }
}
