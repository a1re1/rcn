//! MessageScroller — port of shadcn base-vega `ui/message-scroller.tsx`.
//!
//! The conversation viewport: a sized scroll container stacking messages
//! with the source's gap-8 rhythm. Stick-to-bottom tracking and the
//! scroll-to-bottom button are omitted (TODO(rcn): needs a scroll handle
//! API pass). Sizing and shape overrides come from the caller via [`Styled`].

use gpui::{
    AnyElement, App, ElementId, InteractiveElement as _, IntoElement, ParentElement, Pixels,
    Refineable as _, RenderOnce, StatefulInteractiveElement as _, StyleRefinement, Styled, Window,
    div, prelude::FluentBuilder as _, px,
};

/// Conversation viewport scroll container.
///
/// Sizing and shape overrides come from the caller via [`Styled`].
#[derive(IntoElement)]
pub struct MessageScroller {
    id: ElementId,
    height: Option<Pixels>,
    children: Vec<AnyElement>,
    style: StyleRefinement,
}

impl MessageScroller {
    pub fn new(id: impl Into<ElementId>) -> Self {
        Self {
            id: id.into(),
            height: None,
            children: Vec::new(),
            style: StyleRefinement::default(),
        }
    }

    pub fn h(mut self, height: Pixels) -> Self {
        self.height = Some(height);
        self
    }
}

impl Styled for MessageScroller {
    fn style(&mut self) -> &mut StyleRefinement {
        &mut self.style
    }
}

impl ParentElement for MessageScroller {
    fn extend(&mut self, elements: impl IntoIterator<Item = AnyElement>) {
        self.children.extend(elements);
    }
}

impl RenderOnce for MessageScroller {
    fn render(self, _window: &mut Window, _cx: &mut App) -> impl IntoElement {
        let mut root = div()
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
            );
        root.style().refine(&self.style);
        root
    }
}
