//! Bubble — port of shadcn base-vega `ui/bubble.tsx`.
//!
//! Chat bubbles for [`Message`](crate::components::Message) content:
//! variants color the rounded-xl content pill; `BubbleReactions` floats a
//! reaction pill on a corner. The `tinted` variant approximates the
//! source's oklch relative-color math with a primary tint.

use gpui::{
    AnyElement, App, IntoElement, ParentElement, RenderOnce, Styled, Window, div,
    prelude::FluentBuilder as _, px,
};

use crate::theme::{Theme, alpha};

#[derive(Clone, Copy, PartialEq, Eq, Default, Debug)]
pub enum BubbleVariant {
    #[default]
    Default,
    Secondary,
    Muted,
    Tinted,
    Outline,
    Ghost,
}

#[derive(Clone, Copy, PartialEq, Eq, Default, Debug)]
pub enum BubbleSide {
    Top,
    #[default]
    Bottom,
}

#[derive(Clone, Copy, PartialEq, Eq, Default, Debug)]
pub enum BubbleAlign {
    #[default]
    Start,
    End,
}

/// group: relative flex w-fit max-w-[80%] flex-col gap-1.
#[derive(IntoElement)]
pub struct Bubble {
    variant: BubbleVariant,
    children: Vec<AnyElement>,
}

impl Bubble {
    pub fn new() -> Self {
        Self {
            variant: BubbleVariant::default(),
            children: Vec::new(),
        }
    }

    pub fn variant(mut self, variant: BubbleVariant) -> Self {
        self.variant = variant;
        self
    }

    /// The bubble body text/content (`BubbleContent`).
    pub fn content(mut self, content: impl IntoElement) -> Self {
        let variant = self.variant;
        self.children.push(
            BubbleContent {
                variant,
                children: vec![content.into_any_element()],
            }
            .into_any_element(),
        );
        self
    }
}

impl Default for Bubble {
    fn default() -> Self {
        Self::new()
    }
}

impl ParentElement for Bubble {
    fn extend(&mut self, elements: impl IntoIterator<Item = AnyElement>) {
        self.children.extend(elements);
    }
}

impl RenderOnce for Bubble {
    fn render(self, _window: &mut Window, _cx: &mut App) -> impl IntoElement {
        div()
            .relative()
            .flex()
            .flex_col()
            .gap(px(4.))
            .max_w(gpui::relative(0.8))
            .children(self.children)
    }
}

/// The colored pill: rounded-xl border px-3 py-2 text-sm leading-relaxed.
#[derive(IntoElement)]
struct BubbleContent {
    variant: BubbleVariant,
    children: Vec<AnyElement>,
}

impl RenderOnce for BubbleContent {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let theme = Theme::of(cx).clone();
        let base = div()
            .w_auto()
            .max_w_full()
            .min_w(px(0.))
            .overflow_hidden()
            .rounded(theme.radius_xl())
            .border_1()
            .border_color(gpui::transparent_black())
            .px(px(12.))
            .py(px(8.))
            .text_size(px(14.))
            .line_height(px(22.));
        match self.variant {
            BubbleVariant::Default => base.bg(theme.primary).text_color(theme.primary_foreground),
            BubbleVariant::Secondary => base
                .bg(theme.secondary)
                .text_color(theme.secondary_foreground),
            BubbleVariant::Muted => base.bg(theme.muted).text_color(theme.foreground),
            // oklch(from primary 0.93 c*0.4 h) — approximated with a light
            // primary wash.
            BubbleVariant::Tinted => base
                .bg(alpha(theme.primary, if theme.dark { 0.25 } else { 0.12 }))
                .text_color(theme.foreground),
            BubbleVariant::Outline => base
                .border_color(theme.border)
                .bg(theme.background)
                .text_color(theme.foreground),
            BubbleVariant::Ghost => base.text_color(theme.foreground),
        }
        .children(self.children)
    }
}

/// Reaction pill floated on a corner of the bubble: rounded-full bg-muted
/// px-1.5 py-0.5 ring-card.
#[derive(IntoElement)]
pub struct BubbleReactions {
    side: BubbleSide,
    align: BubbleAlign,
    children: Vec<AnyElement>,
}

impl BubbleReactions {
    pub fn new() -> Self {
        Self {
            side: BubbleSide::default(),
            align: BubbleAlign::default(),
            children: Vec::new(),
        }
    }

    pub fn side(mut self, side: BubbleSide) -> Self {
        self.side = side;
        self
    }

    pub fn align(mut self, align: BubbleAlign) -> Self {
        self.align = align;
        self
    }
}

impl Default for BubbleReactions {
    fn default() -> Self {
        Self::new()
    }
}

impl ParentElement for BubbleReactions {
    fn extend(&mut self, elements: impl IntoIterator<Item = AnyElement>) {
        self.children.extend(elements);
    }
}

impl RenderOnce for BubbleReactions {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let theme = Theme::of(cx).clone();
        div()
            .absolute()
            .flex()
            .flex_row()
            .items_center()
            .justify_center()
            .gap(px(4.))
            .rounded_full()
            .bg(theme.muted)
            .border_2()
            .border_color(theme.card)
            .px(px(6.))
            .py(px(2.))
            .text_size(px(13.))
            .map(|el| match self.side {
                BubbleSide::Top => el.top(px(-12.)),
                BubbleSide::Bottom => el.bottom(px(-12.)),
            })
            .map(|el| match self.align {
                BubbleAlign::Start => el.left(px(12.)),
                BubbleAlign::End => el.right(px(12.)),
            })
            .children(self.children)
    }
}
