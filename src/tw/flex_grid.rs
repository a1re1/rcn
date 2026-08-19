//! Flexbox & Grid family: direction, wrap, grow/shrink, alignment, gap.
//! Docs chapter: <https://tailwindcss.com/docs/flex-basis>
//!
//! Grid templates and placement (`grid-cols-*`, `col-span-*`, …) are ledgered
//! `Todo` — gpui exposes `grid_cols`/`grid_rows`/`grid_location`.

use gpui::{
    AlignContent, AlignItems, FlexDirection, FlexWrap, Length, StyleRefinement, Styled, relative,
};

use super::{Ctx, scale_px};

pub(super) fn apply(mut s: StyleRefinement, t: &str, _cx: &mut Ctx) -> (StyleRefinement, bool) {
    match t {
        "flex-row" => return (s.flex_row(), true),
        "flex-col" => return (s.flex_col(), true),
        "flex-row-reverse" => {
            s.flex_direction = Some(FlexDirection::RowReverse);
            return (s, true);
        }
        "flex-col-reverse" => {
            s.flex_direction = Some(FlexDirection::ColumnReverse);
            return (s, true);
        }
        "flex-wrap" => return (s.flex_wrap(), true),
        "flex-nowrap" => {
            s.flex_wrap = Some(FlexWrap::NoWrap);
            return (s, true);
        }
        "flex-wrap-reverse" => {
            s.flex_wrap = Some(FlexWrap::WrapReverse);
            return (s, true);
        }
        "flex-1" => return (s.flex_1(), true),
        "flex-none" => return (s.flex_none(), true),
        // flex-auto: 1 1 auto; flex-initial: 0 1 auto.
        "flex-auto" => {
            s.flex_grow = Some(1.);
            s.flex_shrink = Some(1.);
            s.flex_basis = Some(Length::Auto);
            return (s, true);
        }
        "flex-initial" => {
            s.flex_grow = Some(0.);
            s.flex_shrink = Some(1.);
            s.flex_basis = Some(Length::Auto);
            return (s, true);
        }
        "grow" => return (s.flex_grow(1.), true),
        "shrink" => return (s.flex_shrink(1.), true),
        "shrink-0" => return (s.flex_shrink_0(), true),
        "items-start" => return (s.items_start(), true),
        "items-center" => return (s.items_center(), true),
        "items-end" => return (s.items_end(), true),
        "items-stretch" => {
            s.align_items = Some(AlignItems::Stretch);
            return (s, true);
        }
        "items-baseline" => {
            s.align_items = Some(AlignItems::Baseline);
            return (s, true);
        }
        "justify-start" => return (s.justify_start(), true),
        "justify-center" => return (s.justify_center(), true),
        "justify-end" => return (s.justify_end(), true),
        "justify-between" => return (s.justify_between(), true),
        "justify-around" => return (s.justify_around(), true),
        "justify-evenly" => {
            s.justify_content = Some(AlignContent::SpaceEvenly);
            return (s, true);
        }
        "justify-stretch" => {
            s.justify_content = Some(AlignContent::Stretch);
            return (s, true);
        }
        "self-start" => {
            s.align_self = Some(AlignItems::Start);
            return (s, true);
        }
        "self-end" => {
            s.align_self = Some(AlignItems::End);
            return (s, true);
        }
        "self-center" => {
            s.align_self = Some(AlignItems::Center);
            return (s, true);
        }
        "self-stretch" => {
            s.align_self = Some(AlignItems::Stretch);
            return (s, true);
        }
        "self-baseline" => {
            s.align_self = Some(AlignItems::Baseline);
            return (s, true);
        }
        "content-start" => {
            s.align_content = Some(AlignContent::Start);
            return (s, true);
        }
        "content-center" => {
            s.align_content = Some(AlignContent::Center);
            return (s, true);
        }
        "content-end" => {
            s.align_content = Some(AlignContent::End);
            return (s, true);
        }
        "content-between" => {
            s.align_content = Some(AlignContent::SpaceBetween);
            return (s, true);
        }
        "content-around" => {
            s.align_content = Some(AlignContent::SpaceAround);
            return (s, true);
        }
        "content-evenly" => {
            s.align_content = Some(AlignContent::SpaceEvenly);
            return (s, true);
        }
        "content-stretch" => {
            s.align_content = Some(AlignContent::Stretch);
            return (s, true);
        }
        _ => {}
    }

    // flex-<n>: grow n, shrink 1, basis 0.
    if let Some(v) = t.strip_prefix("flex-")
        && let Ok(n) = v.parse::<f32>()
    {
        s.flex_grow = Some(n);
        s.flex_shrink = Some(1.);
        s.flex_basis = Some(relative(0.).into());
        return (s, true);
    }
    if let Some(v) = t.strip_prefix("grow-")
        && let Ok(n) = v.parse::<f32>()
    {
        return (s.flex_grow(n), true);
    }
    if let Some(v) = t.strip_prefix("shrink-")
        && let Ok(n) = v.parse::<f32>()
    {
        return (s.flex_shrink(n), true);
    }

    // Gap.
    if let Some(v) = t.strip_prefix("gap-x-")
        && let Some(l) = scale_px(v)
    {
        s.gap.width = Some(l.into());
        return (s, true);
    }
    if let Some(v) = t.strip_prefix("gap-y-")
        && let Some(l) = scale_px(v)
    {
        s.gap.height = Some(l.into());
        return (s, true);
    }
    if let Some(v) = t.strip_prefix("gap-")
        && let Some(l) = scale_px(v)
    {
        return (s.gap(l), true);
    }

    (s, false)
}

#[cfg(test)]
mod tests {
    use super::super::{parse, tests::assert_style_eq};
    use crate::theme::Theme;
    use gpui::{AlignContent, AlignItems, FlexDirection, StyleRefinement, Styled, px};

    #[test]
    fn direction_wrap_and_flexibility() {
        let theme = Theme::light();
        let styles = parse(&theme, "flex-col-reverse flex-wrap-reverse flex-auto");
        let mut expected = StyleRefinement::default();
        expected.flex_direction = Some(FlexDirection::ColumnReverse);
        expected.flex_wrap = Some(gpui::FlexWrap::WrapReverse);
        expected.flex_grow = Some(1.);
        expected.flex_shrink = Some(1.);
        expected.flex_basis = Some(gpui::Length::Auto);
        assert_style_eq(&styles.base, &expected);
    }

    #[test]
    fn alignment_families() {
        let theme = Theme::light();
        let styles = parse(
            &theme,
            "items-baseline justify-evenly self-center content-between",
        );
        let mut expected = StyleRefinement::default();
        expected.align_items = Some(AlignItems::Baseline);
        expected.justify_content = Some(AlignContent::SpaceEvenly);
        expected.align_self = Some(AlignItems::Center);
        expected.align_content = Some(AlignContent::SpaceBetween);
        assert_style_eq(&styles.base, &expected);
    }

    #[test]
    fn gap_axes() {
        let theme = Theme::light();
        let styles = parse(&theme, "gap-x-2 gap-y-1");
        let mut expected = StyleRefinement::default();
        expected.gap.width = Some(px(8.).into());
        expected.gap.height = Some(px(4.).into());
        assert_style_eq(&styles.base, &expected);
    }

    #[test]
    fn numeric_grow_shrink_and_flex() {
        let theme = Theme::light();
        let styles = parse(&theme, "flex-2");
        let mut expected = StyleRefinement::default();
        expected.flex_grow = Some(2.);
        expected.flex_shrink = Some(1.);
        expected.flex_basis = Some(gpui::relative(0.).into());
        assert_style_eq(&styles.base, &expected);
    }
}
