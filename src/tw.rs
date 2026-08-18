//! tw — prototype Tailwind class-string → gpui [`StyleRefinement`] parser.
//!
//! `parse(theme, "flex h-9 px-2.5 bg-primary hover:bg-primary/80")` resolves a
//! Tailwind class string into per-state [`StyleRefinement`] buckets (base,
//! hover, focus, focus-visible, active, disabled) that plug directly into
//! gpui's state-style hooks (`hover()`, `focus_visible()`, `active()` all take
//! `FnOnce(StyleRefinement) -> StyleRefinement`). The extension traits at the
//! bottom apply a whole class string to an element in one call.
//!
//! Scope: the utilities used by the shadcn components rcn ports (layout,
//! flexbox, spacing, sizing, colors with `/NN` alpha, borders, radius
//! including `rounded-[min(var(--radius-md),Npx)]`, typography, shadows,
//! rings, opacity), plus arbitrary values (`w-[137px]`, `bg-[#323232]`).
//!
//! Deliberate deviations from real Tailwind/CSS:
//! - **Order-sensitive**: later classes in the string win over earlier ones
//!   for the same property (like `tailwind-merge`, unlike raw CSS selector
//!   specificity). `dark:` and state variants therefore belong *after* the
//!   classes they override, which is how shadcn's cva strings are written.
//! - **`dark:` resolves at parse time** against `theme.dark` — the token is
//!   applied or dropped, not kept as a runtime condition.
//! - **`ring-*` synthesizes a box shadow** (zero blur, spread = ring width),
//!   appended to the bucket's `box_shadow`, matching `motion::focus_ring`.
//! - **`translate-y-*` maps to a `top` inset** — gpui has no transforms. The
//!   element needs `relative` for the offset to take effect.
//! - **No transitions/animations**: `transition-*`/`duration-*`/`ease-*` are
//!   recorded in `skipped`; state changes are instant in gpui.
//!
//! Unrecognized tokens land in [`TwStyles::unknown`]; recognized-but-unmappable
//! tokens (`select-none`, `w-fit`, `underline-offset-4`, …) in
//! [`TwStyles::skipped`], so components can assert full coverage in tests.

use gpui::{
    BoxShadow, FontWeight, Hsla, InteractiveElement, Pixels, Refineable as _,
    StatefulInteractiveElement, StyleRefinement, Styled, point, px,
};

use crate::theme::{Theme, alpha};

/// Parsed class string: one refinement per interaction state.
#[derive(Default, Debug)]
pub struct TwStyles {
    pub base: StyleRefinement,
    pub hover: Option<StyleRefinement>,
    pub focus: Option<StyleRefinement>,
    pub focus_visible: Option<StyleRefinement>,
    pub active: Option<StyleRefinement>,
    pub disabled: Option<StyleRefinement>,
    /// Recognized Tailwind utilities with no gpui equivalent (dropped).
    pub skipped: Vec<String>,
    /// Tokens the parser does not understand at all.
    pub unknown: Vec<String>,
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum Bucket {
    Base = 0,
    Hover = 1,
    Focus = 2,
    FocusVisible = 3,
    Active = 4,
    Disabled = 5,
}

/// `ring-*` accumulates width + color separately (like CSS custom props
/// `--tw-ring-*`), then [`parse`] synthesizes one box shadow per bucket.
#[derive(Default, Clone, Copy)]
struct RingState {
    width: Option<Pixels>,
    color: Option<Hsla>,
}

enum Outcome {
    Applied,
    Skipped,
    Unknown,
}

/// Parse a Tailwind class string into per-state style refinements.
pub fn parse(theme: &Theme, classes: &str) -> TwStyles {
    let mut out = TwStyles::default();
    let mut rings = [RingState::default(); 6];

    for token in classes.split_whitespace() {
        let mut bucket = Bucket::Base;
        let mut dark_only = false;
        let mut known_prefixes = true;
        let mut rest = token;
        while let Some((prefix, tail)) = rest.split_once(':') {
            match prefix {
                "dark" => dark_only = true,
                "hover" => bucket = Bucket::Hover,
                "focus" => bucket = Bucket::Focus,
                "focus-visible" => bucket = Bucket::FocusVisible,
                "active" => bucket = Bucket::Active,
                "disabled" => bucket = Bucket::Disabled,
                // `[a]:`, `group-hover:`, `aria-invalid:`, `sm:`, …
                _ => known_prefixes = false,
            }
            rest = tail;
        }
        if !known_prefixes {
            out.skipped.push(token.to_string());
            continue;
        }
        if dark_only && !theme.dark {
            continue;
        }

        let slot = bucket_mut(&mut out, bucket);
        let (refined, outcome) = apply(
            std::mem::take(slot),
            rest,
            theme,
            &mut rings[bucket as usize],
        );
        *slot = refined;
        match outcome {
            Outcome::Applied => {}
            Outcome::Skipped => out.skipped.push(token.to_string()),
            Outcome::Unknown => out.unknown.push(token.to_string()),
        }
    }

    for bucket in [
        Bucket::Base,
        Bucket::Hover,
        Bucket::Focus,
        Bucket::FocusVisible,
        Bucket::Active,
        Bucket::Disabled,
    ] {
        let ring = rings[bucket as usize];
        if ring.width.is_none() && ring.color.is_none() {
            continue;
        }
        let shadow = BoxShadow {
            // Tailwind's default ring is currentColor; shadcn always names a
            // color, so default to the theme ring at 50% like `ring-ring/50`.
            color: ring.color.unwrap_or(alpha(theme.ring, 0.5)),
            offset: point(px(0.), px(0.)),
            blur_radius: px(0.),
            spread_radius: ring.width.unwrap_or(px(1.)),
            inset: false,
        };
        let slot = bucket_mut(&mut out, bucket);
        match &mut slot.box_shadow {
            Some(shadows) => shadows.push(shadow),
            None => slot.box_shadow = Some(vec![shadow]),
        }
    }

    out
}

fn bucket_mut(out: &mut TwStyles, bucket: Bucket) -> &mut StyleRefinement {
    match bucket {
        Bucket::Base => &mut out.base,
        Bucket::Hover => out.hover.get_or_insert_default(),
        Bucket::Focus => out.focus.get_or_insert_default(),
        Bucket::FocusVisible => out.focus_visible.get_or_insert_default(),
        Bucket::Active => out.active.get_or_insert_default(),
        Bucket::Disabled => out.disabled.get_or_insert_default(),
    }
}

/// Apply one (variant-stripped) utility token to a refinement.
fn apply(
    s: StyleRefinement,
    token: &str,
    theme: &Theme,
    ring: &mut RingState,
) -> (StyleRefinement, Outcome) {
    use Outcome::*;

    let (t, neg) = match token.strip_prefix('-') {
        Some(rest) => (rest, true),
        None => (token, false),
    };
    let sign = if neg { -1. } else { 1. };

    // Exact utilities.
    match t {
        "flex" | "inline-flex" => return (s.flex(), Applied),
        "flex-row" => return (s.flex_row(), Applied),
        "flex-col" => return (s.flex_col(), Applied),
        "flex-wrap" => return (s.flex_wrap(), Applied),
        "flex-1" => return (s.flex_1(), Applied),
        "flex-none" => return (s.flex_none(), Applied),
        "grow" => return (s.flex_grow(1.), Applied),
        "shrink-0" => return (s.flex_shrink_0(), Applied),
        "items-start" => return (s.items_start(), Applied),
        "items-center" => return (s.items_center(), Applied),
        "items-end" => return (s.items_end(), Applied),
        "justify-start" => return (s.justify_start(), Applied),
        "justify-center" => return (s.justify_center(), Applied),
        "justify-end" => return (s.justify_end(), Applied),
        "justify-between" => return (s.justify_between(), Applied),
        "relative" => return (s.relative(), Applied),
        "absolute" => return (s.absolute(), Applied),
        "overflow-hidden" => return (s.overflow_hidden(), Applied),
        "whitespace-nowrap" => return (s.whitespace_nowrap(), Applied),
        "underline" => return (s.underline(), Applied),
        "w-full" => return (s.w_full(), Applied),
        "h-full" => return (s.h_full(), Applied),
        "border" => return (s.border_1(), Applied),
        "border-0" => return (s.border_0(), Applied),
        "border-2" => return (s.border_2(), Applied),
        "border-4" => return (s.border_4(), Applied),
        "rounded" => return (s.rounded(px(4.)), Applied),
        "font-normal" => return (s.font_weight(FontWeight::NORMAL), Applied),
        "font-medium" => return (s.font_weight(FontWeight::MEDIUM), Applied),
        "font-semibold" => return (s.font_weight(FontWeight::SEMIBOLD), Applied),
        "font-bold" => return (s.font_weight(FontWeight::BOLD), Applied),
        "shadow-2xs" => return (s.shadow_2xs(), Applied),
        "shadow-xs" => return (s.shadow_xs(), Applied),
        "shadow-sm" => return (s.shadow_sm(), Applied),
        "shadow-md" => return (s.shadow_md(), Applied),
        "shadow-lg" => return (s.shadow_lg(), Applied),
        "shadow-none" => return (s.shadow_none(), Applied),
        // Recognized, but gpui has no equivalent (or it's the default).
        "w-fit"
        | "select-none"
        | "outline-none"
        | "pointer-events-none"
        | "no-underline"
        | "cursor-pointer"
        | "cursor-default"
        | "antialiased" => return (s, Skipped),
        _ => {}
    }

    // Skip-list prefixes: real Tailwind, no gpui mapping.
    for skip in [
        "transition",
        "duration-",
        "ease-",
        "underline-offset-",
        "outline-",
        "animate-",
    ] {
        if t.starts_with(skip) {
            return (s, Skipped);
        }
    }

    // Sizing.
    if let Some(v) = t.strip_prefix("size-")
        && let Some(l) = scale_px(v)
    {
        return (s.size(l), Applied);
    }
    if let Some(v) = t.strip_prefix("h-")
        && let Some(l) = scale_px(v)
    {
        return (s.h(l), Applied);
    }
    if let Some(v) = t.strip_prefix("w-")
        && let Some(l) = scale_px(v)
    {
        return (s.w(l), Applied);
    }
    if let Some(v) = t.strip_prefix("min-w-")
        && let Some(l) = scale_px(v)
    {
        return (s.min_w(l), Applied);
    }
    if let Some(v) = t.strip_prefix("min-h-")
        && let Some(l) = scale_px(v)
    {
        return (s.min_h(l), Applied);
    }
    if let Some(v) = t.strip_prefix("max-w-")
        && let Some(l) = scale_px(v)
    {
        return (s.max_w(l), Applied);
    }
    if let Some(v) = t.strip_prefix("max-h-")
        && let Some(l) = scale_px(v)
    {
        return (s.max_h(l), Applied);
    }

    // Padding. Longer prefixes first so `p-` doesn't shadow `pl-`.
    for (prefix, f) in [
        (
            "px-",
            pad_x as fn(StyleRefinement, Pixels) -> StyleRefinement,
        ),
        ("py-", pad_y),
        ("pl-", |s, l| s.pl(l)),
        ("pr-", |s, l| s.pr(l)),
        ("pt-", |s, l| s.pt(l)),
        ("pb-", |s, l| s.pb(l)),
        ("p-", |s, l| s.p(l)),
    ] {
        if let Some(v) = t.strip_prefix(prefix)
            && let Some(l) = scale_px(v)
        {
            return (f(s, l), Applied);
        }
    }

    // Margin (negatives allowed).
    for (prefix, f) in [
        (
            "mx-",
            margin_x as fn(StyleRefinement, Pixels) -> StyleRefinement,
        ),
        ("my-", margin_y),
        ("ml-", |s, l| s.ml(l)),
        ("mr-", |s, l| s.mr(l)),
        ("mt-", |s, l| s.mt(l)),
        ("mb-", |s, l| s.mb(l)),
        ("m-", |s, l| s.m(l)),
    ] {
        if let Some(v) = t.strip_prefix(prefix)
            && let Some(l) = scale_px(v)
        {
            return (f(s, l * sign), Applied);
        }
    }

    // Gap.
    if let Some(v) = t.strip_prefix("gap-x-")
        && let Some(l) = scale_px(v)
    {
        let mut s = s;
        s.gap.width = Some(l.into());
        return (s, Applied);
    }
    if let Some(v) = t.strip_prefix("gap-y-")
        && let Some(l) = scale_px(v)
    {
        let mut s = s;
        s.gap.height = Some(l.into());
        return (s, Applied);
    }
    if let Some(v) = t.strip_prefix("gap-")
        && let Some(l) = scale_px(v)
    {
        return (s.gap(l), Applied);
    }

    // Insets (negatives allowed).
    for (prefix, f) in [
        (
            "inset-",
            (|s, l| s.inset(l)) as fn(StyleRefinement, Pixels) -> StyleRefinement,
        ),
        ("top-", |s, l| s.top(l)),
        ("left-", |s, l| s.left(l)),
        ("right-", |s, l| s.right(l)),
        ("bottom-", |s, l| s.bottom(l)),
    ] {
        if let Some(v) = t.strip_prefix(prefix)
            && let Some(l) = scale_px(v)
        {
            return (f(s, l * sign), Applied);
        }
    }

    // translate-y → top inset (gpui has no transforms; needs `relative`).
    if let Some(v) = t.strip_prefix("translate-y-")
        && let Some(l) = scale_px(v)
    {
        return (s.top(l * sign), Applied);
    }

    // Radius.
    if let Some(v) = t.strip_prefix("rounded-") {
        let radius = match v {
            "none" => Some(px(0.)),
            "sm" => Some(theme.radius_sm()),
            "md" => Some(theme.radius_md()),
            "lg" => Some(theme.radius_lg()),
            "xl" => Some(theme.radius_xl()),
            "2xl" => Some(px(16.)),
            "3xl" => Some(px(24.)),
            "4xl" => Some(px(32.)),
            "full" => Some(px(9999.)),
            _ => arbitrary_radius(theme, v),
        };
        if let Some(r) = radius {
            return (s.rounded(r), Applied);
        }
    }

    // Ring: width and color accumulate, synthesized into a shadow by `parse`.
    if let Some(v) = t.strip_prefix("ring-") {
        // `ring-3` and `ring-[3px]` are widths in raw px (not the 4px scale).
        if let Ok(n) = v.parse::<f32>() {
            ring.width = Some(px(n));
            return (s, Applied);
        }
        if let Some(inner) = v.strip_prefix('[').and_then(|x| x.strip_suffix("px]"))
            && let Ok(n) = inner.parse::<f32>()
        {
            ring.width = Some(px(n));
            return (s, Applied);
        }
        if let Some(c) = color(theme, v) {
            ring.color = Some(c);
            return (s, Applied);
        }
    }

    // Typography + text color. Sizes/alignment before the color fallback.
    if let Some(v) = t.strip_prefix("text-") {
        match v {
            "xs" => return (s.text_size(px(12.)).line_height(px(16.)), Applied),
            "sm" => return (s.text_size(px(14.)).line_height(px(20.)), Applied),
            "base" => return (s.text_size(px(16.)).line_height(px(24.)), Applied),
            "lg" => return (s.text_size(px(18.)).line_height(px(28.)), Applied),
            "xl" => return (s.text_size(px(20.)).line_height(px(28.)), Applied),
            _ => {
                if let Some(c) = color(theme, v) {
                    return (s.text_color(c), Applied);
                }
            }
        }
    }

    // Background color.
    if let Some(v) = t.strip_prefix("bg-")
        && let Some(c) = color(theme, v)
    {
        return (s.bg(c), Applied);
    }

    // Border color ("border"/"border-N" widths matched earlier).
    if let Some(v) = t.strip_prefix("border-")
        && let Some(c) = color(theme, v)
    {
        return (s.border_color(c), Applied);
    }

    // Opacity.
    if let Some(v) = t.strip_prefix("opacity-")
        && let Ok(n) = v.parse::<f32>()
    {
        return (s.opacity(n / 100.), Applied);
    }

    (s, Unknown)
}

fn pad_x(s: StyleRefinement, l: Pixels) -> StyleRefinement {
    s.px(l)
}

fn pad_y(s: StyleRefinement, l: Pixels) -> StyleRefinement {
    s.py(l)
}

fn margin_x(s: StyleRefinement, l: Pixels) -> StyleRefinement {
    s.mx(l)
}

fn margin_y(s: StyleRefinement, l: Pixels) -> StyleRefinement {
    s.my(l)
}

/// Tailwind length: `2.5` → 10px (4px scale), `px` → 1px, `[Npx]`/`[Nrem]`
/// arbitrary values.
fn scale_px(v: &str) -> Option<Pixels> {
    if v == "px" {
        return Some(px(1.));
    }
    if let Some(inner) = v.strip_prefix('[').and_then(|x| x.strip_suffix(']')) {
        if let Some(n) = inner.strip_suffix("px") {
            return n.parse::<f32>().ok().map(px);
        }
        if let Some(n) = inner.strip_suffix("rem") {
            return n.parse::<f32>().ok().map(|r| px(r * 16.));
        }
        return None;
    }
    v.parse::<f32>().ok().map(|n| px(n * 4.))
}

/// `[Npx]` and shadcn's `[min(var(--radius-SCALE),Npx)]` radius clamp.
fn arbitrary_radius(theme: &Theme, v: &str) -> Option<Pixels> {
    let inner = v.strip_prefix('[')?.strip_suffix(']')?;
    if let Some(n) = inner.strip_suffix("px") {
        return n.parse::<f32>().ok().map(px);
    }
    let rest = inner.strip_prefix("min(var(--radius-")?;
    let (scale, tail) = rest.split_once(')')?;
    let cap: f32 = tail.strip_prefix(',')?.strip_suffix("px)")?.parse().ok()?;
    let base = match scale {
        "sm" => theme.radius_sm(),
        "md" => theme.radius_md(),
        "lg" => theme.radius_lg(),
        "xl" => theme.radius_xl(),
        _ => return None,
    };
    Some(base.min(px(cap)))
}

/// Resolve `primary`, `muted-foreground/50`, `transparent`, `[#323232]`, …
/// against the theme's semantic tokens.
fn color(theme: &Theme, spec: &str) -> Option<Hsla> {
    let (name, alpha_pct) = match spec.rsplit_once('/') {
        Some((n, a)) => (n, Some(a)),
        None => (spec, None),
    };
    let base: Hsla = if let Some(inner) = name.strip_prefix('[').and_then(|x| x.strip_suffix(']')) {
        let hex = inner.strip_prefix('#')?;
        if hex.len() != 6 {
            return None;
        }
        gpui::rgb(u32::from_str_radix(hex, 16).ok()?).into()
    } else {
        match name {
            "transparent" => gpui::transparent_black(),
            "white" => gpui::white(),
            "black" => gpui::black(),
            "background" => theme.background,
            "foreground" => theme.foreground,
            "card" => theme.card,
            "card-foreground" => theme.card_foreground,
            "popover" => theme.popover,
            "popover-foreground" => theme.popover_foreground,
            "primary" => theme.primary,
            "primary-foreground" => theme.primary_foreground,
            "secondary" => theme.secondary,
            "secondary-foreground" => theme.secondary_foreground,
            "muted" => theme.muted,
            "muted-foreground" => theme.muted_foreground,
            "accent" => theme.accent,
            "accent-foreground" => theme.accent_foreground,
            "destructive" => theme.destructive,
            "destructive-foreground" => theme.destructive_foreground,
            "border" => theme.border,
            "input" => theme.input,
            "ring" => theme.ring,
            _ => return None,
        }
    };
    match alpha_pct {
        Some(a) => Some(alpha(base, a.parse::<f32>().ok()? / 100.)),
        None => Some(base),
    }
}

/// Apply only the base bucket — for plain [`Styled`] elements (and nested
/// refinements) with no interaction states in the class string.
#[allow(dead_code)] // prototype: not yet used outside tests
pub trait TwStyledExt: Styled + Sized {
    fn tw_base(mut self, theme: &Theme, classes: &str) -> Self {
        // Unknown tokens are ignored here (they stay visible in
        // `TwStyles::unknown` for callers that parse themselves) so live
        // editing surfaces typos as diagnostics instead of panics.
        let styles = parse(theme, classes);
        self.style().refine(&styles.base);
        self
    }
}

impl<T: Styled> TwStyledExt for T {}

fn apply_interactive<T: InteractiveElement + Styled>(mut el: T, styles: TwStyles) -> T {
    el.style().refine(&styles.base);
    let mut el = el;
    if let Some(hover) = styles.hover {
        el = el.hover(move |mut r| {
            r.refine(&hover);
            r
        });
    }
    if let Some(focus) = styles.focus {
        el = el.focus(move |mut r| {
            r.refine(&focus);
            r
        });
    }
    if let Some(focus_visible) = styles.focus_visible {
        el = el.focus_visible(move |mut r| {
            r.refine(&focus_visible);
            r
        });
    }
    el
}

/// Apply base + hover + focus + focus-visible buckets. `active:` needs a
/// stateful element — use [`TwStatefulExt::tw_stateful`] there.
pub trait TwInteractiveExt: InteractiveElement + Styled + Sized {
    fn tw(self, theme: &Theme, classes: &str) -> Self {
        let styles = parse(theme, classes);
        debug_assert!(
            styles.active.is_none() && styles.disabled.is_none(),
            "active:/disabled: classes need tw_stateful / component state"
        );
        apply_interactive(self, styles)
    }
}

impl<T: InteractiveElement + Styled> TwInteractiveExt for T {}

/// Apply every bucket except `disabled:` (a component-state concern — gate it
/// with `.when(disabled, …)` and the [`TwStyles::disabled`] refinement).
pub trait TwStatefulExt: StatefulInteractiveElement + Styled + Sized {
    fn tw_stateful(self, theme: &Theme, classes: &str) -> Self {
        let mut styles = parse(theme, classes);
        let active = styles.active.take();
        let mut el = apply_interactive(self, styles);
        if let Some(active) = active {
            el = el.active(move |mut r| {
                r.refine(&active);
                r
            });
        }
        el
    }
}

impl<T: StatefulInteractiveElement + Styled> TwStatefulExt for T {}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::motion;
    use gpui::transparent_black;

    fn assert_style_eq(got: &StyleRefinement, expected: &StyleRefinement) {
        assert_eq!(format!("{got:?}"), format!("{expected:?}"));
    }

    #[test]
    fn layout_and_spacing() {
        let theme = Theme::light();
        let styles = parse(
            &theme,
            "flex items-center justify-center h-9 px-2.5 gap-1.5 top-px",
        );
        let expected = StyleRefinement::default()
            .flex()
            .items_center()
            .justify_center()
            .h(px(36.))
            .px(px(10.))
            .gap(px(6.))
            .top(px(1.));
        assert_style_eq(&styles.base, &expected);
        assert!(styles.unknown.is_empty() && styles.skipped.is_empty());
    }

    #[test]
    fn colors_alpha_and_hover_bucket() {
        let theme = Theme::light();
        let styles = parse(
            &theme,
            "bg-primary text-primary-foreground border-transparent hover:bg-primary/80",
        );
        let expected_base = StyleRefinement::default()
            .bg(theme.primary)
            .text_color(theme.primary_foreground)
            .border_color(transparent_black());
        assert_style_eq(&styles.base, &expected_base);
        let expected_hover = StyleRefinement::default().bg(alpha(theme.primary, 0.8));
        assert_style_eq(styles.hover.as_ref().unwrap(), &expected_hover);
    }

    #[test]
    fn dark_gating() {
        let classes = "bg-destructive/10 dark:bg-destructive/20";
        let light = parse(&Theme::light(), classes);
        let dark = parse(&Theme::dark(), classes);
        assert_style_eq(
            &light.base,
            &StyleRefinement::default().bg(alpha(Theme::light().destructive, 0.1)),
        );
        assert_style_eq(
            &dark.base,
            &StyleRefinement::default().bg(alpha(Theme::dark().destructive, 0.2)),
        );
    }

    #[test]
    fn ring_synthesis_matches_motion_focus_ring() {
        let theme = Theme::light();
        let styles = parse(
            &theme,
            "focus-visible:border-ring focus-visible:ring-[3px] focus-visible:ring-ring/50",
        );
        let expected = StyleRefinement::default()
            .border_color(theme.ring)
            .shadow(motion::focus_ring(&theme));
        assert_style_eq(styles.focus_visible.as_ref().unwrap(), &expected);
    }

    #[test]
    fn arbitrary_values() {
        let theme = Theme::light();
        let styles = parse(
            &theme,
            "w-[137px] rounded-[min(var(--radius-md),8px)] bg-[#323232]",
        );
        let expected = StyleRefinement::default()
            .w(px(137.))
            .rounded(theme.radius_md().min(px(8.)))
            .bg(Hsla::from(gpui::rgb(0x323232)));
        assert_style_eq(&styles.base, &expected);
    }

    #[test]
    fn last_class_wins_like_tailwind_merge() {
        let theme = Theme::light();
        let styles = parse(&theme, "p-4 px-2");
        let expected = StyleRefinement::default().p(px(16.)).px(px(8.));
        assert_style_eq(&styles.base, &expected);
    }

    #[test]
    fn skipped_and_unknown_are_reported() {
        let theme = Theme::light();
        let styles = parse(
            &theme,
            "select-none underline-offset-4 transition-all [a]:hover:bg-muted totally-fake",
        );
        assert_eq!(
            styles.skipped,
            vec![
                "select-none",
                "underline-offset-4",
                "transition-all",
                "[a]:hover:bg-muted"
            ]
        );
        assert_eq!(styles.unknown, vec!["totally-fake"]);
    }

    #[test]
    fn negative_values() {
        let theme = Theme::light();
        let styles = parse(&theme, "-ml-px -translate-y-0.5");
        let expected = StyleRefinement::default().ml(px(-1.)).top(px(-2.));
        assert_style_eq(&styles.base, &expected);
    }
}
