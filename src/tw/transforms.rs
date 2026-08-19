//! Transforms family. gpui has no style-level transforms, so
//! `translate-x/-y` map to `left`/`top` insets — a close positional
//! approximation that needs `relative` on the element. Everything else
//! (scale, rotate, skew, 3D) is ledgered `NoEquivalent`.
//! Docs chapter: <https://tailwindcss.com/docs/translate>

use gpui::{StyleRefinement, Styled};

use super::{Ctx, scale_px};

pub(super) fn apply(s: StyleRefinement, t: &str, cx: &mut Ctx) -> (StyleRefinement, bool) {
    if let Some(v) = t.strip_prefix("translate-x-")
        && let Some(l) = scale_px(v)
    {
        return (s.left(l * cx.sign), true);
    }
    if let Some(v) = t.strip_prefix("translate-y-")
        && let Some(l) = scale_px(v)
    {
        return (s.top(l * cx.sign), true);
    }
    (s, false)
}

#[cfg(test)]
mod tests {
    use super::super::{parse, tests::assert_style_eq};
    use crate::theme::Theme;
    use gpui::{StyleRefinement, Styled, px};

    #[test]
    fn translate_maps_to_insets() {
        let theme = Theme::light();
        let styles = parse(&theme, "translate-y-px -translate-x-2");
        let expected = StyleRefinement::default().top(px(1.)).left(px(-8.));
        assert_style_eq(&styles.base, &expected);
    }
}
