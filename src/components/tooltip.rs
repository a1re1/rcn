//! Tooltip — port of shadcn base-nova `ui/tooltip.tsx`.
//!
//! Trigger-anchored overlay with side/align placement, instant open (delay 0),
//! hoverable panel, Escape-to-close, base-nova foreground bubble, arrow, and a
//! 150ms ease enter animation. Built on the same relative +
//! `deferred(anchored())` pattern as [`crate::components::Popover`], not gpui's
//! native cursor-anchored `.tooltip()` machinery.
//!
//! # Divergences / TODOs
//!
//! - **Exit animation**: gpui only animates on mount —
//!   `TODO(rcn): exit animation`.
//! - **Hover-through**: Base UI keeps the popup open via safe-polygon
//!   tracking with `closeDelay: 0`. gpui hover hitboxes are per-element, so
//!   this port approximates it with a 100ms grace close that the panel's own
//!   hover cancels — leaving trigger and panel closes after the grace.
//! - **Focus-open**: plain div wrappers have no focus-within observer in this
//!   gpui rev, and children created via `tab_index` don't expose `FocusHandle`s
//!   to the wrapper — `TODO(rcn): open on trigger keyboard focus`.
//! - **`:has([data-slot=kbd])` padding**: CSS `:has` can't inspect
//!   `AnyElement` children — use the explicit [`.has_kbd()`](Tooltip::has_kbd)
//!   builder (sets `pr` 12→6px) instead.
//! - **RTL direction context**: gpui has none; the storybook mirrors sides
//!   manually. `TODO(rcn): direction context for logical inline-start/end`.
//! - **Base UI `trackCursorAxis` / provider skip-delay grouping**: not ported.
//!   `TODO(rcn): TooltipProvider delay-group / skip-delay`.
//!
//! Text-only content uses [`Tooltip::new`]; arbitrary element content (e.g. a
//! label plus [`crate::components::Kbd`] chips) uses [`Tooltip::rich`]. The
//! [`attach_tooltip`] helper wraps an arbitrary trigger with the same
//! mechanics — [`crate::components::Button::tooltip_rich`] routes through it.

use std::rc::Rc;
use std::time::Duration;

use gpui::{
    Anchor, AnyElement, App, ElementId, Entity, InteractiveElement as _, IntoElement,
    ParentElement, RenderOnce, SharedString, StatefulInteractiveElement as _, Styled, Window,
    anchored, deferred, div, point, prelude::FluentBuilder as _, px, svg,
};

use crate::assets::ICON_TOOLTIP_ARROW;
use crate::motion::{self, OverlaySide};
use crate::theme::Theme;

type RichContent = Rc<dyn Fn(&mut Window, &mut App) -> AnyElement + 'static>;
type OpenChangeHandler = Rc<dyn Fn(&bool, &mut Window, &mut App) + 'static>;

/// Placement side relative to the trigger (shadcn `side`).
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub enum TooltipSide {
    #[default]
    Top,
    Right,
    Bottom,
    Left,
}

impl TooltipSide {
    fn to_overlay(self) -> OverlaySide {
        match self {
            Self::Top => OverlaySide::Top,
            Self::Right => OverlaySide::Right,
            Self::Bottom => OverlaySide::Bottom,
            Self::Left => OverlaySide::Left,
        }
    }
}

/// Alignment along the side axis (shadcn `align`).
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub enum TooltipAlign {
    Start,
    #[default]
    Center,
    End,
}

/// Grace period after the pointer leaves trigger or panel before the tooltip
/// closes — long enough to cross the `side_offset` gap onto the panel
/// (approximating Base UI's safe-polygon hover-through), short enough to read
/// as `closeDelay: 0`.
const HOVER_GRACE: Duration = Duration::from_millis(100);

/// Internal open-state for uncontrolled tooltips (and delay generation).
struct TooltipState {
    open: bool,
    /// Bumped on every hover-enter that schedules a delayed open, and on leave.
    /// A pending timer only commits if the generation still matches.
    generation: u64,
}

impl TooltipState {
    fn new(default_open: bool) -> Self {
        Self {
            open: default_open,
            generation: 0,
        }
    }
}

/// Base-nova tooltip bubble: `w-fit max-w-xs inline-flex items-center gap-1.5
/// rounded-md bg-foreground px-3 py-1.5 text-xs text-background`.
///
/// When `has_kbd` is set, right padding drops 12→6px (shadcn's
/// `has-data-[slot=kbd]:pr-1.5`).
fn tooltip_bubble(theme: Theme, body: AnyElement, has_kbd: bool) -> impl IntoElement {
    // shadcn `w-fit` — gpui has no width:fit-content helper; flex row + max_w
    // already shrink-wraps the bubble to its content.
    div()
        .flex()
        .flex_row()
        .items_center()
        .gap(px(6.))
        .max_w(px(320.))
        .rounded(theme.radius_md())
        .bg(theme.foreground)
        .text_color(theme.background)
        .px(px(12.))
        .when(has_kbd, |el| el.pr(px(6.)))
        .py(px(6.))
        .text_size(px(12.))
        .line_height(px(16.))
        .child(body)
}

/// A hover-tooltip wrapper around a trigger element.
///
/// ```ignore
/// Tooltip::new("tip", "Add to library")
///     .side(TooltipSide::Top)
///     .child(Button::new("b").child("Hover"))
/// ```
#[derive(IntoElement)]
pub struct Tooltip {
    id: ElementId,
    text: Option<SharedString>,
    content: Option<RichContent>,
    children: Vec<AnyElement>,
    side: TooltipSide,
    side_offset: f32,
    align: TooltipAlign,
    align_offset: f32,
    delay: Duration,
    disabled: bool,
    default_open: bool,
    /// Controlled open snapshot. `None` → uncontrolled via keyed state.
    open: Option<bool>,
    on_open_change: Option<OpenChangeHandler>,
    has_kbd: bool,
}

impl Tooltip {
    /// Text-only tooltip content.
    pub fn new(id: impl Into<ElementId>, text: impl Into<SharedString>) -> Self {
        Self {
            id: id.into(),
            text: Some(text.into()),
            content: None,
            children: Vec::new(),
            side: TooltipSide::Top,
            side_offset: 4.0,
            align: TooltipAlign::Center,
            align_offset: 0.0,
            delay: Duration::ZERO,
            disabled: false,
            default_open: false,
            open: None,
            on_open_change: None,
            has_kbd: false,
        }
    }

    /// Arbitrary element content (label + kbd chips, etc.).
    pub fn rich(
        id: impl Into<ElementId>,
        content: impl Fn(&mut Window, &mut App) -> AnyElement + 'static,
    ) -> Self {
        Self {
            id: id.into(),
            text: None,
            content: Some(Rc::new(content)),
            children: Vec::new(),
            side: TooltipSide::Top,
            side_offset: 4.0,
            align: TooltipAlign::Center,
            align_offset: 0.0,
            delay: Duration::ZERO,
            disabled: false,
            default_open: false,
            open: None,
            on_open_change: None,
            has_kbd: false,
        }
    }

    /// Side relative to the trigger. Default [`TooltipSide::Top`].
    pub fn side(mut self, side: TooltipSide) -> Self {
        self.side = side;
        self
    }

    // Intentional API surface (shadcn/Base UI contract); the storybook
    // exercises side/align/disabled.
    #[allow(dead_code)]
    /// Gap between trigger and panel edge in px. Default `4.0`.
    pub fn side_offset(mut self, offset: f32) -> Self {
        self.side_offset = offset;
        self
    }

    /// Alignment along the side axis. Default [`TooltipAlign::Center`].
    pub fn align(mut self, align: TooltipAlign) -> Self {
        self.align = align;
        self
    }

    // Intentional API surface (shadcn/Base UI contract); the storybook
    // exercises side/align/disabled.
    #[allow(dead_code)]
    /// Offset along the align axis in px. Default `0.0`.
    pub fn align_offset(mut self, offset: f32) -> Self {
        self.align_offset = offset;
        self
    }

    // Intentional API surface (shadcn/Base UI contract); the storybook
    // exercises side/align/disabled.
    #[allow(dead_code)]
    /// Open delay. Default `Duration::ZERO` (shadcn `TooltipProvider delay={0}`;
    /// **not** Base UI's own 600ms `OPEN_DELAY`).
    pub fn delay(mut self, delay: Duration) -> Self {
        self.delay = delay;
        self
    }

    /// When true the tooltip never opens.
    pub fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }

    // Intentional API surface (shadcn/Base UI contract); the storybook
    // exercises side/align/disabled.
    #[allow(dead_code)]
    /// Uncontrolled initial open state.
    pub fn default_open(mut self, open: bool) -> Self {
        self.default_open = open;
        self
    }

    // Intentional API surface (shadcn/Base UI contract); the storybook
    // exercises side/align/disabled.
    #[allow(dead_code)]
    /// Controlled open snapshot. Pair with [`Self::on_open_change`].
    pub fn open(mut self, open: bool) -> Self {
        self.open = Some(open);
        self
    }

    // Intentional API surface (shadcn/Base UI contract); the storybook
    // exercises side/align/disabled.
    #[allow(dead_code)]
    /// Fires whenever the tooltip wants to open or close.
    pub fn on_open_change(
        mut self,
        handler: impl Fn(&bool, &mut Window, &mut App) + 'static,
    ) -> Self {
        self.on_open_change = Some(Rc::new(handler));
        self
    }

    /// Content includes a Kbd chip — drops right padding 12→6px (explicit
    /// stand-in for shadcn's `:has([data-slot=kbd])` selector).
    pub fn has_kbd(mut self) -> Self {
        self.has_kbd = true;
        self
    }
}

impl ParentElement for Tooltip {
    fn extend(&mut self, elements: impl IntoIterator<Item = AnyElement>) {
        self.children.extend(elements);
    }
}

impl RenderOnce for Tooltip {
    fn render(self, window: &mut Window, cx: &mut App) -> impl IntoElement {
        let theme = Theme::of(cx).clone();
        let side = self.side;
        let side_offset = self.side_offset;
        let align = self.align;
        let align_offset = self.align_offset;
        let delay = self.delay;
        let disabled = self.disabled;
        let has_kbd = self.has_kbd;
        let text = self.text.clone();
        let content = self.content.clone();
        let on_open_change = self.on_open_change.clone();

        // Resolve open: controlled snapshot, or keyed uncontrolled state.
        let default_open = self.default_open;
        let state_key: ElementId = (self.id.clone(), "tooltip-state").into();
        let state: Entity<TooltipState> =
            window.use_keyed_state(state_key, cx, move |_, _| TooltipState::new(default_open));

        let is_controlled = self.open.is_some();
        let open = if disabled {
            false
        } else if let Some(v) = self.open {
            v
        } else {
            state.read(cx).open
        };

        // Shared open/close applicator for hover + escape + delay timer.
        let set_open = {
            let state = state.clone();
            let on_open_change = on_open_change.clone();
            Rc::new(move |next: bool, window: &mut Window, cx: &mut App| {
                if !is_controlled {
                    state.update(cx, |s, cx| {
                        if s.open != next {
                            s.open = next;
                            cx.notify();
                        }
                    });
                }
                if let Some(cb) = on_open_change.as_ref() {
                    cb(&next, window, cx);
                }
            }) as Rc<dyn Fn(bool, &mut Window, &mut App) + 'static>
        };

        // Bump the generation, cancelling any pending delayed open or grace
        // close, and return the new value.
        let bump = {
            let state = state.clone();
            Rc::new(move |cx: &mut App| {
                state.update(cx, |s, _| {
                    s.generation = s.generation.saturating_add(1);
                    s.generation
                })
            }) as Rc<dyn Fn(&mut App) -> u64>
        };

        // Close after [`HOVER_GRACE`] unless the generation moves on first
        // (the pointer re-entered the trigger or landed on the panel).
        let grace_close = {
            let state = state.clone();
            let set_open = set_open.clone();
            let bump = bump.clone();
            Rc::new(move |window: &mut Window, cx: &mut App| {
                let scheduled_gen = bump(cx);
                let state = state.clone();
                let set_open = set_open.clone();
                window
                    .spawn(cx, async move |cx| {
                        cx.background_executor().timer(HOVER_GRACE).await;
                        cx.update(|window, cx| {
                            if state.read(cx).generation == scheduled_gen {
                                set_open(false, window, cx);
                            }
                        })
                        .ok();
                    })
                    .detach();
            }) as Rc<dyn Fn(&mut Window, &mut App)>
        };

        let trigger_id = self.id.clone();
        let panel_id: ElementId = (self.id.clone(), "tooltip-panel").into();

        // Hover opens (instantly by default); leaving trigger or panel closes
        // after the shared grace so the pointer can hover through onto the
        // panel (Base UI hoverable-popup default).
        div()
            .id(trigger_id)
            .relative()
            .flex()
            .flex_col()
            .items_start()
            .when(!disabled, {
                let set_open_h = set_open.clone();
                let state_h = state.clone();
                let set_open_k = set_open.clone();
                let bump_h = bump.clone();
                let grace_close_h = grace_close.clone();
                move |el| {
                    el.on_hover(move |hovered, window, cx| {
                        if *hovered {
                            if delay.is_zero() {
                                bump_h(cx);
                                set_open_h(true, window, cx);
                            } else {
                                // Schedule open after `delay` only if still
                                // hovered (generation still matches when the
                                // timer fires).
                                let scheduled_gen = bump_h(cx);
                                let state = state_h.clone();
                                let set_open = set_open_h.clone();
                                let delay = delay;
                                // window.spawn gives an AsyncWindowContext so
                                // the timer can call on_open_change with a real
                                // Window (App-only spawn cannot).
                                window
                                    .spawn(cx, async move |cx| {
                                        cx.background_executor().timer(delay).await;
                                        cx.update(|window, cx| {
                                            let still = state.read(cx).generation == scheduled_gen;
                                            if still {
                                                set_open(true, window, cx);
                                            }
                                        })
                                        .ok();
                                    })
                                    .detach();
                            }
                        } else {
                            grace_close_h(window, cx);
                        }
                    })
                    // Escape closes while a focused child (e.g. Button) holds
                    // keyboard focus — gpui dispatches keys up the ancestor
                    // chain. Only handled while open so a closed tooltip's
                    // trigger doesn't swallow Escape from enclosing overlays.
                    .on_key_down(
                        move |event: &gpui::KeyDownEvent, window, cx| {
                            if open && event.keystroke.key == "escape" {
                                set_open_k(false, window, cx);
                                cx.stop_propagation();
                            }
                        },
                    )
                }
            })
            .children(self.children)
            .when(open, |el| {
                let body = build_body(window, cx, text, content);
                let panel_hover = {
                    let bump = bump.clone();
                    let grace_close = grace_close.clone();
                    Rc::new(move |hovered: bool, window: &mut Window, cx: &mut App| {
                        if hovered {
                            bump(cx);
                        } else {
                            grace_close(window, cx);
                        }
                    }) as Rc<dyn Fn(bool, &mut Window, &mut App)>
                };
                el.child(positioned_panel(
                    &theme,
                    side,
                    side_offset,
                    align,
                    align_offset,
                    body,
                    has_kbd,
                    panel_id,
                    panel_hover,
                ))
            })
    }
}

fn build_body(
    window: &mut Window,
    cx: &mut App,
    text: Option<SharedString>,
    content: Option<RichContent>,
) -> AnyElement {
    if let Some(content) = content {
        content(window, cx)
    } else {
        text.unwrap_or_default().into_any_element()
    }
}

/// Build the absolute-positioned, side/align-aware panel with arrow + enter motion.
#[allow(clippy::too_many_arguments)]
fn positioned_panel(
    theme: &Theme,
    side: TooltipSide,
    side_offset: f32,
    align: TooltipAlign,
    align_offset: f32,
    body: AnyElement,
    has_kbd: bool,
    panel_id: ElementId,
    on_hover: Rc<dyn Fn(bool, &mut Window, &mut App)>,
) -> AnyElement {
    let bubble = tooltip_bubble(theme.clone(), body, has_kbd);
    let arrow = arrow_el(theme, side);

    // Stack bubble + arrow. Arrow is absolutely positioned on the bubble's
    // trigger-facing edge (centered on the bubble — accepted simplification).
    // Hovering the stack keeps the tooltip open (hoverable-popup default);
    // crossing the side_offset gap is covered by the grace close.
    let stack = div()
        .id(panel_id)
        .relative()
        .on_hover(move |hovered, window, cx| on_hover(*hovered, window, cx))
        .child(bubble)
        .child(arrow);

    let animated = motion::tooltip_in("tooltip-in", side.to_overlay(), stack);

    // gpui's `anchored` is itself position:absolute — it contributes zero
    // size to its parent and paints its child from the parent's laid-out
    // origin toward the given `Anchor` corner. So placement works as: pin a
    // zero-size wrapper point on the trigger's side/align anchor location,
    // then have `anchored` put the panel's matching corner (or edge-center)
    // at that point. (Laying the panel out in-flow and aligning with
    // flex/justify does NOT work — a zero-size absolute child can't be
    // justified, and the default TopLeft anchor paints down/right, covering
    // the trigger for side Top/Left.)
    let anchor = match (side, align) {
        (TooltipSide::Top, TooltipAlign::Start) => Anchor::BottomLeft,
        (TooltipSide::Top, TooltipAlign::Center) => Anchor::BottomCenter,
        (TooltipSide::Top, TooltipAlign::End) => Anchor::BottomRight,
        (TooltipSide::Bottom, TooltipAlign::Start) => Anchor::TopLeft,
        (TooltipSide::Bottom, TooltipAlign::Center) => Anchor::TopCenter,
        (TooltipSide::Bottom, TooltipAlign::End) => Anchor::TopRight,
        (TooltipSide::Left, TooltipAlign::Start) => Anchor::TopRight,
        (TooltipSide::Left, TooltipAlign::Center) => Anchor::RightCenter,
        (TooltipSide::Left, TooltipAlign::End) => Anchor::BottomRight,
        (TooltipSide::Right, TooltipAlign::Start) => Anchor::TopLeft,
        (TooltipSide::Right, TooltipAlign::Center) => Anchor::LeftCenter,
        (TooltipSide::Right, TooltipAlign::End) => Anchor::BottomLeft,
    };
    // side_offset pushes the panel away from the trigger along the side
    // axis; align_offset slides it along the align axis (positive toward
    // the end, matching floating-ui's alignOffset).
    let offset = match side {
        TooltipSide::Top => point(px(align_offset), px(-side_offset)),
        TooltipSide::Bottom => point(px(align_offset), px(side_offset)),
        TooltipSide::Left => point(px(-side_offset), px(align_offset)),
        TooltipSide::Right => point(px(side_offset), px(align_offset)),
    };

    let wrapper = div().absolute().child(
        // deferred + anchored keeps the panel above siblings and snaps inside
        // the window if it would overflow (same pattern as Popover).
        deferred(
            anchored()
                .anchor(anchor)
                .offset(offset)
                .snap_to_window_with_margin(px(8.))
                .child(animated),
        ),
    );

    // Pin the zero-size wrapper's origin to the anchor point on the trigger:
    // the trigger-facing edge (side) at the aligned position (align).
    let wrapper = match side {
        TooltipSide::Top => wrapper.bottom(gpui::relative(1.)),
        TooltipSide::Bottom => wrapper.top(gpui::relative(1.)),
        TooltipSide::Left => wrapper.right(gpui::relative(1.)),
        TooltipSide::Right => wrapper.left(gpui::relative(1.)),
    };
    let wrapper = match (side, align) {
        (TooltipSide::Top | TooltipSide::Bottom, TooltipAlign::Start) => wrapper.left_0(),
        (TooltipSide::Top | TooltipSide::Bottom, TooltipAlign::Center) => {
            wrapper.left(gpui::relative(0.5))
        }
        (TooltipSide::Top | TooltipSide::Bottom, TooltipAlign::End) => wrapper.right_0(),
        (TooltipSide::Left | TooltipSide::Right, TooltipAlign::Start) => wrapper.top_0(),
        (TooltipSide::Left | TooltipSide::Right, TooltipAlign::Center) => {
            wrapper.top(gpui::relative(0.5))
        }
        (TooltipSide::Left | TooltipSide::Right, TooltipAlign::End) => wrapper.bottom_0(),
    };

    wrapper.into_any_element()
}

/// 10×10 pre-rotated diamond arrow (`icons/tooltip-arrow.svg`), centered on
/// the bubble's trigger-facing edge. Centering on the bubble rather than the
/// anchor is an accepted simplification — the arrow tip pokes ~5px into the
/// side_offset gap (half the 10px diamond).
fn arrow_el(theme: &Theme, side: TooltipSide) -> impl IntoElement {
    // Arrow size = 10px; half = 5px used to center on the edge.
    let arrow = svg()
        .path(ICON_TOOLTIP_ARROW)
        .size(px(10.))
        .text_color(theme.foreground);

    let positioned = div().absolute().child(arrow);

    match side {
        // Panel above trigger → arrow on the bubble's bottom edge.
        TooltipSide::Top => positioned
            .left(gpui::relative(0.5))
            .ml(px(-5.))
            .bottom(px(-5.)),
        // Panel below trigger → arrow on the bubble's top edge.
        TooltipSide::Bottom => positioned
            .left(gpui::relative(0.5))
            .ml(px(-5.))
            .top(px(-5.)),
        // Panel left of trigger → arrow on the bubble's right edge.
        TooltipSide::Left => positioned
            .top(gpui::relative(0.5))
            .mt(px(-5.))
            .right(px(-5.)),
        // Panel right of trigger → arrow on the bubble's left edge.
        TooltipSide::Right => positioned
            .top(gpui::relative(0.5))
            .mt(px(-5.))
            .left(px(-5.)),
    }
}

/// Attach an anchored tooltip to an arbitrary styled element (used by
/// [`crate::components::Button::tooltip_rich`]). Defaults: side Top, align
/// Center, instant open, hoverable panel.
///
/// Returns a relative wrapper around `trigger` that owns hover/Escape state
/// and paints the bubble — the trigger's own focus/click/hover is unchanged.
pub fn attach_tooltip<E: IntoElement>(
    id: impl Into<ElementId>,
    trigger: E,
    content: impl Fn(&mut Window, &mut App) -> AnyElement + 'static,
) -> impl IntoElement {
    Tooltip::rich(id, content).child(trigger)
}
