//! Checkbox — port of shadcn base-nova `ui/checkbox.tsx`.
//!
//! Controlled via `checked` + `on_checked_change`, or uncontrolled via
//! `default_checked` + keyed state (Base UI contract). Supports `disabled`,
//! `read_only`, `invalid` (aria-invalid destructive border + ring; when also
//! checked, border/bg stay primary while the destructive ring remains), and
//! `indeterminate` (check icon on unchecked chrome; `aria-checked="mixed"` has
//! no gpui a11y-tree equivalent). Size is fixed at 16×16 with a 14px check icon.
//! Extended hit area mirrors `after:-inset-x-3 after:-inset-y-2`.
//!
//! Omitted (no gpui form-submission equivalent): `name`, `value`, `required`,
//! `uncheckedValue`, `inputRef`, `parent` (checkbox-group not ported),
//! `nativeButton`. RTL is out of scope repo-wide.
//!
//! `transition-shadow` (150ms focus-ring fade) is omitted — gpui focus styles
//! are instant (TODO, same convention as toggle.rs's transition-all note).
//! `group-has-disabled/field:opacity-50` is handled by explicit `.disabled(true)`
//! per repo convention.

use std::rc::Rc;

use gpui::{
    App, ClickEvent, ElementId, Entity, InteractiveElement as _, IntoElement, ParentElement as _,
    RenderOnce, StatefulInteractiveElement as _, Styled, Window, div, prelude::FluentBuilder as _,
    px, svg,
};

use crate::motion;
use crate::theme::{Theme, alpha};

type ChangeHandler = Box<dyn Fn(&bool, &mut Window, &mut App) + 'static>;

#[derive(IntoElement)]
pub struct Checkbox {
    id: ElementId,
    /// `Some` = controlled; `None` = never set → resolve via keyed uncontrolled state.
    checked: Option<bool>,
    default_checked: bool,
    disabled: bool,
    read_only: bool,
    invalid: bool,
    indeterminate: bool,
    on_checked_change: Option<ChangeHandler>,
}

impl Checkbox {
    pub fn new(id: impl Into<ElementId>) -> Self {
        Self {
            id: id.into(),
            checked: None,
            default_checked: false,
            disabled: false,
            read_only: false,
            invalid: false,
            indeterminate: false,
            on_checked_change: None,
        }
    }

    /// Controlled checked override. When set, wins over keyed uncontrolled state.
    pub fn checked(mut self, checked: bool) -> Self {
        self.checked = Some(checked);
        self
    }

    /// Initial checked value for uncontrolled mode (Base UI `defaultChecked`).
    pub fn default_checked(mut self, default_checked: bool) -> Self {
        self.default_checked = default_checked;
        self
    }

    pub fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }

    /// Focusable with the focus ring, but activation (click/Enter/Space) is a
    /// no-op. Distinct from `disabled`, which also drops focusability and dims
    /// the control.
    pub fn read_only(mut self, read_only: bool) -> Self {
        self.read_only = read_only;
        self
    }

    /// Aria-invalid styles: destructive ring always-on; destructive border
    /// unless also checked (then border/bg stay primary).
    pub fn invalid(mut self, invalid: bool) -> Self {
        self.invalid = invalid;
        self
    }

    /// Base UI `indeterminate`. Renders the check indicator on unchecked chrome
    /// when set and not checked. Toggling behaves like any unchecked checkbox
    /// (next value = `!checked`). `aria-checked="mixed"` has no gpui a11y-tree
    /// equivalent.
    pub fn indeterminate(mut self, indeterminate: bool) -> Self {
        self.indeterminate = indeterminate;
        self
    }

    /// Base UI `onCheckedChange`. Receives the next checked value.
    pub fn on_checked_change(
        mut self,
        handler: impl Fn(&bool, &mut Window, &mut App) + 'static,
    ) -> Self {
        self.on_checked_change = Some(Box::new(handler));
        self
    }
}

impl RenderOnce for Checkbox {
    fn render(self, window: &mut Window, cx: &mut App) -> impl IntoElement {
        let theme = Theme::of(cx).clone();
        let disabled = self.disabled;
        let read_only = self.read_only;
        let invalid = self.invalid;
        let indeterminate = self.indeterminate;
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

        // Base UI shows the indicator when checked || indeterminate.
        let show_indicator = checked || indeterminate;

        // One shared toggle for the two click surfaces: the focusable root
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

        // size-4 rounded-[4px] border border-input shadow-xs;
        // checked: border-primary bg-primary text-primary-foreground;
        // dark unchecked: bg-input/30
        // indeterminate && !checked: unchecked chrome + foreground check icon
        div()
            .id(self.id)
            .relative()
            .flex()
            .flex_shrink_0()
            .size(px(16.))
            .items_center()
            .justify_center()
            .rounded(px(4.))
            .border_1()
            .shadow_xs()
            .map(|el| {
                if checked {
                    el.border_color(theme.primary).bg(theme.primary)
                } else if theme.dark {
                    el.border_color(theme.input).bg(alpha(theme.input, 0.3))
                } else {
                    el.border_color(theme.input)
                }
            })
            .when(disabled, |el| el.opacity(0.5))
            .when(invalid, |el| {
                // Always-on destructive ring. Border is destructive unless
                // also checked (`aria-invalid:aria-checked:border-primary`) —
                // then border/bg stay primary (set above) while the ring remains.
                let ring = motion::focus_ring_destructive(&theme);
                if checked {
                    el.shadow(ring)
                } else {
                    let border = if theme.dark {
                        alpha(theme.destructive, 0.5)
                    } else {
                        theme.destructive
                    };
                    el.border_color(border).shadow(ring)
                }
            })
            .when(!disabled, |el| {
                let theme_focus = theme.clone();
                el.tab_index(0).when(!invalid, move |el| {
                    let ring = motion::focus_ring(&theme_focus);
                    el.focus_visible(move |s| s.border_color(theme_focus.ring).shadow(ring.clone()))
                })
            })
            .when_some(toggle.clone(), |el, toggle| {
                // Keyboard only: pointer clicks land on the hit-area child
                // below, whose hitbox overlaps this one — handling both here
                // would double-toggle.
                el.on_click(move |event, window, cx| {
                    if matches!(event, ClickEvent::Keyboard(_)) {
                        toggle(window, cx);
                    }
                })
            })
            // Extended pointer target mirroring `after:absolute
            // after:-inset-x-3 after:-inset-y-2`: 12px/8px beyond the box
            // (gpui hit-tests children painted outside parent bounds; insets
            // are padding-box-relative, hence the extra 1px for the border).
            .child(
                div()
                    .id((root_id, "hit"))
                    .absolute()
                    .left(px(-13.))
                    .right(px(-13.))
                    .top(px(-9.))
                    .bottom(px(-9.))
                    .when_some(toggle, |el, toggle| {
                        el.on_click(move |_, window, cx| toggle(window, cx))
                    }),
            )
            .when(show_indicator, |el| {
                let icon_color = if checked {
                    theme.primary_foreground
                } else {
                    // indeterminate on unchecked chrome: text-current → foreground
                    theme.foreground
                };
                el.child(
                    svg()
                        .path(theme.icons.check())
                        .size(px(14.))
                        .text_color(icon_color),
                )
            })
    }
}
