//! Alert — port of shadcn base-vega `ui/alert.tsx`.
//!
//! Callout shell with optional leading icon plus Title / Description children.
//! Variants: `default` | `destructive`.
//!
//! Sizing and shape overrides come from the caller via [`Styled`] (gpui's
//! equivalent of shadcn's `className` passthrough).
//!
//! Omissions vs source (no gpui equivalent yet):
//! - CSS grid template / `has-[>svg]` selectors — layout is an explicit flex row
//!   when `.icon(..)` is set, otherwise a flex column (no auto-detect of svg children).
//! - `alert-action` slot (absolute top-right action area).
//! - Anchor underline / hover-foreground rules on title and description.
//! - Destructive description tint at 90% opacity when nested under a destructive
//!   alert — gpui has no parent-context styling, so AlertDescription always uses
//!   `theme.muted_foreground`. TODO(rcn): parent-context destructive/90 tint.

use gpui::{
    AnyElement, App, FontWeight, IntoElement, ParentElement, Refineable as _, RenderOnce,
    SharedString, StyleRefinement, Styled, Window, div, px, svg,
};

use crate::theme::Theme;

/// Alert visual variant — maps to the source `variant` cva key.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum AlertVariant {
    #[default]
    Default,
    Destructive,
}

/// w-full rounded-lg border px-4 py-3 text-sm — callout shell.
///
/// Sizing and shape overrides come from the caller via [`Styled`].
#[derive(IntoElement)]
pub struct Alert {
    variant: AlertVariant,
    icon: Option<SharedString>,
    children: Vec<AnyElement>,
    style: StyleRefinement,
}

impl Alert {
    pub fn new() -> Self {
        Self {
            variant: AlertVariant::Default,
            icon: None,
            children: Vec::new(),
            style: StyleRefinement::default(),
        }
    }

    pub fn variant(mut self, variant: AlertVariant) -> Self {
        self.variant = variant;
        self
    }

    /// Optional leading svg asset path (e.g. `crate::assets::ICON_CIRCLE_CHECK`).
    pub fn icon(mut self, icon: impl Into<SharedString>) -> Self {
        self.icon = Some(icon.into());
        self
    }
}

impl Default for Alert {
    fn default() -> Self {
        Self::new()
    }
}

impl ParentElement for Alert {
    fn extend(&mut self, elements: impl IntoIterator<Item = AnyElement>) {
        self.children.extend(elements);
    }
}

impl Styled for Alert {
    fn style(&mut self) -> &mut StyleRefinement {
        &mut self.style
    }
}

impl RenderOnce for Alert {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let theme = Theme::of(cx);

        // default: bg-card text-card-foreground
        // destructive: bg-card text-destructive (children inherit)
        let text = match self.variant {
            AlertVariant::Default => theme.card_foreground,
            AlertVariant::Destructive => theme.destructive,
        };

        // Shared shell: w-full rounded-lg border px-4 py-3 text-sm bg-card
        let base = div()
            .w_full()
            .rounded(theme.radius_lg())
            .border_1()
            .border_color(theme.border)
            .px(px(16.))
            .py(px(12.))
            .text_size(px(14.))
            .line_height(px(20.))
            .bg(theme.card)
            .text_color(text);

        let mut root = if let Some(icon) = self.icon {
            // has-[>svg]: flex row gap-x-2.5; svg size-4 shrink-0 translate-y-0.5 text-current
            base.flex()
                .flex_row()
                .gap(px(10.))
                .child(
                    svg()
                        .path(icon)
                        .size(px(16.))
                        .flex_shrink_0()
                        .mt(px(2.))
                        .text_color(text), // text-current
                )
                .child(
                    div()
                        .flex()
                        .flex_col()
                        .flex_1()
                        .gap(px(2.))
                        .children(self.children),
                )
        } else {
            // no icon: flex col gap-0.5 (source gap-0.5 ≈ 2px)
            base.flex().flex_col().gap(px(2.)).children(self.children)
        };
        root.style().refine(&self.style);
        root
    }
}

/// font-medium title row inside an [`Alert`].
///
/// Sizing and shape overrides come from the caller via [`Styled`].
#[derive(IntoElement)]
pub struct AlertTitle {
    children: Vec<AnyElement>,
    style: StyleRefinement,
}

impl AlertTitle {
    pub fn new() -> Self {
        Self {
            children: Vec::new(),
            style: StyleRefinement::default(),
        }
    }
}

impl Default for AlertTitle {
    fn default() -> Self {
        Self::new()
    }
}

impl ParentElement for AlertTitle {
    fn extend(&mut self, elements: impl IntoIterator<Item = AnyElement>) {
        self.children.extend(elements);
    }
}

impl Styled for AlertTitle {
    fn style(&mut self) -> &mut StyleRefinement {
        &mut self.style
    }
}

impl RenderOnce for AlertTitle {
    fn render(self, _window: &mut Window, _cx: &mut App) -> impl IntoElement {
        // font-medium — color inherits from parent Alert
        let mut root = div()
            .font_weight(FontWeight::MEDIUM)
            .children(self.children);
        root.style().refine(&self.style);
        root
    }
}

/// text-sm text-muted-foreground description body inside an [`Alert`].
///
/// Sizing and shape overrides come from the caller via [`Styled`].
///
/// TODO(rcn): when nested under a destructive Alert the source tints description
/// to destructive/90 via parent-context selectors — not available in gpui, so
/// this always uses `theme.muted_foreground`.
#[derive(IntoElement)]
pub struct AlertDescription {
    children: Vec<AnyElement>,
    style: StyleRefinement,
}

impl AlertDescription {
    pub fn new() -> Self {
        Self {
            children: Vec::new(),
            style: StyleRefinement::default(),
        }
    }
}

impl Default for AlertDescription {
    fn default() -> Self {
        Self::new()
    }
}

impl ParentElement for AlertDescription {
    fn extend(&mut self, elements: impl IntoIterator<Item = AnyElement>) {
        self.children.extend(elements);
    }
}

impl Styled for AlertDescription {
    fn style(&mut self) -> &mut StyleRefinement {
        &mut self.style
    }
}

impl RenderOnce for AlertDescription {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let theme = Theme::of(cx);

        // text-sm text-muted-foreground
        let mut root = div()
            .text_size(px(14.))
            .line_height(px(20.))
            .text_color(theme.muted_foreground)
            .children(self.children);
        root.style().refine(&self.style);
        root
    }
}
