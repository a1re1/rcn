//! Bubble — port of shadcn base-vega `ui/bubble.tsx`.
//!
//! Chat bubbles for [`Message`](crate::components::Message) content:
//! variants color the rounded-xl content pill; `BubbleReactions` floats a
//! reaction pill on a corner. The `tinted` variant derives chroma/hue from
//! the theme primary via HSL→sRGB→OKLab→LCh, then rebuilds with
//! `oklch(l, c*0.4, h)` (light L=0.93 / dark L=0.3).
//!
//! Interactive content (shadcn `BubbleContent render={<button/>}`): set
//! [`Bubble::id`] + [`Bubble::on_click`] so the content pill is focusable and
//! clickable. Per-variant hover backgrounds match the source's
//! `:is(button,a):hover` rules. Hover is instant — source
//! `transition-colors` (150ms) cannot animate under gpui hover styles (same
//! divergence as Button / Item).

use gpui::{
    AnyElement, App, BoxShadow, ClickEvent, ElementId, Hsla, InteractiveElement as _, IntoElement,
    ParentElement, RenderOnce, StatefulInteractiveElement as _, Styled, Window, div, point,
    prelude::FluentBuilder as _, px,
};

use crate::motion;
use crate::theme::{Theme, alpha, oklch};

#[derive(Clone, Copy, PartialEq, Eq, Default, Debug)]
pub enum BubbleVariant {
    #[default]
    Default,
    Secondary,
    Muted,
    Tinted,
    Outline,
    Ghost,
    Destructive,
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

/// group/bubble: relative flex w-fit max-w-[80%] min-w-0 flex-col gap-1
/// data-[align=end]:self-end; ghost also gets max-w-full.
#[derive(IntoElement)]
pub struct Bubble {
    variant: BubbleVariant,
    align: BubbleAlign,
    id: Option<ElementId>,
    on_click: Option<Box<dyn Fn(&ClickEvent, &mut Window, &mut App) + 'static>>,
    /// Body nodes for the content pill (built in `render` so id/on_click apply).
    content_children: Vec<AnyElement>,
    /// Non-content children (e.g. [`BubbleReactions`]) via [`ParentElement`].
    children: Vec<AnyElement>,
}

impl Bubble {
    pub fn new() -> Self {
        Self {
            variant: BubbleVariant::default(),
            align: BubbleAlign::default(),
            id: None,
            on_click: None,
            content_children: Vec::new(),
            children: Vec::new(),
        }
    }

    pub fn variant(mut self, variant: BubbleVariant) -> Self {
        self.variant = variant;
        self
    }

    /// Horizontal alignment within a column flex parent (`data-align`).
    /// Start → `self-start`, End → `self-end` (content-hugging + side).
    pub fn align(mut self, align: BubbleAlign) -> Self {
        self.align = align;
        self
    }

    /// Element id for interactive content (shadcn `render={<button/>}`).
    /// Required with [`Bubble::on_click`] so the pill is focusable.
    pub fn id(mut self, id: impl Into<ElementId>) -> Self {
        self.id = Some(id.into());
        self
    }

    /// Click handler for interactive content. With [`Bubble::id`], the content
    /// pill becomes a real control: tab-focus, focus-visible ring, cursor, and
    /// per-variant hover (instant — see module docs).
    pub fn on_click(
        mut self,
        handler: impl Fn(&ClickEvent, &mut Window, &mut App) + 'static,
    ) -> Self {
        self.on_click = Some(Box::new(handler));
        self
    }

    /// The bubble body text/content (`BubbleContent`).
    pub fn content(mut self, content: impl IntoElement) -> Self {
        self.content_children.push(content.into_any_element());
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
        let mut kids = Vec::with_capacity(1 + self.children.len());
        if !self.content_children.is_empty() || self.id.is_some() || self.on_click.is_some() {
            kids.push(
                BubbleContent {
                    variant: self.variant,
                    id: self.id,
                    on_click: self.on_click,
                    children: self.content_children,
                }
                .into_any_element(),
            );
        }
        kids.extend(self.children);

        div()
            .relative()
            .flex()
            .flex_col()
            .gap(px(4.))
            .min_w(px(0.))
            .map(|el| match self.variant {
                // ghost: max-w-full (no 80% cap)
                BubbleVariant::Ghost => el.max_w_full(),
                _ => el.max_w(gpui::relative(0.8)),
            })
            .map(|el| match self.align {
                BubbleAlign::Start => el.self_start(),
                BubbleAlign::End => el.self_end(),
            })
            .children(kids)
    }
}

/// Stack of bubbles: `flex min-w-0 flex-col gap-2`.
///
/// Groups consecutive messages from the same side so they share spacing
/// without wrapping each bubble in a separate row container.
#[derive(IntoElement)]
pub struct BubbleGroup {
    children: Vec<AnyElement>,
}

impl BubbleGroup {
    pub fn new() -> Self {
        Self {
            children: Vec::new(),
        }
    }
}

impl Default for BubbleGroup {
    fn default() -> Self {
        Self::new()
    }
}

impl ParentElement for BubbleGroup {
    fn extend(&mut self, elements: impl IntoIterator<Item = AnyElement>) {
        self.children.extend(elements);
    }
}

impl RenderOnce for BubbleGroup {
    fn render(self, _window: &mut Window, _cx: &mut App) -> impl IntoElement {
        div()
            .flex()
            .flex_col()
            .min_w(px(0.))
            .gap(px(8.))
            .children(self.children)
    }
}

/// Linear interpolate two colors in linear-sRGB (approx. for color-mix).
fn mix(a: Hsla, b: Hsla, t: f32) -> Hsla {
    let (ar, ag, ab) = hsla_to_linear_srgb(a);
    let (br, bg, bb) = hsla_to_linear_srgb(b);
    let r = ar + (br - ar) * t;
    let g = ag + (bg - ag) * t;
    let bl = ab + (bb - ab) * t;
    let enc = |x: f32| {
        let x = x.clamp(0., 1.);
        if x <= 0.0031308 {
            12.92 * x
        } else {
            1.055 * x.powf(1. / 2.4) - 0.055
        }
    };
    gpui::Rgba {
        r: enc(r),
        g: enc(g),
        b: enc(bl),
        a: a.a + (b.a - a.a) * t,
    }
    .into()
}

fn hsla_to_linear_srgb(c: Hsla) -> (f32, f32, f32) {
    let (r, g, b) = hsla_to_srgb(c);
    let dec = |x: f32| {
        if x <= 0.04045 {
            x / 12.92
        } else {
            ((x + 0.055) / 1.055).powf(2.4)
        }
    };
    (dec(r), dec(g), dec(b))
}

fn hsla_to_srgb(c: Hsla) -> (f32, f32, f32) {
    let h = c.h.fract();
    let s = c.s.clamp(0., 1.);
    let l = c.l.clamp(0., 1.);
    if s == 0. {
        return (l, l, l);
    }
    let q = if l < 0.5 { l * (1. + s) } else { l + s - l * s };
    let p = 2. * l - q;
    let hue = |t: f32| {
        let t = t.rem_euclid(1.);
        if t < 1. / 6. {
            p + (q - p) * 6. * t
        } else if t < 1. / 2. {
            q
        } else if t < 2. / 3. {
            p + (q - p) * (2. / 3. - t) * 6.
        } else {
            p
        }
    };
    (hue(h + 1. / 3.), hue(h), hue(h - 1. / 3.))
}

/// Extract OKLCh (L, C, h_deg) from an Hsla via HSL→sRGB→linear→OKLab→LCh.
fn hsla_to_oklch(c: Hsla) -> (f32, f32, f32) {
    let (r, g, b) = hsla_to_linear_srgb(c);
    let l_ = 0.4122214708 * r + 0.5363325363 * g + 0.0514459929 * b;
    let m_ = 0.2119034982 * r + 0.6806995451 * g + 0.1073969566 * b;
    let s_ = 0.0883024619 * r + 0.2817188376 * g + 0.6299787005 * b;
    let l_c = l_.cbrt();
    let m_c = m_.cbrt();
    let s_c = s_.cbrt();
    let l = 0.2104542553 * l_c + 0.7936177850 * m_c - 0.0040720468 * s_c;
    let a = 1.9779984951 * l_c - 2.4285922050 * m_c + 0.4505937099 * s_c;
    let b_ok = 0.0259040371 * l_c + 0.7827717662 * m_c - 0.8086757660 * s_c;
    let chroma = (a * a + b_ok * b_ok).sqrt();
    let h_deg = if chroma < 1e-6 {
        0.
    } else {
        b_ok.atan2(a).to_degrees().rem_euclid(360.)
    };
    (l, chroma, h_deg)
}

/// The colored pill: rounded-xl border px-3 py-2 text-sm leading-relaxed.
/// Ghost drops padding/rounding (`rounded-none bg-transparent p-0`).
///
/// With `id` set (interactive / `render={<button/>}`), applies tab focus,
/// focus-visible ring, pointer cursor, and per-variant hover. Hover is
/// instant — source `transition-colors` cannot animate under gpui.
#[derive(IntoElement)]
struct BubbleContent {
    variant: BubbleVariant,
    id: Option<ElementId>,
    on_click: Option<Box<dyn Fn(&ClickEvent, &mut Window, &mut App) + 'static>>,
    children: Vec<AnyElement>,
}

impl RenderOnce for BubbleContent {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let theme = Theme::of(cx).clone();
        let dark = theme.dark;
        // leading-relaxed = 1.625 × 14px
        let base = div()
            .w_auto()
            .max_w_full()
            .min_w(px(0.))
            .overflow_hidden()
            .border_1()
            .border_color(gpui::transparent_black())
            .text_size(px(14.))
            .line_height(px(22.75));
        let base = match self.variant {
            BubbleVariant::Ghost => base.rounded(px(0.)).px(px(0.)).py(px(0.)),
            _ => base.rounded(theme.radius_xl()).px(px(12.)).py(px(8.)),
        };

        // Tinted needs primary chroma/hue for both resting + hover.
        let (_l, tint_c, tint_h) = hsla_to_oklch(theme.primary);

        let base = match self.variant {
            BubbleVariant::Default => base.bg(theme.primary).text_color(theme.primary_foreground),
            BubbleVariant::Secondary => base
                .bg(theme.secondary)
                .text_color(theme.secondary_foreground),
            BubbleVariant::Muted => base.bg(theme.muted).text_color(theme.foreground),
            // oklch(from primary 0.93 c*0.4 h) light / 0.3 dark; text-foreground
            BubbleVariant::Tinted => {
                let bg = if dark {
                    oklch(0.3, tint_c * 0.4, tint_h)
                } else {
                    oklch(0.93, tint_c * 0.4, tint_h)
                };
                base.bg(bg).text_color(theme.foreground)
            }
            BubbleVariant::Outline => base
                .border_color(theme.border)
                .bg(theme.background)
                .text_color(theme.foreground),
            // ghost: border-none rounded-none bg-transparent p-0
            BubbleVariant::Ghost => base.text_color(theme.foreground),
            // bg-destructive/10 text-destructive; dark bg-destructive/20
            BubbleVariant::Destructive => base
                .bg(alpha(theme.destructive, if dark { 0.2 } else { 0.1 }))
                .text_color(theme.destructive),
        };

        // Interactive branch (shadcn `:is(button,a)`): id → Stateful, tab +
        // focus ring + cursor + per-variant hover + on_click. Enter/Space
        // activate via gpui's focused-element click handling (same as Button).
        // Source `transition-colors` (150ms) is not animatable under gpui hover.
        if let Some(id) = self.id {
            let ring = motion::focus_ring(&theme);
            let ring_border = theme.ring;
            let hover_bg = match self.variant {
                BubbleVariant::Default => alpha(theme.primary, 0.8),
                BubbleVariant::Secondary => mix(theme.secondary, theme.foreground, 0.05),
                BubbleVariant::Muted => mix(theme.muted, theme.foreground, 0.05),
                // hover oklch(from primary 0.88 c*0.5 h) / dark 0.35
                BubbleVariant::Tinted => {
                    if dark {
                        oklch(0.35, tint_c * 0.5, tint_h)
                    } else {
                        oklch(0.88, tint_c * 0.5, tint_h)
                    }
                }
                // outline: hover bg-muted text-foreground; dark hover bg-input/30
                BubbleVariant::Outline => {
                    if dark {
                        alpha(theme.input, 0.3)
                    } else {
                        theme.muted
                    }
                }
                // ghost: hover bg-muted; dark hover bg-muted/50
                BubbleVariant::Ghost => {
                    if dark {
                        alpha(theme.muted, 0.5)
                    } else {
                        theme.muted
                    }
                }
                // destructive: hover bg-destructive/20; dark /30
                BubbleVariant::Destructive => {
                    alpha(theme.destructive, if dark { 0.3 } else { 0.2 })
                }
            };
            let hover_fg = match self.variant {
                BubbleVariant::Outline | BubbleVariant::Ghost => Some(theme.foreground),
                _ => None,
            };

            base.id(id)
                .tab_index(0)
                .focus_visible(move |s| s.border_color(ring_border).shadow(ring.clone()))
                .hover(move |s| {
                    let s = s.bg(hover_bg);
                    match hover_fg {
                        Some(fg) => s.text_color(fg),
                        None => s,
                    }
                })
                .when_some(self.on_click, |el, on_click| el.on_click(on_click))
                .children(self.children)
                .into_any_element()
        } else {
            base.children(self.children).into_any_element()
        }
    }
}

/// Reaction pill floated on a corner of the bubble: rounded-full bg-muted
/// px-1.5 py-0.5 with a 3px `ring-card` (BoxShadow spread, blur 0).
///
/// Defaults: side=bottom, align=end. Offsets approximate `-translate-y-3/4`
/// of the ~24px pill (`top/bottom: -18px`) and `left/right: 12px`.
#[derive(IntoElement)]
pub struct BubbleReactions {
    side: BubbleSide,
    align: BubbleAlign,
    /// When true, zero padding — shadcn `has-[button]:p-0` equivalent for
    /// button children.
    buttons: bool,
    children: Vec<AnyElement>,
}

impl BubbleReactions {
    pub fn new() -> Self {
        Self {
            side: BubbleSide::default(),
            align: BubbleAlign::default(),
            buttons: false,
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

    /// Zero the pill padding — equivalent of shadcn `has-[button]:p-0`.
    /// Use when children are Buttons (e.g. action chips inside reactions).
    pub fn buttons(mut self) -> Self {
        self.buttons = true;
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
        // ring-3 ring-card: opaque pill bg so a 3px spread shadow reads as a ring.
        let ring = vec![BoxShadow {
            color: theme.card,
            offset: point(px(0.), px(0.)),
            blur_radius: px(0.),
            spread_radius: px(3.),
            inset: false,
        }];
        div()
            .absolute()
            .flex()
            .flex_row()
            .items_center()
            .justify_center()
            .gap(px(4.))
            .rounded_full()
            .bg(theme.muted)
            .shadow(ring)
            .text_size(px(14.))
            .when(!self.buttons, |el| el.px(px(6.)).py(px(2.)))
            .map(|el| match self.side {
                // -translate-y-3/4 of ~24px pill ≈ 18px
                BubbleSide::Top => el.top(px(-18.)),
                BubbleSide::Bottom => el.bottom(px(-18.)),
            })
            .map(|el| match self.align {
                BubbleAlign::Start => el.left(px(12.)),
                BubbleAlign::End => el.right(px(12.)),
            })
            .children(self.children)
    }
}
