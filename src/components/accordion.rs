//! Accordion — port of shadcn base-vega `ui/accordion.tsx`.
//!
//! Controlled: each item takes `open` and reports clicks via `on_toggle`.
//! The chevron flips between lucide chevron-down/up exactly as the source
//! does. The open/close height animation is omitted (no CSS keyframe
//! equivalent wired up yet); content simply shows or hides.

use gpui::{
    AnimationExt as _, AnyElement, App, ClickEvent, ElementId, FontWeight, InteractiveElement as _,
    IntoElement, ParentElement, RenderOnce, StatefulInteractiveElement as _, Styled, Window, div,
    prelude::FluentBuilder as _, px, svg,
};

use crate::motion;
use crate::theme::Theme;

type ToggleHandler = Box<dyn Fn(&ClickEvent, &mut Window, &mut App) + 'static>;

/// flex w-full flex-col; the container for [`AccordionItem`]s.
#[derive(IntoElement)]
pub struct Accordion {
    children: Vec<AnyElement>,
}

impl Accordion {
    pub fn new() -> Self {
        Self {
            children: Vec::new(),
        }
    }
}

impl Default for Accordion {
    fn default() -> Self {
        Self::new()
    }
}

impl ParentElement for Accordion {
    fn extend(&mut self, elements: impl IntoIterator<Item = AnyElement>) {
        self.children.extend(elements);
    }
}

impl RenderOnce for Accordion {
    fn render(self, _window: &mut Window, _cx: &mut App) -> impl IntoElement {
        div().flex().flex_col().w_full().children(self.children)
    }
}

#[derive(IntoElement)]
pub struct AccordionItem {
    id: ElementId,
    trigger: Option<AnyElement>,
    content: Option<AnyElement>,
    open: bool,
    last: bool,
    disabled: bool,
    on_toggle: Option<ToggleHandler>,
}

impl AccordionItem {
    pub fn new(id: impl Into<ElementId>) -> Self {
        Self {
            id: id.into(),
            trigger: None,
            content: None,
            open: false,
            last: false,
            disabled: false,
            on_toggle: None,
        }
    }

    /// The always-visible header row (usually a label string).
    pub fn trigger(mut self, trigger: impl IntoElement) -> Self {
        self.trigger = Some(trigger.into_any_element());
        self
    }

    /// The collapsible body.
    pub fn content(mut self, content: impl IntoElement) -> Self {
        self.content = Some(content.into_any_element());
        self
    }

    pub fn open(mut self, open: bool) -> Self {
        self.open = open;
        self
    }

    /// Marks the last item, which drops its bottom border
    /// (the source's `not-last:border-b`).
    pub fn last(mut self, last: bool) -> Self {
        self.last = last;
        self
    }

    /// aria-disabled: half opacity, no pointer events, unfocusable.
    pub fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
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

impl RenderOnce for AccordionItem {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let theme = Theme::of(cx).clone();

        let chevron = if self.open {
            theme.icons.chevron_up()
        } else {
            theme.icons.chevron_down()
        };

        div()
            .w_full()
            .when(!self.last, |el| el.border_b_1().border_color(theme.border))
            // Trigger: flex flex-1 items-start justify-between py-4 text-left
            // text-sm font-medium hover:underline
            .child(
                div()
                    .id(self.id.clone())
                    .flex()
                    .flex_row()
                    .w_full()
                    .items_center()
                    .justify_between()
                    .gap(px(16.))
                    .py(px(16.))
                    .text_size(px(14.))
                    .line_height(px(20.))
                    .font_weight(FontWeight::MEDIUM)
                    .text_color(theme.foreground)
                    .rounded(theme.radius_md())
                    .border_1()
                    .border_color(gpui::transparent_black())
                    .when(self.disabled, |el| el.opacity(0.5))
                    .when(!self.disabled, |el| {
                        let ring = motion::focus_ring(&theme);
                        el.hover(|s| s.underline())
                            .tab_index(0)
                            .focus_visible(move |s| s.border_color(theme.ring).shadow(ring.clone()))
                            .when_some(self.on_toggle, |el, on_toggle| el.on_click(on_toggle))
                    })
                    .children(self.trigger)
                    // **:data-[slot=accordion-trigger-icon]:size-4
                    // text-muted-foreground
                    .child(
                        svg()
                            .path(chevron)
                            .size(px(16.))
                            .flex_shrink_0()
                            .text_color(theme.muted_foreground),
                    ),
            )
            // Content: text-sm, pb-4 when open.
            .when(self.open, |el| {
                // accordion-down: 200ms ease-out. True height animation
                // needs pre-measured content (TODO(rcn)); the reveal fades
                // and slides in on the same clock instead.
                el.child(
                    div()
                        .overflow_hidden()
                        .child(
                            div()
                                .pb(px(16.))
                                .text_size(px(14.))
                                .line_height(px(20.))
                                .text_color(theme.foreground)
                                .children(self.content),
                        )
                        .with_animation(self.id.clone(), motion::expand(), |el, delta| {
                            el.opacity(delta).mt(px(-8. * (1. - delta)))
                        }),
                )
            })
    }
}
