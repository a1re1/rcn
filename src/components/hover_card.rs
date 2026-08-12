//! HoverCard — port of shadcn base-vega `ui/hover-card.tsx`.
//!
//! Rich content revealed on hover, built on gpui's hoverable-tooltip
//! machinery (the panel stays open while the pointer is over it). The
//! panel matches the source: `w-64 rounded-md bg-popover p-4 shadow-md
//! ring-1 ring-foreground/10`. Open/close animations are omitted.
//!
//! Sizing and shape overrides come from the caller via [`Styled`] and apply
//! to the floating card panel (the element carrying background, border, and
//! shadow), not the trigger wrapper.

use std::rc::Rc;

use gpui::{
    AnyElement, App, AppContext as _, Context, ElementId, InteractiveElement as _, IntoElement,
    ParentElement, Refineable as _, Render, RenderOnce, StatefulInteractiveElement as _,
    StyleRefinement, Styled, Window, div, prelude::FluentBuilder as _, px,
};

use crate::theme::{Theme, alpha};

type ContentBuilder = Rc<dyn Fn(&mut App) -> AnyElement + 'static>;

/// The floating panel view. Caller [`Styled`] refinements are applied to the
/// inner panel root (bg/border/shadow), not the outer margin wrapper.
struct HoverCardView {
    content: ContentBuilder,
    style: StyleRefinement,
}

impl Render for HoverCardView {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let theme = Theme::of(cx).clone();
        let mut panel = div()
            .w(px(256.))
            .rounded(theme.radius_md())
            .bg(theme.popover)
            .text_color(theme.popover_foreground)
            .p(px(16.))
            .shadow_md()
            .border_1()
            .border_color(alpha(theme.foreground, 0.1))
            .text_size(px(14.))
            .line_height(px(20.))
            .child((self.content)(cx));
        // Caller styles win over panel defaults.
        panel.style().refine(&self.style);
        div().m(px(4.)).child(panel)
    }
}

/// A hover-card wrapper: trigger children + a content builder.
///
/// Sizing and shape overrides via [`Styled`] target the floating card panel
/// root (bg/border/shadow), not the trigger.
#[derive(IntoElement)]
pub struct HoverCard {
    id: ElementId,
    content: Option<ContentBuilder>,
    children: Vec<AnyElement>,
    style: StyleRefinement,
}

impl HoverCard {
    pub fn new(id: impl Into<ElementId>) -> Self {
        Self {
            id: id.into(),
            content: None,
            children: Vec::new(),
            style: StyleRefinement::default(),
        }
    }

    /// Builds the panel content each time the card opens.
    pub fn content(mut self, content: impl Fn(&mut App) -> AnyElement + 'static) -> Self {
        self.content = Some(Rc::new(content));
        self
    }
}

impl ParentElement for HoverCard {
    fn extend(&mut self, elements: impl IntoIterator<Item = AnyElement>) {
        self.children.extend(elements);
    }
}

impl Styled for HoverCard {
    fn style(&mut self) -> &mut StyleRefinement {
        &mut self.style
    }
}

impl RenderOnce for HoverCard {
    fn render(self, _window: &mut Window, _cx: &mut App) -> impl IntoElement {
        let style = self.style;
        div()
            .id(self.id)
            .when_some(self.content, |el, content| {
                el.hoverable_tooltip(move |_, cx| {
                    let content = content.clone();
                    let style = style.clone();
                    cx.new(|_| HoverCardView { content, style }).into()
                })
            })
            .children(self.children)
    }
}
