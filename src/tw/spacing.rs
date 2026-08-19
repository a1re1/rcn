//! Spacing family: padding and margin (physical + logical, `auto`, negatives).
//! Docs chapter: <https://tailwindcss.com/docs/padding>

use gpui::{Length, Pixels, StyleRefinement, Styled};

use super::{Ctx, scale_px};

pub(super) fn apply(s: StyleRefinement, t: &str, cx: &mut Ctx) -> (StyleRefinement, bool) {
    // Margin auto statics (mx-auto centers in flex/block parents).
    match t {
        "m-auto" => return (s.m(Length::Auto), true),
        "mx-auto" => return (s.mx(Length::Auto), true),
        "my-auto" => return (s.my(Length::Auto), true),
        "mt-auto" => return (s.mt(Length::Auto), true),
        "mr-auto" | "me-auto" => return (s.mr(Length::Auto), true),
        "mb-auto" => return (s.mb(Length::Auto), true),
        "ml-auto" | "ms-auto" => return (s.ml(Length::Auto), true),
        _ => {}
    }

    // Padding. Logical ps/pe map to pl/pr (LTR). Longer prefixes first so
    // `p-` doesn't shadow `pl-`.
    for (prefix, f) in [
        (
            "px-",
            (|s: StyleRefinement, l: Pixels| s.px(l))
                as fn(StyleRefinement, Pixels) -> StyleRefinement,
        ),
        ("py-", |s, l| s.py(l)),
        ("pl-", |s, l| s.pl(l)),
        ("ps-", |s, l| s.pl(l)),
        ("pr-", |s, l| s.pr(l)),
        ("pe-", |s, l| s.pr(l)),
        ("pt-", |s, l| s.pt(l)),
        ("pb-", |s, l| s.pb(l)),
        ("p-", |s, l| s.p(l)),
    ] {
        if let Some(v) = t.strip_prefix(prefix)
            && let Some(l) = scale_px(v)
        {
            return (f(s, l), true);
        }
    }

    // Margin (negatives allowed).
    for (prefix, f) in [
        (
            "mx-",
            (|s: StyleRefinement, l: Pixels| s.mx(l))
                as fn(StyleRefinement, Pixels) -> StyleRefinement,
        ),
        ("my-", |s, l| s.my(l)),
        ("ml-", |s, l| s.ml(l)),
        ("ms-", |s, l| s.ml(l)),
        ("mr-", |s, l| s.mr(l)),
        ("me-", |s, l| s.mr(l)),
        ("mt-", |s, l| s.mt(l)),
        ("mb-", |s, l| s.mb(l)),
        ("m-", |s, l| s.m(l)),
    ] {
        if let Some(v) = t.strip_prefix(prefix)
            && let Some(l) = scale_px(v)
        {
            return (f(s, l * cx.sign), true);
        }
    }

    (s, false)
}

#[cfg(test)]
mod tests {
    use super::super::{parse, tests::assert_style_eq};
    use crate::theme::Theme;
    use gpui::{Length, StyleRefinement, Styled, px};

    #[test]
    fn padding_scale_and_logical() {
        let theme = Theme::light();
        let styles = parse(&theme, "p-4 ps-1.5 pe-2");
        let expected = StyleRefinement::default().p(px(16.)).pl(px(6.)).pr(px(8.));
        assert_style_eq(&styles.base, &expected);
    }

    #[test]
    fn margin_auto_and_negative() {
        let theme = Theme::light();
        let styles = parse(&theme, "mx-auto -mt-2");
        let expected = StyleRefinement::default().mx(Length::Auto).mt(px(-8.));
        assert_style_eq(&styles.base, &expected);
    }
}
