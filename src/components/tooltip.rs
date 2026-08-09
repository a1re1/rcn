//! Tooltip — port of shadcn base-vega `ui/tooltip.tsx`.
//!
//! Wraps a trigger element with gpui's native hover-tooltip machinery; the
//! panel matches the source's primary-inverted bubble (`bg-primary
//! text-primary-foreground rounded-md px-3 py-1.5 text-xs`). The pointing
//! arrow and open/close animations are omitted.

use gpui::{
    AnyElement, App, AppContext as _, Context, ElementId, InteractiveElement as _, IntoElement,
    ParentElement, Render, RenderOnce, SharedString, StatefulInteractiveElement as _, Styled,
    Window, div, px,
};

use crate::theme::Theme;

/// The tooltip bubble view given to gpui's tooltip machinery.
pub struct TooltipView {
    text: SharedString,
}

impl TooltipView {
    pub fn new(text: impl Into<SharedString>) -> Self {
        Self { text: text.into() }
    }
}

impl Render for TooltipView {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let theme = Theme::of(cx);
        // Wrapped in a margin container so the bubble floats a step away
        // from the trigger (sideOffset).
        div().m(px(4.)).child(
            div()
                .rounded(theme.radius_md())
                .bg(theme.primary)
                .text_color(theme.primary_foreground)
                .px(px(12.))
                .py(px(6.))
                .text_size(px(12.))
                .line_height(px(16.))
                .child(self.text.clone()),
        )
    }
}

/// A hover-tooltip wrapper around a trigger element.
#[derive(IntoElement)]
pub struct Tooltip {
    id: ElementId,
    text: SharedString,
    children: Vec<AnyElement>,
}

impl Tooltip {
    pub fn new(id: impl Into<ElementId>, text: impl Into<SharedString>) -> Self {
        Self {
            id: id.into(),
            text: text.into(),
            children: Vec::new(),
        }
    }
}

impl ParentElement for Tooltip {
    fn extend(&mut self, elements: impl IntoIterator<Item = AnyElement>) {
        self.children.extend(elements);
    }
}

impl RenderOnce for Tooltip {
    fn render(self, _window: &mut Window, _cx: &mut App) -> impl IntoElement {
        let text = self.text.clone();
        div()
            .id(self.id)
            .tooltip(move |_, cx| cx.new(|_| TooltipView::new(text.clone())).into())
            .children(self.children)
    }
}
