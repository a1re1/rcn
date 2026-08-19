//! Backgrounds family: solid background colors (theme tokens, default
//! palette, arbitrary hex) plus two-stop linear gradients via
//! `bg-linear-to-{t,tr,r,br,b,bl,l,tl}` + `from-<color>` / `to-<color>`.
//! `via-*` multi-stop gradients have no gpui equivalent.
//! Docs chapter: <https://tailwindcss.com/docs/background-image>

use gpui::{StyleRefinement, Styled};

use super::{Ctx, color};

/// CSS `linear-gradient` angles for Tailwind `bg-linear-to-*` directions
/// (`to top` = 0°, `to right` = 90°, …).
fn linear_to_angle(dir: &str) -> Option<f32> {
    Some(match dir {
        "t" => 0.,
        "tr" => 45.,
        "r" => 90.,
        "br" => 135.,
        "b" => 180.,
        "bl" => 225.,
        "l" => 270.,
        "tl" => 315.,
        _ => return None,
    })
}

pub(super) fn apply(s: StyleRefinement, t: &str, cx: &mut Ctx) -> (StyleRefinement, bool) {
    // Direction first so `bg-linear-to-r` is not swallowed by solid `bg-*`.
    if let Some(dir) = t.strip_prefix("bg-linear-to-")
        && let Some(angle) = linear_to_angle(dir)
    {
        cx.gradient.angle = Some(angle);
        return (s, true);
    }

    if let Some(v) = t.strip_prefix("from-")
        && let Some(c) = color(cx.theme, v)
    {
        cx.gradient.from = Some(c);
        return (s, true);
    }

    if let Some(v) = t.strip_prefix("to-")
        && let Some(c) = color(cx.theme, v)
    {
        cx.gradient.to = Some(c);
        return (s, true);
    }

    if let Some(v) = t.strip_prefix("bg-")
        && let Some(c) = color(cx.theme, v)
    {
        return (s.bg(c), true);
    }
    (s, false)
}

#[cfg(test)]
mod tests {
    use super::super::{parse, tests::assert_style_eq};
    use crate::theme::{Theme, alpha};
    use gpui::{StyleRefinement, Styled, linear_color_stop, linear_gradient};

    #[test]
    fn token_palette_and_alpha() {
        let theme = Theme::light();
        let styles = parse(&theme, "bg-secondary/80");
        assert_style_eq(
            &styles.base,
            &StyleRefinement::default().bg(alpha(theme.secondary, 0.8)),
        );
        assert!(parse(&theme, "bg-emerald-300").unknown.is_empty());
    }

    #[test]
    fn unmappable_bg_values_are_skipped_not_applied() {
        let theme = Theme::light();
        let styles = parse(&theme, "bg-cover bg-no-repeat");
        assert!(styles.base.background.is_none());
        assert_eq!(styles.skipped, vec!["bg-cover", "bg-no-repeat"]);
    }

    #[test]
    fn linear_gradient_from_to() {
        let theme = Theme::light();
        let styles = parse(&theme, "bg-linear-to-r from-red-500 to-blue-500");
        assert!(styles.unknown.is_empty(), "unknown: {:?}", styles.unknown);
        let from = super::super::color(&theme, "red-500").unwrap();
        let to = super::super::color(&theme, "blue-500").unwrap();
        let expected = StyleRefinement::default().bg(linear_gradient(
            90.,
            linear_color_stop(from, 0.),
            linear_color_stop(to, 1.),
        ));
        assert_style_eq(&styles.base, &expected);
        assert!(styles.base.background.is_some());
    }
}
