//! Progress — port of shadcn base-vega `ui/progress.tsx`.
//!
//! A value-driven bar: optional label and percentage readout above a muted
//! track with a primary indicator. The source's indeterminate state and
//! width transition are omitted. Sizing and shape overrides come from the
//! caller via [`Styled`].

use gpui::{
    App, IntoElement, ParentElement, Refineable as _, RenderOnce, SharedString, StyleRefinement,
    Styled, Window, div, prelude::FluentBuilder as _, px, relative,
};

use crate::theme::Theme;

#[derive(IntoElement)]
pub struct Progress {
    /// 0..=100, like the source's `value` prop.
    value: f32,
    label: Option<SharedString>,
    show_value: bool,
    style: StyleRefinement,
}

impl Progress {
    pub fn new(value: f32) -> Self {
        Self {
            value: value.clamp(0., 100.),
            label: None,
            show_value: false,
            style: StyleRefinement::default(),
        }
    }

    /// Renders a ProgressLabel row above the track.
    pub fn label(mut self, label: impl Into<SharedString>) -> Self {
        self.label = Some(label.into());
        self
    }

    /// Renders the ProgressValue percentage readout at the row's end.
    pub fn show_value(mut self) -> Self {
        self.show_value = true;
        self
    }
}

impl Styled for Progress {
    fn style(&mut self) -> &mut StyleRefinement {
        &mut self.style
    }
}

impl RenderOnce for Progress {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let theme = Theme::of(cx).clone();
        let has_header = self.label.is_some() || self.show_value;

        // Root: flex flex-wrap gap-3 (label/value row + track)
        let mut root = div()
            .flex()
            .flex_col()
            .gap(px(12.))
            .w_full()
            .when(has_header, |el| {
                el.child(
                    div()
                        .flex()
                        .flex_row()
                        .items_center()
                        .w_full()
                        .when_some(self.label.clone(), |el, label| {
                            // text-sm font-medium
                            el.child(
                                div()
                                    .text_size(px(14.))
                                    .line_height(px(20.))
                                    .font_weight(gpui::FontWeight::MEDIUM)
                                    .child(label),
                            )
                        })
                        .when(self.show_value, |el| {
                            // ml-auto text-sm text-muted-foreground tabular-nums
                            el.child(
                                div()
                                    .ml_auto()
                                    .text_size(px(14.))
                                    .line_height(px(20.))
                                    .text_color(theme.muted_foreground)
                                    .child(format!("{:.0}%", self.value)),
                            )
                        }),
                )
            })
            // Track: h-1.5 w-full rounded-full bg-muted, overflow hidden
            .child(
                div()
                    .h(px(6.))
                    .w_full()
                    .rounded_full()
                    .bg(theme.muted)
                    .overflow_hidden()
                    // Indicator: h-full bg-primary at value%
                    .child(
                        div()
                            .h_full()
                            .w(relative(self.value / 100.))
                            .rounded_full()
                            .bg(theme.primary),
                    ),
            );
        root.style().refine(&self.style);
        root
    }
}
