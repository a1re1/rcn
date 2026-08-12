//! Pagination — port of shadcn base-vega `ui/pagination.tsx`.
//!
//! Page navigation built on [`Button`] styles: ghost links (outline when
//! active), previous/next with icon-library chevrons, and an ellipsis for
//! collapsed ranges.
//!
//! Composition mirrors the source: `Pagination` (nav) > `PaginationContent`
//! (ul) > `PaginationItem` (li) > link / previous / next / ellipsis.
//!
//! Omissions:
//! - aria attributes (`role=navigation`, `aria-label="pagination"`,
//!   `aria-current="page"`, go-to-previous/next labels, sr-only "More pages",
//!   `data-slot` / `data-active`) — gpui exposes no accessibility tree.
//! - RTL layout (source flips chevrons via `rtl:rotate-180`) — no RTL support
//!   in rcn yet. TODO.
//!
//! Sizing and shape overrides come from the caller via [`Styled`]
//! (delegating parts forward the refinement into their inner [`Button`]).

use gpui::{
    AnyElement, App, ClickEvent, ElementId, IntoElement, ParentElement, Refineable as _,
    RenderOnce, SharedString, StyleRefinement, Styled, Window, div, prelude::FluentBuilder as _,
    px,
};

use crate::assets::ICON_ELLIPSIS;
use crate::components::button::{Button, ButtonSize, ButtonVariant};
use crate::components::icon::Icon;
use crate::theme::Theme;

type ClickHandler = Box<dyn Fn(&ClickEvent, &mut Window, &mut App) + 'static>;

/// nav: mx-auto flex w-full justify-center.
#[derive(IntoElement)]
pub struct Pagination {
    children: Vec<AnyElement>,
    /// When true, width is auto instead of w-full (Icons Only docs: `w-auto`).
    w_auto: bool,
    style: StyleRefinement,
}

impl Pagination {
    pub fn new() -> Self {
        Self {
            children: Vec::new(),
            w_auto: false,
            style: StyleRefinement::default(),
        }
    }

    /// `w-auto` instead of the default `w-full` (ports Icons Only `className="mx-0 w-auto"`).
    pub fn w_auto(mut self) -> Self {
        self.w_auto = true;
        self
    }
}

impl Default for Pagination {
    fn default() -> Self {
        Self::new()
    }
}

impl ParentElement for Pagination {
    fn extend(&mut self, elements: impl IntoIterator<Item = AnyElement>) {
        self.children.extend(elements);
    }
}

impl Styled for Pagination {
    fn style(&mut self) -> &mut StyleRefinement {
        &mut self.style
    }
}

impl RenderOnce for Pagination {
    fn render(self, _window: &mut Window, _cx: &mut App) -> impl IntoElement {
        let mut root = div()
            .flex()
            .flex_row()
            .when(self.w_auto, |el| el.w_auto())
            .when(!self.w_auto, |el| el.w_full())
            .justify_center()
            .children(self.children);
        root.style().refine(&self.style);
        root
    }
}

/// ul: flex items-center gap-1.
#[derive(IntoElement)]
pub struct PaginationContent {
    children: Vec<AnyElement>,
    style: StyleRefinement,
}

impl PaginationContent {
    pub fn new() -> Self {
        Self {
            children: Vec::new(),
            style: StyleRefinement::default(),
        }
    }
}

impl Default for PaginationContent {
    fn default() -> Self {
        Self::new()
    }
}

impl ParentElement for PaginationContent {
    fn extend(&mut self, elements: impl IntoIterator<Item = AnyElement>) {
        self.children.extend(elements);
    }
}

impl Styled for PaginationContent {
    fn style(&mut self) -> &mut StyleRefinement {
        &mut self.style
    }
}

impl RenderOnce for PaginationContent {
    fn render(self, _window: &mut Window, _cx: &mut App) -> impl IntoElement {
        let mut root = div()
            .flex()
            .flex_row()
            .items_center()
            .gap(px(4.))
            .children(self.children);
        root.style().refine(&self.style);
        root
    }
}

/// li: unstyled passthrough wrapper.
#[derive(IntoElement)]
pub struct PaginationItem {
    children: Vec<AnyElement>,
    style: StyleRefinement,
}

impl PaginationItem {
    pub fn new() -> Self {
        Self {
            children: Vec::new(),
            style: StyleRefinement::default(),
        }
    }
}

impl Default for PaginationItem {
    fn default() -> Self {
        Self::new()
    }
}

impl ParentElement for PaginationItem {
    fn extend(&mut self, elements: impl IntoIterator<Item = AnyElement>) {
        self.children.extend(elements);
    }
}

impl Styled for PaginationItem {
    fn style(&mut self) -> &mut StyleRefinement {
        &mut self.style
    }
}

impl RenderOnce for PaginationItem {
    fn render(self, _window: &mut Window, _cx: &mut App) -> impl IntoElement {
        let mut root = div().children(self.children);
        root.style().refine(&self.style);
        root
    }
}

/// A page-number link: ghost, outline when active (`isActive`).
///
/// size defaults to `"icon"` (shadcn `Pick<ButtonProps, "size">`).
#[derive(IntoElement)]
pub struct PaginationLink {
    id: ElementId,
    active: bool,
    label: SharedString,
    size: ButtonSize,
    on_click: Option<ClickHandler>,
    style: StyleRefinement,
}

impl PaginationLink {
    pub fn new(id: impl Into<ElementId>, label: impl Into<SharedString>) -> Self {
        Self {
            id: id.into(),
            active: false,
            label: label.into(),
            size: ButtonSize::Icon,
            on_click: None,
            style: StyleRefinement::default(),
        }
    }

    pub fn active(mut self, active: bool) -> Self {
        self.active = active;
        self
    }

    /// Button size — shadcn default `"icon"`.
    pub fn size(mut self, size: ButtonSize) -> Self {
        self.size = size;
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

impl Styled for PaginationLink {
    fn style(&mut self) -> &mut StyleRefinement {
        &mut self.style
    }
}

impl RenderOnce for PaginationLink {
    fn render(self, _window: &mut Window, _cx: &mut App) -> impl IntoElement {
        // Forward the caller refinement into the inner Button (it wins there too).
        let mut button = Button::new(self.id)
            .variant(if self.active {
                ButtonVariant::Outline
            } else {
                ButtonVariant::Ghost
            })
            .size(self.size)
            .when_some(self.on_click, |el, on_click| el.on_click(on_click))
            .child(self.label);
        button.style().refine(&self.style);
        button
    }
}

/// Previous-page link: ghost, default size, chevron start + label.
///
/// Label uses viewport `sm` (≥640px) visibility — ports `hidden sm:block`
/// (window media query, not a container query); gpui re-renders on resize.
#[derive(IntoElement)]
pub struct PaginationPrevious {
    id: ElementId,
    text: SharedString,
    on_click: Option<ClickHandler>,
    style: StyleRefinement,
}

impl PaginationPrevious {
    pub fn new(id: impl Into<ElementId>) -> Self {
        Self {
            id: id.into(),
            text: "Previous".into(),
            on_click: None,
            style: StyleRefinement::default(),
        }
    }

    /// Label text — shadcn `text` prop, default `"Previous"`.
    pub fn text(mut self, text: impl Into<SharedString>) -> Self {
        self.text = text.into();
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

impl Styled for PaginationPrevious {
    fn style(&mut self) -> &mut StyleRefinement {
        &mut self.style
    }
}

impl RenderOnce for PaginationPrevious {
    fn render(self, window: &mut Window, cx: &mut App) -> impl IntoElement {
        let theme = Theme::of(cx);
        let show_label = window.viewport_size().width >= px(640.);
        // Forward the caller refinement into the inner Button (it wins there too).
        let mut button = Button::new(self.id)
            .variant(ButtonVariant::Ghost)
            .icon_inline_start()
            .when_some(self.on_click, |el, on_click| el.on_click(on_click))
            .child(Icon::new(theme.icons.chevron_left()))
            .when(show_label, |el| el.child(self.text));
        button.style().refine(&self.style);
        button
    }
}

/// Next-page link: ghost, default size, label + chevron end.
///
/// Label uses viewport `sm` (≥640px) visibility — ports `hidden sm:block`
/// (window media query, not a container query); gpui re-renders on resize.
#[derive(IntoElement)]
pub struct PaginationNext {
    id: ElementId,
    text: SharedString,
    on_click: Option<ClickHandler>,
    style: StyleRefinement,
}

impl PaginationNext {
    pub fn new(id: impl Into<ElementId>) -> Self {
        Self {
            id: id.into(),
            text: "Next".into(),
            on_click: None,
            style: StyleRefinement::default(),
        }
    }

    /// Label text — shadcn `text` prop, default `"Next"`.
    pub fn text(mut self, text: impl Into<SharedString>) -> Self {
        self.text = text.into();
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

impl Styled for PaginationNext {
    fn style(&mut self) -> &mut StyleRefinement {
        &mut self.style
    }
}

impl RenderOnce for PaginationNext {
    fn render(self, window: &mut Window, cx: &mut App) -> impl IntoElement {
        let theme = Theme::of(cx);
        let show_label = window.viewport_size().width >= px(640.);
        // Forward the caller refinement into the inner Button (it wins there too).
        let mut button = Button::new(self.id)
            .variant(ButtonVariant::Ghost)
            .icon_inline_end()
            .when_some(self.on_click, |el, on_click| el.on_click(on_click))
            .when(show_label, |el| el.child(self.text))
            .child(Icon::new(theme.icons.chevron_right()));
        button.style().refine(&self.style);
        button
    }
}

/// span: flex size-9 items-center justify-center — collapsed range marker.
///
/// Icon inherits default foreground (no muted color — shadcn has no color class).
#[derive(IntoElement)]
pub struct PaginationEllipsis {
    style: StyleRefinement,
}

impl PaginationEllipsis {
    pub fn new() -> Self {
        Self {
            style: StyleRefinement::default(),
        }
    }
}

impl Default for PaginationEllipsis {
    fn default() -> Self {
        Self::new()
    }
}

impl Styled for PaginationEllipsis {
    fn style(&mut self) -> &mut StyleRefinement {
        &mut self.style
    }
}

impl RenderOnce for PaginationEllipsis {
    fn render(self, _window: &mut Window, _cx: &mut App) -> impl IntoElement {
        let mut root = div()
            .flex()
            .size(px(36.))
            .items_center()
            .justify_center()
            .child(Icon::new(ICON_ELLIPSIS));
        root.style().refine(&self.style);
        root
    }
}
