//! ContainerQuery — a measure-then-render stand-in for CSS container queries
//! (`@container`), which gpui has no analog for.
//!
//! The child is built from the container width measured on a previous frame:
//! the first frame builds with `None` (unmeasured), prepaint records the
//! actual width, and whenever the width the child was built with no longer
//! matches the measured width the element schedules one more frame so the
//! child rebuilds with the fresh value. Steady state costs nothing; a resize
//! converges in one extra frame.
//!
//! The element itself spans the full width of its parent (that width *is*
//! the measurement), so use it where a `w-full` child is correct — e.g.
//! [`crate::components::field::Field`]'s responsive orientation. Inside
//! width-of-content parents the measurement would chase its own child;
//! give such parents a definite width instead.
//!
//! ```ignore
//! container_query("my-widget", |width, _window, _cx| {
//!     let horizontal = width.is_some_and(|w| w >= px(448.));
//!     if horizontal { row_layout() } else { column_layout() }
//! })
//! ```

use gpui::{
    AnyElement, App, Bounds, Element, ElementId, GlobalElementId, InspectorElementId, IntoElement,
    LayoutId, Pixels, Style, Window, relative,
};

/// Widths within half a physical pixel are the same measurement; anything
/// larger re-renders. Guards against float jitter re-render loops.
const EPSILON: f32 = 0.5;

/// Persisted across frames via gpui element state, keyed by the element's
/// global id: the width most recently measured at prepaint.
#[derive(Clone, Copy, Default)]
struct MeasuredWidth(Option<Pixels>);

/// Decide whether a child built with `built_with` must rebuild now that the
/// container measures `measured` wide.
fn needs_rebuild(built_with: Option<Pixels>, measured: Pixels) -> bool {
    match built_with {
        None => true,
        Some(w) => (f32::from(w) - f32::from(measured)).abs() > EPSILON,
    }
}

type RenderFn = Box<dyn FnOnce(Option<Pixels>, &mut Window, &mut App) -> AnyElement>;

/// See [`container_query`].
pub struct ContainerQuery {
    id: ElementId,
    render: Option<RenderFn>,
}

/// Build `render`'s element from the measured container width — `None` on
/// the first, not-yet-measured frame. The `id` keys the persisted
/// measurement; give siblings distinct ids.
pub fn container_query<F, E>(id: impl Into<ElementId>, render: F) -> ContainerQuery
where
    F: FnOnce(Option<Pixels>, &mut Window, &mut App) -> E + 'static,
    E: IntoElement,
{
    ContainerQuery {
        id: id.into(),
        render: Some(Box::new(move |width, window, cx| {
            render(width, window, cx).into_any_element()
        })),
    }
}

pub struct ContainerQueryLayout {
    child: AnyElement,
    /// Width the child was built with, to compare against the real bounds.
    built_with: Option<Pixels>,
}

impl IntoElement for ContainerQuery {
    type Element = Self;

    fn into_element(self) -> Self::Element {
        self
    }
}

impl Element for ContainerQuery {
    type RequestLayoutState = ContainerQueryLayout;
    type PrepaintState = ();

    fn id(&self) -> Option<ElementId> {
        Some(self.id.clone())
    }

    fn source_location(&self) -> Option<&'static core::panic::Location<'static>> {
        None
    }

    fn request_layout(
        &mut self,
        global_id: Option<&GlobalElementId>,
        _inspector_id: Option<&InspectorElementId>,
        window: &mut Window,
        cx: &mut App,
    ) -> (LayoutId, Self::RequestLayoutState) {
        let render = self.render.take().expect("should only be called once");
        window.with_element_state(
            global_id.unwrap(),
            |state: Option<MeasuredWidth>, window| {
                let state = state.unwrap_or_default();
                let mut child = render(state.0, window, cx);
                let child_id = child.request_layout(window, cx);

                let mut style = Style::default();
                style.size.width = relative(1.).into();
                let layout_id = window.request_layout(style, [child_id], cx);

                (
                    (
                        layout_id,
                        ContainerQueryLayout {
                            child,
                            built_with: state.0,
                        },
                    ),
                    state,
                )
            },
        )
    }

    fn prepaint(
        &mut self,
        global_id: Option<&GlobalElementId>,
        _inspector_id: Option<&InspectorElementId>,
        bounds: Bounds<Pixels>,
        layout: &mut Self::RequestLayoutState,
        window: &mut Window,
        cx: &mut App,
    ) -> Self::PrepaintState {
        let measured = bounds.size.width;
        window.with_element_state(global_id.unwrap(), |_, _| ((), MeasuredWidth(Some(measured))));
        if needs_rebuild(layout.built_with, measured) {
            // The child on screen was built from a stale width; draw it this
            // frame (avoids a blank flash) and rebuild on the next.
            window.request_animation_frame();
        }
        layout.child.prepaint(window, cx);
    }

    fn paint(
        &mut self,
        _id: Option<&GlobalElementId>,
        _inspector_id: Option<&InspectorElementId>,
        _bounds: Bounds<Pixels>,
        layout: &mut Self::RequestLayoutState,
        _prepaint: &mut Self::PrepaintState,
        window: &mut Window,
        cx: &mut App,
    ) {
        layout.child.paint(window, cx);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use gpui::px;

    #[test]
    fn unmeasured_always_rebuilds() {
        assert!(needs_rebuild(None, px(0.)));
        assert!(needs_rebuild(None, px(448.)));
    }

    #[test]
    fn matching_width_is_stable() {
        assert!(!needs_rebuild(Some(px(448.)), px(448.)));
        // Sub-epsilon float jitter must not trigger a re-render loop.
        assert!(!needs_rebuild(Some(px(448.)), px(448.4)));
        assert!(!needs_rebuild(Some(px(448.)), px(447.6)));
    }

    #[test]
    fn changed_width_rebuilds() {
        assert!(needs_rebuild(Some(px(448.)), px(447.)));
        assert!(needs_rebuild(Some(px(320.)), px(448.)));
        assert!(needs_rebuild(Some(px(448.)), px(449.)));
    }
}
