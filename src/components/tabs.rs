//! Tabs — port of shadcn base-vega `ui/tabs.tsx`.
//!
//! Controlled: the list holds typed triggers; the caller owns the active
//! index and re-renders the matching content. Variants: Default (muted
//! pill list, active tab lifted on background) and Line (transparent
//! list). Vertical orientation is omitted.

use gpui::{
    AnyElement, App, ClickEvent, ElementId, FontWeight, InteractiveElement as _, IntoElement,
    ParentElement, RenderOnce, StatefulInteractiveElement as _, Styled, Window, div,
    prelude::FluentBuilder as _, px,
};

use crate::motion;
use crate::theme::{Theme, alpha};

type SelectHandler = Box<dyn Fn(&ClickEvent, &mut Window, &mut App) + 'static>;

#[derive(Clone, Copy, PartialEq, Eq, Default, Debug)]
pub enum TabsVariant {
    #[default]
    Default,
    Line,
}

/// flex flex-col gap-2 — the root: list + content.
#[derive(IntoElement)]
pub struct Tabs {
    children: Vec<AnyElement>,
}

impl Tabs {
    pub fn new() -> Self {
        Self {
            children: Vec::new(),
        }
    }
}

impl Default for Tabs {
    fn default() -> Self {
        Self::new()
    }
}

impl ParentElement for Tabs {
    fn extend(&mut self, elements: impl IntoIterator<Item = AnyElement>) {
        self.children.extend(elements);
    }
}

impl RenderOnce for Tabs {
    fn render(self, _window: &mut Window, _cx: &mut App) -> impl IntoElement {
        div().flex().flex_col().gap(px(8.)).children(self.children)
    }
}

pub struct TabsTrigger {
    id: ElementId,
    active: bool,
    disabled: bool,
    on_select: Option<SelectHandler>,
    children: Vec<AnyElement>,
}

impl TabsTrigger {
    pub fn new(id: impl Into<ElementId>) -> Self {
        Self {
            id: id.into(),
            active: false,
            disabled: false,
            on_select: None,
            children: Vec::new(),
        }
    }

    pub fn active(mut self, active: bool) -> Self {
        self.active = active;
        self
    }

    pub fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }

    pub fn on_select(
        mut self,
        handler: impl Fn(&ClickEvent, &mut Window, &mut App) + 'static,
    ) -> Self {
        self.on_select = Some(Box::new(handler));
        self
    }

    pub fn child(mut self, child: impl IntoElement) -> Self {
        self.children.push(child.into_any_element());
        self
    }
}

/// inline-flex h-9 w-fit items-center rounded-lg p-[3px]; default: bg-muted.
#[derive(IntoElement)]
pub struct TabsList {
    variant: TabsVariant,
    triggers: Vec<TabsTrigger>,
}

impl TabsList {
    pub fn new() -> Self {
        Self {
            variant: TabsVariant::default(),
            triggers: Vec::new(),
        }
    }

    pub fn variant(mut self, variant: TabsVariant) -> Self {
        self.variant = variant;
        self
    }

    pub fn trigger(mut self, trigger: TabsTrigger) -> Self {
        self.triggers.push(trigger);
        self
    }
}

impl Default for TabsList {
    fn default() -> Self {
        Self::new()
    }
}

impl RenderOnce for TabsList {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let theme = Theme::of(cx).clone();
        let variant = self.variant;
        div()
            .flex()
            .flex_row()
            .items_center()
            .justify_center()
            .h(px(36.))
            .p(px(3.))
            .rounded(theme.radius_lg())
            .when(variant == TabsVariant::Default, |el| el.bg(theme.muted))
            .when(variant == TabsVariant::Line, |el| el.gap(px(4.)))
            .text_color(theme.muted_foreground)
            .children(self.triggers.into_iter().map(|trigger| {
                let active = trigger.active;
                let inactive_text = alpha(theme.foreground, 0.6);
                let hover_text = theme.foreground;
                div()
                    .id(trigger.id)
                    .flex()
                    .flex_row()
                    .flex_1()
                    .h_full()
                    .items_center()
                    .justify_center()
                    .gap(px(6.))
                    .rounded(theme.radius_md())
                    .border_1()
                    .border_color(gpui::transparent_black())
                    .px(px(8.))
                    .py(px(4.))
                    .text_size(px(14.))
                    .line_height(px(20.))
                    .font_weight(FontWeight::MEDIUM)
                    .whitespace_nowrap()
                    .map(|el| {
                        if active && variant == TabsVariant::Default {
                            // data-active:bg-background text-foreground shadow-sm
                            // dark: border-input bg-input/30
                            if theme.dark {
                                el.border_color(theme.input)
                                    .bg(alpha(theme.input, 0.3))
                                    .text_color(theme.foreground)
                            } else {
                                el.bg(theme.background)
                                    .text_color(theme.foreground)
                                    .shadow_sm()
                            }
                        } else if active {
                            el.text_color(theme.foreground)
                        } else {
                            el.text_color(inactive_text)
                        }
                    })
                    .when(trigger.disabled, |el| el.opacity(0.5))
                    .when(!trigger.disabled, |el| {
                        let ring = motion::focus_ring(&theme);
                        el.tab_index(0)
                            .focus_visible(move |s| s.border_color(theme.ring).shadow(ring.clone()))
                            .hover(move |s| s.text_color(hover_text))
                            .when_some(trigger.on_select, |el, on_select| el.on_click(on_select))
                    })
                    .children(trigger.children)
            }))
    }
}

/// flex-1 text-sm — the panel for the active tab.
#[derive(IntoElement)]
pub struct TabsContent {
    children: Vec<AnyElement>,
}

impl TabsContent {
    pub fn new() -> Self {
        Self {
            children: Vec::new(),
        }
    }
}

impl Default for TabsContent {
    fn default() -> Self {
        Self::new()
    }
}

impl ParentElement for TabsContent {
    fn extend(&mut self, elements: impl IntoIterator<Item = AnyElement>) {
        self.children.extend(elements);
    }
}

impl RenderOnce for TabsContent {
    fn render(self, _window: &mut Window, _cx: &mut App) -> impl IntoElement {
        div()
            .flex_1()
            .text_size(px(14.))
            .line_height(px(20.))
            .children(self.children)
    }
}
