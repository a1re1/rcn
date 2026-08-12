//! Kbd — port of shadcn base-vega `ui/kbd.tsx`.
//!
//! ```text
//! "pointer-events-none inline-flex h-5 w-fit min-w-5 items-center justify-center gap-1
//!  rounded-sm bg-muted px-1 font-sans text-xs font-medium text-muted-foreground select-none
//!  in-data-[slot=tooltip-content]:bg-background/20 in-data-[slot=tooltip-content]:text-background
//!  dark:in-data-[slot=tooltip-content]:bg-background/10 [&_svg:not([class*='size-'])]:size-3"
//! ```
//!
//! Tooltip-context styles are ported via [`Kbd::in_tooltip`] (gpui has no CSS context
//! selectors). SVG children should be passed pre-sized `px(12.)` — there is no cascade
//! equivalent of `[&_svg:not([class*='size-'])]:size-3`. `pointer-events-none` /
//! `select-none` are inherent in gpui (plain divs, unselectable text).
//! Sizing and shape overrides come from the caller via [`Styled`].

use gpui::{
    AnyElement, App, FontWeight, IntoElement, ParentElement, Refineable as _, RenderOnce,
    StyleRefinement, Styled, Window, div, px,
};

use crate::theme::{Theme, alpha};

/// Inline keyboard-key chip.
///
/// Sizing and shape overrides come from the caller via [`Styled`].
#[derive(IntoElement)]
pub struct Kbd {
    children: Vec<AnyElement>,
    in_tooltip: bool,
    style: StyleRefinement,
}

impl Kbd {
    pub fn new() -> Self {
        Self {
            children: Vec::new(),
            in_tooltip: false,
            style: StyleRefinement::default(),
        }
    }

    /// Explicit port of the `in-data-[slot=tooltip-content]` context styles
    /// (`bg-background/20 text-background`, dark `bg-background/10`). gpui has no
    /// CSS context selectors, so callers opt in when rendering a Kbd inside tooltip content.
    pub fn in_tooltip(mut self) -> Self {
        self.in_tooltip = true;
        self
    }
}

impl Default for Kbd {
    fn default() -> Self {
        Self::new()
    }
}

impl Styled for Kbd {
    fn style(&mut self) -> &mut StyleRefinement {
        &mut self.style
    }
}

impl ParentElement for Kbd {
    fn extend(&mut self, elements: impl IntoIterator<Item = AnyElement>) {
        self.children.extend(elements);
    }
}

impl RenderOnce for Kbd {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let theme = Theme::of(cx);

        // pointer-events-none inline-flex h-5 w-fit min-w-5 items-center justify-center gap-1
        // rounded-sm bg-muted px-1 font-sans text-xs font-medium text-muted-foreground select-none
        // in-data-[slot=tooltip-content]:bg-background/20 in-data-[slot=tooltip-content]:text-background
        // dark:in-data-[slot=tooltip-content]:bg-background/10 [&_svg:not([class*='size-'])]:size-3
        let (bg, fg) = if self.in_tooltip {
            (
                alpha(theme.background, if theme.dark { 0.10 } else { 0.20 }),
                theme.background,
            )
        } else {
            (theme.muted, theme.muted_foreground)
        };

        let mut root = div()
            .flex()
            .flex_row()
            .h(px(20.))
            .min_w(px(20.))
            .items_center()
            .justify_center()
            .gap(px(4.))
            .rounded(theme.radius_sm())
            .bg(bg)
            .px(px(4.))
            .text_size(px(12.))
            .line_height(px(16.))
            .font_weight(FontWeight::MEDIUM)
            .text_color(fg)
            .children(self.children);
        root.style().refine(&self.style);
        root
    }
}

/// Group of keyboard-key chips (`inline-flex items-center gap-1`).
///
/// Sizing and shape overrides come from the caller via [`Styled`].
#[derive(IntoElement)]
pub struct KbdGroup {
    children: Vec<AnyElement>,
    style: StyleRefinement,
}

impl KbdGroup {
    pub fn new() -> Self {
        Self {
            children: Vec::new(),
            style: StyleRefinement::default(),
        }
    }
}

impl Default for KbdGroup {
    fn default() -> Self {
        Self::new()
    }
}

impl Styled for KbdGroup {
    fn style(&mut self) -> &mut StyleRefinement {
        &mut self.style
    }
}

impl ParentElement for KbdGroup {
    fn extend(&mut self, elements: impl IntoIterator<Item = AnyElement>) {
        self.children.extend(elements);
    }
}

impl RenderOnce for KbdGroup {
    fn render(self, _window: &mut Window, _cx: &mut App) -> impl IntoElement {
        // inline-flex items-center gap-1
        let mut root = div()
            .flex()
            .flex_row()
            .items_center()
            .gap(px(4.))
            .children(self.children);
        root.style().refine(&self.style);
        root
    }
}
