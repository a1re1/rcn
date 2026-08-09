//! Collapsible — port of shadcn base-vega `ui/collapsible.tsx`.
//!
//! Controlled: the caller owns `open`, the trigger reports clicks via
//! `on_toggle`, and the content renders only while open (no height
//! animation, like the accordion).

use gpui::{
    AnyElement, App, ClickEvent, ElementId, InteractiveElement as _, IntoElement, ParentElement,
    RenderOnce, StatefulInteractiveElement as _, Styled, Window, div, prelude::FluentBuilder as _,
    px,
};

type ToggleHandler = Box<dyn Fn(&ClickEvent, &mut Window, &mut App) + 'static>;

/// The root: renders the trigger row, then the content when open.
#[derive(IntoElement)]
pub struct Collapsible {
    id: ElementId,
    open: bool,
    trigger: Option<AnyElement>,
    content: Option<AnyElement>,
    on_toggle: Option<ToggleHandler>,
}

impl Collapsible {
    pub fn new(id: impl Into<ElementId>) -> Self {
        Self {
            id: id.into(),
            open: false,
            trigger: None,
            content: None,
            on_toggle: None,
        }
    }

    pub fn open(mut self, open: bool) -> Self {
        self.open = open;
        self
    }

    /// The always-visible row that toggles the content.
    pub fn trigger(mut self, trigger: impl IntoElement) -> Self {
        self.trigger = Some(trigger.into_any_element());
        self
    }

    /// The collapsible body.
    pub fn content(mut self, content: impl IntoElement) -> Self {
        self.content = Some(content.into_any_element());
        self
    }

    pub fn on_toggle(
        mut self,
        handler: impl Fn(&ClickEvent, &mut Window, &mut App) + 'static,
    ) -> Self {
        self.on_toggle = Some(Box::new(handler));
        self
    }
}

impl RenderOnce for Collapsible {
    fn render(self, _window: &mut Window, _cx: &mut App) -> impl IntoElement {
        div()
            .flex()
            .flex_col()
            .gap(px(8.))
            .child(
                div()
                    .id(self.id)
                    .when_some(self.on_toggle, |el, on_toggle| el.on_click(on_toggle))
                    .children(self.trigger),
            )
            .when(self.open, |el| el.children(self.content))
    }
}
