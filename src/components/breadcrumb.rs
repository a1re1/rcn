//! Breadcrumb — port of shadcn base-vega `ui/breadcrumb.tsx`.
//!
//! Path navigation: List of Items with Links (hover to foreground),
//! the current Page, chevron Separators (following the active icon
//! library), and an Ellipsis for collapsed middles.
//!
//! Sizing and shape overrides come from the caller via [`Styled`].

use gpui::{
    AnyElement, App, ClickEvent, ElementId, InteractiveElement as _, IntoElement, ParentElement,
    Refineable as _, RenderOnce, StatefulInteractiveElement as _, StyleRefinement, Styled, Window,
    div, prelude::FluentBuilder as _, px, svg,
};

use crate::assets::ICON_ELLIPSIS;
use crate::motion;
use crate::theme::Theme;

type ClickHandler = Box<dyn Fn(&ClickEvent, &mut Window, &mut App) + 'static>;

/// nav wrapper.
#[derive(IntoElement)]
pub struct Breadcrumb {
    children: Vec<AnyElement>,
    style: StyleRefinement,
}

impl Breadcrumb {
    pub fn new() -> Self {
        Self {
            children: Vec::new(),
            style: StyleRefinement::default(),
        }
    }
}

impl Default for Breadcrumb {
    fn default() -> Self {
        Self::new()
    }
}

impl ParentElement for Breadcrumb {
    fn extend(&mut self, elements: impl IntoIterator<Item = AnyElement>) {
        self.children.extend(elements);
    }
}

impl Styled for Breadcrumb {
    fn style(&mut self) -> &mut StyleRefinement {
        &mut self.style
    }
}

impl RenderOnce for Breadcrumb {
    fn render(self, _window: &mut Window, _cx: &mut App) -> impl IntoElement {
        let mut root = div().children(self.children);
        root.style().refine(&self.style);
        root
    }
}

/// ol: flex flex-wrap items-center gap-2.5 text-sm text-muted-foreground.
#[derive(IntoElement)]
pub struct BreadcrumbList {
    children: Vec<AnyElement>,
    style: StyleRefinement,
}

impl BreadcrumbList {
    pub fn new() -> Self {
        Self {
            children: Vec::new(),
            style: StyleRefinement::default(),
        }
    }
}

impl Default for BreadcrumbList {
    fn default() -> Self {
        Self::new()
    }
}

impl ParentElement for BreadcrumbList {
    fn extend(&mut self, elements: impl IntoIterator<Item = AnyElement>) {
        self.children.extend(elements);
    }
}

impl Styled for BreadcrumbList {
    fn style(&mut self) -> &mut StyleRefinement {
        &mut self.style
    }
}

impl RenderOnce for BreadcrumbList {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let theme = Theme::of(cx);
        let mut root = div()
            .flex()
            .flex_row()
            .flex_wrap()
            .items_center()
            .gap(px(10.))
            .text_size(px(14.))
            .line_height(px(20.))
            .text_color(theme.muted_foreground)
            .children(self.children);
        root.style().refine(&self.style);
        root
    }
}

/// li: inline-flex items-center gap-1.5.
#[derive(IntoElement)]
pub struct BreadcrumbItem {
    children: Vec<AnyElement>,
    style: StyleRefinement,
}

impl BreadcrumbItem {
    pub fn new() -> Self {
        Self {
            children: Vec::new(),
            style: StyleRefinement::default(),
        }
    }
}

impl Default for BreadcrumbItem {
    fn default() -> Self {
        Self::new()
    }
}

impl ParentElement for BreadcrumbItem {
    fn extend(&mut self, elements: impl IntoIterator<Item = AnyElement>) {
        self.children.extend(elements);
    }
}

impl Styled for BreadcrumbItem {
    fn style(&mut self) -> &mut StyleRefinement {
        &mut self.style
    }
}

impl RenderOnce for BreadcrumbItem {
    fn render(self, _window: &mut Window, _cx: &mut App) -> impl IntoElement {
        let mut root = div()
            .flex()
            .flex_row()
            .items_center()
            .gap(px(6.))
            .children(self.children);
        root.style().refine(&self.style);
        root
    }
}

/// a: transition-colors hover:text-foreground.
#[derive(IntoElement)]
pub struct BreadcrumbLink {
    id: ElementId,
    on_click: Option<ClickHandler>,
    children: Vec<AnyElement>,
    style: StyleRefinement,
}

impl BreadcrumbLink {
    pub fn new(id: impl Into<ElementId>) -> Self {
        Self {
            id: id.into(),
            on_click: None,
            children: Vec::new(),
            style: StyleRefinement::default(),
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

impl ParentElement for BreadcrumbLink {
    fn extend(&mut self, elements: impl IntoIterator<Item = AnyElement>) {
        self.children.extend(elements);
    }
}

impl Styled for BreadcrumbLink {
    fn style(&mut self) -> &mut StyleRefinement {
        &mut self.style
    }
}

impl RenderOnce for BreadcrumbLink {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let theme = Theme::of(cx).clone();
        let ring = motion::focus_ring(&theme);
        let mut root = div()
            .id(self.id)
            .tab_index(0)
            .focus_visible(move |s| s.border_color(theme.ring).shadow(ring.clone()))
            .hover(move |s| s.text_color(theme.foreground))
            .when_some(self.on_click, |el, on_click| el.on_click(on_click))
            .children(self.children);
        root.style().refine(&self.style);
        root
    }
}

/// span: font-normal text-foreground — the current page.
#[derive(IntoElement)]
pub struct BreadcrumbPage {
    children: Vec<AnyElement>,
    style: StyleRefinement,
}

impl BreadcrumbPage {
    pub fn new() -> Self {
        Self {
            children: Vec::new(),
            style: StyleRefinement::default(),
        }
    }
}

impl Default for BreadcrumbPage {
    fn default() -> Self {
        Self::new()
    }
}

impl ParentElement for BreadcrumbPage {
    fn extend(&mut self, elements: impl IntoIterator<Item = AnyElement>) {
        self.children.extend(elements);
    }
}

impl Styled for BreadcrumbPage {
    fn style(&mut self) -> &mut StyleRefinement {
        &mut self.style
    }
}

impl RenderOnce for BreadcrumbPage {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let theme = Theme::of(cx);
        let mut root = div().text_color(theme.foreground).children(self.children);
        root.style().refine(&self.style);
        root
    }
}

/// li: the chevron between items ([&>svg]:size-3.5).
#[derive(IntoElement)]
pub struct BreadcrumbSeparator {
    style: StyleRefinement,
}

impl BreadcrumbSeparator {
    pub fn new() -> Self {
        Self {
            style: StyleRefinement::default(),
        }
    }
}

impl Default for BreadcrumbSeparator {
    fn default() -> Self {
        Self::new()
    }
}

impl Styled for BreadcrumbSeparator {
    fn style(&mut self) -> &mut StyleRefinement {
        &mut self.style
    }
}

impl RenderOnce for BreadcrumbSeparator {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let theme = Theme::of(cx);
        let mut root = svg()
            .path(theme.icons.chevron_right())
            .size(px(14.))
            .flex_shrink_0()
            .text_color(theme.muted_foreground);
        root.style().refine(&self.style);
        root
    }
}

/// span: flex size-5 items-center justify-center — collapsed middle.
#[derive(IntoElement)]
pub struct BreadcrumbEllipsis {
    style: StyleRefinement,
}

impl BreadcrumbEllipsis {
    pub fn new() -> Self {
        Self {
            style: StyleRefinement::default(),
        }
    }
}

impl Default for BreadcrumbEllipsis {
    fn default() -> Self {
        Self::new()
    }
}

impl Styled for BreadcrumbEllipsis {
    fn style(&mut self) -> &mut StyleRefinement {
        &mut self.style
    }
}

impl RenderOnce for BreadcrumbEllipsis {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let theme = Theme::of(cx);
        let mut root = div()
            .flex()
            .size(px(20.))
            .items_center()
            .justify_center()
            .child(
                svg()
                    .path(ICON_ELLIPSIS)
                    .size(px(16.))
                    .text_color(theme.muted_foreground),
            );
        root.style().refine(&self.style);
        root
    }
}
