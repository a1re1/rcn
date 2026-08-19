//! Typography family: text size/color/align, font weight/style, decoration,
//! whitespace and truncation.
//! Docs chapter: <https://tailwindcss.com/docs/font-size>

use std::sync::Arc;

use gpui::{
    FontFeatures, FontStyle, FontWeight, StyleRefinement, Styled, TextAlign, UnderlineStyle, px,
    relative,
};

use super::{Ctx, color, scale_px};

pub(super) fn apply(mut s: StyleRefinement, t: &str, cx: &mut Ctx) -> (StyleRefinement, bool) {
    match t {
        "italic" => {
            s.text.font_style = Some(FontStyle::Italic);
            return (s, true);
        }
        "not-italic" => {
            s.text.font_style = Some(FontStyle::Normal);
            return (s, true);
        }
        "underline" => return (s.underline(), true),
        "line-through" => return (s.line_through(), true),
        // No "remove decoration" in a refinement — a zero-thickness underline
        // is visually identical.
        "no-underline" => {
            s.text.underline = Some(UnderlineStyle {
                thickness: px(0.),
                color: None,
                wavy: false,
            });
            return (s, true);
        }
        "decoration-wavy" => {
            s.text.underline.get_or_insert_default().wavy = true;
            return (s, true);
        }
        "whitespace-normal" | "text-wrap" => return (s.whitespace_normal(), true),
        "whitespace-nowrap" | "text-nowrap" => return (s.whitespace_nowrap(), true),
        "truncate" => return (s.truncate(), true),
        "text-ellipsis" => return (s.text_ellipsis(), true),
        "text-left" | "text-start" => {
            s.text.text_align = Some(TextAlign::Left);
            return (s, true);
        }
        "text-center" => {
            s.text.text_align = Some(TextAlign::Center);
            return (s, true);
        }
        "text-right" | "text-end" => {
            s.text.text_align = Some(TextAlign::Right);
            return (s, true);
        }
        // OpenType feature statics (FontFeatures is a public Arc<Vec<(tag, val)>>).
        "ordinal" => return (with_feature(s, "ordn"), true),
        "slashed-zero" => return (with_feature(s, "zero"), true),
        "lining-nums" => return (with_feature(s, "lnum"), true),
        "oldstyle-nums" => return (with_feature(s, "onum"), true),
        "proportional-nums" => return (with_feature(s, "pnum"), true),
        "tabular-nums" => return (with_feature(s, "tnum"), true),
        "diagonal-fractions" => return (with_feature(s, "frac"), true),
        "stacked-fractions" => return (with_feature(s, "afrc"), true),
        _ => {}
    }

    // Font weights (`font-<family>` is ledgered Todo; unmatched values fall
    // through).
    if let Some(v) = t.strip_prefix("font-") {
        let weight = match v {
            "thin" => Some(FontWeight::THIN),
            "extralight" => Some(FontWeight::EXTRA_LIGHT),
            "light" => Some(FontWeight::LIGHT),
            "normal" => Some(FontWeight::NORMAL),
            "medium" => Some(FontWeight::MEDIUM),
            "semibold" => Some(FontWeight::SEMIBOLD),
            "bold" => Some(FontWeight::BOLD),
            "extrabold" => Some(FontWeight::EXTRA_BOLD),
            "black" => Some(FontWeight::BLACK),
            _ => None,
        };
        if let Some(w) = weight {
            return (s.font_weight(w), true);
        }
    }

    // Line height: named relative ratios + numeric 4px scale.
    if let Some(v) = t.strip_prefix("leading-") {
        let lh = match v {
            "none" => Some(relative(1.0)),
            "tight" => Some(relative(1.25)),
            "snug" => Some(relative(1.375)),
            "normal" => Some(relative(1.5)),
            "relaxed" => Some(relative(1.625)),
            "loose" => Some(relative(2.0)),
            _ => scale_px(v).map(|p| p.into()),
        };
        if let Some(lh) = lh {
            return (s.line_height(lh), true);
        }
    }

    // Line clamp: show N lines then truncate.
    if let Some(v) = t.strip_prefix("line-clamp-") {
        if let Ok(n) = v.parse::<usize>() {
            s.text.line_clamp = Some(n);
            return (s, true);
        }
    }

    // Text decoration color / thickness. Apply to both underline and
    // strikethrough via get_or_insert_default so ordering vs underline /
    // line-through does not matter.
    if let Some(v) = t.strip_prefix("decoration-") {
        if let Some(c) = color(cx.theme, v) {
            s.text.underline.get_or_insert_default().color = Some(c);
            s.text.strikethrough.get_or_insert_default().color = Some(c);
            return (s, true);
        }
        // Thickness is raw px (`decoration-2` = 2px), not the 4px scale.
        let thickness = if let Ok(n) = v.parse::<f32>() {
            Some(px(n))
        } else {
            v.strip_prefix('[')
                .and_then(|x| x.strip_suffix("px]"))
                .and_then(|n| n.parse::<f32>().ok().map(px))
        };
        if let Some(thickness) = thickness {
            s.text.underline.get_or_insert_default().thickness = thickness;
            s.text.strikethrough.get_or_insert_default().thickness = thickness;
            return (s, true);
        }
    }

    // Text sizes (Tailwind v4 size/line-height pairs), then text color.
    if let Some(v) = t.strip_prefix("text-") {
        let size = match v {
            "xs" => Some((12., 16.)),
            "sm" => Some((14., 20.)),
            "base" => Some((16., 24.)),
            "lg" => Some((18., 28.)),
            "xl" => Some((20., 28.)),
            "2xl" => Some((24., 32.)),
            "3xl" => Some((30., 36.)),
            "4xl" => Some((36., 40.)),
            "5xl" => Some((48., 48.)),
            "6xl" => Some((60., 60.)),
            "7xl" => Some((72., 72.)),
            "8xl" => Some((96., 96.)),
            "9xl" => Some((128., 128.)),
            _ => None,
        };
        if let Some((size, line_height)) = size {
            return (s.text_size(px(size)).line_height(px(line_height)), true);
        }
        if let Some(c) = color(cx.theme, v) {
            return (s.text_color(c), true);
        }
    }

    // `fill-<color>` paints SVG fills. gpui SVG elements use the text color,
    // so this is equivalent to `text-<color>`.
    if let Some(v) = t.strip_prefix("fill-") {
        if let Some(c) = color(cx.theme, v) {
            return (s.text_color(c), true);
        }
    }

    (s, false)
}

fn with_feature(mut s: StyleRefinement, tag: &str) -> StyleRefinement {
    let mut features = s
        .text
        .font_features
        .take()
        .map(|f| f.0.as_ref().clone())
        .unwrap_or_default();
    features.retain(|(existing, _)| existing != tag);
    features.push((tag.to_string(), 1));
    s.text.font_features = Some(FontFeatures(Arc::new(features)));
    s
}

#[cfg(test)]
mod tests {
    use super::super::{parse, tests::assert_style_eq};
    use crate::theme::Theme;
    use gpui::{
        FontFeatures, FontStyle, FontWeight, StyleRefinement, Styled, TextAlign, px, relative,
    };
    use std::sync::Arc;

    #[test]
    fn sizes_set_size_and_line_height() {
        let theme = Theme::light();
        for (class, size, lh) in [("text-xs", 12., 16.), ("text-2xl", 24., 32.)] {
            let styles = parse(&theme, class);
            let expected = StyleRefinement::default()
                .text_size(px(size))
                .line_height(px(lh));
            assert_style_eq(&styles.base, &expected);
        }
    }

    #[test]
    fn weights_style_and_align() {
        let theme = Theme::light();
        let styles = parse(&theme, "font-semibold italic text-center");
        let mut expected = StyleRefinement::default().font_weight(FontWeight::SEMIBOLD);
        expected.text.font_style = Some(FontStyle::Italic);
        expected.text.text_align = Some(TextAlign::Center);
        assert_style_eq(&styles.base, &expected);
    }

    #[test]
    fn colors_after_sizes() {
        let theme = Theme::light();
        let styles = parse(&theme, "text-muted-foreground");
        let expected = StyleRefinement::default().text_color(theme.muted_foreground);
        assert_style_eq(&styles.base, &expected);
    }

    #[test]
    fn decoration_statics() {
        let theme = Theme::light();
        let styles = parse(&theme, "line-through");
        assert_style_eq(&styles.base, &StyleRefinement::default().line_through());
        assert!(parse(&theme, "no-underline").base.text.underline.is_some());
    }

    #[test]
    fn leading_numeric_and_named() {
        let theme = Theme::light();
        let styles = parse(&theme, "leading-6");
        assert_style_eq(
            &styles.base,
            &StyleRefinement::default().line_height(px(24.)),
        );
        let styles = parse(&theme, "leading-none");
        assert_style_eq(
            &styles.base,
            &StyleRefinement::default().line_height(relative(1.0)),
        );
        let styles = parse(&theme, "leading-tight");
        assert_style_eq(
            &styles.base,
            &StyleRefinement::default().line_height(relative(1.25)),
        );
    }

    #[test]
    fn line_clamp_sets_text_line_clamp() {
        let theme = Theme::light();
        let styles = parse(&theme, "line-clamp-2");
        let mut expected = StyleRefinement::default();
        expected.text.line_clamp = Some(2);
        assert_style_eq(&styles.base, &expected);
    }

    #[test]
    fn decoration_color_and_thickness() {
        let theme = Theme::light();
        let styles = parse(&theme, "decoration-red-500");
        let c = super::super::color(&theme, "red-500").unwrap();
        let mut expected = StyleRefinement::default();
        expected.text.underline.get_or_insert_default().color = Some(c);
        expected.text.strikethrough.get_or_insert_default().color = Some(c);
        assert_style_eq(&styles.base, &expected);

        let styles = parse(&theme, "decoration-2");
        let mut expected = StyleRefinement::default();
        expected.text.underline.get_or_insert_default().thickness = px(2.);
        expected
            .text
            .strikethrough
            .get_or_insert_default()
            .thickness = px(2.);
        assert_style_eq(&styles.base, &expected);
    }

    #[test]
    fn font_features_stack_across_classes() {
        let theme = Theme::light();
        let styles = parse(&theme, "tabular-nums slashed-zero");
        let features = styles.base.text.font_features.as_ref().unwrap();
        assert_eq!(
            features.0.as_ref().as_slice(),
            &[("tnum".to_string(), 1), ("zero".to_string(), 1)]
        );
    }

    #[test]
    fn decoration_wavy_sets_underline_wavy() {
        let theme = Theme::light();
        let styles = parse(&theme, "decoration-wavy");
        let mut expected = StyleRefinement::default();
        expected.text.underline.get_or_insert_default().wavy = true;
        assert_style_eq(&styles.base, &expected);
    }

    #[test]
    fn font_feature_statics() {
        let theme = Theme::light();
        let styles = parse(&theme, "tabular-nums");
        let mut expected = StyleRefinement::default();
        expected.text.font_features = Some(FontFeatures(Arc::new(vec![("tnum".to_string(), 1)])));
        assert_style_eq(&styles.base, &expected);

        let styles = parse(&theme, "ordinal");
        let mut expected = StyleRefinement::default();
        expected.text.font_features = Some(FontFeatures(Arc::new(vec![("ordn".to_string(), 1)])));
        assert_style_eq(&styles.base, &expected);
    }

    #[test]
    fn fill_color_maps_to_text_color() {
        let theme = Theme::light();
        let styles = parse(&theme, "fill-red-500");
        let c = super::super::color(&theme, "red-500").unwrap();
        let expected = StyleRefinement::default().text_color(c);
        assert_style_eq(&styles.base, &expected);
    }
}
