//! Message — port of shadcn base-vega `ui/message.tsx`.
//!
//! Chat message rows: `MessageGroup` stacks messages; `Message` lays an
//! avatar beside content (align end flips the row for the sender);
//! Header/Footer are muted meta rows. The footer-offset avatar translate
//! is omitted.
//!
//! Sizing and shape overrides come from the caller via [`Styled`].

use gpui::{
    AnyElement, App, IntoElement, ParentElement, Refineable as _, RenderOnce, StyleRefinement,
    Styled, Window, div, prelude::FluentBuilder as _, px,
};

use crate::theme::Theme;

#[derive(Clone, Copy, PartialEq, Eq, Default, Debug)]
pub enum MessageAlign {
    #[default]
    Start,
    End,
}

/// flex min-w-0 flex-col gap-2 — the conversation column.
///
/// Sizing and shape overrides come from the caller via [`Styled`].
#[derive(IntoElement)]
pub struct MessageGroup {
    style: StyleRefinement,
    children: Vec<AnyElement>,
}

impl MessageGroup {
    pub fn new() -> Self {
        Self {
            style: StyleRefinement::default(),
            children: Vec::new(),
        }
    }
}

impl Default for MessageGroup {
    fn default() -> Self {
        Self::new()
    }
}

impl Styled for MessageGroup {
    fn style(&mut self) -> &mut StyleRefinement {
        &mut self.style
    }
}

impl ParentElement for MessageGroup {
    fn extend(&mut self, elements: impl IntoIterator<Item = AnyElement>) {
        self.children.extend(elements);
    }
}

impl RenderOnce for MessageGroup {
    fn render(self, _window: &mut Window, _cx: &mut App) -> impl IntoElement {
        let mut root = div().flex().flex_col().min_w(px(0.)).gap(px(8.));
        root.style().refine(&self.style);
        root.children(self.children)
    }
}

/// flex w-full gap-2 text-sm; align end reverses the row.
///
/// Sizing and shape overrides come from the caller via [`Styled`].
#[derive(IntoElement)]
pub struct Message {
    style: StyleRefinement,
    align: MessageAlign,
    children: Vec<AnyElement>,
}

impl Message {
    pub fn new() -> Self {
        Self {
            style: StyleRefinement::default(),
            align: MessageAlign::default(),
            children: Vec::new(),
        }
    }

    pub fn align(mut self, align: MessageAlign) -> Self {
        self.align = align;
        self
    }
}

impl Default for Message {
    fn default() -> Self {
        Self::new()
    }
}

impl Styled for Message {
    fn style(&mut self) -> &mut StyleRefinement {
        &mut self.style
    }
}

impl ParentElement for Message {
    fn extend(&mut self, elements: impl IntoIterator<Item = AnyElement>) {
        self.children.extend(elements);
    }
}

impl RenderOnce for Message {
    fn render(self, _window: &mut Window, _cx: &mut App) -> impl IntoElement {
        let mut root = div()
            .flex()
            .w_full()
            .min_w(px(0.))
            .gap(px(8.))
            .text_size(px(14.))
            .line_height(px(20.))
            .map(|el| match self.align {
                MessageAlign::Start => el.flex_row(),
                MessageAlign::End => el.flex_row_reverse(),
            });
        root.style().refine(&self.style);
        root.children(self.children)
    }
}

/// min-w-8 self-end rounded-full bg-muted avatar slot.
///
/// Sizing and shape overrides come from the caller via [`Styled`].
#[derive(IntoElement)]
pub struct MessageAvatar {
    style: StyleRefinement,
    children: Vec<AnyElement>,
}

impl MessageAvatar {
    pub fn new() -> Self {
        Self {
            style: StyleRefinement::default(),
            children: Vec::new(),
        }
    }
}

impl Default for MessageAvatar {
    fn default() -> Self {
        Self::new()
    }
}

impl Styled for MessageAvatar {
    fn style(&mut self) -> &mut StyleRefinement {
        &mut self.style
    }
}

impl ParentElement for MessageAvatar {
    fn extend(&mut self, elements: impl IntoIterator<Item = AnyElement>) {
        self.children.extend(elements);
    }
}

impl RenderOnce for MessageAvatar {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let theme = Theme::of(cx);
        let mut root = div()
            .flex()
            .flex_shrink_0()
            .min_w(px(32.))
            .items_center()
            .justify_center()
            .self_end()
            .overflow_hidden()
            .rounded_full()
            .bg(theme.muted);
        root.style().refine(&self.style);
        root.children(self.children)
    }
}

/// flex w-full min-w-0 flex-col gap-2.5 — the bubbles column; align end
/// is applied by the parent Message reversing the row.
///
/// Sizing and shape overrides come from the caller via [`Styled`].
#[derive(IntoElement)]
pub struct MessageContent {
    style: StyleRefinement,
    align: MessageAlign,
    children: Vec<AnyElement>,
}

impl MessageContent {
    pub fn new() -> Self {
        Self {
            style: StyleRefinement::default(),
            align: MessageAlign::default(),
            children: Vec::new(),
        }
    }

    /// Match the parent Message's alignment so bubbles hug the same edge.
    pub fn align(mut self, align: MessageAlign) -> Self {
        self.align = align;
        self
    }
}

impl Default for MessageContent {
    fn default() -> Self {
        Self::new()
    }
}

impl Styled for MessageContent {
    fn style(&mut self) -> &mut StyleRefinement {
        &mut self.style
    }
}

impl ParentElement for MessageContent {
    fn extend(&mut self, elements: impl IntoIterator<Item = AnyElement>) {
        self.children.extend(elements);
    }
}

impl RenderOnce for MessageContent {
    fn render(self, _window: &mut Window, _cx: &mut App) -> impl IntoElement {
        let mut root = div()
            .flex()
            .flex_col()
            .w_full()
            .min_w(px(0.))
            .gap(px(10.))
            .when(self.align == MessageAlign::End, |el| el.items_end());
        root.style().refine(&self.style);
        root.children(self.children)
    }
}

/// text-xs muted meta row above the bubbles.
///
/// Sizing and shape overrides come from the caller via [`Styled`].
#[derive(IntoElement)]
pub struct MessageHeader {
    style: StyleRefinement,
    children: Vec<AnyElement>,
}

impl MessageHeader {
    pub fn new() -> Self {
        Self {
            style: StyleRefinement::default(),
            children: Vec::new(),
        }
    }
}

impl Default for MessageHeader {
    fn default() -> Self {
        Self::new()
    }
}

impl Styled for MessageHeader {
    fn style(&mut self) -> &mut StyleRefinement {
        &mut self.style
    }
}

impl ParentElement for MessageHeader {
    fn extend(&mut self, elements: impl IntoIterator<Item = AnyElement>) {
        self.children.extend(elements);
    }
}

impl RenderOnce for MessageHeader {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let theme = Theme::of(cx);
        let mut root = div()
            .flex()
            .flex_row()
            .items_center()
            .gap(px(8.))
            .text_size(px(12.))
            .line_height(px(16.))
            .text_color(theme.muted_foreground);
        root.style().refine(&self.style);
        root.children(self.children)
    }
}

/// text-xs muted meta row below the bubbles.
///
/// Sizing and shape overrides come from the caller via [`Styled`].
#[derive(IntoElement)]
pub struct MessageFooter {
    style: StyleRefinement,
    children: Vec<AnyElement>,
}

impl MessageFooter {
    pub fn new() -> Self {
        Self {
            style: StyleRefinement::default(),
            children: Vec::new(),
        }
    }
}

impl Default for MessageFooter {
    fn default() -> Self {
        Self::new()
    }
}

impl Styled for MessageFooter {
    fn style(&mut self) -> &mut StyleRefinement {
        &mut self.style
    }
}

impl ParentElement for MessageFooter {
    fn extend(&mut self, elements: impl IntoIterator<Item = AnyElement>) {
        self.children.extend(elements);
    }
}

impl RenderOnce for MessageFooter {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let theme = Theme::of(cx);
        let mut root = div()
            .flex()
            .flex_row()
            .items_center()
            .gap(px(8.))
            .text_size(px(12.))
            .line_height(px(16.))
            .text_color(theme.muted_foreground);
        root.style().refine(&self.style);
        root.children(self.children)
    }
}
