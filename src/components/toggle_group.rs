//! ToggleGroup — port of shadcn base-vega `ui/toggle-group.tsx`.
//!
//! A joined row of toggles: middle items square, ends rounded, outline
//! borders collapsed between neighbors. Items are typed (`.item(..)`)
//! rather than free children so the group can apply position-aware
//! styling. Vertical orientation and the spacing>0 mode are omitted.

use gpui::{
    AnyElement, App, ElementId, FontWeight, InteractiveElement as _, IntoElement,
    ParentElement as _, RenderOnce, StatefulInteractiveElement as _, Styled, Window, div,
    prelude::FluentBuilder as _, px,
};

use crate::theme::Theme;

pub use crate::components::toggle::{ToggleSize, ToggleVariant};

type ChangeHandler = Box<dyn Fn(&bool, &mut Window, &mut App) + 'static>;

pub struct ToggleGroupItem {
    id: ElementId,
    pressed: bool,
    disabled: bool,
    on_change: Option<ChangeHandler>,
    children: Vec<AnyElement>,
}

impl ToggleGroupItem {
    pub fn new(id: impl Into<ElementId>) -> Self {
        Self {
            id: id.into(),
            pressed: false,
            disabled: false,
            on_change: None,
            children: Vec::new(),
        }
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

    pub fn child(mut self, child: impl IntoElement) -> Self {
        self.children.push(child.into_any_element());
        self
    }
}

#[derive(IntoElement)]
pub struct ToggleGroup {
    variant: ToggleVariant,
    size: ToggleSize,
    items: Vec<ToggleGroupItem>,
}

impl ToggleGroup {
    pub fn new() -> Self {
        Self {
            variant: ToggleVariant::default(),
            size: ToggleSize::default(),
            items: Vec::new(),
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

    pub fn item(mut self, item: ToggleGroupItem) -> Self {
        self.items.push(item);
        self
    }
}

impl Default for ToggleGroup {
    fn default() -> Self {
        Self::new()
    }
}

impl RenderOnce for ToggleGroup {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let theme = Theme::of(cx).clone();
        let (height, min_width) = match self.size {
            ToggleSize::Default => (36., 36.),
            ToggleSize::Sm => (32., 32.),
            ToggleSize::Lg => (40., 40.),
        };
        let count = self.items.len();
        let variant = self.variant;
        let radius = theme.radius_md();

        div()
            .flex()
            .flex_row()
            .items_center()
            .rounded(radius)
            .when(variant == ToggleVariant::Outline, |el| el.shadow_xs())
            .children(self.items.into_iter().enumerate().map(|(index, item)| {
                let first = index == 0;
                let last = index + 1 == count;
                let pressed = item.pressed;
                let hover_bg = theme.muted;
                div()
                    .id(item.id)
                    .flex()
                    .flex_row()
                    .flex_shrink_0()
                    .items_center()
                    .justify_center()
                    .gap(px(4.))
                    .h(px(height))
                    .min_w(px(min_width))
                    .px(px(8.))
                    .text_size(px(14.))
                    .line_height(px(20.))
                    .font_weight(FontWeight::MEDIUM)
                    .text_color(theme.foreground)
                    .whitespace_nowrap()
                    // first:rounded-l-md last:rounded-r-md, square in between
                    .when(first, |el| el.rounded_l(radius))
                    .when(last, |el| el.rounded_r(radius))
                    // outline: border with collapsed left edges
                    .when(variant == ToggleVariant::Outline, |el| {
                        el.border_t_1()
                            .border_b_1()
                            .border_r_1()
                            .when(first, |el| el.border_l_1())
                            .border_color(theme.input)
                    })
                    .when(pressed, |el| el.bg(theme.muted))
                    .when(item.disabled, |el| el.opacity(0.5))
                    .when(!item.disabled, |el| {
                        el.hover(move |s| s.bg(hover_bg)).when_some(
                            item.on_change,
                            |el, on_change| {
                                el.on_click(move |_, window, cx| on_change(&!pressed, window, cx))
                            },
                        )
                    })
                    .children(item.children)
            }))
    }
}
