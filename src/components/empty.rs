//! Empty — port of shadcn base-vega `ui/empty.tsx`.
//!
//! An empty-state block: Media (plain or icon-tile) + Title + Description
//! inside a Header, then a Content area for actions. The source's dashed
//! border utility is approximated with a plain border (gpui has no dashed
//! borders yet — TODO(rcn)).

use gpui::{
    AnyElement, App, FontWeight, IntoElement, ParentElement, RenderOnce, Styled, Window, div,
    prelude::FluentBuilder as _, px,
};

use crate::theme::Theme;

/// flex w-full flex-col items-center justify-center gap-4 rounded-lg p-12
#[derive(IntoElement)]
pub struct Empty {
    children: Vec<AnyElement>,
}

impl Empty {
    pub fn new() -> Self {
        Self {
            children: Vec::new(),
        }
    }
}

impl Default for Empty {
    fn default() -> Self {
        Self::new()
    }
}

impl ParentElement for Empty {
    fn extend(&mut self, elements: impl IntoIterator<Item = AnyElement>) {
        self.children.extend(elements);
    }
}

impl RenderOnce for Empty {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let theme = Theme::of(cx);
        div()
            .flex()
            .flex_col()
            .w_full()
            .flex_1()
            .items_center()
            .justify_center()
            .gap(px(16.))
            .rounded(theme.radius_lg())
            .border_1()
            .border_color(theme.border)
            .p(px(48.))
            .children(self.children)
    }
}

/// flex max-w-sm flex-col items-center gap-2
#[derive(IntoElement)]
pub struct EmptyHeader {
    children: Vec<AnyElement>,
}

impl EmptyHeader {
    pub fn new() -> Self {
        Self {
            children: Vec::new(),
        }
    }
}

impl Default for EmptyHeader {
    fn default() -> Self {
        Self::new()
    }
}

impl ParentElement for EmptyHeader {
    fn extend(&mut self, elements: impl IntoIterator<Item = AnyElement>) {
        self.children.extend(elements);
    }
}

impl RenderOnce for EmptyHeader {
    fn render(self, _window: &mut Window, _cx: &mut App) -> impl IntoElement {
        div()
            .flex()
            .flex_col()
            .max_w(px(384.))
            .items_center()
            .gap(px(8.))
            .children(self.children)
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Default, Debug)]
pub enum EmptyMediaVariant {
    #[default]
    Default,
    /// size-10 rounded-lg bg-muted icon tile.
    Icon,
}

/// mb-2 flex shrink-0 items-center justify-center (+ icon tile variant)
#[derive(IntoElement)]
pub struct EmptyMedia {
    variant: EmptyMediaVariant,
    children: Vec<AnyElement>,
}

impl EmptyMedia {
    pub fn new() -> Self {
        Self {
            variant: EmptyMediaVariant::default(),
            children: Vec::new(),
        }
    }

    pub fn variant(mut self, variant: EmptyMediaVariant) -> Self {
        self.variant = variant;
        self
    }
}

impl Default for EmptyMedia {
    fn default() -> Self {
        Self::new()
    }
}

impl ParentElement for EmptyMedia {
    fn extend(&mut self, elements: impl IntoIterator<Item = AnyElement>) {
        self.children.extend(elements);
    }
}

impl RenderOnce for EmptyMedia {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let theme = Theme::of(cx);
        div()
            .flex()
            .flex_shrink_0()
            .items_center()
            .justify_center()
            .mb(px(8.))
            .when(self.variant == EmptyMediaVariant::Icon, |el| {
                el.size(px(40.))
                    .rounded(theme.radius_lg())
                    .bg(theme.muted)
                    .text_color(theme.foreground)
            })
            .children(self.children)
    }
}

/// cn-font-heading text-lg font-medium
#[derive(IntoElement)]
pub struct EmptyTitle {
    children: Vec<AnyElement>,
}

impl EmptyTitle {
    pub fn new() -> Self {
        Self {
            children: Vec::new(),
        }
    }
}

impl Default for EmptyTitle {
    fn default() -> Self {
        Self::new()
    }
}

impl ParentElement for EmptyTitle {
    fn extend(&mut self, elements: impl IntoIterator<Item = AnyElement>) {
        self.children.extend(elements);
    }
}

impl RenderOnce for EmptyTitle {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let theme = Theme::of(cx);
        div()
            .text_size(px(18.))
            .line_height(px(28.))
            .font_weight(FontWeight::MEDIUM)
            .when_some(theme.heading_font(), |el, font| el.font_family(font))
            .children(self.children)
    }
}

/// text-sm/relaxed text-muted-foreground
#[derive(IntoElement)]
pub struct EmptyDescription {
    children: Vec<AnyElement>,
}

impl EmptyDescription {
    pub fn new() -> Self {
        Self {
            children: Vec::new(),
        }
    }
}

impl Default for EmptyDescription {
    fn default() -> Self {
        Self::new()
    }
}

impl ParentElement for EmptyDescription {
    fn extend(&mut self, elements: impl IntoIterator<Item = AnyElement>) {
        self.children.extend(elements);
    }
}

impl RenderOnce for EmptyDescription {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let theme = Theme::of(cx);
        div()
            .text_size(px(14.))
            .line_height(px(22.))
            .text_color(theme.muted_foreground)
            .children(self.children)
    }
}

/// flex w-full max-w-sm flex-col items-center gap-4 text-sm
#[derive(IntoElement)]
pub struct EmptyContent {
    children: Vec<AnyElement>,
}

impl EmptyContent {
    pub fn new() -> Self {
        Self {
            children: Vec::new(),
        }
    }
}

impl Default for EmptyContent {
    fn default() -> Self {
        Self::new()
    }
}

impl ParentElement for EmptyContent {
    fn extend(&mut self, elements: impl IntoIterator<Item = AnyElement>) {
        self.children.extend(elements);
    }
}

impl RenderOnce for EmptyContent {
    fn render(self, _window: &mut Window, _cx: &mut App) -> impl IntoElement {
        div()
            .flex()
            .flex_col()
            .w_full()
            .max_w(px(384.))
            .items_center()
            .gap(px(16.))
            .text_size(px(14.))
            .line_height(px(20.))
            .children(self.children)
    }
}
