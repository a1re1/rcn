//! Flexbox & Grid family: direction, wrap, grow/shrink, alignment, gap,
//! grid templates (`grid-cols-*`) and placement (`col-span-*`, `col-start-*`,
//! …) via gpui's `grid_cols`/`grid_rows`/`grid_location`.
//! Docs chapter: <https://tailwindcss.com/docs/flex-basis>

use gpui::{
    AlignContent, AlignItems, FlexDirection, FlexWrap, GridAutoFlow, GridAutoTrackSize, Length,
    StyleRefinement, Styled, relative,
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

    // Grid-only alignment (justify-items / justify-self), the place-*
    // shorthands (set both axes), and grid auto-flow. These Style fields come
    // from the rcn gpui fork branch (see docs/tw-roadmap.md phase 3).
    match t {
        "justify-items-start" => return (justify_items(s, AlignItems::Start), true),
        "justify-items-center" => return (justify_items(s, AlignItems::Center), true),
        "justify-items-end" => return (justify_items(s, AlignItems::End), true),
        "justify-items-stretch" => return (justify_items(s, AlignItems::Stretch), true),
        "justify-self-start" => return (justify_self(s, AlignItems::Start), true),
        "justify-self-center" => return (justify_self(s, AlignItems::Center), true),
        "justify-self-end" => return (justify_self(s, AlignItems::End), true),
        "justify-self-stretch" => return (justify_self(s, AlignItems::Stretch), true),
        "place-items-start" => return (place_items(s, AlignItems::Start), true),
        "place-items-center" => return (place_items(s, AlignItems::Center), true),
        "place-items-end" => return (place_items(s, AlignItems::End), true),
        "place-items-stretch" => return (place_items(s, AlignItems::Stretch), true),
        "place-items-baseline" => return (place_items(s, AlignItems::Baseline), true),
        "place-self-start" => return (place_self(s, AlignItems::Start), true),
        "place-self-center" => return (place_self(s, AlignItems::Center), true),
        "place-self-end" => return (place_self(s, AlignItems::End), true),
        "place-self-stretch" => return (place_self(s, AlignItems::Stretch), true),
        "place-content-start" => return (place_content(s, AlignContent::Start), true),
        "place-content-center" => return (place_content(s, AlignContent::Center), true),
        "place-content-end" => return (place_content(s, AlignContent::End), true),
        "place-content-between" => return (place_content(s, AlignContent::SpaceBetween), true),
        "place-content-around" => return (place_content(s, AlignContent::SpaceAround), true),
        "place-content-evenly" => return (place_content(s, AlignContent::SpaceEvenly), true),
        "place-content-stretch" => return (place_content(s, AlignContent::Stretch), true),
        "grid-flow-row" => return (grid_flow(s, GridAutoFlow::Row), true),
        "grid-flow-col" => return (grid_flow(s, GridAutoFlow::Column), true),
        "grid-flow-dense" | "grid-flow-row-dense" => {
            return (grid_flow(s, GridAutoFlow::RowDense), true);
        }
        "grid-flow-col-dense" => return (grid_flow(s, GridAutoFlow::ColumnDense), true),
        "auto-cols-auto" => return (auto_cols(s, GridAutoTrackSize::Auto), true),
        "auto-cols-min" => return (auto_cols(s, GridAutoTrackSize::MinContent), true),
        "auto-cols-max" => return (auto_cols(s, GridAutoTrackSize::MaxContent), true),
        "auto-cols-fr" => return (auto_cols(s, GridAutoTrackSize::Fr), true),
        "auto-rows-auto" => return (auto_rows(s, GridAutoTrackSize::Auto), true),
        "auto-rows-min" => return (auto_rows(s, GridAutoTrackSize::MinContent), true),
        "auto-rows-max" => return (auto_rows(s, GridAutoTrackSize::MaxContent), true),
        "auto-rows-fr" => return (auto_rows(s, GridAutoTrackSize::Fr), true),
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

fn justify_items(mut s: StyleRefinement, v: AlignItems) -> StyleRefinement {
    s.justify_items = Some(v);
    s
}

fn justify_self(mut s: StyleRefinement, v: AlignItems) -> StyleRefinement {
    s.justify_self = Some(v);
    s
}

fn place_items(mut s: StyleRefinement, v: AlignItems) -> StyleRefinement {
    s.align_items = Some(v);
    s.justify_items = Some(v);
    s
}

fn place_self(mut s: StyleRefinement, v: AlignItems) -> StyleRefinement {
    s.align_self = Some(v);
    s.justify_self = Some(v);
    s
}

fn place_content(mut s: StyleRefinement, v: AlignContent) -> StyleRefinement {
    s.align_content = Some(v);
    s.justify_content = Some(v);
    s
}

fn grid_flow(mut s: StyleRefinement, v: GridAutoFlow) -> StyleRefinement {
    s.grid_auto_flow = Some(v);
    s
}

fn auto_cols(mut s: StyleRefinement, v: GridAutoTrackSize) -> StyleRefinement {
    s.grid_auto_cols = Some(v);
    s
}

fn auto_rows(mut s: StyleRefinement, v: GridAutoTrackSize) -> StyleRefinement {
    s.grid_auto_rows = Some(v);
    s
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
    fn place_shorthands_justify_grid_and_flow() {
        let theme = Theme::light();
        let styles = parse(
            &theme,
            "place-items-center place-content-between grid-flow-col",
        );
        let mut expected = StyleRefinement::default();
        expected.align_items = Some(AlignItems::Center);
        expected.justify_items = Some(AlignItems::Center);
        expected.align_content = Some(AlignContent::SpaceBetween);
        expected.justify_content = Some(AlignContent::SpaceBetween);
        expected.grid_auto_flow = Some(gpui::GridAutoFlow::Column);
        assert_style_eq(&styles.base, &expected);

        let styles = parse(&theme, "justify-items-end place-self-center");
        let mut expected = StyleRefinement::default();
        expected.justify_items = Some(AlignItems::End);
        expected.align_self = Some(AlignItems::Center);
        expected.justify_self = Some(AlignItems::Center);
        assert_style_eq(&styles.base, &expected);
    }

    #[test]
    fn auto_track_presets() {
        let theme = Theme::light();
        let styles = parse(&theme, "auto-cols-fr auto-rows-min");
        let mut expected = StyleRefinement::default();
        expected.grid_auto_cols = Some(gpui::GridAutoTrackSize::Fr);
        expected.grid_auto_rows = Some(gpui::GridAutoTrackSize::MinContent);
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
