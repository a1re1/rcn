//! RadioGroup — port of shadcn base-vega `ui/radio-group.tsx`.
//!
//! Controlled: the group is layout (grid gap-3); each item owns `checked`
//! and reports selection via `on_select`. The indicator is a filled dot,
//! like the source's `RadioGroupIndicator`. Focus-visible ring and
//! aria-invalid styles are omitted.

use gpui::{
    AnyElement, App, ClickEvent, ElementId, InteractiveElement as _, IntoElement, ParentElement,
    RenderOnce, StatefulInteractiveElement as _, Styled, Window, div, prelude::FluentBuilder as _,
    px,
};

use crate::theme::{Theme, alpha};

type SelectHandler = Box<dyn Fn(&ClickEvent, &mut Window, &mut App) + 'static>;

/// grid gap-3 — the group container.
#[derive(IntoElement)]
pub struct RadioGroup {
    children: Vec<AnyElement>,
}

impl RadioGroup {
    pub fn new() -> Self {
        Self {
            children: Vec::new(),
        }
    }
}

impl Default for RadioGroup {
    fn default() -> Self {
        Self::new()
    }
}

impl ParentElement for RadioGroup {
    fn extend(&mut self, elements: impl IntoIterator<Item = AnyElement>) {
        self.children.extend(elements);
    }
}

impl RenderOnce for RadioGroup {
    fn render(self, _window: &mut Window, _cx: &mut App) -> impl IntoElement {
        div().flex().flex_col().gap(px(12.)).children(self.children)
    }
}

/// size-4 rounded-full border; checked → primary border + filled dot.
#[derive(IntoElement)]
pub struct RadioGroupItem {
    id: ElementId,
    checked: bool,
    disabled: bool,
    on_select: Option<SelectHandler>,
}

impl RadioGroupItem {
    pub fn new(id: impl Into<ElementId>) -> Self {
        Self {
            id: id.into(),
            checked: false,
            disabled: false,
            on_select: None,
        }
    }

    pub fn checked(mut self, checked: bool) -> Self {
        self.checked = checked;
        self
    }

    pub fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }

    pub fn on_select(
        mut self,
        handler: impl Fn(&ClickEvent, &mut Window, &mut App) + 'static,
    ) -> Self {
        self.on_select = Some(Box::new(handler));
        self
    }
}

impl RenderOnce for RadioGroupItem {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let theme = Theme::of(cx).clone();
        div()
            .id(self.id)
            .flex()
            .flex_shrink_0()
            .size(px(16.))
            .items_center()
            .justify_center()
            .rounded_full()
            .border_1()
            .shadow_xs()
            .map(|el| {
                if self.checked {
                    el.border_color(theme.primary)
                } else if theme.dark {
                    el.border_color(theme.input).bg(alpha(theme.input, 0.3))
                } else {
                    el.border_color(theme.input)
                }
            })
            .when(self.disabled, |el| el.opacity(0.5))
            .when(!self.disabled, |el| {
                el.when_some(self.on_select, |el, on_select| el.on_click(on_select))
            })
            .when(self.checked, |el| {
                el.child(div().size(px(8.)).rounded_full().bg(theme.primary))
            })
    }
}
