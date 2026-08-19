//! Typography family: text size/color/align, font weight/style, decoration,
//! whitespace and truncation.
//! Docs chapter: <https://tailwindcss.com/docs/font-size>

use gpui::{FontStyle, FontWeight, StyleRefinement, Styled, TextAlign, UnderlineStyle, px};

use super::{Ctx, color};

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

    (s, false)
}

#[cfg(test)]
mod tests {
    use super::super::{parse, tests::assert_style_eq};
    use crate::theme::Theme;
    use gpui::{FontStyle, FontWeight, StyleRefinement, Styled, TextAlign, px};

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
}
