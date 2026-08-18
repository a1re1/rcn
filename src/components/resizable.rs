//! Resizable — port of shadcn base-nova `ui/resizable.tsx`
//! (react-resizable-panels upstream).
//!
//! N-panel group with self-managed layout state, wide handle hit areas,
//! optional grip, vertical orientation, nested groups, keyboard resize,
//! and min/max/default/collapsible panel sizes.
//!
//! Sizing and shape overrides come from the caller via [`Styled`].

use std::rc::Rc;

use gpui::{
    AnyElement, App, AppContext as _, Context, DragMoveEvent, ElementId, Entity,
    InteractiveElement as _, IntoElement, ParentElement as _, Refineable as _, Render, RenderOnce,
    StatefulInteractiveElement as _, StyleRefinement, Styled, Window, div,
    prelude::FluentBuilder as _, px,
};

use crate::theme::Theme;

/// Layout direction for a [`ResizablePanelGroup`].
#[derive(Clone, Copy, PartialEq, Eq, Default, Debug)]
pub enum ResizableDirection {
    #[default]
    Horizontal,
    Vertical,
}

type LayoutChangeHandler = Rc<dyn Fn(&Vec<f32>, &mut Window, &mut App) + 'static>;

/// Typed drag payload so nested groups don't cross-talk.
struct HandleDrag {
    group: ElementId,
    /// Handle sits between panel `index` and panel `index + 1`.
    index: usize,
}

/// Empty drag preview (matches slider).
struct HandleDragPreview;

impl Render for HandleDragPreview {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        gpui::Empty
    }
}

/// Child slot stored in call order: panel or handle.
enum ResizableChild {
    Panel(ResizablePanel),
    Handle(ResizableHandle),
}

/// Constraints + content for one panel (used during drag/keyboard resize).
#[derive(Clone, Copy)]
struct PanelConstraints {
    min_size: f32,
    max_size: f32,
    collapsible: bool,
    collapsed_size: f32,
}

/// Keyboard step as a fraction of the group (react-resizable-panels default).
const KEYBOARD_STEP: f32 = 0.10;

// ─── ResizablePanelGroup ────────────────────────────────────────────────────

/// A flex group of panels separated by draggable handles.
///
/// Compose with [`.panel`](Self::panel) / [`.handle`](Self::handle) in
/// shadcn order: Panel, Handle, Panel, …
/// Sizing and shape overrides come from the caller via [`Styled`].
#[derive(IntoElement)]
pub struct ResizablePanelGroup {
    id: ElementId,
    direction: ResizableDirection,
    children: Vec<ResizableChild>,
    on_layout_change: Option<LayoutChangeHandler>,
    style: StyleRefinement,
}

impl ResizablePanelGroup {
    pub fn new(id: impl Into<ElementId>) -> Self {
        Self {
            id: id.into(),
            direction: ResizableDirection::default(),
            children: Vec::new(),
            on_layout_change: None,
            style: StyleRefinement::default(),
        }
    }

    pub fn direction(mut self, direction: ResizableDirection) -> Self {
        self.direction = direction;
        self
    }

    pub fn panel(mut self, panel: ResizablePanel) -> Self {
        self.children.push(ResizableChild::Panel(panel));
        self
    }

    pub fn handle(mut self, handle: ResizableHandle) -> Self {
        self.children.push(ResizableChild::Handle(handle));
        self
    }

    // Public API callback — exercised by consumers; storybook demos are uncontrolled.
    #[allow(dead_code)]
    pub fn on_layout_change(
        mut self,
        handler: impl Fn(&Vec<f32>, &mut Window, &mut App) + 'static,
    ) -> Self {
        self.on_layout_change = Some(Rc::new(handler));
        self
    }
}

impl Styled for ResizablePanelGroup {
    fn style(&mut self) -> &mut StyleRefinement {
        &mut self.style
    }
}

// ─── ResizablePanel ─────────────────────────────────────────────────────────

/// One panel inside a [`ResizablePanelGroup`].
/// Sizing and shape overrides come from the caller via [`Styled`].
#[derive(IntoElement)]
pub struct ResizablePanel {
    default_size: Option<f32>,
    min_size: f32,
    max_size: f32,
    collapsible: bool,
    collapsed_size: f32,
    children: Vec<AnyElement>,
    style: StyleRefinement,
}

impl ResizablePanel {
    pub fn new() -> Self {
        Self {
            default_size: None,
            min_size: 0.10,
            max_size: 1.0,
            collapsible: false,
            collapsed_size: 0.0,
            children: Vec::new(),
            style: StyleRefinement::default(),
        }
    }

    /// Initial fraction of the group (e.g. `0.25`). Panels without a default
    /// split the remainder equally.
    pub fn default_size(mut self, size: f32) -> Self {
        self.default_size = Some(size);
        self
    }

    /// Minimum fraction while expanded (default `0.10`).
    pub fn min_size(mut self, size: f32) -> Self {
        self.min_size = size;
        self
    }

    /// Maximum fraction (default `1.0`).
    #[allow(dead_code)]
    pub fn max_size(mut self, size: f32) -> Self {
        self.max_size = size;
        self
    }

    /// When true, dragging past the halfway threshold snaps to
    /// [`collapsed_size`](Self::collapsed_size).
    pub fn collapsible(mut self, collapsible: bool) -> Self {
        self.collapsible = collapsible;
        self
    }

    /// Size while collapsed (default `0.0`).
    pub fn collapsed_size(mut self, size: f32) -> Self {
        self.collapsed_size = size;
        self
    }

    /// Append panel content. May be called multiple times.
    pub fn child(mut self, child: impl IntoElement) -> Self {
        self.children.push(child.into_any_element());
        self
    }
}

impl Default for ResizablePanel {
    fn default() -> Self {
        Self::new()
    }
}

impl Styled for ResizablePanel {
    fn style(&mut self) -> &mut StyleRefinement {
        &mut self.style
    }
}

// ─── ResizableHandle ────────────────────────────────────────────────────────

/// Draggable separator between two panels.
/// Sizing and shape overrides come from the caller via [`Styled`].
#[derive(IntoElement)]
pub struct ResizableHandle {
    with_handle: bool,
    style: StyleRefinement,
}

impl ResizableHandle {
    pub fn new() -> Self {
        Self {
            with_handle: false,
            style: StyleRefinement::default(),
        }
    }

    /// Show the centered grip pill (shadcn `withHandle`).
    pub fn with_handle(mut self, with_handle: bool) -> Self {
        self.with_handle = with_handle;
        self
    }
}

impl Default for ResizableHandle {
    fn default() -> Self {
        Self::new()
    }
}

impl Styled for ResizableHandle {
    fn style(&mut self) -> &mut StyleRefinement {
        &mut self.style
    }
}

// ─── Layout helpers ─────────────────────────────────────────────────────────

fn compute_initial_sizes(panels: &[&ResizablePanel]) -> Vec<f32> {
    let n = panels.len();
    if n == 0 {
        return Vec::new();
    }

    let mut sizes = vec![0.0_f32; n];
    let mut remainder = 1.0_f32;
    let mut unset = 0usize;

    for (i, panel) in panels.iter().enumerate() {
        if let Some(d) = panel.default_size {
            sizes[i] = d;
            remainder -= d;
        } else {
            unset += 1;
        }
    }

    if unset > 0 {
        let each = (remainder / unset as f32).max(0.0);
        for (i, panel) in panels.iter().enumerate() {
            if panel.default_size.is_none() {
                sizes[i] = each;
            }
        }
    }

    normalize_sizes(&mut sizes);
    sizes
}

fn normalize_sizes(sizes: &mut [f32]) {
    let sum: f32 = sizes.iter().sum();
    if sum <= f32::EPSILON {
        let n = sizes.len();
        if n > 0 {
            let each = 1.0 / n as f32;
            for s in sizes.iter_mut() {
                *s = each;
            }
        }
        return;
    }
    for s in sizes.iter_mut() {
        *s /= sum;
    }
}

/// Resize the pair of panels around `index` so panel `index` becomes `desired_a`
/// (subject to min/max/collapse). Mutates `sizes` in place.
fn apply_pair_resize(
    sizes: &mut [f32],
    constraints: &[PanelConstraints],
    index: usize,
    desired_a: f32,
) {
    if index + 1 >= sizes.len() || index >= constraints.len() {
        return;
    }

    let min_a = constraints[index].min_size;
    let max_a = constraints[index].max_size;
    let collapsible_a = constraints[index].collapsible;
    let collapsed_a = constraints[index].collapsed_size;

    let min_b = constraints[index + 1].min_size;
    let max_b = constraints[index + 1].max_size;
    let collapsible_b = constraints[index + 1].collapsible;
    let collapsed_b = constraints[index + 1].collapsed_size;

    let combined = sizes[index] + sizes[index + 1];
    let lo = min_a.max(combined - max_b);
    let hi = max_a.min(combined - min_b);

    let a = if desired_a < lo && collapsible_a && desired_a < (collapsed_a + min_a) / 2.0 {
        collapsed_a.min(combined)
    } else if desired_a > hi
        && collapsible_b
        && (combined - desired_a) < (collapsed_b + min_b) / 2.0
    {
        (combined - collapsed_b).max(0.0)
    } else {
        desired_a.clamp(lo, hi)
    };

    sizes[index] = a;
    sizes[index + 1] = combined - a;
}

/// Toggle collapse for the pair around `index` (Enter key).
fn apply_toggle_collapse(sizes: &mut [f32], constraints: &[PanelConstraints], index: usize) {
    if index + 1 >= sizes.len() || index >= constraints.len() {
        return;
    }

    let a = constraints[index];
    let b = constraints[index + 1];
    let combined = sizes[index] + sizes[index + 1];

    if a.collapsible {
        let currently_collapsed = sizes[index] <= a.collapsed_size + f32::EPSILON;
        let new_a = if currently_collapsed {
            a.min_size.min(combined)
        } else {
            a.collapsed_size.min(combined)
        };
        sizes[index] = new_a;
        sizes[index + 1] = combined - new_a;
    } else if b.collapsible {
        let currently_collapsed = sizes[index + 1] <= b.collapsed_size + f32::EPSILON;
        let new_b = if currently_collapsed {
            b.min_size.min(combined)
        } else {
            b.collapsed_size.min(combined)
        };
        sizes[index + 1] = new_b;
        sizes[index] = combined - new_b;
    }
}

fn write_layout(
    state: &Entity<Vec<f32>>,
    constraints: &[PanelConstraints],
    index: usize,
    desired_a: f32,
    on_layout_change: &Option<LayoutChangeHandler>,
    window: &mut Window,
    cx: &mut App,
) {
    state.update(cx, |sizes, cx| {
        apply_pair_resize(sizes, constraints, index, desired_a);
        cx.notify();
    });
    if let Some(cb) = on_layout_change {
        let sizes = state.read(cx).clone();
        cb(&sizes, window, cx);
    }
}

fn write_toggle(
    state: &Entity<Vec<f32>>,
    constraints: &[PanelConstraints],
    index: usize,
    on_layout_change: &Option<LayoutChangeHandler>,
    window: &mut Window,
    cx: &mut App,
) {
    state.update(cx, |sizes, cx| {
        apply_toggle_collapse(sizes, constraints, index);
        cx.notify();
    });
    if let Some(cb) = on_layout_change {
        let sizes = state.read(cx).clone();
        cb(&sizes, window, cx);
    }
}

// ─── Render ─────────────────────────────────────────────────────────────────

impl RenderOnce for ResizablePanelGroup {
    fn render(self, window: &mut Window, cx: &mut App) -> impl IntoElement {
        let theme = Theme::of(cx).clone();
        let horizontal = self.direction == ResizableDirection::Horizontal;
        let group_id = self.id.clone();
        let on_layout_change = self.on_layout_change.clone();

        // Split children into ordered panels + (handle_index → handle).
        // handle_index is the panel index to the left/above the handle.
        let mut panels: Vec<ResizablePanel> = Vec::new();
        let mut handles: Vec<(usize, ResizableHandle)> = Vec::new();
        let mut panel_count = 0usize;

        for child in self.children {
            match child {
                ResizableChild::Panel(p) => {
                    panels.push(p);
                    panel_count += 1;
                }
                ResizableChild::Handle(h) => {
                    // Handle sits between the last-added panel and the next.
                    let index = panel_count.saturating_sub(1);
                    handles.push((index, h));
                }
            }
        }

        let panel_refs: Vec<&ResizablePanel> = panels.iter().collect();
        let initial_sizes = compute_initial_sizes(&panel_refs);

        let constraints: Vec<PanelConstraints> = panels
            .iter()
            .map(|p| PanelConstraints {
                min_size: p.min_size,
                max_size: p.max_size,
                collapsible: p.collapsible,
                collapsed_size: p.collapsed_size,
            })
            .collect();

        let state_key: ElementId = (self.id.clone(), "layout").into();
        let panel_n = panels.len();
        let state = window.use_keyed_state(state_key, cx, {
            let initial = initial_sizes.clone();
            move |_, _| initial
        });

        // If panel count changed, re-seed layout.
        if state.read(cx).len() != panel_n {
            let fresh = initial_sizes.clone();
            state.update(cx, |sizes, cx| {
                *sizes = fresh;
                cx.notify();
            });
        }

        let sizes = state.read(cx).clone();

        // Build children in original order, pairing handles with panels.
        let mut handle_iter = handles.into_iter().peekable();
        let mut built: Vec<AnyElement> = Vec::new();

        for (pi, panel) in panels.into_iter().enumerate() {
            let size = sizes.get(pi).copied().unwrap_or(0.0);

            let mut panel_el = div()
                .overflow_hidden()
                .flex_grow(size)
                .flex_shrink(1.)
                .flex_basis(px(0.))
                .children(panel.children);

            panel_el = if horizontal {
                panel_el.h_full()
            } else {
                panel_el.w_full()
            };
            panel_el.style().refine(&panel.style);

            built.push(panel_el.into_any_element());

            // Emit any handle that belongs after this panel (index == pi).
            while let Some((h_idx, _)) = handle_iter.peek() {
                if *h_idx != pi {
                    break;
                }
                let (index, handle) = handle_iter.next().unwrap();

                let hit_id: ElementId = (group_id.clone(), format!("handle-{index}")).into();
                let drag_group = group_id.clone();
                let ring = crate::motion::focus_ring(&theme);

                // Invisible wide hit area (stateful for on_drag).
                let hit = div()
                    .id(hit_id)
                    .absolute()
                    .occlude()
                    .map(|el| {
                        if horizontal {
                            el.left(px(-4.))
                                .w(px(9.))
                                .top_0()
                                .bottom_0()
                                .cursor_col_resize()
                        } else {
                            el.top(px(-4.))
                                .h(px(9.))
                                .left_0()
                                .right_0()
                                .cursor_row_resize()
                        }
                    })
                    .on_drag(
                        HandleDrag {
                            group: drag_group,
                            index,
                        },
                        |_, _, _, cx| cx.new(|_| HandleDragPreview),
                    );

                // Optional grip pill.
                let grip = handle.with_handle.then(|| {
                    div()
                        .flex_shrink_0()
                        .rounded(px(8.))
                        .bg(theme.border)
                        .map(|el| {
                            if horizontal {
                                el.h(px(24.)).w(px(4.))
                            } else {
                                el.w(px(24.)).h(px(4.))
                            }
                        })
                });

                // Separator (focusable keyboard target).
                let state_k = state.clone();
                let constraints_k = constraints.clone();
                let on_change_k = on_layout_change.clone();
                let ring_k = ring.clone();

                let mut separator = div()
                    .id(ElementId::from((group_id.clone(), format!("sep-{index}"))))
                    .flex_shrink_0()
                    .relative()
                    .flex()
                    .items_center()
                    .justify_center()
                    .bg(theme.border)
                    .map(|el| {
                        if horizontal {
                            el.w(px(1.)).h_full()
                        } else {
                            el.h(px(1.)).w_full()
                        }
                    })
                    .tab_index(0)
                    .focus_visible(move |s| s.shadow(ring_k.clone()))
                    .on_key_down({
                        let state = state_k;
                        let constraints = constraints_k;
                        let on_change = on_change_k;
                        move |event, window, cx| {
                            let key = event.keystroke.key.as_str();
                            let sizes_now = state.read(cx).clone();
                            if index + 1 >= sizes_now.len() {
                                return;
                            }

                            let combined = sizes_now[index] + sizes_now[index + 1];
                            let min_a = constraints[index].min_size;
                            let max_a = constraints[index].max_size;
                            let min_b = constraints[index + 1].min_size;
                            let max_b = constraints[index + 1].max_size;
                            let lo = min_a.max(combined - max_b);
                            let hi = max_a.min(combined - min_b);
                            let cur_a = sizes_now[index];

                            let handled = match key {
                                "left" if horizontal => {
                                    write_layout(
                                        &state,
                                        &constraints,
                                        index,
                                        cur_a - KEYBOARD_STEP,
                                        &on_change,
                                        window,
                                        cx,
                                    );
                                    true
                                }
                                "right" if horizontal => {
                                    write_layout(
                                        &state,
                                        &constraints,
                                        index,
                                        cur_a + KEYBOARD_STEP,
                                        &on_change,
                                        window,
                                        cx,
                                    );
                                    true
                                }
                                "up" if !horizontal => {
                                    write_layout(
                                        &state,
                                        &constraints,
                                        index,
                                        cur_a - KEYBOARD_STEP,
                                        &on_change,
                                        window,
                                        cx,
                                    );
                                    true
                                }
                                "down" if !horizontal => {
                                    write_layout(
                                        &state,
                                        &constraints,
                                        index,
                                        cur_a + KEYBOARD_STEP,
                                        &on_change,
                                        window,
                                        cx,
                                    );
                                    true
                                }
                                "home" => {
                                    write_layout(
                                        &state,
                                        &constraints,
                                        index,
                                        lo,
                                        &on_change,
                                        window,
                                        cx,
                                    );
                                    true
                                }
                                "end" => {
                                    write_layout(
                                        &state,
                                        &constraints,
                                        index,
                                        hi,
                                        &on_change,
                                        window,
                                        cx,
                                    );
                                    true
                                }
                                "enter" => {
                                    write_toggle(
                                        &state,
                                        &constraints,
                                        index,
                                        &on_change,
                                        window,
                                        cx,
                                    );
                                    true
                                }
                                _ => false,
                            };

                            if handled {
                                cx.stop_propagation();
                            }
                        }
                    })
                    .child(hit)
                    .children(grip);
                separator.style().refine(&handle.style);

                built.push(separator.into_any_element());
            }
        }

        let drag_state = state.clone();
        let drag_constraints = constraints.clone();
        let drag_on_change = on_layout_change.clone();
        let drag_group_id = group_id.clone();

        let mut root = div()
            .id(self.id)
            .flex()
            .map(|el| {
                if horizontal {
                    el.flex_row()
                } else {
                    el.flex_col()
                }
            })
            .w_full()
            .h_full()
            .on_drag_move(move |event: &DragMoveEvent<HandleDrag>, window, cx| {
                let drag = event.drag(cx);
                if drag.group != drag_group_id {
                    return;
                }
                let index = drag.index;

                let pointer_fraction = if horizontal {
                    let w = f32::from(event.bounds.size.width);
                    if w <= f32::EPSILON {
                        return;
                    }
                    ((f32::from(event.event.position.x - event.bounds.origin.x)) / w)
                        .clamp(0.0, 1.0)
                } else {
                    let h = f32::from(event.bounds.size.height);
                    if h <= f32::EPSILON {
                        return;
                    }
                    ((f32::from(event.event.position.y - event.bounds.origin.y)) / h)
                        .clamp(0.0, 1.0)
                };

                let sizes_now = drag_state.read(cx).clone();
                if index + 1 >= sizes_now.len() {
                    return;
                }
                let prefix: f32 = sizes_now[..index].iter().sum();
                let desired_a = pointer_fraction - prefix;

                write_layout(
                    &drag_state,
                    &drag_constraints,
                    index,
                    desired_a,
                    &drag_on_change,
                    window,
                    cx,
                );
            })
            .children(built);
        root.style().refine(&self.style);
        root
    }
}

// ResizablePanel / ResizableHandle are builder data only; they render via the group.
impl RenderOnce for ResizablePanel {
    fn render(self, _window: &mut Window, _cx: &mut App) -> impl IntoElement {
        // Panels are consumed by ResizablePanelGroup; standalone render is a no-op shell.
        let mut root = div().children(self.children);
        root.style().refine(&self.style);
        root
    }
}

impl RenderOnce for ResizableHandle {
    fn render(self, _window: &mut Window, _cx: &mut App) -> impl IntoElement {
        let mut root = div();
        root.style().refine(&self.style);
        root
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn plain(min: f32, max: f32) -> PanelConstraints {
        PanelConstraints {
            min_size: min,
            max_size: max,
            collapsible: false,
            collapsed_size: 0.0,
        }
    }

    fn collapsible(min: f32, collapsed: f32) -> PanelConstraints {
        PanelConstraints {
            min_size: min,
            max_size: 1.0,
            collapsible: true,
            collapsed_size: collapsed,
        }
    }

    fn panel(default_size: Option<f32>) -> ResizablePanel {
        let mut p = ResizablePanel::new();
        p.default_size = default_size;
        p
    }

    fn assert_close(actual: f32, expected: f32) {
        assert!(
            (actual - expected).abs() < 1e-5,
            "expected {expected}, got {actual}"
        );
    }

    #[test]
    fn initial_sizes() {
        // Explicit defaults pass through.
        let p = [panel(Some(0.25)), panel(Some(0.75))];
        let sizes = compute_initial_sizes(&p.iter().collect::<Vec<_>>());
        assert_close(sizes[0], 0.25);
        assert_close(sizes[1], 0.75);

        // Unset panels split the remainder equally.
        let p = [panel(Some(0.5)), panel(None), panel(None)];
        let sizes = compute_initial_sizes(&p.iter().collect::<Vec<_>>());
        assert_close(sizes[0], 0.5);
        assert_close(sizes[1], 0.25);
        assert_close(sizes[2], 0.25);

        // No defaults at all: equal split.
        let p = [panel(None), panel(None), panel(None)];
        let sizes = compute_initial_sizes(&p.iter().collect::<Vec<_>>());
        for s in sizes {
            assert_close(s, 1.0 / 3.0);
        }

        // Over-committed defaults normalize back to a sum of 1.
        let p = [panel(Some(0.75)), panel(Some(0.75))];
        let sizes = compute_initial_sizes(&p.iter().collect::<Vec<_>>());
        assert_close(sizes[0], 0.5);
        assert_close(sizes[1], 0.5);
    }

    #[test]
    fn pair_resize_moves_and_clamps() {
        let c = [plain(0.1, 1.0), plain(0.1, 1.0)];

        // Free move within bounds; the pair total is preserved.
        let mut sizes = vec![0.5, 0.5];
        apply_pair_resize(&mut sizes, &c, 0, 0.3);
        assert_close(sizes[0], 0.3);
        assert_close(sizes[1], 0.7);

        // Clamped at panel a's min.
        let mut sizes = vec![0.5, 0.5];
        apply_pair_resize(&mut sizes, &c, 0, 0.02);
        assert_close(sizes[0], 0.1);
        assert_close(sizes[1], 0.9);

        // Clamped by panel b's min on the far side.
        let mut sizes = vec![0.5, 0.5];
        apply_pair_resize(&mut sizes, &c, 0, 0.99);
        assert_close(sizes[0], 0.9);
        assert_close(sizes[1], 0.1);

        // Panel b's max bounds the low side.
        let c = [plain(0.1, 1.0), plain(0.1, 0.6)];
        let mut sizes = vec![0.5, 0.5];
        apply_pair_resize(&mut sizes, &c, 0, 0.0);
        assert_close(sizes[0], 0.4);
        assert_close(sizes[1], 0.6);
    }

    #[test]
    fn pair_resize_collapse_snaps() {
        let c = [collapsible(0.2, 0.05), plain(0.1, 1.0)];

        // Above the halfway threshold ((0.05 + 0.2) / 2 = 0.125): clamps to min.
        let mut sizes = vec![0.5, 0.5];
        apply_pair_resize(&mut sizes, &c, 0, 0.15);
        assert_close(sizes[0], 0.2);

        // Below the threshold: snaps to collapsed_size.
        let mut sizes = vec![0.5, 0.5];
        apply_pair_resize(&mut sizes, &c, 0, 0.1);
        assert_close(sizes[0], 0.05);
        assert_close(sizes[1], 0.95);

        // Dragging back out re-expands through the same math.
        apply_pair_resize(&mut sizes, &c, 0, 0.3);
        assert_close(sizes[0], 0.3);

        // Collapse from the far side snaps panel b.
        let c = [plain(0.1, 1.0), collapsible(0.2, 0.0)];
        let mut sizes = vec![0.5, 0.5];
        apply_pair_resize(&mut sizes, &c, 0, 0.95);
        assert_close(sizes[0], 1.0);
        assert_close(sizes[1], 0.0);
    }

    #[test]
    fn toggle_collapse() {
        // Enter collapses panel a, then restores it to its min.
        let c = [collapsible(0.2, 0.0), plain(0.1, 1.0)];
        let mut sizes = vec![0.5, 0.5];
        apply_toggle_collapse(&mut sizes, &c, 0);
        assert_close(sizes[0], 0.0);
        assert_close(sizes[1], 1.0);
        apply_toggle_collapse(&mut sizes, &c, 0);
        assert_close(sizes[0], 0.2);
        assert_close(sizes[1], 0.8);

        // Falls through to panel b when a is not collapsible.
        let c = [plain(0.1, 1.0), collapsible(0.2, 0.0)];
        let mut sizes = vec![0.5, 0.5];
        apply_toggle_collapse(&mut sizes, &c, 0);
        assert_close(sizes[1], 0.0);

        // Neither collapsible: no-op.
        let c = [plain(0.1, 1.0), plain(0.1, 1.0)];
        let mut sizes = vec![0.5, 0.5];
        apply_toggle_collapse(&mut sizes, &c, 0);
        assert_close(sizes[0], 0.5);
    }
}
