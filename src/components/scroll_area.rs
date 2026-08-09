//! ScrollArea — port of shadcn base-vega `ui/scroll-area.tsx`.
//!
//! A sized viewport that scrolls its content vertically. gpui scroll
//! containers don't paint scrollbars; the source's custom scrollbar and
//! horizontal orientation are omitted (TODO(rcn)).

use gpui::{
    AnyElement, App, ElementId, InteractiveElement as _, IntoElement, ParentElement, Pixels,
    RenderOnce, StatefulInteractiveElement as _, Styled, Window, div, prelude::FluentBuilder as _,
};

#[derive(IntoElement)]
pub struct ScrollArea {
    id: ElementId,
    height: Option<Pixels>,
    width: Option<Pixels>,
    children: Vec<AnyElement>,
}

impl ScrollArea {
    pub fn new(id: impl Into<ElementId>) -> Self {
        Self {
            id: id.into(),
            height: None,
            width: None,
            children: Vec::new(),
        }
    }

    pub fn h(mut self, height: Pixels) -> Self {
        self.height = Some(height);
        self
    }

    pub fn w(mut self, width: Pixels) -> Self {
        self.width = Some(width);
        self
    }
}

impl ParentElement for ScrollArea {
    fn extend(&mut self, elements: impl IntoIterator<Item = AnyElement>) {
        self.children.extend(elements);
    }
}

impl RenderOnce for ScrollArea {
    fn render(self, _window: &mut Window, _cx: &mut App) -> impl IntoElement {
        div()
            .id(self.id)
            .relative()
            .overflow_y_scroll()
            .when_some(self.height, |el, height| el.h(height))
            .when_some(self.width, |el, width| el.w(width))
            .children(self.children)
    }
}
