//! Marker — port of shadcn base-vega `ui/marker.tsx`.
//!
//! An inline conversation marker (date dividers, event notes): muted text
//! row with optional icon, as a plain row, a separator (rules on both
//! sides), or a bottom-bordered heading.

use gpui::{
    AnyElement, App, IntoElement, ParentElement, RenderOnce, Styled, Window, div,
    prelude::FluentBuilder as _, px,
};

use crate::theme::Theme;

#[derive(Clone, Copy, PartialEq, Eq, Default, Debug)]
pub enum MarkerVariant {
    #[default]
    Default,
    /// Hairlines flanking the content.
    Separator,
    /// Bottom border under the row.
    Border,
}

#[derive(IntoElement)]
pub struct Marker {
    variant: MarkerVariant,
    children: Vec<AnyElement>,
}

impl Marker {
    pub fn new() -> Self {
        Self {
            variant: MarkerVariant::default(),
            children: Vec::new(),
        }
    }

    pub fn variant(mut self, variant: MarkerVariant) -> Self {
        self.variant = variant;
        self
    }
}

impl Default for Marker {
    fn default() -> Self {
        Self::new()
    }
}

impl ParentElement for Marker {
    fn extend(&mut self, elements: impl IntoIterator<Item = AnyElement>) {
        self.children.extend(elements);
    }
}

impl RenderOnce for Marker {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let theme = Theme::of(cx).clone();
        let rule = || div().h(px(1.)).flex_1().min_w(px(0.)).bg(theme.border);

        div()
            .flex()
            .flex_row()
            .w_full()
            .min_h(px(16.))
            .items_center()
            .gap(px(8.))
            .text_size(px(14.))
            .line_height(px(20.))
            .text_color(theme.muted_foreground)
            .when(self.variant == MarkerVariant::Border, |el| {
                el.border_b_1().border_color(theme.border).pb(px(8.))
            })
            .map(|el| {
                if self.variant == MarkerVariant::Separator {
                    el.child(rule()).children(self.children).child(rule())
                } else {
                    el.children(self.children)
                }
            })
    }
}
