//! Flexbox & Grid family: direction, wrap, grow/shrink, alignment, gap,
//! grid templates (`grid-cols-*`) and placement (`col-span-*`, `col-start-*`,
//! …) via gpui's `grid_cols`/`grid_rows`/`grid_location`.
//! Docs chapter: <https://tailwindcss.com/docs/flex-basis>

use gpui::{
    AlignContent, AlignItems, FlexDirection, FlexWrap, Length, StyleRefinement, Styled, relative,
};

use super::{Ctx, scale_len, scale_px};

pub(super) fn apply(mut s: StyleRefinement, t: &str, cx: &mut Ctx) -> (StyleRefinement, bool) {
    match t {
        // Grid placement autos. `col-auto` is CSS `grid-column: auto`.
        "col-auto" => return (s.col_start_auto().col_end_auto(), true),
        "row-auto" => return (s.row_start_auto().row_end_auto(), true),
        "col-start-auto" => return (s.col_start_auto(), true),
        "col-end-auto" => return (s.col_end_auto(), true),
        "row-start-auto" => return (s.row_start_auto(), true),
        "row-end-auto" => return (s.row_end_auto(), true),
        "col-span-full" => return (s.col_span_full(), true),
        "row-span-full" => return (s.row_span_full(), true),
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

    // basis-<n> / basis-1/2 / basis-auto / basis-full
    if let Some(v) = t.strip_prefix("basis-")
        && let Some(l) = scale_len(v)
    {
        s.flex_basis = Some(l);
        return (s, true);
    }

    // Grid templates and numeric placement. Longer prefixes first so
    // `col-` doesn't shadow `col-span-`. Negative line indices come from the
    // token's leading `-` (`-col-start-1`), carried in `cx.sign`.
    if let Some(v) = t.strip_prefix("grid-cols-")
        && let Ok(n) = v.parse::<u16>()
    {
        return (s.grid_cols(n), true);
    }
    if let Some(v) = t.strip_prefix("grid-rows-")
        && let Ok(n) = v.parse::<u16>()
    {
        return (s.grid_rows(n), true);
    }
    for (prefix, f) in [
        (
            "col-span-",
            (|s: StyleRefinement, n: f32| s.col_span(n as u16))
                as fn(StyleRefinement, f32) -> StyleRefinement,
        ),
        ("row-span-", |s, n| s.row_span(n as u16)),
        ("col-start-", |s, n| s.col_start(n as i16)),
        ("col-end-", |s, n| s.col_end(n as i16)),
        ("row-start-", |s, n| s.row_start(n as i16)),
        ("row-end-", |s, n| s.row_end(n as i16)),
        // CSS `grid-column: N` = start at line N, end auto.
        ("col-", |s, n| s.col_start(n as i16).col_end_auto()),
        ("row-", |s, n| s.row_start(n as i16).row_end_auto()),
    ] {
        if let Some(v) = t.strip_prefix(prefix)
            && let Ok(n) = v.parse::<f32>()
        {
            return (f(s, n * cx.sign), true);
        }
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
    use gpui::{
        AlignContent, AlignItems, FlexDirection, Length, StyleRefinement, Styled, px, relative,
    };

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
    fn grid_templates_and_placement() {
        let theme = Theme::light();
        let styles = parse(&theme, "grid-cols-3 grid-rows-2");
        let expected = StyleRefinement::default().grid_cols(3).grid_rows(2);
        assert_style_eq(&styles.base, &expected);

        let styles = parse(&theme, "col-span-2 row-start-1 row-end-3");
        let expected = StyleRefinement::default()
            .col_span(2)
            .row_start(1)
            .row_end(3);
        assert_style_eq(&styles.base, &expected);

        // CSS `grid-column: 3` shorthand: start at line 3, end auto.
        let styles = parse(&theme, "col-3");
        let expected = StyleRefinement::default().col_start(3).col_end_auto();
        assert_style_eq(&styles.base, &expected);

        // Negative line index counts from the end.
        let styles = parse(&theme, "-col-start-1");
        let expected = StyleRefinement::default().col_start(-1);
        assert_style_eq(&styles.base, &expected);
    }

    #[test]
    fn grid_placement_autos_and_full() {
        let theme = Theme::light();
        let styles = parse(&theme, "col-span-full row-auto");
        let expected = StyleRefinement::default()
            .col_span_full()
            .row_start_auto()
            .row_end_auto();
        assert_style_eq(&styles.base, &expected);
    }

    #[test]
    fn numeric_grow_shrink_and_flex() {
        let theme = Theme::light();
        let styles = parse(&theme, "flex-2");
        let mut expected = StyleRefinement::default();
        expected.flex_grow = Some(2.);
        expected.flex_shrink = Some(1.);
        expected.flex_basis = Some(relative(0.).into());
        assert_style_eq(&styles.base, &expected);
    }

    #[test]
    fn basis_scale_fraction_auto_full() {
        let theme = Theme::light();

        let styles = parse(&theme, "basis-4");
        let mut expected = StyleRefinement::default();
        expected.flex_basis = Some(px(16.).into());
        assert_style_eq(&styles.base, &expected);

        let styles = parse(&theme, "basis-1/2");
        let mut expected = StyleRefinement::default();
        expected.flex_basis = Some(relative(0.5).into());
        assert_style_eq(&styles.base, &expected);

        let styles = parse(&theme, "basis-auto");
        let mut expected = StyleRefinement::default();
        expected.flex_basis = Some(Length::Auto);
        assert_style_eq(&styles.base, &expected);

        let styles = parse(&theme, "basis-full");
        let mut expected = StyleRefinement::default();
        expected.flex_basis = Some(relative(1.).into());
        assert_style_eq(&styles.base, &expected);
    }
}
