//! Pagination — port of shadcn base-vega `ui/pagination.tsx`.
//!
//! Page navigation built on [`Button`] styles: ghost links (outline when
//! active), previous/next with icon-library chevrons, and an ellipsis for
//! collapsed ranges.

use gpui::{
    App, ClickEvent, ElementId, IntoElement, ParentElement, RenderOnce, Styled, Window, div,
    prelude::FluentBuilder as _, px, svg,
};

use crate::assets::ICON_ELLIPSIS;
use crate::components::button::{Button, ButtonSize, ButtonVariant};
use crate::theme::Theme;

/// nav: mx-auto flex w-full justify-center.
#[derive(IntoElement)]
pub struct Pagination {
    children: Vec<gpui::AnyElement>,
}

impl Pagination {
    pub fn new() -> Self {
        Self {
            children: Vec::new(),
        }
    }
}

impl Default for Pagination {
    fn default() -> Self {
        Self::new()
    }
}

impl ParentElement for Pagination {
    fn extend(&mut self, elements: impl IntoIterator<Item = gpui::AnyElement>) {
        self.children.extend(elements);
    }
}

impl RenderOnce for Pagination {
    fn render(self, _window: &mut Window, _cx: &mut App) -> impl IntoElement {
        div()
            .flex()
            .flex_row()
            .w_full()
            .items_center()
            .justify_center()
            .gap(px(4.))
            .children(self.children)
    }
}

/// A page-number link: ghost, outline when active (`isActive`).
#[derive(IntoElement)]
pub struct PaginationLink {
    id: ElementId,
    active: bool,
    label: gpui::SharedString,
    on_click: Option<Box<dyn Fn(&ClickEvent, &mut Window, &mut App) + 'static>>,
}

impl PaginationLink {
    pub fn new(id: impl Into<ElementId>, label: impl Into<gpui::SharedString>) -> Self {
        Self {
            id: id.into(),
            active: false,
            label: label.into(),
            on_click: None,
        }
    }

    pub fn active(mut self, active: bool) -> Self {
        self.active = active;
        self
    }

    pub fn on_click(
        mut self,
        handler: impl Fn(&ClickEvent, &mut Window, &mut App) + 'static,
    ) -> Self {
        self.on_click = Some(Box::new(handler));
        self
    }
}

impl RenderOnce for PaginationLink {
    fn render(self, _window: &mut Window, _cx: &mut App) -> impl IntoElement {
        Button::new(self.id)
            .variant(if self.active {
                ButtonVariant::Outline
            } else {
                ButtonVariant::Ghost
            })
            .size(ButtonSize::Icon)
            .when_some(self.on_click, |el, on_click| el.on_click(on_click))
            .child(self.label)
    }
}

/// The "Previous" link with a leading chevron.
#[derive(IntoElement)]
pub struct PaginationPrevious {
    id: ElementId,
    on_click: Option<Box<dyn Fn(&ClickEvent, &mut Window, &mut App) + 'static>>,
}

impl PaginationPrevious {
    pub fn new(id: impl Into<ElementId>) -> Self {
        Self {
            id: id.into(),
            on_click: None,
        }
    }

    pub fn on_click(
        mut self,
        handler: impl Fn(&ClickEvent, &mut Window, &mut App) + 'static,
    ) -> Self {
        self.on_click = Some(Box::new(handler));
        self
    }
}

impl RenderOnce for PaginationPrevious {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let theme = Theme::of(cx).clone();
        Button::new(self.id)
            .variant(ButtonVariant::Ghost)
            .when_some(self.on_click, |el, on_click| el.on_click(on_click))
            .child(
                svg()
                    .path(theme.icons.chevron_left())
                    .size(px(16.))
                    .text_color(theme.foreground),
            )
            .child("Previous")
    }
}

/// The "Next" link with a trailing chevron.
#[derive(IntoElement)]
pub struct PaginationNext {
    id: ElementId,
    on_click: Option<Box<dyn Fn(&ClickEvent, &mut Window, &mut App) + 'static>>,
}

impl PaginationNext {
    pub fn new(id: impl Into<ElementId>) -> Self {
        Self {
            id: id.into(),
            on_click: None,
        }
    }

    pub fn on_click(
        mut self,
        handler: impl Fn(&ClickEvent, &mut Window, &mut App) + 'static,
    ) -> Self {
        self.on_click = Some(Box::new(handler));
        self
    }
}

impl RenderOnce for PaginationNext {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let theme = Theme::of(cx).clone();
        Button::new(self.id)
            .variant(ButtonVariant::Ghost)
            .when_some(self.on_click, |el, on_click| el.on_click(on_click))
            .child("Next")
            .child(
                svg()
                    .path(theme.icons.chevron_right())
                    .size(px(16.))
                    .text_color(theme.foreground),
            )
    }
}

/// Collapsed range marker.
#[derive(IntoElement)]
pub struct PaginationEllipsis;

impl PaginationEllipsis {
    pub fn new() -> Self {
        Self
    }
}

impl Default for PaginationEllipsis {
    fn default() -> Self {
        Self::new()
    }
}

impl RenderOnce for PaginationEllipsis {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let theme = Theme::of(cx);
        div()
            .flex()
            .size(px(36.))
            .items_center()
            .justify_center()
            .child(
                svg()
                    .path(ICON_ELLIPSIS)
                    .size(px(16.))
                    .text_color(theme.muted_foreground),
            )
    }
}
