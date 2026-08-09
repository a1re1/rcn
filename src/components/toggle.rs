//! Toggle — port of shadcn base-vega `ui/toggle.tsx`.
//!
//! A two-state button. Controlled: the caller owns `pressed` and receives
//! the next value in `on_change`. Focus-visible ring and aria-invalid
//! styles are omitted.

use gpui::{
    AnyElement, App, ElementId, FontWeight, InteractiveElement as _, IntoElement, ParentElement,
    RenderOnce, StatefulInteractiveElement as _, Styled, Window, div, prelude::FluentBuilder as _,
    px,
};

use crate::theme::{Theme, alpha};

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
    /// (height, min-width)
    fn metrics(self) -> (f32, f32) {
        match self {
            ToggleSize::Default => (36., 36.),
            ToggleSize::Sm => (32., 32.),
            ToggleSize::Lg => (40., 40.),
        }
    }
}

type ChangeHandler = Box<dyn Fn(&bool, &mut Window, &mut App) + 'static>;

#[derive(IntoElement)]
pub struct Toggle {
    id: ElementId,
    variant: ToggleVariant,
    size: ToggleSize,
    pressed: bool,
    disabled: bool,
    on_change: Option<ChangeHandler>,
    children: Vec<AnyElement>,
}

impl Toggle {
    pub fn new(id: impl Into<ElementId>) -> Self {
        Self {
            id: id.into(),
            variant: ToggleVariant::default(),
            size: ToggleSize::default(),
            pressed: false,
            disabled: false,
            on_change: None,
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

    pub fn pressed(mut self, pressed: bool) -> Self {
        self.pressed = pressed;
        self
    }

    pub fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }

    pub fn on_change(mut self, handler: impl Fn(&bool, &mut Window, &mut App) + 'static) -> Self {
        self.on_change = Some(Box::new(handler));
        self
    }
}

impl ParentElement for Toggle {
    fn extend(&mut self, elements: impl IntoIterator<Item = AnyElement>) {
        self.children.extend(elements);
    }
}

impl RenderOnce for Toggle {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let theme = Theme::of(cx).clone();
        let (height, min_width) = self.size.metrics();
        let pressed = self.pressed;

        // inline-flex items-center justify-center gap-1 rounded-md text-sm
        // font-medium hover:bg-muted hover:text-foreground
        // aria-pressed:bg-muted; outline: border border-input shadow-xs
        let hover_bg = theme.muted;
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
            .px(px(10.))
            .rounded(theme.radius_md())
            .text_size(px(14.))
            .line_height(px(20.))
            .font_weight(FontWeight::MEDIUM)
            .text_color(theme.foreground)
            .whitespace_nowrap()
            .when(self.variant == ToggleVariant::Outline, |el| {
                el.border_1().border_color(theme.input).shadow_xs()
            })
            .when(pressed, |el| el.bg(theme.muted))
            .when(self.disabled, |el| el.opacity(0.5))
            .when(!self.disabled, |el| {
                el.hover(move |s| s.bg(alpha(hover_bg, if pressed { 1. } else { 0.8 })))
                    .when_some(self.on_change, |el, on_change| {
                        el.on_click(move |_, window, cx| on_change(&!pressed, window, cx))
                    })
            })
            .children(self.children)
    }
}
