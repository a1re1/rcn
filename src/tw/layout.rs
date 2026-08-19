//! Layout family: display, position, insets, overflow, visibility.
//! Docs chapter: <https://tailwindcss.com/docs/display>

use gpui::{Display, Length, Overflow, StyleRefinement, Styled, relative};

use super::{Ctx, scale_len};

pub(super) fn apply(mut s: StyleRefinement, t: &str, cx: &mut Ctx) -> (StyleRefinement, bool) {
    // Display. `inline-*` variants fold to their block-level layout — taffy
    // has no inline formatting context, and inside a flex/grid parent they
    // behave identically anyway.
    match t {
        "flex" | "inline-flex" => return (s.flex(), true),
        "grid" | "inline-grid" => {
            s.display = Some(Display::Grid);
            return (s, true);
        }
        "block" => {
            s.display = Some(Display::Block);
            return (s, true);
        }
        "hidden" => {
            s.display = Some(Display::None);
            return (s, true);
        }
        "relative" => return (s.relative(), true),
        "absolute" => return (s.absolute(), true),
        "visible" => return (s.visible(), true),
        "invisible" => return (s.invisible(), true),
        _ => {}
    }

    // Overflow: overflow / overflow-x / overflow-y × auto|hidden|clip|visible|scroll.
    if let Some(v) = t.strip_prefix("overflow-") {
        let (axis, value) = match v.split_once('-') {
            Some(("x", rest)) => (Some(false), rest),
            Some(("y", rest)) => (Some(true), rest),
            _ => (None, v),
        };
        let overflow = match value {
            "auto" | "scroll" => Overflow::Scroll,
            "hidden" => Overflow::Hidden,
            "clip" => Overflow::Clip,
            "visible" => Overflow::Visible,
            _ => return (s, false),
        };
        match axis {
            Some(false) => s.overflow.x = Some(overflow),
            Some(true) => s.overflow.y = Some(overflow),
            None => {
                s.overflow.x = Some(overflow);
                s.overflow.y = Some(overflow);
            }
        }
        return (s, true);
    }

    // Aspect ratio: square / video / arbitrary `[W/H]`.
    match t {
        "aspect-square" => return (s.aspect_ratio(1.0), true),
        "aspect-video" => return (s.aspect_ratio(16. / 9.), true),
        _ => {}
    }
    if let Some(inner) = t.strip_prefix("aspect-[").and_then(|x| x.strip_suffix(']'))
        && let Some((w, h)) = inner.split_once('/')
        && let (Ok(w), Ok(h)) = (w.parse::<f32>(), h.parse::<f32>())
        && h != 0.
    {
        return (s.aspect_ratio(w / h), true);
    }

    // Insets. Logical start/end map to left/right (rcn is LTR-only).
    for (prefix, apply) in [
        (
            "inset-x-",
            (|s: &mut StyleRefinement, l: Length| {
                s.inset.left = Some(l);
                s.inset.right = Some(l);
            }) as fn(&mut StyleRefinement, Length),
        ),
        ("inset-y-", |s, l| {
            s.inset.top = Some(l);
            s.inset.bottom = Some(l);
        }),
        ("inset-", |s, l| {
            s.inset.top = Some(l);
            s.inset.right = Some(l);
            s.inset.bottom = Some(l);
            s.inset.left = Some(l);
        }),
        ("top-", |s, l| s.inset.top = Some(l)),
        ("right-", |s, l| s.inset.right = Some(l)),
        ("bottom-", |s, l| s.inset.bottom = Some(l)),
        ("left-", |s, l| s.inset.left = Some(l)),
        ("start-", |s, l| s.inset.left = Some(l)),
        ("end-", |s, l| s.inset.right = Some(l)),
    ] {
        if let Some(v) = t.strip_prefix(prefix)
            && let Some(l) = inset_len(v, cx.sign)
        {
            apply(&mut s, l);
            return (s, true);
        }
    }

    (s, false)
}

/// Inset values: the shared length scale plus `auto`/`full`, sign-aware.
fn inset_len(v: &str, sign: f32) -> Option<Length> {
    match scale_len(v)? {
        Length::Auto => Some(Length::Auto),
        Length::Definite(l) => Some(match l {
            gpui::DefiniteLength::Absolute(a) => {
                Length::Definite(gpui::DefiniteLength::Absolute(match a {
                    gpui::AbsoluteLength::Pixels(p) => gpui::AbsoluteLength::Pixels(p * sign),
                    gpui::AbsoluteLength::Rems(r) => gpui::AbsoluteLength::Rems(r * sign),
                }))
            }
            gpui::DefiniteLength::Fraction(f) => relative(f * sign).into(),
        }),
    }
}

#[cfg(test)]
mod tests {
    use super::super::{parse, tests::assert_style_eq};
    use crate::theme::Theme;
    use gpui::{Display, Overflow, StyleRefinement, Styled, px, relative};

    #[test]
    fn display_utilities() {
        let theme = Theme::light();
        for (class, expected) in [
            ("flex", StyleRefinement::default().flex()),
            ("block", {
                let mut s = StyleRefinement::default();
                s.display = Some(Display::Block);
                s
            }),
            ("grid", {
                let mut s = StyleRefinement::default();
                s.display = Some(Display::Grid);
                s
            }),
            ("hidden", {
                let mut s = StyleRefinement::default();
                s.display = Some(Display::None);
                s
            }),
        ] {
            let styles = parse(&theme, class);
            assert_style_eq(&styles.base, &expected);
        }
    }

    #[test]
    fn overflow_axes_and_values() {
        let theme = Theme::light();
        let styles = parse(&theme, "overflow-x-auto overflow-y-clip");
        let mut expected = StyleRefinement::default();
        expected.overflow.x = Some(Overflow::Scroll);
        expected.overflow.y = Some(Overflow::Clip);
        assert_style_eq(&styles.base, &expected);

        let styles = parse(&theme, "overflow-hidden");
        assert_style_eq(&styles.base, &StyleRefinement::default().overflow_hidden());
    }

    #[test]
    fn insets_including_logical_and_auto() {
        let theme = Theme::light();
        let styles = parse(&theme, "inset-x-2 start-4 top-auto bottom-full");
        let mut expected = StyleRefinement::default();
        expected.inset.left = Some(px(16.).into()); // start-4 overrode inset-x-2's left
        expected.inset.right = Some(px(8.).into());
        expected.inset.top = Some(gpui::Length::Auto);
        expected.inset.bottom = Some(relative(1.).into());
        assert_style_eq(&styles.base, &expected);
    }

    #[test]
    fn aspect_square_video_and_arbitrary() {
        let theme = Theme::light();
        let styles = parse(&theme, "aspect-square");
        assert_style_eq(&styles.base, &StyleRefinement::default().aspect_ratio(1.0));

        let styles = parse(&theme, "aspect-video");
        assert_style_eq(
            &styles.base,
            &StyleRefinement::default().aspect_ratio(16. / 9.),
        );

        let styles = parse(&theme, "aspect-[4/3]");
        assert_style_eq(
            &styles.base,
            &StyleRefinement::default().aspect_ratio(4. / 3.),
        );
    }
}
