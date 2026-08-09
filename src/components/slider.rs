//! Slider — port of shadcn base-vega `ui/slider.tsx`.
//!
//! Controlled: the caller owns `value` (in `min..=max`) and receives the
//! next value in `on_change` while dragging. Single-thumb only (the
//! source's multi-thumb ranges are omitted). Keyboard control and
//! focus-visible ring are omitted.

use gpui::{
    App, AppContext as _, DragMoveEvent, ElementId, InteractiveElement as _, IntoElement,
    ParentElement as _, Render, RenderOnce, StatefulInteractiveElement as _, Styled, Window, div,
    prelude::FluentBuilder as _, px, relative,
};

use crate::theme::Theme;

type ChangeHandler = Box<dyn Fn(&f32, &mut Window, &mut App) + 'static>;

/// Typed drag payload carrying the slider's id, so concurrent sliders
/// don't cross-talk.
struct SliderDrag(ElementId);

/// Sliders drag a value, not a visual.
struct SliderDragPreview;

impl Render for SliderDragPreview {
    fn render(&mut self, _window: &mut Window, _cx: &mut gpui::Context<Self>) -> impl IntoElement {
        gpui::Empty
    }
}

#[derive(IntoElement)]
pub struct Slider {
    id: ElementId,
    value: f32,
    min: f32,
    max: f32,
    step: f32,
    disabled: bool,
    on_change: Option<ChangeHandler>,
}

impl Slider {
    pub fn new(id: impl Into<ElementId>) -> Self {
        Self {
            id: id.into(),
            value: 0.,
            min: 0.,
            max: 100.,
            step: 1.,
            disabled: false,
            on_change: None,
        }
    }

    pub fn value(mut self, value: f32) -> Self {
        self.value = value;
        self
    }

    pub fn min(mut self, min: f32) -> Self {
        self.min = min;
        self
    }

    pub fn max(mut self, max: f32) -> Self {
        self.max = max;
        self
    }

    /// Snap increment (default 1; use e.g. 0.01 for continuous feel).
    pub fn step(mut self, step: f32) -> Self {
        self.step = step;
        self
    }

    pub fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }

    pub fn on_change(mut self, handler: impl Fn(&f32, &mut Window, &mut App) + 'static) -> Self {
        self.on_change = Some(Box::new(handler));
        self
    }
}

impl RenderOnce for Slider {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let theme = Theme::of(cx).clone();
        let span = (self.max - self.min).max(f32::EPSILON);
        let fraction = ((self.value - self.min) / span).clamp(0., 1.);
        let (id, min, step) = (self.id.clone(), self.min, self.step);

        div()
            .id(self.id)
            .h(px(16.))
            .w_full()
            .flex()
            .items_center()
            .when(self.disabled, |el| el.opacity(0.5))
            .when(!self.disabled, |el| {
                el.on_drag(SliderDrag(id.clone()), |_, _, _, cx| {
                    cx.new(|_| SliderDragPreview)
                })
                .when_some(self.on_change, |el, on_change| {
                    el.on_drag_move(move |event: &DragMoveEvent<SliderDrag>, window, cx| {
                        if event.drag(cx).0 == id {
                            let f = ((event.event.position.x - event.bounds.origin.x)
                                / event.bounds.size.width)
                                .clamp(0., 1.);
                            let raw = min + f * span;
                            let snapped = (raw / step).round() * step;
                            on_change(&snapped, window, cx);
                        }
                    })
                })
            })
            .child(
                // Track: h-1.5 w-full rounded-full bg-muted
                div()
                    .relative()
                    .w_full()
                    .h(px(6.))
                    .rounded_full()
                    .bg(theme.muted)
                    // Range: h-full bg-primary
                    .child(
                        div()
                            .h_full()
                            .w(relative(fraction))
                            .rounded_full()
                            .bg(theme.primary),
                    )
                    // Thumb: size-4 rounded-full border-primary bg-background shadow-sm
                    .child(
                        div()
                            .absolute()
                            .top(px(-5.))
                            .left(relative(fraction))
                            .ml(px(-8.))
                            .size(px(16.))
                            .rounded_full()
                            .border_1()
                            .border_color(theme.primary)
                            .bg(theme.background)
                            .shadow_sm(),
                    ),
            )
    }
}
