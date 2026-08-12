//! Chart — gpui-native adaptation of shadcn's chart components (recharts
//! upstream, which has no gpui equivalent).
//!
//! `BarChart` draws a categorical bar chart from the theme's chart tokens:
//! gridline background, value-scaled bars, axis labels, and a legend when
//! multiple series are given. Line/area/pie charts and tooltips are
//! omitted. Sizing and shape overrides come from the caller via [`Styled`].

use gpui::{
    App, IntoElement, ParentElement as _, Refineable as _, RenderOnce, SharedString,
    StyleRefinement, Styled, Window, div, prelude::FluentBuilder as _, px, relative,
};

use crate::theme::Theme;

pub struct ChartSeries {
    label: SharedString,
    values: Vec<f32>,
}

impl ChartSeries {
    pub fn new(label: impl Into<SharedString>, values: impl IntoIterator<Item = f32>) -> Self {
        Self {
            label: label.into(),
            values: values.into_iter().collect(),
        }
    }
}

/// Categorical bar chart. Sizing and shape overrides come from the caller via [`Styled`].
#[derive(IntoElement)]
pub struct BarChart {
    categories: Vec<SharedString>,
    series: Vec<ChartSeries>,
    height: f32,
    style: StyleRefinement,
}

impl BarChart {
    pub fn new(categories: impl IntoIterator<Item = impl Into<SharedString>>) -> Self {
        Self {
            categories: categories.into_iter().map(Into::into).collect(),
            series: Vec::new(),
            height: 160.,
            style: StyleRefinement::default(),
        }
    }

    pub fn series(mut self, series: ChartSeries) -> Self {
        self.series.push(series);
        self
    }

    pub fn height(mut self, height: f32) -> Self {
        self.height = height.max(40.);
        self
    }
}

impl Styled for BarChart {
    fn style(&mut self) -> &mut StyleRefinement {
        &mut self.style
    }
}

impl RenderOnce for BarChart {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let theme = Theme::of(cx).clone();
        let max = self
            .series
            .iter()
            .flat_map(|series| series.values.iter().copied())
            .fold(1.0_f32, f32::max);
        let series_count = self.series.len().max(1);

        let mut root =
            div()
                .flex()
                .flex_col()
                .gap(px(8.))
                .w_full()
                // Plot area: bottom-aligned bar clusters over faint gridlines.
                .child(
                    div()
                        .relative()
                        .h(px(self.height))
                        .w_full()
                        .border_b_1()
                        .border_color(theme.border)
                        // Gridlines at 25/50/75%.
                        .children([0.25f32, 0.5, 0.75].map(|fraction| {
                            div()
                                .absolute()
                                .left_0()
                                .w_full()
                                .bottom(relative(fraction))
                                .h(px(1.))
                                .bg(theme.border)
                        }))
                        .child(
                            div()
                                .flex()
                                .flex_row()
                                .items_end()
                                .size_full()
                                .gap(px(12.))
                                .px(px(8.))
                                .children(self.categories.iter().enumerate().map(|(column, _)| {
                                    div()
                                        .flex()
                                        .flex_row()
                                        .flex_1()
                                        .h_full()
                                        .items_end()
                                        .justify_center()
                                        .gap(px(4.))
                                        .children(self.series.iter().enumerate().map(
                                            |(series_index, series)| {
                                                let value = series
                                                    .values
                                                    .get(column)
                                                    .copied()
                                                    .unwrap_or_default();
                                                div()
                                                    .w(px((24. / series_count as f32).max(6.)))
                                                    .h(relative((value / max).clamp(0., 1.)))
                                                    .rounded_t(px(4.))
                                                    .bg(theme.chart
                                                        [series_index % theme.chart.len()])
                                            },
                                        ))
                                })),
                        ),
                )
                // Axis labels.
                .child(div().flex().flex_row().gap(px(12.)).px(px(8.)).children(
                    self.categories.iter().map(|category| {
                        div()
                            .flex_1()
                            .flex()
                            .justify_center()
                            .text_size(px(12.))
                            .text_color(theme.muted_foreground)
                            .child(category.clone())
                    }),
                ))
                // Legend for multi-series charts.
                .when(self.series.len() > 1, |el| {
                    el.child(
                        div()
                            .flex()
                            .flex_row()
                            .justify_center()
                            .gap(px(16.))
                            .children(self.series.iter().enumerate().map(
                                |(series_index, series)| {
                                    div()
                                        .flex()
                                        .flex_row()
                                        .items_center()
                                        .gap(px(6.))
                                        .child(
                                            div()
                                                .size(px(10.))
                                                .rounded(px(2.))
                                                .bg(theme.chart[series_index % theme.chart.len()]),
                                        )
                                        .child(
                                            div()
                                                .text_size(px(12.))
                                                .text_color(theme.muted_foreground)
                                                .child(series.label.clone()),
                                        )
                                },
                            )),
                    )
                });
        root.style().refine(&self.style);
        root
    }
}
