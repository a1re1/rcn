//! Backgrounds family: solid background colors (theme tokens, default
//! palette, arbitrary hex). Gradients are ledgered `Todo`; background images
//! have no gpui equivalent.
//! Docs chapter: <https://tailwindcss.com/docs/background-color>

use gpui::{StyleRefinement, Styled};

use super::{Ctx, color};

pub(super) fn apply(s: StyleRefinement, t: &str, cx: &mut Ctx) -> (StyleRefinement, bool) {
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
    use gpui::{StyleRefinement, Styled};

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
}
