//! Sizing family: width, height, size, min/max — scale, fractions, full/auto.
//! Docs chapter: <https://tailwindcss.com/docs/width>

use gpui::{Length, Pixels, StyleRefinement, Styled};

use super::{Ctx, scale_len};

/// Viewport units. Explicit units pick their own axis (`h-svw` is 100svw =
/// viewport *width*); `screen` follows the root's axis. All small/large/
/// dynamic variants are equal in a desktop window.
fn viewport_len(
    v: &str,
    root_is_width: bool,
    viewport: Option<gpui::Size<Pixels>>,
) -> Option<Length> {
    let vp = viewport?;
    let px = match v {
        "screen" => {
            if root_is_width {
                vp.width
            } else {
                vp.height
            }
        }
        "svw" | "lvw" | "dvw" => vp.width,
        "svh" | "lvh" | "dvh" => vp.height,
        _ => return None,
    };
    Some(px.into())
}

pub(super) fn apply(mut s: StyleRefinement, t: &str, _cx: &mut Ctx) -> (StyleRefinement, bool) {
    match t {
        // `none` on max sizes = unconstrained = taffy auto.
        "max-w-none" => return (s.max_w(Length::Auto), true),
        "max-h-none" => return (s.max_h(Length::Auto), true),
        "size-auto" => return (s.size(Length::Auto), true),
        _ => {}
    }

    if let Some(v) = t.strip_prefix("size-")
        && let Some(l) = scale_len(v).or_else(|| viewport_len(v, true, _cx.viewport))
    {
        return (s.size(l), true);
    }
    // Longer prefixes before w-/h-.
    for (prefix, f) in [
        (
            "min-w-",
            (|s: &mut StyleRefinement, l: Length| s.min_size.width = Some(l))
                as fn(&mut StyleRefinement, Length),
        ),
        ("min-h-", |s, l| s.min_size.height = Some(l)),
        ("max-w-", |s, l| s.max_size.width = Some(l)),
        ("max-h-", |s, l| s.max_size.height = Some(l)),
        ("w-", |s, l| s.size.width = Some(l)),
        ("h-", |s, l| s.size.height = Some(l)),
    ] {
        if let Some(v) = t.strip_prefix(prefix)
            && let Some(l) =
                scale_len(v).or_else(|| viewport_len(v, prefix.contains("w-"), _cx.viewport))
        {
            f(&mut s, l);
            return (s, true);
        }
    }

    (s, false)
}

#[cfg(test)]
mod tests {
    use super::super::{parse, tests::assert_style_eq};
    use crate::theme::Theme;
    use gpui::{Length, StyleRefinement, Styled, px, relative};

    #[test]
    fn scale_full_auto_and_fractions() {
        let theme = Theme::light();
        let styles = parse(&theme, "w-8 h-full min-w-auto max-w-none size-9");
        // size-9 wins over the earlier w-8/h-full (last class wins).
        let expected = StyleRefinement::default()
            .w(px(32.))
            .h(relative(1.))
            .min_w(Length::Auto)
            .max_w(Length::Auto)
            .size(px(36.));
        assert_style_eq(&styles.base, &expected);

        let styles = parse(&theme, "w-1/3");
        assert_style_eq(
            &styles.base,
            &StyleRefinement::default().w(relative(1. / 3.)),
        );
    }

    #[test]
    fn min_max_scale() {
        let theme = Theme::light();
        let styles = parse(&theme, "min-h-8 max-h-64");
        let expected = StyleRefinement::default().min_h(px(32.)).max_h(px(256.));
        assert_style_eq(&styles.base, &expected);
    }
}
