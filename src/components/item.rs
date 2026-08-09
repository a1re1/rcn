//! Item — port of shadcn base-vega `ui/item.tsx`.
//!
//! A flexible list row: Media (plain / icon / image tile) + Content
//! (Title, Description) + Actions, with Header/Footer spanning rows,
//! grouped by ItemGroup with ItemSeparator between rows. Focus-visible
//! ring and link-context hover styles are omitted.

use gpui::{
    AnyElement, App, FontWeight, IntoElement, ParentElement, RenderOnce, Styled, Window, div,
    prelude::FluentBuilder as _, px,
};

use crate::components::separator::Separator;
use crate::theme::{Theme, alpha};

#[derive(Clone, Copy, PartialEq, Eq, Default, Debug)]
pub enum ItemVariant {
    #[default]
    Default,
    Outline,
    Muted,
}

#[derive(Clone, Copy, PartialEq, Eq, Default, Debug)]
pub enum ItemSize {
    #[default]
    Default,
    Sm,
    Xs,
}

impl ItemSize {
    /// gap / horizontal padding / vertical padding per size.
    fn metrics(self) -> (f32, f32, f32) {
        match self {
            ItemSize::Default => (14., 16., 14.),
            ItemSize::Sm => (10., 12., 10.),
            ItemSize::Xs => (8., 10., 8.),
        }
    }
}

/// flex w-full flex-wrap items-center rounded-md border text-sm
#[derive(IntoElement)]
pub struct Item {
    variant: ItemVariant,
    size: ItemSize,
    children: Vec<AnyElement>,
}

impl Item {
    pub fn new() -> Self {
        Self {
            variant: ItemVariant::default(),
            size: ItemSize::default(),
            children: Vec::new(),
        }
    }

    pub fn variant(mut self, variant: ItemVariant) -> Self {
        self.variant = variant;
        self
    }

    pub fn size(mut self, size: ItemSize) -> Self {
        self.size = size;
        self
    }
}

impl Default for Item {
    fn default() -> Self {
        Self::new()
    }
}

impl ParentElement for Item {
    fn extend(&mut self, elements: impl IntoIterator<Item = AnyElement>) {
        self.children.extend(elements);
    }
}

impl RenderOnce for Item {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let theme = Theme::of(cx);
        let (gap, padding_x, padding_y) = self.size.metrics();
        let base = div()
            .flex()
            .flex_row()
            .flex_wrap()
            .w_full()
            .items_center()
            .rounded(theme.radius_md())
            .border_1()
            .border_color(gpui::transparent_black())
            .gap(px(gap))
            .px(px(padding_x))
            .py(px(padding_y))
            .text_size(px(14.))
            .line_height(px(20.));
        match self.variant {
            ItemVariant::Default => base,
            ItemVariant::Outline => base.border_color(theme.border),
            ItemVariant::Muted => base.bg(alpha(theme.muted, 0.5)),
        }
        .children(self.children)
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Default, Debug)]
pub enum ItemMediaVariant {
    #[default]
    Default,
    Icon,
    /// size-10 rounded-sm image tile.
    Image,
}

/// flex shrink-0 items-center justify-center gap-2
#[derive(IntoElement)]
pub struct ItemMedia {
    variant: ItemMediaVariant,
    children: Vec<AnyElement>,
}

impl ItemMedia {
    pub fn new() -> Self {
        Self {
            variant: ItemMediaVariant::default(),
            children: Vec::new(),
        }
    }

    pub fn variant(mut self, variant: ItemMediaVariant) -> Self {
        self.variant = variant;
        self
    }
}

impl Default for ItemMedia {
    fn default() -> Self {
        Self::new()
    }
}

impl ParentElement for ItemMedia {
    fn extend(&mut self, elements: impl IntoIterator<Item = AnyElement>) {
        self.children.extend(elements);
    }
}

impl RenderOnce for ItemMedia {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let theme = Theme::of(cx);
        div()
            .flex()
            .flex_shrink_0()
            .items_center()
            .justify_center()
            .gap(px(8.))
            .when(self.variant == ItemMediaVariant::Image, |el| {
                el.size(px(40.))
                    .overflow_hidden()
                    .rounded(theme.radius_sm())
            })
            .children(self.children)
    }
}

/// flex flex-1 flex-col gap-1
#[derive(IntoElement)]
pub struct ItemContent {
    children: Vec<AnyElement>,
}

impl ItemContent {
    pub fn new() -> Self {
        Self {
            children: Vec::new(),
        }
    }
}

impl Default for ItemContent {
    fn default() -> Self {
        Self::new()
    }
}

impl ParentElement for ItemContent {
    fn extend(&mut self, elements: impl IntoIterator<Item = AnyElement>) {
        self.children.extend(elements);
    }
}

impl RenderOnce for ItemContent {
    fn render(self, _window: &mut Window, _cx: &mut App) -> impl IntoElement {
        div()
            .flex()
            .flex_col()
            .flex_1()
            .gap(px(4.))
            .children(self.children)
    }
}

/// line-clamp-1 flex w-fit items-center gap-2 text-sm font-medium
#[derive(IntoElement)]
pub struct ItemTitle {
    children: Vec<AnyElement>,
}

impl ItemTitle {
    pub fn new() -> Self {
        Self {
            children: Vec::new(),
        }
    }
}

impl Default for ItemTitle {
    fn default() -> Self {
        Self::new()
    }
}

impl ParentElement for ItemTitle {
    fn extend(&mut self, elements: impl IntoIterator<Item = AnyElement>) {
        self.children.extend(elements);
    }
}

impl RenderOnce for ItemTitle {
    fn render(self, _window: &mut Window, _cx: &mut App) -> impl IntoElement {
        div()
            .flex()
            .flex_row()
            .items_center()
            .gap(px(8.))
            .text_size(px(14.))
            .line_height(px(19.))
            .font_weight(FontWeight::MEDIUM)
            .children(self.children)
    }
}

/// line-clamp-2 text-sm text-muted-foreground
#[derive(IntoElement)]
pub struct ItemDescription {
    children: Vec<AnyElement>,
}

impl ItemDescription {
    pub fn new() -> Self {
        Self {
            children: Vec::new(),
        }
    }
}

impl Default for ItemDescription {
    fn default() -> Self {
        Self::new()
    }
}

impl ParentElement for ItemDescription {
    fn extend(&mut self, elements: impl IntoIterator<Item = AnyElement>) {
        self.children.extend(elements);
    }
}

impl RenderOnce for ItemDescription {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let theme = Theme::of(cx);
        div()
            .text_size(px(14.))
            .line_height(px(21.))
            .text_color(theme.muted_foreground)
            .children(self.children)
    }
}

/// flex items-center gap-2 — trailing action cluster.
#[derive(IntoElement)]
pub struct ItemActions {
    children: Vec<AnyElement>,
}

impl ItemActions {
    pub fn new() -> Self {
        Self {
            children: Vec::new(),
        }
    }
}

impl Default for ItemActions {
    fn default() -> Self {
        Self::new()
    }
}

impl ParentElement for ItemActions {
    fn extend(&mut self, elements: impl IntoIterator<Item = AnyElement>) {
        self.children.extend(elements);
    }
}

impl RenderOnce for ItemActions {
    fn render(self, _window: &mut Window, _cx: &mut App) -> impl IntoElement {
        div()
            .flex()
            .flex_row()
            .items_center()
            .gap(px(8.))
            .children(self.children)
    }
}

/// flex basis-full items-center justify-between gap-2
#[derive(IntoElement)]
pub struct ItemHeader {
    children: Vec<AnyElement>,
}

impl ItemHeader {
    pub fn new() -> Self {
        Self {
            children: Vec::new(),
        }
    }
}

impl Default for ItemHeader {
    fn default() -> Self {
        Self::new()
    }
}

impl ParentElement for ItemHeader {
    fn extend(&mut self, elements: impl IntoIterator<Item = AnyElement>) {
        self.children.extend(elements);
    }
}

impl RenderOnce for ItemHeader {
    fn render(self, _window: &mut Window, _cx: &mut App) -> impl IntoElement {
        div()
            .flex()
            .flex_row()
            .w_full()
            .items_center()
            .justify_between()
            .gap(px(8.))
            .children(self.children)
    }
}

/// flex basis-full items-center justify-between gap-2
#[derive(IntoElement)]
pub struct ItemFooter {
    children: Vec<AnyElement>,
}

impl ItemFooter {
    pub fn new() -> Self {
        Self {
            children: Vec::new(),
        }
    }
}

impl Default for ItemFooter {
    fn default() -> Self {
        Self::new()
    }
}

impl ParentElement for ItemFooter {
    fn extend(&mut self, elements: impl IntoIterator<Item = AnyElement>) {
        self.children.extend(elements);
    }
}

impl RenderOnce for ItemFooter {
    fn render(self, _window: &mut Window, _cx: &mut App) -> impl IntoElement {
        div()
            .flex()
            .flex_row()
            .w_full()
            .items_center()
            .justify_between()
            .gap(px(8.))
            .children(self.children)
    }
}

/// flex w-full flex-col gap-4 — the list container.
#[derive(IntoElement)]
pub struct ItemGroup {
    children: Vec<AnyElement>,
}

impl ItemGroup {
    pub fn new() -> Self {
        Self {
            children: Vec::new(),
        }
    }
}

impl Default for ItemGroup {
    fn default() -> Self {
        Self::new()
    }
}

impl ParentElement for ItemGroup {
    fn extend(&mut self, elements: impl IntoIterator<Item = AnyElement>) {
        self.children.extend(elements);
    }
}

impl RenderOnce for ItemGroup {
    fn render(self, _window: &mut Window, _cx: &mut App) -> impl IntoElement {
        div()
            .flex()
            .flex_col()
            .w_full()
            .gap(px(16.))
            .children(self.children)
    }
}

/// A horizontal separator with my-2, for use between items.
#[derive(IntoElement)]
pub struct ItemSeparator;

impl ItemSeparator {
    pub fn new() -> Self {
        Self
    }
}

impl Default for ItemSeparator {
    fn default() -> Self {
        Self::new()
    }
}

impl RenderOnce for ItemSeparator {
    fn render(self, _window: &mut Window, _cx: &mut App) -> impl IntoElement {
        div().my(px(8.)).w_full().child(Separator::new())
    }
}
