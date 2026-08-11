//! Switch — port of shadcn base-nova `ui/switch.tsx`.
//!
//! Controlled via `checked` + `on_checked_change`, or uncontrolled via
//! `default_checked` + keyed state (Base UI contract). Supports `disabled`,
//! `read_only`, and `invalid` (aria-invalid destructive border + ring).
//! Sizes: Sm (24×14, 12px thumb) and Default (32×18.4, 16px thumb).
//! Thumb slides 150ms on toggle (`transition-transform` + ease-transition);
//! track/thumb color cross-fade during the slide is omitted (snap to target).
//! Extended hit area mirrors `after:-inset-x-3 after:-inset-y-2`.
//!
//! Omitted (no gpui form-submission equivalent): `name`, `required`,
//! `uncheckedValue`, `inputRef`, `id` as a form field id. RTL is out of scope
//! repo-wide.

use std::rc::Rc;

use gpui::{
    AnimationExt as _, App, ClickEvent, ElementId, Entity, InteractiveElement as _, IntoElement,
    ParentElement, RenderOnce, StatefulInteractiveElement as _, Styled, Window, div,
    prelude::FluentBuilder as _, px,
};

use crate::motion;
use crate::theme::{Theme, alpha};

#[derive(Clone, Copy, PartialEq, Eq, Default, Debug)]
pub enum SwitchSize {
    Sm,
    #[default]
    Default,
}

impl SwitchSize {
    /// Track (width, height) and thumb diameter.
    fn track(self) -> (f32, f32) {
        match self {
            Self::Sm => (24., 14.),
            // Source: data-[size=default]:h-[18.4px] data-[size=default]:w-[32px]
            Self::Default => (32., 18.4),
        }
    }

    fn thumb(self) -> f32 {
        match self {
            Self::Sm => 12.,
            Self::Default => 16.,
        }
    }
}

type ChangeHandler = Box<dyn Fn(&bool, &mut Window, &mut App) + 'static>;

/// Keyed per-switch state driving the 150ms thumb slide: animate only on an
/// actual toggle, never on first mount (accordion `PanelState` pattern).
struct SlideState {
    checked: bool,
    generation: usize,
    animating: bool,
}

#[derive(IntoElement)]
pub struct Switch {
    id: ElementId,
    /// `Some` = controlled; `None` = never set → resolve via keyed uncontrolled state.
    checked: Option<bool>,
    default_checked: bool,
    size: SwitchSize,
    disabled: bool,
    read_only: bool,
    invalid: bool,
    on_checked_change: Option<ChangeHandler>,
}

impl Switch {
    pub fn new(id: impl Into<ElementId>) -> Self {
        Self {
            id: id.into(),
            checked: None,
            default_checked: false,
            size: SwitchSize::default(),
            disabled: false,
            read_only: false,
            invalid: false,
            on_checked_change: None,
        }
    }

    /// Controlled checked override. When set, wins over keyed uncontrolled state.
    pub fn checked(mut self, checked: bool) -> Self {
        self.checked = Some(checked);
        self
    }

    /// Initial checked value for uncontrolled use (`defaultChecked`, default false).
    // Intentional API surface; the storybook's uncontrolled demos start unchecked
    // and its choice cards are controlled (the card highlight needs the state).
    #[allow(dead_code)]
    pub fn default_checked(mut self, default_checked: bool) -> Self {
        self.default_checked = default_checked;
        self
    }

    pub fn size(mut self, size: SwitchSize) -> Self {
        self.size = size;
        self
    }

    pub fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }

    /// Focusable with the focus ring, but activation (click/Enter/Space) is a no-op.
    /// Distinct from [`Self::disabled`] (unfocusable, 50% opacity, no pointer events).
    pub fn read_only(mut self, read_only: bool) -> Self {
        self.read_only = read_only;
        self
    }

    /// `aria-invalid`: destructive border + always-visible destructive ring.
    pub fn invalid(mut self, invalid: bool) -> Self {
        self.invalid = invalid;
        self
    }

    /// Base UI `onCheckedChange` — called with the next checked value.
    pub fn on_checked_change(
        mut self,
        handler: impl Fn(&bool, &mut Window, &mut App) + 'static,
    ) -> Self {
        self.on_checked_change = Some(Box::new(handler));
        self
    }
}

impl RenderOnce for Switch {
    fn render(self, window: &mut Window, cx: &mut App) -> impl IntoElement {
        let theme = Theme::of(cx).clone();
        let (track_w, track_h) = self.size.track();
        let thumb = self.size.thumb();
        let disabled = self.disabled;
        let read_only = self.read_only;
        let invalid = self.invalid;
        let on_checked_change = self.on_checked_change;
        let root_id = self.id.clone();

        // Resolve checked: controlled snapshot, or keyed uncontrolled state.
        let (checked, uncontrolled_state): (bool, Option<Entity<bool>>) =
            if let Some(value) = self.checked {
                (value, None)
            } else {
                let default_checked = self.default_checked;
                let state_key: ElementId = (self.id.clone(), "checked").into();
                let state = window.use_keyed_state(state_key, cx, move |_, _| default_checked);
                (*state.read(cx), Some(state))
            };

        // Detect checked transitions → bump generation, start the slide, and
        // settle after the transition clock so re-renders mid-flight (each
        // animation frame is one) keep the animation mounted.
        let slide_key: ElementId = (root_id.clone(), "slide").into();
        let slide = window.use_keyed_state(slide_key, cx, move |_, _| SlideState {
            checked,
            generation: 0,
            animating: false,
        });
        if slide.read(cx).checked != checked {
            let slide_entity = slide.clone();
            slide.update(cx, |state, cx| {
                state.checked = checked;
                state.generation = state.generation.saturating_add(1);
                state.animating = true;
                cx.notify();
            });
            let settled_generation = slide.read(cx).generation;
            cx.spawn(async move |cx| {
                cx.background_executor()
                    .timer(motion::TRANSITION_DURATION)
                    .await;
                cx.update(|cx| {
                    slide_entity.update(cx, |state, cx| {
                        if state.generation == settled_generation && state.animating {
                            state.animating = false;
                            cx.notify();
                        }
                    });
                });
            })
            .detach();
        }
        let (generation, animating) = {
            let snap = slide.read(cx);
            (snap.generation, snap.animating)
        };

        // data-checked:bg-primary data-unchecked:bg-input
        // (dark unchecked: bg-input/80)
        let track_bg = if checked {
            theme.primary
        } else if theme.dark {
            alpha(theme.input, 0.8)
        } else {
            theme.input
        };

        // Thumb: bg-background; dark checked → primary-foreground,
        // dark unchecked → foreground.
        let thumb_bg = if theme.dark {
            if checked {
                theme.primary_foreground
            } else {
                theme.foreground
            }
        } else {
            theme.background
        };

        // translate-x-[calc(100%-2px)] from a content box inset by a 1px border:
        // thumb rests 1px from either end of the track's outer edge.
        let thumb_x = if checked { track_w - thumb - 1. } else { 1. };
        // Opposite rest — animation start when toggling.
        let from_x = if checked { 1. } else { track_w - thumb - 1. };
        // Sm: 1.0px top; Default 18.4 with 16px thumb → 1.2px top.
        let thumb_top = (track_h - thumb) / 2.;

        let thumb_el = div()
            .absolute()
            .top(px(thumb_top))
            .left(px(thumb_x))
            .size(px(thumb))
            .rounded_full()
            .bg(thumb_bg);

        // Source thumb: transition-transform 150ms ease-transition. gpui
        // animates on mount — the generation-keyed id starts a fresh slide per
        // toggle (a mid-flight re-toggle restarts from the opposite rest
        // rather than the current position; CSS would continue from current).
        let thumb_child = if animating {
            let anim_id: ElementId = (root_id.clone(), format!("thumb-{generation}")).into();
            let to_x = thumb_x;
            thumb_el
                .with_animation(anim_id, motion::transition(), move |el, delta| {
                    let x = from_x + (to_x - from_x) * delta;
                    el.left(px(x))
                })
                .into_any_element()
        } else {
            thumb_el.into_any_element()
        };

        // One shared toggle for the two click surfaces: the focusable track
        // (keyboard Enter/Space) and the extended hit-area child (pointer).
        let toggle: Option<Rc<dyn Fn(&mut Window, &mut App)>> =
            (!disabled && !read_only).then(|| {
                Rc::new(move |window: &mut Window, cx: &mut App| {
                    let next = !checked;
                    if let Some(state) = &uncontrolled_state {
                        state.update(cx, |value, cx| {
                            *value = next;
                            cx.notify();
                        });
                    }
                    if let Some(on_change) = &on_checked_change {
                        on_change(&next, window, cx);
                    }
                }) as Rc<dyn Fn(&mut Window, &mut App)>
            });

        div()
            .id(self.id)
            .relative()
            .flex_shrink_0()
            .w(px(track_w))
            .h(px(track_h))
            .rounded_full()
            // Source: border border-transparent — real 1px so focus-visible /
            // aria-invalid border colors actually paint.
            .border_1()
            .border_color(gpui::transparent_black())
            .bg(track_bg)
            .shadow_xs()
            .when(disabled, |s| s.opacity(0.5))
            .when(invalid, |s| {
                let ring = motion::focus_ring_destructive(&theme);
                let border = if theme.dark {
                    alpha(theme.destructive, 0.5)
                } else {
                    theme.destructive
                };
                s.border_color(border).shadow(ring)
            })
            .when(!disabled, |s| {
                let theme_focus = theme.clone();
                s.tab_index(0).when(!invalid, move |s| {
                    let ring = motion::focus_ring(&theme_focus);
                    s.focus_visible(move |s| s.border_color(theme_focus.ring).shadow(ring.clone()))
                })
            })
            .when_some(toggle.clone(), |s, toggle| {
                // Keyboard only: pointer clicks land on the hit-area child
                // below, whose hitbox overlaps this one — handling both here
                // would double-toggle.
                s.on_click(move |event, window, cx| {
                    if matches!(event, ClickEvent::Keyboard(_)) {
                        toggle(window, cx);
                    }
                })
            })
            // Extended pointer target mirroring `after:absolute
            // after:-inset-x-3 after:-inset-y-2`: 12px/8px beyond the track
            // (gpui hit-tests children painted outside parent bounds).
            .child(
                div()
                    .id((root_id, "hit"))
                    .absolute()
                    .left(px(-12.))
                    .right(px(-12.))
                    .top(px(-8.))
                    .bottom(px(-8.))
                    .when_some(toggle, |s, toggle| {
                        s.on_click(move |_, window, cx| toggle(window, cx))
                    }),
            )
            .child(thumb_child)
    }
}
