//! Extension-channel utilities: real Tailwind classes whose effect is not a
//! gpui `Style` but an element-level behavior. They parse into
//! [`super::TwExt`] instead of a `StyleRefinement`; consumers apply them at
//! the element layer:
//!
//! - `space-x/y-*` and `divide-x/y-*` / `divide-<color>` — child-combinator
//!   spacing/separators, consumed by [`super::element::TwDiv`] (`tw_div`).
//! - `object-{contain,cover,fill,none,scale-down}` and `grayscale` — image
//!   properties, applied to a gpui `Img` via [`super::apply_img`]
//!   (`object-<position>` values stay no-equivalent).
//!
//! These are ledgered `Status::Extended`: the enforcement tests assert their
//! samples parse into the ext channel with nothing unknown or skipped.

use super::{Ctx, StyleRefinement, TwObjectFit, color, scale_px};

pub(super) fn apply(s: StyleRefinement, t: &str, cx: &mut Ctx) -> (StyleRefinement, bool) {
    match t {
        "divide-x" => {
            cx.ext.divide_x = Some(gpui::px(1.));
            return (s, true);
        }
        "divide-y" => {
            cx.ext.divide_y = Some(gpui::px(1.));
            return (s, true);
        }
        "object-contain" => return (set_fit(s, cx, TwObjectFit::Contain), true),
        "object-cover" => return (set_fit(s, cx, TwObjectFit::Cover), true),
        "object-fill" => return (set_fit(s, cx, TwObjectFit::Fill), true),
        "object-none" => return (set_fit(s, cx, TwObjectFit::None), true),
        "object-scale-down" => return (set_fit(s, cx, TwObjectFit::ScaleDown), true),
        "grayscale" => {
            cx.ext.grayscale = true;
            return (s, true);
        }
        _ => {}
    }

    // Child spacing (negatives collapse children toward each other).
    if let Some(v) = t.strip_prefix("space-x-")
        && let Some(l) = scale_px(v)
    {
        cx.ext.space_x = Some(l * cx.sign);
        return (s, true);
    }
    if let Some(v) = t.strip_prefix("space-y-")
        && let Some(l) = scale_px(v)
    {
        cx.ext.space_y = Some(l * cx.sign);
        return (s, true);
    }

    // Divide widths are raw px (`divide-x-2` = 2px), then the divide color.
    if let Some(v) = t.strip_prefix("divide-x-")
        && let Ok(n) = v.parse::<f32>()
    {
        cx.ext.divide_x = Some(gpui::px(n));
        return (s, true);
    }
    if let Some(v) = t.strip_prefix("divide-y-")
        && let Ok(n) = v.parse::<f32>()
    {
        cx.ext.divide_y = Some(gpui::px(n));
        return (s, true);
    }
    if let Some(v) = t.strip_prefix("divide-")
        && let Some(c) = color(cx.theme, v)
    {
        cx.ext.divide_color = Some(c);
        return (s, true);
    }

    (s, false)
}

fn set_fit(s: StyleRefinement, cx: &mut Ctx, fit: TwObjectFit) -> StyleRefinement {
    cx.ext.object_fit = Some(fit);
    s
}

#[cfg(test)]
mod tests {
    use super::super::{TwObjectFit, parse};
    use crate::theme::Theme;
    use gpui::px;

    #[test]
    fn space_and_divide_parse_into_ext() {
        let theme = Theme::light();
        let styles = parse(
            &theme,
            "space-x-2 -space-y-1 divide-y divide-x-2 divide-border",
        );
        assert!(styles.unknown.is_empty() && styles.skipped.is_empty());
        assert_eq!(styles.ext.space_x, Some(px(8.)));
        assert_eq!(styles.ext.space_y, Some(px(-4.)));
        assert_eq!(styles.ext.divide_y, Some(px(1.)));
        assert_eq!(styles.ext.divide_x, Some(px(2.)));
        assert_eq!(styles.ext.divide_color, Some(theme.border));
    }

    #[test]
    fn image_properties_parse_into_ext() {
        let theme = Theme::light();
        let styles = parse(&theme, "object-cover grayscale");
        assert_eq!(styles.ext.object_fit, Some(TwObjectFit::Cover));
        assert!(styles.ext.grayscale);
        // Object positions remain no-equivalent.
        assert_eq!(parse(&theme, "object-top").skipped, vec!["object-top"]);
    }
}
