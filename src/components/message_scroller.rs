//! MessageScroller — port of shadcn base-vega `ui/message-scroller.tsx`.
//!
//! The conversation viewport: a sized scroll container stacking messages
//! with the source's gap-8 rhythm. Stick-to-bottom tracking and the
//! scroll-to-bottom button are omitted (TODO(rcn): needs a scroll handle
//! API pass).

use gpui::{
    AnyElement, App, ElementId, InteractiveElement as _, IntoElement, ParentElement, Pixels,
    RenderOnce, StatefulInteractiveElement as _, Styled, Window, div, prelude::FluentBuilder as _,
    px,
};

#[derive(IntoElement)]
pub struct MessageScroller {
    id: ElementId,
    height: Option<Pixels>,
    children: Vec<AnyElement>,
}

impl MessageScroller {
    pub fn new(id: impl Into<ElementId>) -> Self {
        Self {
            id: id.into(),
            height: None,
            children: Vec::new(),
        }
    }

    pub fn h(mut self, height: Pixels) -> Self {
        self.height = Some(height);
        self
    }
}

impl ParentElement for MessageScroller {
    fn extend(&mut self, elements: impl IntoIterator<Item = AnyElement>) {
        self.children.extend(elements);
    }
}

impl RenderOnce for MessageScroller {
    fn render(self, _window: &mut Window, _cx: &mut App) -> impl IntoElement {
        div()
            .id(self.id)
            .relative()
            .overflow_y_scroll()
            .when_some(self.height, |el, height| el.h(height))
            .child(
                // flex h-max min-h-full flex-col gap-8
                div()
                    .flex()
                    .flex_col()
                    .min_h_full()
                    .gap(px(32.))
                    .children(self.children),
            )
    }
}
