//! Tooltip — port of shadcn base-vega `ui/tooltip.tsx`.
//!
//! Wraps a trigger element with gpui's native hover-tooltip machinery; the
//! panel matches the source's primary-inverted bubble (`bg-primary
//! text-primary-foreground rounded-md px-3 py-1.5 text-xs`). The pointing
//! arrow and open/close animations are omitted.
//!
//! Text-only content uses [`Tooltip::new`] / [`TooltipView::new`]. Arbitrary
//! element content (e.g. a label plus [`crate::components::Kbd`] chips) uses
//! [`Tooltip::rich`] / [`TooltipView::rich`].

use std::rc::Rc;

use gpui::{
    AnyElement, App, AppContext as _, Context, ElementId, InteractiveElement as _, IntoElement,
    ParentElement, Render, RenderOnce, SharedString, StatefulInteractiveElement as _, Styled,
    Window, div, px,
};

use crate::theme::Theme;

type RichContent = Rc<dyn Fn(&mut Window, &mut App) -> AnyElement>;

/// The tooltip bubble view given to gpui's tooltip machinery.
pub struct TooltipView {
    text: Option<SharedString>,
    content: Option<RichContent>,
}

impl TooltipView {
    pub fn new(text: impl Into<SharedString>) -> Self {
        Self {
            text: Some(text.into()),
            content: None,
        }
    }

    /// Rich element content built on each render (shadcn's `TooltipContent`
    /// accepts arbitrary children).
    pub fn rich(content: impl Fn(&mut Window, &mut App) -> AnyElement + 'static) -> Self {
        Self {
            text: None,
            content: Some(Rc::new(content)),
        }
    }
}

impl Render for TooltipView {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let theme = Theme::of(cx).clone();
        // Wrapped in a margin container so the bubble floats a step away
        // from the trigger (sideOffset).
        // Clone the Rc first so we do not hold a borrow on `self` while
        // invoking the content builder with `&mut cx`.
        let body: AnyElement = if let Some(content) = self.content.clone() {
            content(window, cx)
        } else {
            self.text.clone().unwrap_or_default().into_any_element()
        };
        div().m(px(4.)).child(
            div()
                .rounded(theme.radius_md())
                .bg(theme.primary)
                .text_color(theme.primary_foreground)
                .px(px(12.))
                .py(px(6.))
                .text_size(px(12.))
                .line_height(px(16.))
                .child(body),
        )
    }
}

/// A hover-tooltip wrapper around a trigger element.
#[derive(IntoElement)]
pub struct Tooltip {
    id: ElementId,
    text: Option<SharedString>,
    content: Option<RichContent>,
    children: Vec<AnyElement>,
}

impl Tooltip {
    pub fn new(id: impl Into<ElementId>, text: impl Into<SharedString>) -> Self {
        Self {
            id: id.into(),
            text: Some(text.into()),
            content: None,
            children: Vec::new(),
        }
    }

    /// Rich counterpart of [`Tooltip::new`]: the closure builds the bubble
    /// body on each render (arbitrary elements, not just a string).
    pub fn rich(
        id: impl Into<ElementId>,
        content: impl Fn(&mut Window, &mut App) -> AnyElement + 'static,
    ) -> Self {
        Self {
            id: id.into(),
            text: None,
            content: Some(Rc::new(content)),
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
        let content = self.content.clone();
        div()
            .id(self.id)
            .tooltip(move |_window, cx| {
                if let Some(ref content) = content {
                    let content = Rc::clone(content);
                    cx.new(move |_| TooltipView::rich(move |window, app| content(window, app)))
                        .into()
                } else {
                    let text = text.clone().unwrap_or_default();
                    cx.new(|_| TooltipView::new(text)).into()
                }
            })
            .children(self.children)
    }
}
