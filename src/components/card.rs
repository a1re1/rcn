//! Card — port of shadcn base-nova `ui/card.tsx`.
//!
//! Compound card layout: Card shell plus Header / Title / Description / Action /
//! Content / Footer. Size (`default` | `sm`) drives the shared spacing token
//! (16px / 12px). Parts also accept an explicit `.spacing(Pixels)` override that
//! mirrors shadcn's `[--card-spacing:*]` arbitrary value.
//!
//! Sizing and shape overrides come from the caller via [`Styled`] (gpui's
//! equivalent of shadcn's `className` passthrough).
//!
//! Omissions vs source (no gpui equivalent yet):
//! - CSS variable cascade (`--card-spacing`) — each part takes `.size(CardSize)`
//!   and/or `.spacing(Pixels)` explicitly (gpui has no CSS variable cascade).
//!   Callers should pass the same size/spacing used on the parent Card.
//! - `:has` selectors — `has-[>img:first-child]:pt-0` → explicit `.flush_top()`;
//!   `has-data-[slot=card-footer]:pb-0` → CardFooter always applies `mb(-spacing)`
//!   so the card root's bottom `py` is cancelled when the footer is the last child.
//! - `[.border-b]:pb-(--card-spacing)` / `[.border-t]:pt-(--card-spacing)` utilities
//! - `@container` / grid template header (`has-data-[slot=card-action]:grid-cols-…`)
//!   → explicit `CardHeader::action()` slot (row with 1fr + auto columns)
//! - first/last child image rounding (`*:[img:first-child]:rounded-t-xl`, etc.) —
//!   free via the card's `overflow_hidden` + rounded shell for full-bleed media
//! - RTL docs example not ported

use gpui::{
    AnyElement, App, FontWeight, IntoElement, ParentElement, Pixels, Refineable as _, RenderOnce,
    StyleRefinement, Styled, Window, div, prelude::FluentBuilder as _, px,
};

use crate::theme::{Theme, alpha};

/// Card spacing size. Maps to shadcn `data-size` / `--card-spacing`.
#[derive(Clone, Copy, PartialEq, Eq, Default, Debug)]
pub enum CardSize {
    /// 16px spacing (`--spacing(4)`).
    #[default]
    Default,
    /// 12px spacing (`--spacing(3)`).
    Sm,
}

impl CardSize {
    fn spacing(self) -> f32 {
        match self {
            CardSize::Default => 16.,
            CardSize::Sm => 12.,
        }
    }
}

/// group/card flex flex-col gap-(--card-spacing) overflow-hidden rounded-xl
/// bg-card py-(--card-spacing) text-sm text-card-foreground ring-1
/// ring-foreground/10 [--card-spacing:--spacing(4)]
/// has-data-[slot=card-footer]:pb-0 has-[>img:first-child]:pt-0
/// data-[size=sm]:[--card-spacing:--spacing(3)]
///
/// Footer `pb-0` is emulated by CardFooter applying `mb(-spacing)` (footer must
/// be the last child). Leading full-bleed media uses `.flush_top()` (`pt-0`).
/// Sizing and shape overrides come from the caller via [`Styled`].
#[derive(IntoElement)]
pub struct Card {
    size: CardSize,
    spacing_override: Option<Pixels>,
    flush_top: bool,
    style: StyleRefinement,
    children: Vec<AnyElement>,
}

impl Card {
    pub fn new() -> Self {
        Self {
            size: CardSize::Default,
            spacing_override: None,
            flush_top: false,
            style: StyleRefinement::default(),
            children: Vec::new(),
        }
    }

    pub fn size(mut self, size: CardSize) -> Self {
        self.size = size;
        self
    }

    /// Override the size preset spacing (shadcn `[--card-spacing:*]`).
    pub fn spacing(mut self, spacing: Pixels) -> Self {
        self.spacing_override = Some(spacing);
        self
    }

    /// Drop top padding for a full-bleed leading image
    /// (`has-[>img:first-child]:pt-0`). Corner rounding of the media comes free
    /// from the card's `overflow_hidden` + rounded shell.
    pub fn flush_top(mut self) -> Self {
        self.flush_top = true;
        self
    }

    fn spacing_px(&self) -> Pixels {
        self.spacing_override
            .unwrap_or_else(|| px(self.size.spacing()))
    }
}

impl Default for Card {
    fn default() -> Self {
        Self::new()
    }
}

impl Styled for Card {
    fn style(&mut self) -> &mut StyleRefinement {
        &mut self.style
    }
}

impl ParentElement for Card {
    fn extend(&mut self, elements: impl IntoIterator<Item = AnyElement>) {
        self.children.extend(elements);
    }
}

impl RenderOnce for Card {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let theme = Theme::of(cx).clone();
        let spacing = self.spacing_px();

        let mut root = div()
            .flex()
            .flex_col()
            .gap(spacing)
            .overflow_hidden()
            .rounded(theme.radius_xl())
            .bg(theme.card)
            .py(spacing)
            .when(self.flush_top, |el| el.pt(px(0.)))
            .text_size(px(14.))
            .line_height(px(20.))
            .text_color(theme.card_foreground)
            .border_1()
            .border_color(alpha(theme.foreground, 0.1))
            .children(self.children);
        root.style().refine(&self.style);
        root
    }
}

/// group/card-header @container/card-header grid auto-rows-min items-start
/// gap-1 rounded-t-xl px-(--card-spacing)
/// has-data-[slot=card-action]:grid-cols-[1fr_auto]
/// has-data-[slot=card-description]:grid-rows-[auto_auto]
///
/// Size/spacing must match the parent Card — gpui has no CSS variable cascade.
/// When `.action(...)` is set, renders as a row: left column (title + description)
/// + action at the end (`grid-cols-[1fr_auto]` / `row-span-2`).
/// Sizing and shape overrides come from the caller via [`Styled`].
#[derive(IntoElement)]
pub struct CardHeader {
    size: CardSize,
    spacing_override: Option<Pixels>,
    action: Option<AnyElement>,
    style: StyleRefinement,
    children: Vec<AnyElement>,
}

impl CardHeader {
    pub fn new() -> Self {
        Self {
            size: CardSize::Default,
            spacing_override: None,
            action: None,
            style: StyleRefinement::default(),
            children: Vec::new(),
        }
    }

    pub fn size(mut self, size: CardSize) -> Self {
        self.size = size;
        self
    }

    /// Override the size preset spacing (shadcn `[--card-spacing:*]`).
    pub fn spacing(mut self, spacing: Pixels) -> Self {
        self.spacing_override = Some(spacing);
        self
    }

    /// Slot for a trailing header action (button, badge, …). Mirrors nova's
    /// `has-data-[slot=card-action]:grid-cols-[1fr_auto]` layout: children sit in
    /// a flex-1 column, action is self-start at the row end.
    pub fn action(mut self, action: impl IntoElement) -> Self {
        self.action = Some(action.into_any_element());
        self
    }

    fn spacing_px(&self) -> Pixels {
        self.spacing_override
            .unwrap_or_else(|| px(self.size.spacing()))
    }
}

impl Default for CardHeader {
    fn default() -> Self {
        Self::new()
    }
}

impl Styled for CardHeader {
    fn style(&mut self) -> &mut StyleRefinement {
        &mut self.style
    }
}

impl ParentElement for CardHeader {
    fn extend(&mut self, elements: impl IntoIterator<Item = AnyElement>) {
        self.children.extend(elements);
    }
}

impl RenderOnce for CardHeader {
    fn render(self, _window: &mut Window, _cx: &mut App) -> impl IntoElement {
        let spacing = self.spacing_px();
        let body = div()
            .flex()
            .flex_col()
            .gap(px(4.))
            .flex_1()
            .min_w_0()
            .children(self.children);

        let mut root = div()
            .flex()
            .when(self.action.is_some(), |el| {
                el.flex_row().items_start().gap(px(4.))
            })
            .when(self.action.is_none(), |el| el.flex_col().gap(px(4.)))
            .px(spacing)
            .child(body)
            .when_some(self.action, |el, action| {
                el.child(CardAction::new().child(action))
            });
        root.style().refine(&self.style);
        root
    }
}

/// cn-font-heading text-base leading-snug font-medium
/// group-data-[size=sm]/card:text-sm
///
/// Sizing and shape overrides come from the caller via [`Styled`].
#[derive(IntoElement)]
pub struct CardTitle {
    size: CardSize,
    style: StyleRefinement,
    children: Vec<AnyElement>,
}

impl CardTitle {
    pub fn new() -> Self {
        Self {
            size: CardSize::Default,
            style: StyleRefinement::default(),
            children: Vec::new(),
        }
    }

    /// Title size follows the parent card's `data-size`
    /// (`group-data-[size=sm]/card:text-sm`). Default = 16px / 22px (leading-snug);
    /// Sm = 14px / 19px.
    pub fn size(mut self, size: CardSize) -> Self {
        self.size = size;
        self
    }
}

impl Default for CardTitle {
    fn default() -> Self {
        Self::new()
    }
}

impl Styled for CardTitle {
    fn style(&mut self) -> &mut StyleRefinement {
        &mut self.style
    }
}

impl ParentElement for CardTitle {
    fn extend(&mut self, elements: impl IntoIterator<Item = AnyElement>) {
        self.children.extend(elements);
    }
}

impl RenderOnce for CardTitle {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let theme = Theme::of(cx);
        let (text_size, line_height) = match self.size {
            CardSize::Default => (px(16.), px(22.)),
            CardSize::Sm => (px(14.), px(19.)),
        };

        let mut root = div()
            .text_size(text_size)
            .line_height(line_height)
            .font_weight(FontWeight::MEDIUM)
            .when_some(theme.heading_font(), |el, f| el.font_family(f))
            .children(self.children);
        root.style().refine(&self.style);
        root
    }
}

/// text-sm text-muted-foreground
///
/// Sizing and shape overrides come from the caller via [`Styled`].
#[derive(IntoElement)]
pub struct CardDescription {
    style: StyleRefinement,
    children: Vec<AnyElement>,
}

impl CardDescription {
    pub fn new() -> Self {
        Self {
            style: StyleRefinement::default(),
            children: Vec::new(),
        }
    }
}

impl Default for CardDescription {
    fn default() -> Self {
        Self::new()
    }
}

impl Styled for CardDescription {
    fn style(&mut self) -> &mut StyleRefinement {
        &mut self.style
    }
}

impl ParentElement for CardDescription {
    fn extend(&mut self, elements: impl IntoIterator<Item = AnyElement>) {
        self.children.extend(elements);
    }
}

impl RenderOnce for CardDescription {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let theme = Theme::of(cx);

        let mut root = div()
            .text_size(px(14.))
            .text_color(theme.muted_foreground)
            .children(self.children);
        root.style().refine(&self.style);
        root
    }
}

/// col-start-2 row-span-2 row-start-1 self-start justify-self-end
///
/// Aligned to the end of a flex row (source places action in a grid end cell).
/// `CardHeader::action(...)` wraps its element in this part; it also composes
/// standalone for call sites that build the header row themselves.
/// Sizing and shape overrides come from the caller via [`Styled`].
#[derive(IntoElement)]
pub struct CardAction {
    style: StyleRefinement,
    children: Vec<AnyElement>,
}

impl CardAction {
    pub fn new() -> Self {
        Self {
            style: StyleRefinement::default(),
            children: Vec::new(),
        }
    }
}

impl Default for CardAction {
    fn default() -> Self {
        Self::new()
    }
}

impl Styled for CardAction {
    fn style(&mut self) -> &mut StyleRefinement {
        &mut self.style
    }
}

impl ParentElement for CardAction {
    fn extend(&mut self, elements: impl IntoIterator<Item = AnyElement>) {
        self.children.extend(elements);
    }
}

impl RenderOnce for CardAction {
    fn render(self, _window: &mut Window, _cx: &mut App) -> impl IntoElement {
        let mut root = div().ml_auto().self_start().children(self.children);
        root.style().refine(&self.style);
        root
    }
}

/// px-(--card-spacing)
///
/// Size/spacing must match the parent Card — gpui has no CSS variable cascade.
/// Use `.flush_bottom()` (`-mb-(--card-spacing)`) for edge-to-edge content above
/// a footer. Sizing and shape overrides come from the caller via [`Styled`].
#[derive(IntoElement)]
pub struct CardContent {
    size: CardSize,
    spacing_override: Option<Pixels>,
    flush_bottom: bool,
    style: StyleRefinement,
    children: Vec<AnyElement>,
}

impl CardContent {
    pub fn new() -> Self {
        Self {
            size: CardSize::Default,
            spacing_override: None,
            flush_bottom: false,
            style: StyleRefinement::default(),
            children: Vec::new(),
        }
    }

    pub fn size(mut self, size: CardSize) -> Self {
        self.size = size;
        self
    }

    /// Override the size preset spacing (shadcn `[--card-spacing:*]`).
    pub fn spacing(mut self, spacing: Pixels) -> Self {
        self.spacing_override = Some(spacing);
        self
    }

    /// Negative bottom margin equal to the card spacing
    /// (`-mb-(--card-spacing)`) so content can run edge-to-edge above a footer.
    pub fn flush_bottom(mut self) -> Self {
        self.flush_bottom = true;
        self
    }

    fn spacing_px(&self) -> Pixels {
        self.spacing_override
            .unwrap_or_else(|| px(self.size.spacing()))
    }
}

impl Default for CardContent {
    fn default() -> Self {
        Self::new()
    }
}

impl Styled for CardContent {
    fn style(&mut self) -> &mut StyleRefinement {
        &mut self.style
    }
}

impl ParentElement for CardContent {
    fn extend(&mut self, elements: impl IntoIterator<Item = AnyElement>) {
        self.children.extend(elements);
    }
}

impl RenderOnce for CardContent {
    fn render(self, _window: &mut Window, _cx: &mut App) -> impl IntoElement {
        let spacing = self.spacing_px();

        let mut root = div()
            .px(spacing)
            .when(self.flush_bottom, |el| el.mb(-spacing))
            .children(self.children);
        root.style().refine(&self.style);
        root
    }
}

/// flex items-center rounded-b-xl border-t bg-muted/50 p-(--card-spacing)
///
/// Size/spacing must match the parent Card — gpui has no CSS variable cascade.
/// Always applies `mb(-spacing)` so the card root's bottom `py` is cancelled
/// (`has-data-[slot=card-footer]:pb-0`). **The footer must be the last child**
/// of the Card for this emulation to hold.
/// Sizing and shape overrides come from the caller via [`Styled`].
#[derive(IntoElement)]
pub struct CardFooter {
    size: CardSize,
    spacing_override: Option<Pixels>,
    style: StyleRefinement,
    children: Vec<AnyElement>,
}

impl CardFooter {
    pub fn new() -> Self {
        Self {
            size: CardSize::Default,
            spacing_override: None,
            style: StyleRefinement::default(),
            children: Vec::new(),
        }
    }

    pub fn size(mut self, size: CardSize) -> Self {
        self.size = size;
        self
    }

    /// Override the size preset spacing (shadcn `[--card-spacing:*]`).
    pub fn spacing(mut self, spacing: Pixels) -> Self {
        self.spacing_override = Some(spacing);
        self
    }

    fn spacing_px(&self) -> Pixels {
        self.spacing_override
            .unwrap_or_else(|| px(self.size.spacing()))
    }
}

impl Default for CardFooter {
    fn default() -> Self {
        Self::new()
    }
}

impl Styled for CardFooter {
    fn style(&mut self) -> &mut StyleRefinement {
        &mut self.style
    }
}

impl ParentElement for CardFooter {
    fn extend(&mut self, elements: impl IntoIterator<Item = AnyElement>) {
        self.children.extend(elements);
    }
}

impl RenderOnce for CardFooter {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let theme = Theme::of(cx);
        let spacing = self.spacing_px();

        let mut root = div()
            .flex()
            .flex_row()
            .items_center()
            .p(spacing)
            .border_t_1()
            .border_color(theme.border)
            .bg(alpha(theme.muted, 0.5))
            // Cancel the card root's bottom py — footer must be last child.
            .mb(-spacing)
            .children(self.children);
        root.style().refine(&self.style);
        root
    }
}
