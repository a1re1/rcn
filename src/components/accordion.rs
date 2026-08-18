//! Accordion — port of shadcn base-nova `ui/accordion.tsx`.
//!
//! Root owns the open set (Base UI contract): uncontrolled via
//! `default_value` + keyed state, or controlled via `value` +
//! `on_value_change`. Items decompose into Trigger / Content parts; the
//! panel animates height on both expand and collapse with `motion::expand()`
//! (200ms ease-out, matching tw-animate-css `accordion-down`/`accordion-up`).

use std::rc::Rc;

use gpui::{
    AnimationExt as _, App, Bounds, ClickEvent, ElementId, Entity, FontWeight,
    InteractiveElement as _, IntoElement, ParentElement, Pixels, Refineable as _, RenderOnce,
    StatefulInteractiveElement as _, StyleRefinement, Styled, Window, div,
    prelude::FluentBuilder as _, px, svg,
};

use crate::motion;
use crate::theme::Theme;

type ToggleHandler = Box<dyn Fn(&ClickEvent, &mut Window, &mut App) + 'static>;
type ValueChangeHandler = Box<dyn Fn(&[ElementId], &mut Window, &mut App) + 'static>;

/// Per-item panel animation state (keyed on the item's ElementId).
struct PanelState {
    measured: Option<Pixels>,
    /// Last open flag observed during render.
    open: bool,
    /// Bumped on every open-flag transition so `with_animation`'s element id
    /// changes and the 200ms clock remounts (enables close animation).
    generation: u64,
    /// True while the expand/collapse animation is in flight.
    animating: bool,
}

impl PanelState {
    fn new(open: bool) -> Self {
        Self {
            measured: None,
            open,
            generation: 0,
            animating: false,
        }
    }
}

/// Compute the next open set after toggling `id`.
fn toggle_value(open: &[ElementId], id: &ElementId, multiple: bool) -> Vec<ElementId> {
    let is_open = open.iter().any(|x| x == id);
    if multiple {
        if is_open {
            open.iter().filter(|x| *x != id).cloned().collect()
        } else {
            let mut next = open.to_vec();
            next.push(id.clone());
            next
        }
    } else if is_open {
        Vec::new()
    } else {
        vec![id.clone()]
    }
}

// ── Accordion (root) ────────────────────────────────────────────────────────

/// flex w-full flex-col — container for [`AccordionItem`]s.
///
/// Mirrors Base UI `Accordion.Root`: `multiple`, `disabled`, `defaultValue`,
/// `value`, `onValueChange`. Sizing and shape overrides come from the caller
/// via [`Styled`].
#[derive(IntoElement)]
pub struct Accordion {
    id: ElementId,
    items: Vec<AccordionItem>,
    multiple: bool,
    disabled: bool,
    bordered: bool,
    default_value: Vec<ElementId>,
    /// `Some` ⇒ controlled mode; render from this set and report toggles.
    value: Option<Vec<ElementId>>,
    on_value_change: Option<ValueChangeHandler>,
    style: StyleRefinement,
}

impl Accordion {
    pub fn new(id: impl Into<ElementId>) -> Self {
        Self {
            id: id.into(),
            items: Vec::new(),
            multiple: false,
            disabled: false,
            bordered: false,
            default_value: Vec::new(),
            value: None,
            on_value_change: None,
            style: StyleRefinement::default(),
        }
    }

    /// When true, more than one item may be open (Base UI `multiple`).
    /// Default **false** — single-open, matching the shadcn docs.
    pub fn multiple(mut self, multiple: bool) -> Self {
        self.multiple = multiple;
        self
    }

    /// Disables every item (half opacity, no pointer events, unfocusable).
    pub fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }

    /// Outer border + rounded shell with horizontal item padding
    /// (`border`, `rounded-lg`, item `px-4`).
    pub fn bordered(mut self, bordered: bool) -> Self {
        self.bordered = bordered;
        self
    }

    /// Uncontrolled initial open set (`defaultValue`).
    pub fn default_value(mut self, ids: impl IntoIterator<Item = impl Into<ElementId>>) -> Self {
        self.default_value = ids.into_iter().map(Into::into).collect();
        self
    }

    /// Controlled open set (`value`). When set, the root does not store state.
    pub fn value(mut self, ids: impl IntoIterator<Item = impl Into<ElementId>>) -> Self {
        self.value = Some(ids.into_iter().map(Into::into).collect());
        self
    }

    /// Controlled-mode toggle reporter (`onValueChange`).
    pub fn on_value_change(
        mut self,
        handler: impl Fn(&[ElementId], &mut Window, &mut App) + 'static,
    ) -> Self {
        self.on_value_change = Some(Box::new(handler));
        self
    }

    /// Append a typed item (inherent — not [`ParentElement`]).
    pub fn child(mut self, item: AccordionItem) -> Self {
        self.items.push(item);
        self
    }

    /// Append typed items (inherent — not [`ParentElement`]).
    pub fn children(mut self, items: impl IntoIterator<Item = AccordionItem>) -> Self {
        self.items.extend(items);
        self
    }
}

impl Styled for Accordion {
    fn style(&mut self) -> &mut StyleRefinement {
        &mut self.style
    }
}

impl RenderOnce for Accordion {
    fn render(self, window: &mut Window, cx: &mut App) -> impl IntoElement {
        let theme = Theme::of(cx).clone();
        let multiple = self.multiple;
        let root_disabled = self.disabled;
        let bordered = self.bordered;
        let item_count = self.items.len();

        // Resolve the open set: controlled snapshot, or keyed uncontrolled state.
        let on_value_change = self.on_value_change.map(Rc::new);
        let (open_set, uncontrolled_state): (Vec<ElementId>, Option<Entity<Vec<ElementId>>>) =
            if let Some(value) = self.value {
                (value, None)
            } else {
                let default_value = self.default_value;
                let state_key: ElementId = (self.id.clone(), "open-set").into();
                let state = window.use_keyed_state(state_key, cx, move |_, _| default_value);
                (state.read(cx).clone(), Some(state))
            };

        let items = self.items.into_iter().enumerate().map(|(index, mut item)| {
            let is_last = index + 1 == item_count;
            item.last = is_last;
            item.bordered = bordered;
            item.root_disabled = root_disabled;

            // Root wires open + toggle into each item (the gpui analog of
            // Base UI's AccordionRootContext).
            item.open = open_set.iter().any(|id| id == &item.id);

            let id = item.id.clone();
            if let Some(state) = uncontrolled_state.clone() {
                item.on_toggle = Some(Box::new(move |_, _window, cx| {
                    state.update(cx, |open, cx| {
                        *open = toggle_value(open, &id, multiple);
                        cx.notify();
                    });
                }));
            } else if let Some(cb) = on_value_change.clone() {
                let current = open_set.clone();
                item.on_toggle = Some(Box::new(move |_, window, cx| {
                    let next = toggle_value(&current, &id, multiple);
                    cb(&next, window, cx);
                }));
            }

            item
        });

        let mut root = div()
            .flex()
            .flex_col()
            .w_full()
            // bordered: border + rounded-lg shell (items keep between-borders)
            .when(bordered, |el| {
                el.border_1()
                    .border_color(theme.border)
                    .rounded(theme.radius_lg())
            })
            .children(items);
        root.style().refine(&self.style);
        root
    }
}

// ── AccordionItem ───────────────────────────────────────────────────────────

/// Typed child of [`AccordionItem`] — trigger or content part.
pub enum AccordionPart {
    Trigger(AccordionTrigger),
    Content(AccordionContent),
}

impl From<AccordionTrigger> for AccordionPart {
    fn from(value: AccordionTrigger) -> Self {
        Self::Trigger(value)
    }
}

impl From<AccordionContent> for AccordionPart {
    fn from(value: AccordionContent) -> Self {
        Self::Content(value)
    }
}

/// border-b (unless last); hosts the trigger row and animated panel.
/// Sizing and shape overrides come from the caller via [`Styled`].
#[derive(IntoElement)]
pub struct AccordionItem {
    id: ElementId,
    trigger: Option<AccordionTrigger>,
    content: Option<AccordionContent>,
    /// Set by the root from its open set (Base UI items have no `open` prop).
    open: bool,
    /// Set by the root (auto last-item detection).
    last: bool,
    disabled: bool,
    /// Root-level disabled, OR'd with [`Self::disabled`] at render.
    root_disabled: bool,
    /// Horizontal padding when the root is `.bordered(true)`.
    bordered: bool,
    on_toggle: Option<ToggleHandler>,
    style: StyleRefinement,
}

impl AccordionItem {
    pub fn new(id: impl Into<ElementId>) -> Self {
        Self {
            id: id.into(),
            trigger: None,
            content: None,
            open: false,
            last: false,
            disabled: false,
            root_disabled: false,
            bordered: false,
            on_toggle: None,
            style: StyleRefinement::default(),
        }
    }

    /// Append a typed trigger or content part.
    pub fn child(mut self, part: impl Into<AccordionPart>) -> Self {
        match part.into() {
            AccordionPart::Trigger(t) => self.trigger = Some(t),
            AccordionPart::Content(c) => self.content = Some(c),
        }
        self
    }

    /// Sugar: wrap `trigger` into an [`AccordionTrigger`].
    pub fn trigger(self, trigger: impl IntoElement) -> Self {
        self.child(AccordionTrigger::new().child(trigger))
    }

    /// Sugar: wrap `content` into an [`AccordionContent`].
    pub fn content(self, content: impl IntoElement) -> Self {
        self.child(AccordionContent::new().child(content))
    }

    /// aria-disabled: half opacity, no pointer events, unfocusable.
    pub fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }
}

impl Styled for AccordionItem {
    fn style(&mut self) -> &mut StyleRefinement {
        &mut self.style
    }
}

impl RenderOnce for AccordionItem {
    fn render(self, window: &mut Window, cx: &mut App) -> impl IntoElement {
        let theme = Theme::of(cx).clone();
        let disabled = self.disabled || self.root_disabled;
        let open = self.open;
        let id = self.id.clone();

        // ── panel state (height animation) ──────────────────────────────────
        let panel_key: ElementId = (id.clone(), "panel").into();
        let panel = window.use_keyed_state(panel_key, cx, move |_, _| PanelState::new(open));

        // Detect open-flag transitions → bump generation, start animation.
        let state_open = panel.read(cx).open;

        if state_open != open {
            let panel_entity = panel.clone();
            panel.update(cx, |state, cx| {
                state.open = open;
                state.generation = state.generation.saturating_add(1);
                state.animating = true;
                cx.notify();
            });
            let settled_generation = panel.read(cx).generation;
            // After 200ms, settle: drop animating so open→auto height and
            // closed→unmount (no stray hit-testing).
            cx.spawn(async move |cx| {
                cx.background_executor()
                    .timer(motion::EXPAND_DURATION)
                    .await;
                cx.update(|cx| {
                    panel_entity.update(cx, |state, cx| {
                        if state.generation == settled_generation && state.animating {
                            state.animating = false;
                            cx.notify();
                        }
                    });
                });
            })
            .detach();
        }

        let (generation, animating, measured) = {
            let snap = panel.read(cx);
            (snap.generation, snap.animating, snap.measured)
        };

        // Mount content while open or mid-close animation.
        let show_panel = open || animating;

        let chevron = if open {
            theme.icons.chevron_up()
        } else {
            theme.icons.chevron_down()
        };

        let h_pad = if self.bordered { Some(px(16.)) } else { None };

        let mut root = div()
            .w_full()
            .when(!self.last, |el| el.border_b_1().border_color(theme.border))
            // Trigger: flex flex-1 items-start justify-between gap-4 rounded-md
            // py-4 text-left text-sm font-medium hover:underline
            .child(
                div()
                    .id(id.clone())
                    .flex()
                    .flex_row()
                    .w_full()
                    .items_center()
                    .justify_between()
                    .gap(px(16.))
                    .py(px(16.))
                    .when_some(h_pad, |el, p| el.px(p))
                    .text_size(px(14.))
                    .line_height(px(20.))
                    .font_weight(FontWeight::MEDIUM)
                    .text_color(theme.foreground)
                    .rounded(theme.radius_md())
                    // transparent border base so focus rings don't shift layout
                    .border_1()
                    .border_color(gpui::transparent_black())
                    .when(disabled, |el| el.opacity(0.5))
                    .when(!disabled, |el| {
                        let ring = motion::focus_ring(&theme);
                        el.hover(|s| s.underline())
                            .tab_index(0)
                            .focus_visible(move |s| s.border_color(theme.ring).shadow(ring.clone()))
                            .when_some(self.on_toggle, |el, on_toggle| el.on_click(on_toggle))
                    })
                    .children(self.trigger)
                    // **:data-[slot=accordion-trigger-icon]:size-4 text-muted-foreground
                    .child(
                        svg()
                            .path(chevron)
                            .size(px(16.))
                            .flex_shrink_0()
                            .text_color(theme.muted_foreground),
                    ),
            )
            .when(show_panel, |el| {
                let panel_entity = panel.clone();
                let anim_id: ElementId = (id.clone(), format!("anim-{}", generation)).into();
                let measured_for_anim = measured;
                let opening = open;

                // Inner content carries pb-4 so measured height includes padding.
                let inner = div()
                    .when_some(h_pad, |el, p| el.px(p))
                    .child(self.content.unwrap_or_else(AccordionContent::new))
                    .on_children_prepainted(move |bounds: Vec<Bounds<Pixels>>, _window, cx| {
                        let height = bounds
                            .iter()
                            .map(|b| b.size.height)
                            .fold(px(0.), |acc, h| acc + h);
                        panel_entity.update(cx, |state, cx| {
                            if state.measured != Some(height) {
                                state.measured = Some(height);
                                cx.notify();
                            }
                        });
                    });

                let outer = div().overflow_hidden().child(inner);

                // Steady open: auto height (content reflows naturally).
                // Animating / first frame: fixed h interpolated by delta.
                if !animating && open {
                    el.child(outer)
                } else {
                    el.child(
                        outer.with_animation(anim_id, motion::expand(), move |el, delta| {
                            let h = match measured_for_anim {
                                Some(m) if opening => m * delta,
                                Some(m) => m * (1. - delta),
                                None => px(0.),
                            };
                            el.h(h)
                        }),
                    )
                }
            });
        root.style().refine(&self.style);
        root
    }
}

// ── AccordionTrigger ────────────────────────────────────────────────────────

/// Trigger label/slot — flex-1 so the chevron stays end-aligned.
///
/// Children are arbitrary (`ParentElement`). Sizing and shape overrides come
/// from the caller via [`Styled`].
#[derive(IntoElement)]
pub struct AccordionTrigger {
    children: Vec<gpui::AnyElement>,
    style: StyleRefinement,
}

impl AccordionTrigger {
    pub fn new() -> Self {
        Self {
            children: Vec::new(),
            style: StyleRefinement::default(),
        }
    }
}

impl Default for AccordionTrigger {
    fn default() -> Self {
        Self::new()
    }
}

impl ParentElement for AccordionTrigger {
    fn extend(&mut self, elements: impl IntoIterator<Item = gpui::AnyElement>) {
        self.children.extend(elements);
    }
}

impl Styled for AccordionTrigger {
    fn style(&mut self) -> &mut StyleRefinement {
        &mut self.style
    }
}

impl RenderOnce for AccordionTrigger {
    fn render(self, _window: &mut Window, _cx: &mut App) -> impl IntoElement {
        // flex-1 min-w-0 — shares the trigger row with the chevron.
        let mut root = div().flex().flex_1().min_w_0().children(self.children);
        root.style().refine(&self.style);
        root
    }
}

// ── AccordionContent ────────────────────────────────────────────────────────

/// text-sm pb-4 — padding lives here so height measurement includes it.
///
/// Children are arbitrary (`ParentElement`). Sizing and shape overrides come
/// from the caller via [`Styled`].
#[derive(IntoElement)]
pub struct AccordionContent {
    children: Vec<gpui::AnyElement>,
    style: StyleRefinement,
}

impl AccordionContent {
    pub fn new() -> Self {
        Self {
            children: Vec::new(),
            style: StyleRefinement::default(),
        }
    }
}

impl Default for AccordionContent {
    fn default() -> Self {
        Self::new()
    }
}

impl ParentElement for AccordionContent {
    fn extend(&mut self, elements: impl IntoIterator<Item = gpui::AnyElement>) {
        self.children.extend(elements);
    }
}

impl Styled for AccordionContent {
    fn style(&mut self) -> &mut StyleRefinement {
        &mut self.style
    }
}

impl RenderOnce for AccordionContent {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let theme = Theme::of(cx);
        // pb-4 text-sm — padding inside the measured box (shadcn inner div).
        let mut root = div()
            .pb(px(16.))
            .text_size(px(14.))
            .line_height(px(20.))
            .text_color(theme.foreground)
            .children(self.children);
        root.style().refine(&self.style);
        root
    }
}
