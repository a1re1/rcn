//! Toggle — port of shadcn base-nova `ui/toggle.tsx`.
//!
//! A two-state button. Controlled: the caller owns `pressed` via `.pressed(bool)`
//! and receives the next value in `on_pressed_change`. Uncontrolled: omit
//! `.pressed` and optionally set `.default_pressed(bool)` (default false); the
//! toggle owns the bool via keyed state and still fires `on_pressed_change`.
//! Aria-invalid styles are omitted.
//!
//! Nova default descendant svg size is 16px and Sm's is 14px
//! (`[&_svg:not([class*='size-'])]:size-4` / `size-3.5`). gpui cannot style
//! descendant svgs, so storybook examples pass icon sizes explicitly.
//!
//! TODO: nova's `transition-all` hover transition (150ms
//! `cubic-bezier(0.4,0,0.2,1)`, `motion::ease_transition`) is omitted because
//! gpui hover styles are instant.

use gpui::{
    AnyElement, App, ElementId, Entity, FontWeight, InteractiveElement as _, IntoElement,
    ParentElement, RenderOnce, StatefulInteractiveElement as _, Styled, Window, div,
    prelude::FluentBuilder as _, px,
};

use crate::motion;
use crate::theme::Theme;

#[derive(Clone, Copy, PartialEq, Eq, Default, Debug)]
pub enum ToggleVariant {
    #[default]
    Default,
    Outline,
}

#[derive(Clone, Copy, PartialEq, Eq, Default, Debug)]
pub enum ToggleSize {
    #[default]
    Default,
    Sm,
    Lg,
}

impl ToggleSize {
    /// (height, min-width) — base-nova h-8/h-7/h-9 + min-w-8/min-w-7/min-w-9.
    fn metrics(self) -> (f32, f32) {
        match self {
            ToggleSize::Default => (32., 32.),
            ToggleSize::Sm => (28., 28.),
            ToggleSize::Lg => (36., 36.),
        }
    }
}

type ChangeHandler = Box<dyn Fn(&bool, &mut Window, &mut App) + 'static>;

#[derive(IntoElement)]
pub struct Toggle {
    id: ElementId,
    variant: ToggleVariant,
    size: ToggleSize,
    /// `Some` = controlled; `None` = uncontrolled (starts at `default_pressed`).
    pressed: Option<bool>,
    default_pressed: bool,
    disabled: bool,
    icon_inline_start: bool,
    icon_inline_end: bool,
    on_pressed_change: Option<ChangeHandler>,
    children: Vec<AnyElement>,
}

impl Toggle {
    pub fn new(id: impl Into<ElementId>) -> Self {
        Self {
            id: id.into(),
            variant: ToggleVariant::default(),
            size: ToggleSize::default(),
            pressed: None,
            default_pressed: false,
            disabled: false,
            icon_inline_start: false,
            icon_inline_end: false,
            on_pressed_change: None,
            children: Vec::new(),
        }
    }

    pub fn variant(mut self, variant: ToggleVariant) -> Self {
        self.variant = variant;
        self
    }

    pub fn size(mut self, size: ToggleSize) -> Self {
        self.size = size;
        self
    }

    /// Controlled pressed state. Takes precedence over `default_pressed`.
    pub fn pressed(mut self, pressed: bool) -> Self {
        self.pressed = Some(pressed);
        self
    }

    /// Uncontrolled initial pressed state (Base UI `defaultPressed`). Default false.
    /// Ignored when `.pressed(bool)` is set.
    pub fn default_pressed(mut self, pressed: bool) -> Self {
        self.default_pressed = pressed;
        self
    }

    pub fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }

    /// Child `data-icon="inline-start"` — trim start padding
    /// (`has-data-[icon=inline-start]:pl-2` / `pl-1.5` for Sm).
    pub fn icon_inline_start(mut self) -> Self {
        self.icon_inline_start = true;
        self
    }

    /// Child `data-icon="inline-end"` — trim end padding
    /// (`has-data-[icon=inline-end]:pr-2` / `pr-1.5` for Sm).
    // Intentional API surface; the shadcn docs examples only use start icons.
    #[allow(dead_code)]
    pub fn icon_inline_end(mut self) -> Self {
        self.icon_inline_end = true;
        self
    }

    pub fn on_pressed_change(
        mut self,
        handler: impl Fn(&bool, &mut Window, &mut App) + 'static,
    ) -> Self {
        self.on_pressed_change = Some(Box::new(handler));
        self
    }
}

impl ParentElement for Toggle {
    fn extend(&mut self, elements: impl IntoIterator<Item = AnyElement>) {
        self.children.extend(elements);
    }
}

impl RenderOnce for Toggle {
    fn render(self, window: &mut Window, cx: &mut App) -> impl IntoElement {
        let theme = Theme::of(cx).clone();
        let (height, min_width) = self.size.metrics();

        // Resolve pressed: controlled snapshot, or keyed uncontrolled state.
        let on_pressed_change = self.on_pressed_change;
        let (pressed, uncontrolled_state): (bool, Option<Entity<bool>>) =
            if let Some(pressed) = self.pressed {
                (pressed, None)
            } else {
                let default_pressed = self.default_pressed;
                let state_key = self.id.clone();
                let state = window.use_keyed_state(state_key, cx, move |_, _| default_pressed);
                (*state.read(cx), Some(state))
            };

        // Base horizontal padding px-2.5; icon_inline_* trims to pl/pr-2 (Default/Lg)
        // or pl/pr-1.5 (Sm). Rounding: rounded-lg; Sm uses min(radius-md, 12px).
        let base_pl = px(10.);
        let base_pr = px(10.);
        let (pl, pr) = match self.size {
            ToggleSize::Sm => {
                let pl = if self.icon_inline_start {
                    px(6.)
                } else {
                    base_pl
                };
                let pr = if self.icon_inline_end {
                    px(6.)
                } else {
                    base_pr
                };
                (pl, pr)
            }
            ToggleSize::Default | ToggleSize::Lg => {
                let pl = if self.icon_inline_start {
                    px(8.)
                } else {
                    base_pl
                };
                let pr = if self.icon_inline_end {
                    px(8.)
                } else {
                    base_pr
                };
                (pl, pr)
            }
        };

        let rounded = match self.size {
            ToggleSize::Sm => theme.radius_md().min(px(12.)),
            ToggleSize::Default | ToggleSize::Lg => theme.radius_lg(),
        };

        let text_size = match self.size {
            ToggleSize::Sm => px(12.8),
            ToggleSize::Default | ToggleSize::Lg => px(14.),
        };

        // inline-flex items-center justify-center gap-1 rounded-lg text-sm
        // font-medium hover:bg-muted hover:text-foreground
        // aria-pressed:bg-muted; outline: border border-input (no shadow)
        div()
            .id(self.id)
            .flex()
            .flex_row()
            .flex_shrink_0()
            .items_center()
            .justify_center()
            .gap(px(4.))
            .h(px(height))
            .min_w(px(min_width))
            .pl(pl)
            .pr(pr)
            .rounded(rounded)
            .text_size(text_size)
            .line_height(px(20.))
            .font_weight(FontWeight::MEDIUM)
            .text_color(theme.foreground)
            .whitespace_nowrap()
            .when(self.variant == ToggleVariant::Outline, |el| {
                el.border_1().border_color(theme.input)
            })
            .when(pressed, |el| el.bg(theme.muted))
            .when(self.disabled, |el| el.opacity(0.5))
            .when(!self.disabled, |el| {
                let ring = motion::focus_ring(&theme);
                let hover_bg = theme.muted;
                let hover_fg = theme.foreground;
                el.tab_index(0)
                    .focus_visible(move |s| s.border_color(theme.ring).shadow(ring.clone()))
                    .hover(move |s| s.bg(hover_bg).text_color(hover_fg))
                    .on_click(move |_, window, cx| {
                        let next = !pressed;
                        if let Some(state) = &uncontrolled_state {
                            state.update(cx, |value, cx| {
                                *value = next;
                                cx.notify();
                            });
                        }
                        if let Some(on_pressed_change) = &on_pressed_change {
                            on_pressed_change(&next, window, cx);
                        }
                    })
            })
            .children(self.children)
    }
}
