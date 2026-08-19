//! Effects family: box shadows and opacity.
//! Docs chapter: <https://tailwindcss.com/docs/box-shadow>

use gpui::{StyleRefinement, Styled};

use super::Ctx;

pub(super) fn apply(s: StyleRefinement, t: &str, _cx: &mut Ctx) -> (StyleRefinement, bool) {
    match t {
        "shadow-2xs" => return (s.shadow_2xs(), true),
        "shadow-xs" => return (s.shadow_xs(), true),
        "shadow-sm" | "shadow" => return (s.shadow_sm(), true),
        "shadow-md" => return (s.shadow_md(), true),
        "shadow-lg" => return (s.shadow_lg(), true),
        "shadow-xl" => return (s.shadow_xl(), true),
        "shadow-2xl" => return (s.shadow_2xl(), true),
        "shadow-none" => return (s.shadow_none(), true),
        _ => {}
    }

    if let Some(v) = t.strip_prefix("opacity-")
        && let Ok(n) = v.parse::<f32>()
    {
        return (s.opacity(n / 100.), true);
    }

    (s, false)
}

#[cfg(test)]
mod tests {
    use super::super::{parse, tests::assert_style_eq};
    use crate::theme::Theme;
    use gpui::{StyleRefinement, Styled};

    #[test]
    fn shadow_scale() {
        let theme = Theme::light();
        for (class, expected) in [
            ("shadow-xs", StyleRefinement::default().shadow_xs()),
            ("shadow", StyleRefinement::default().shadow_sm()),
            ("shadow-2xl", StyleRefinement::default().shadow_2xl()),
            ("shadow-none", StyleRefinement::default().shadow_none()),
        ] {
            assert_style_eq(&parse(&theme, class).base, &expected);
        }
    }

    #[test]
    fn opacity_percentage() {
        let theme = Theme::light();
        assert_style_eq(
            &parse(&theme, "opacity-50").base,
            &StyleRefinement::default().opacity(0.5),
        );
    }
}
