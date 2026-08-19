//! Effects family: box shadows and opacity.
//! Docs chapter: <https://tailwindcss.com/docs/box-shadow>

use gpui::{BoxShadow, StyleRefinement, Styled, hsla, px};

use super::Ctx;

pub(super) fn apply(mut s: StyleRefinement, t: &str, _cx: &mut Ctx) -> (StyleRefinement, bool) {
    match t {
        "shadow-2xs" => return (s.shadow_2xs(), true),
        "shadow-xs" => return (s.shadow_xs(), true),
        "shadow-sm" | "shadow" => return (s.shadow_sm(), true),
        "shadow-md" => return (s.shadow_md(), true),
        "shadow-lg" => return (s.shadow_lg(), true),
        "shadow-xl" => return (s.shadow_xl(), true),
        "shadow-2xl" => return (s.shadow_2xl(), true),
        "shadow-none" => return (s.shadow_none(), true),
        // Inset shadows (Tailwind v4): append with `inset: true`.
        "inset-shadow-2xs" => {
            push_shadow(
                &mut s,
                BoxShadow::new(px(0.), px(1.), hsla(0., 0., 0., 0.05)).inset(),
            );
            return (s, true);
        }
        "inset-shadow-xs" => {
            push_shadow(
                &mut s,
                BoxShadow::new(px(0.), px(1.), hsla(0., 0., 0., 0.05))
                    .blur_radius(px(1.))
                    .inset(),
            );
            return (s, true);
        }
        "inset-shadow-sm" => {
            push_shadow(
                &mut s,
                BoxShadow::new(px(0.), px(2.), hsla(0., 0., 0., 0.05))
                    .blur_radius(px(4.))
                    .inset(),
            );
            return (s, true);
        }
        _ => {}
    }

    if let Some(v) = t.strip_prefix("opacity-")
        && let Ok(n) = v.parse::<f32>()
    {
        return (s.opacity(n / 100.), true);
    }

    (s, false)
}

fn push_shadow(s: &mut StyleRefinement, shadow: BoxShadow) {
    match &mut s.box_shadow {
        Some(shadows) => shadows.push(shadow),
        None => s.box_shadow = Some(vec![shadow]),
    }
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

    #[test]
    fn inset_shadow_sm_is_inset() {
        use gpui::{BoxShadow, hsla, px};

        let theme = Theme::light();
        let styles = parse(&theme, "inset-shadow-sm");
        let shadows = styles.base.box_shadow.as_ref().unwrap();
        assert_eq!(shadows.len(), 1);
        assert!(shadows[0].inset);
        assert_eq!(
            shadows[0],
            BoxShadow::new(px(0.), px(2.), hsla(0., 0., 0., 0.05))
                .blur_radius(px(4.))
                .inset()
        );
    }
}
