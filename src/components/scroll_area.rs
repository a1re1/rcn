//! ScrollArea — port of shadcn base-vega `ui/scroll-area.tsx`.
//!
//! A sized viewport that scrolls its content with a custom painted scrollbar
//! (vertical by default, optional horizontal via [`.horizontal`](Self::horizontal)).
//!
//! Ported from shadcn base-nova + Base UI ScrollArea:
//! - Root `relative` shell with a focusable viewport (`tab_index(0)` + focus ring)
//! - Custom painted scrollbar track (`w-2.5` / `h-2.5`, `p-px`) and fully-rounded
//!   thumb colored with `theme.border`
//! - Proportional thumb sizing (`MIN_THUMB_SIZE = 16`) and offset from scroll ratio
//! - Thumb drag (Base UI `handlePointerMove` math) and track-click jump (centers thumb)
//! - Dual-axis corner reservation (each track inset by the other scrollbar's thickness)
//! - Scrollbar hidden on an axis when content fits (`max_offset == 0`)
//!
//! Omissions (not ported):
//! - TODO(rcn): RTL (`dir="rtl"` mirrors horizontal scrollbar edge + drag math)
//! - Base UI `data-hovering` / `data-scrolling` 500ms fade visibility states
//! - Scrollbar `transition-colors` (no-op visual in base-nova without hover styles)

use gpui::{
    AnyElement, App, AppContext as _, DragMoveEvent, ElementId, InteractiveElement as _,
    IntoElement, MouseButton, ParentElement, Pixels, Render, RenderOnce, ScrollHandle,
    StatefulInteractiveElement as _, Styled, Window, div, point, prelude::FluentBuilder as _, px,
};

use crate::motion;
use crate::theme::Theme;

/// shadcn `w-2.5` / `h-2.5` scrollbar thickness.
const SCROLLBAR_SIZE: f32 = 10.0;
/// shadcn `p-px` track padding.
const SCROLLBAR_PADDING: f32 = 1.0;
/// Base UI `MIN_THUMB_SIZE`.
const MIN_THUMB_SIZE: f32 = 16.0;

/// Drag payload identifying which ScrollArea thumb is being dragged.
/// Geometry + start scroll/pointer live in [`DragSeed`] (captured on mouse-down).
#[derive(Clone, PartialEq, Eq)]
struct ScrollThumbDrag {
    id: ElementId,
    /// `true` = vertical axis (y); `false` = horizontal (x).
    vertical: bool,
}

/// Empty drag preview — scrolling is applied via `ScrollHandle`, not a ghost.
struct ScrollThumbDragPreview;

impl Render for ScrollThumbDragPreview {
    fn render(&mut self, _window: &mut Window, _cx: &mut gpui::Context<Self>) -> impl IntoElement {
        gpui::Empty
    }
}

/// Pointer/scroll snapshot taken on thumb mouse-down for drag math.
#[derive(Clone, Default)]
struct DragSeed {
    pointer: f32,
    scroll: f32,
    vertical: bool,
    track_inner: f32,
    thumb_len: f32,
    max_scroll: f32,
    active: bool,
    id: Option<ElementId>,
}

/// Notify-only entity so wheel scroll / set_offset re-render painted thumbs.
#[derive(Default)]
struct ScrollPaintState;

#[derive(IntoElement)]
pub struct ScrollArea {
    id: ElementId,
    height: Option<Pixels>,
    width: Option<Pixels>,
    /// When set, enable horizontal overflow + a bottom scrollbar (shadcn
    /// `<ScrollBar orientation="horizontal" />`). Vertical remains available
    /// whenever content overflows on y.
    horizontal: bool,
    children: Vec<AnyElement>,
}

impl ScrollArea {
    pub fn new(id: impl Into<ElementId>) -> Self {
        Self {
            id: id.into(),
            height: None,
            width: None,
            horizontal: false,
            children: Vec::new(),
        }
    }

    pub fn h(mut self, height: Pixels) -> Self {
        self.height = Some(height);
        self
    }

    pub fn w(mut self, width: Pixels) -> Self {
        self.width = Some(width);
        self
    }

    /// Paint a horizontal scrollbar along the bottom edge and enable
    /// `overflow_x_scroll` on the viewport (shadcn horizontal ScrollBar).
    pub fn horizontal(mut self) -> Self {
        self.horizontal = true;
        self
    }
}

impl ParentElement for ScrollArea {
    fn extend(&mut self, elements: impl IntoIterator<Item = AnyElement>) {
        self.children.extend(elements);
    }
}

/// Thumb length + free travel along one axis. `None` when that axis does not overflow.
fn axis_thumb(viewport_len: f32, max_scroll: f32, track_outer_len: f32) -> Option<(f32, f32)> {
    if max_scroll <= 0.0 || track_outer_len <= 0.0 || viewport_len <= 0.0 {
        return None;
    }
    let track_inner = (track_outer_len - 2.0 * SCROLLBAR_PADDING).max(0.0);
    if track_inner <= 0.0 {
        return None;
    }
    let content_len = viewport_len + max_scroll;
    let thumb_len = (track_inner * viewport_len / content_len)
        .max(MIN_THUMB_SIZE)
        .min(track_inner);
    Some((track_inner, thumb_len))
}

/// Positive scrolled px from a gpui offset component.
///
/// gpui `ScrollHandle::offset` becomes **more negative** as content scrolls
/// down/right (see `impl ScrollHandle` in gpui `div.rs`); `max_offset` stays
/// positive. Base UI math uses positive scrolled distances, so callers negate.
fn scrolled_px(offset_component: Pixels, max_scroll: f32) -> f32 {
    (-f32::from(offset_component)).clamp(0.0, max_scroll.max(0.0))
}

/// gpui offset component from a positive scrolled-px value.
fn offset_from_scrolled(scrolled: f32, max_scroll: f32) -> Pixels {
    px((-scrolled.clamp(0.0, max_scroll.max(0.0))).clamp(-max_scroll.max(0.0), 0.0))
}

/// Thumb offset from the track's padded start (Base UI scroll ratio math).
fn thumb_offset(scrolled: f32, max_scroll: f32, track_inner: f32, thumb_len: f32) -> f32 {
    let travel = (track_inner - thumb_len).max(0.0);
    if max_scroll <= 0.0 || travel <= 0.0 {
        return 0.0;
    }
    (scrolled / max_scroll).clamp(0.0, 1.0) * travel
}

/// Track-click: jump so the thumb centers on the click (Base UI onPointerDown).
fn scroll_for_track_click(
    click_local: f32,
    track_inner: f32,
    thumb_len: f32,
    max_scroll: f32,
) -> f32 {
    let travel = (track_inner - thumb_len).max(0.0);
    if travel <= 0.0 || max_scroll <= 0.0 {
        return 0.0;
    }
    let ratio = ((click_local - thumb_len / 2.0) / travel).clamp(0.0, 1.0);
    ratio * max_scroll
}

impl RenderOnce for ScrollArea {
    fn render(self, window: &mut Window, cx: &mut App) -> impl IntoElement {
        let theme = Theme::of(cx).clone();
        let id = self.id.clone();
        let want_horizontal = self.horizontal;

        // Persist ScrollHandle + paint notify entity across frames.
        let handle_key: ElementId = (id.clone(), "scroll-handle").into();
        let handle = window.use_keyed_state(handle_key, cx, move |_, _| ScrollHandle::new());
        let handle = handle.read(cx).clone();

        let paint_key: ElementId = (id.clone(), "paint").into();
        let scroll_state =
            window.use_keyed_state(paint_key, cx, move |_, _| ScrollPaintState::default());

        let drag_key: ElementId = (id.clone(), "drag-seed").into();
        let drag_seed = window.use_keyed_state(drag_key, cx, move |_, _| DragSeed::default());

        // Read live geometry from the handle (bounds/offset update after layout).
        let bounds = handle.bounds();
        let max_offset = handle.max_offset();
        let offset = handle.offset();

        let vw = f32::from(bounds.size.width);
        let vh = f32::from(bounds.size.height);
        let max_x = f32::from(max_offset.x).max(0.0);
        let max_y = f32::from(max_offset.y).max(0.0);
        let scrolled_x = scrolled_px(offset.x, max_x);
        let scrolled_y = scrolled_px(offset.y, max_y);

        // Show a scrollbar only when that axis overflows (Base UI hide when max==0).
        // Horizontal track is only requested when the builder opts in; vertical is
        // always available when content overflows y.
        let show_v = max_y > 0.0;
        let show_h = want_horizontal && max_x > 0.0;

        // Corner reservation: inset each track by the other scrollbar's thickness.
        let v_track_outer = if show_h {
            (vh - SCROLLBAR_SIZE).max(0.0)
        } else {
            vh
        };
        let h_track_outer = if show_v {
            (vw - SCROLLBAR_SIZE).max(0.0)
        } else {
            vw
        };

        let v_geo = if show_v {
            axis_thumb(vh, max_y, v_track_outer)
        } else {
            None
        };
        let h_geo = if show_h {
            axis_thumb(vw, max_x, h_track_outer)
        } else {
            None
        };

        let ring = motion::focus_ring(&theme);
        let thumb_bg = theme.border;

        // ── viewport ──────────────────────────────────────────────────────────
        let viewport = div()
            .id(ElementId::from((id.clone(), "viewport")))
            .size_full()
            .tab_index(0)
            .focus_visible(move |s| s.shadow(ring.clone()))
            .track_scroll(&handle)
            .map(|el| {
                // Enable overflow axes: y always (default ScrollArea), x when
                // `.horizontal()` was requested. Dual-axis when both can scroll.
                if want_horizontal {
                    el.overflow_scroll()
                } else {
                    el.overflow_y_scroll()
                }
            })
            .children(self.children);

        // ── vertical track + thumb ─────────────────────────────────────────────
        let v_bar = v_geo.map(|(track_inner, thumb_len)| {
            let thumb_top = thumb_offset(scrolled_y, max_y, track_inner, thumb_len);
            let track_id: ElementId = (id.clone(), "v-track").into();
            let thumb_id: ElementId = (id.clone(), "v-thumb").into();
            let drag_id = id.clone();
            let handle_track = handle.clone();
            let handle_for_seed = handle.clone();
            let notify_state = scroll_state.clone();
            let seed = drag_seed.clone();

            div()
                .id(track_id)
                .absolute()
                .top_0()
                .right_0()
                .w(px(SCROLLBAR_SIZE))
                .h(px(v_track_outer))
                .p(px(SCROLLBAR_PADDING))
                .on_mouse_down(MouseButton::Left, move |event, _window, cx| {
                    // Track-click jump: center thumb on click (Base UI).
                    let b = handle_track.bounds();
                    let track_top = f32::from(b.origin.y) + SCROLLBAR_PADDING;
                    let click_local = f32::from(event.position.y) - track_top;
                    let max_y = f32::from(handle_track.max_offset().y).max(0.0);
                    let vh = f32::from(b.size.height);
                    let show_h = f32::from(handle_track.max_offset().x) > 0.0;
                    let outer = if show_h {
                        (vh - SCROLLBAR_SIZE).max(0.0)
                    } else {
                        vh
                    };
                    let Some((track_inner, thumb_len)) = axis_thumb(vh, max_y, outer) else {
                        return;
                    };
                    let next = scroll_for_track_click(click_local, track_inner, thumb_len, max_y);
                    let cur = handle_track.offset();
                    handle_track.set_offset(point(cur.x, offset_from_scrolled(next, max_y)));
                    notify_state.update(cx, |_, cx| cx.notify());
                })
                .child(
                    div()
                        .id(thumb_id)
                        .w_full()
                        .h(px(thumb_len))
                        .mt(px(thumb_top))
                        .rounded_full()
                        .bg(thumb_bg)
                        .on_mouse_down(MouseButton::Left, {
                            let seed = seed.clone();
                            let drag_id = drag_id.clone();
                            move |event, _window, cx| {
                                // Don't let the track-click handler jump under the thumb.
                                cx.stop_propagation();
                                let max_y = f32::from(handle_for_seed.max_offset().y).max(0.0);
                                let scrolled = scrolled_px(handle_for_seed.offset().y, max_y);
                                seed.update(cx, |s, _| {
                                    s.pointer = f32::from(event.position.y);
                                    s.scroll = scrolled;
                                    s.vertical = true;
                                    s.track_inner = track_inner;
                                    s.thumb_len = thumb_len;
                                    s.max_scroll = max_y;
                                    s.active = true;
                                    s.id = Some(drag_id.clone());
                                });
                            }
                        })
                        .on_drag(
                            ScrollThumbDrag {
                                id: drag_id,
                                vertical: true,
                            },
                            |_, _, _, cx| cx.new(|_| ScrollThumbDragPreview),
                        ),
                )
        });

        // ── horizontal track + thumb ──────────────────────────────────────────
        let h_bar = h_geo.map(|(track_inner, thumb_len)| {
            let thumb_left = thumb_offset(scrolled_x, max_x, track_inner, thumb_len);
            let track_id: ElementId = (id.clone(), "h-track").into();
            let thumb_id: ElementId = (id.clone(), "h-thumb").into();
            let drag_id = id.clone();
            let handle_track = handle.clone();
            let handle_for_seed = handle.clone();
            let notify_state = scroll_state.clone();
            let seed = drag_seed.clone();

            div()
                .id(track_id)
                .absolute()
                .left_0()
                .bottom_0()
                .h(px(SCROLLBAR_SIZE))
                .w(px(h_track_outer))
                .p(px(SCROLLBAR_PADDING))
                .on_mouse_down(MouseButton::Left, move |event, _window, cx| {
                    let b = handle_track.bounds();
                    let track_left = f32::from(b.origin.x) + SCROLLBAR_PADDING;
                    let click_local = f32::from(event.position.x) - track_left;
                    let max_x = f32::from(handle_track.max_offset().x).max(0.0);
                    let vw = f32::from(b.size.width);
                    let show_v = f32::from(handle_track.max_offset().y) > 0.0;
                    let outer = if show_v {
                        (vw - SCROLLBAR_SIZE).max(0.0)
                    } else {
                        vw
                    };
                    let Some((track_inner, thumb_len)) = axis_thumb(vw, max_x, outer) else {
                        return;
                    };
                    let next = scroll_for_track_click(click_local, track_inner, thumb_len, max_x);
                    let cur = handle_track.offset();
                    handle_track.set_offset(point(offset_from_scrolled(next, max_x), cur.y));
                    notify_state.update(cx, |_, cx| cx.notify());
                })
                .child(
                    div()
                        .id(thumb_id)
                        .h_full()
                        .w(px(thumb_len))
                        .ml(px(thumb_left))
                        .rounded_full()
                        .bg(thumb_bg)
                        .on_mouse_down(MouseButton::Left, {
                            let seed = seed.clone();
                            let drag_id = drag_id.clone();
                            move |event, _window, cx| {
                                cx.stop_propagation();
                                let max_x = f32::from(handle_for_seed.max_offset().x).max(0.0);
                                let scrolled = scrolled_px(handle_for_seed.offset().x, max_x);
                                seed.update(cx, |s, _| {
                                    s.pointer = f32::from(event.position.x);
                                    s.scroll = scrolled;
                                    s.vertical = false;
                                    s.track_inner = track_inner;
                                    s.thumb_len = thumb_len;
                                    s.max_scroll = max_x;
                                    s.active = true;
                                    s.id = Some(drag_id.clone());
                                });
                            }
                        })
                        .on_drag(
                            ScrollThumbDrag {
                                id: drag_id,
                                vertical: false,
                            },
                            |_, _, _, cx| cx.new(|_| ScrollThumbDragPreview),
                        ),
                )
        });

        let handle_move = handle.clone();
        let notify_move = scroll_state.clone();
        let seed_move = drag_seed.clone();
        let move_id = id.clone();

        div()
            .id(id)
            .relative()
            .when_some(self.height, |el, height| el.h(height))
            .when_some(self.width, |el, width| el.w(width))
            // Re-render painted thumbs after native wheel scroll.
            .on_scroll_wheel({
                let notify = scroll_state.clone();
                move |_, _, cx| {
                    notify.update(cx, |_, cx| cx.notify());
                }
            })
            .on_drag_move(move |event: &DragMoveEvent<ScrollThumbDrag>, _window, cx| {
                let drag = event.drag(cx);
                if drag.id != move_id {
                    return;
                }
                let seed = seed_move.read(cx).clone();
                if !seed.active || seed.id.as_ref() != Some(&move_id) {
                    return;
                }
                // Prefer the axis recorded at mouse-down (matches the thumb hit).
                let vertical = seed.vertical;
                let pointer = if vertical {
                    f32::from(event.event.position.y)
                } else {
                    f32::from(event.event.position.x)
                };
                let delta = pointer - seed.pointer;
                let travel = (seed.track_inner - seed.thumb_len).max(0.0);
                // Base UI handlePointerMove:
                //   new_scroll = start_scroll + (delta / travel) * max_scroll
                let next = if travel > 0.0 {
                    seed.scroll + (delta / travel) * seed.max_scroll
                } else {
                    seed.scroll
                };
                let cur = handle_move.offset();
                // gpui offsets are negative as content scrolls down/right.
                if vertical {
                    handle_move
                        .set_offset(point(cur.x, offset_from_scrolled(next, seed.max_scroll)));
                } else {
                    handle_move
                        .set_offset(point(offset_from_scrolled(next, seed.max_scroll), cur.y));
                }
                notify_move.update(cx, |_, cx| cx.notify());
            })
            .child(viewport)
            .children(v_bar)
            .children(h_bar)
    }
}
