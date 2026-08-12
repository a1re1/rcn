//! Sidebar — port of shadcn base-vega `ui/sidebar.tsx` (core subset).
//!
//! The app-shell sidebar: provider row (rail + inset content), header/
//! content/footer stack, labeled groups, and menu buttons with active
//! state. Controlled collapse via `open` + a trigger button. The
//! dedicated `--sidebar` token family, icon-rail collapse mode, mobile
//! sheet mode, and submenu machinery are omitted (backgrounds approximate
//! with the base tokens). Sizing and shape overrides come from the caller
//! via [`Styled`].

use gpui::{
    AnyElement, App, ClickEvent, ElementId, FontWeight, InteractiveElement as _, IntoElement,
    ParentElement, Refineable as _, RenderOnce, StatefulInteractiveElement as _, StyleRefinement,
    Styled, Window, div, prelude::FluentBuilder as _, px, svg,
};

use crate::motion;
use crate::theme::Theme;

/// The shell: sidebar rail + inset main content, filling its container.
/// Sizing and shape overrides come from the caller via [`Styled`].
#[derive(IntoElement)]
pub struct SidebarProvider {
    open: bool,
    sidebar: Option<AnyElement>,
    inset: Option<AnyElement>,
    style: StyleRefinement,
}

impl SidebarProvider {
    pub fn new() -> Self {
        Self {
            open: true,
            sidebar: None,
            inset: None,
            style: StyleRefinement::default(),
        }
    }

    /// Whether the sidebar rail is visible.
    pub fn open(mut self, open: bool) -> Self {
        self.open = open;
        self
    }

    pub fn sidebar(mut self, sidebar: impl IntoElement) -> Self {
        self.sidebar = Some(sidebar.into_any_element());
        self
    }

    pub fn inset(mut self, inset: impl IntoElement) -> Self {
        self.inset = Some(inset.into_any_element());
        self
    }
}

impl Default for SidebarProvider {
    fn default() -> Self {
        Self::new()
    }
}

impl Styled for SidebarProvider {
    fn style(&mut self) -> &mut StyleRefinement {
        &mut self.style
    }
}

impl RenderOnce for SidebarProvider {
    fn render(self, _window: &mut Window, _cx: &mut App) -> impl IntoElement {
        let mut root = div()
            .flex()
            .flex_row()
            .size_full()
            .when(self.open, |el| el.children(self.sidebar))
            .child(
                div()
                    .flex()
                    .flex_col()
                    .flex_1()
                    .min_w(px(0.))
                    .children(self.inset),
            );
        root.style().refine(&self.style);
        root
    }
}

/// The rail: w-64 full-height column on a muted background.
/// Sizing and shape overrides come from the caller via [`Styled`].
#[derive(IntoElement)]
pub struct Sidebar {
    children: Vec<AnyElement>,
    style: StyleRefinement,
}

impl Sidebar {
    pub fn new() -> Self {
        Self {
            children: Vec::new(),
            style: StyleRefinement::default(),
        }
    }
}

impl Default for Sidebar {
    fn default() -> Self {
        Self::new()
    }
}

impl ParentElement for Sidebar {
    fn extend(&mut self, elements: impl IntoIterator<Item = AnyElement>) {
        self.children.extend(elements);
    }
}

impl Styled for Sidebar {
    fn style(&mut self) -> &mut StyleRefinement {
        &mut self.style
    }
}

impl RenderOnce for Sidebar {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let theme = Theme::of(cx);
        let mut root = div()
            .flex()
            .flex_col()
            .flex_shrink_0()
            .w(px(256.))
            .h_full()
            .border_r_1()
            .border_color(theme.border)
            .bg(if theme.dark {
                theme.card
            } else {
                theme.secondary
            })
            .children(self.children);
        root.style().refine(&self.style);
        root
    }
}

/// p-2 stack pinned to the top of the rail.
/// Sizing and shape overrides come from the caller via [`Styled`].
#[derive(IntoElement)]
pub struct SidebarHeader {
    children: Vec<AnyElement>,
    style: StyleRefinement,
}

impl SidebarHeader {
    pub fn new() -> Self {
        Self {
            children: Vec::new(),
            style: StyleRefinement::default(),
        }
    }
}

impl Default for SidebarHeader {
    fn default() -> Self {
        Self::new()
    }
}

impl ParentElement for SidebarHeader {
    fn extend(&mut self, elements: impl IntoIterator<Item = AnyElement>) {
        self.children.extend(elements);
    }
}

impl Styled for SidebarHeader {
    fn style(&mut self) -> &mut StyleRefinement {
        &mut self.style
    }
}

impl RenderOnce for SidebarHeader {
    fn render(self, _window: &mut Window, _cx: &mut App) -> impl IntoElement {
        let mut root = div()
            .flex()
            .flex_col()
            .gap(px(8.))
            .p(px(8.))
            .children(self.children);
        root.style().refine(&self.style);
        root
    }
}

/// flex-1 scrollable middle of the rail.
/// Sizing and shape overrides come from the caller via [`Styled`].
#[derive(IntoElement)]
pub struct SidebarContent {
    children: Vec<AnyElement>,
    style: StyleRefinement,
}

impl SidebarContent {
    pub fn new() -> Self {
        Self {
            children: Vec::new(),
            style: StyleRefinement::default(),
        }
    }
}

impl Default for SidebarContent {
    fn default() -> Self {
        Self::new()
    }
}

impl ParentElement for SidebarContent {
    fn extend(&mut self, elements: impl IntoIterator<Item = AnyElement>) {
        self.children.extend(elements);
    }
}

impl Styled for SidebarContent {
    fn style(&mut self) -> &mut StyleRefinement {
        &mut self.style
    }
}

impl RenderOnce for SidebarContent {
    fn render(self, _window: &mut Window, _cx: &mut App) -> impl IntoElement {
        let mut root = div()
            .id("sidebar-content")
            .flex()
            .flex_col()
            .flex_1()
            .min_h(px(0.))
            .gap(px(8.))
            .overflow_y_scroll()
            .children(self.children);
        root.style().refine(&self.style);
        root
    }
}

/// p-2 stack pinned to the bottom of the rail.
/// Sizing and shape overrides come from the caller via [`Styled`].
#[derive(IntoElement)]
pub struct SidebarFooter {
    children: Vec<AnyElement>,
    style: StyleRefinement,
}

impl SidebarFooter {
    pub fn new() -> Self {
        Self {
            children: Vec::new(),
            style: StyleRefinement::default(),
        }
    }
}

impl Default for SidebarFooter {
    fn default() -> Self {
        Self::new()
    }
}

impl ParentElement for SidebarFooter {
    fn extend(&mut self, elements: impl IntoIterator<Item = AnyElement>) {
        self.children.extend(elements);
    }
}

impl Styled for SidebarFooter {
    fn style(&mut self) -> &mut StyleRefinement {
        &mut self.style
    }
}

impl RenderOnce for SidebarFooter {
    fn render(self, _window: &mut Window, _cx: &mut App) -> impl IntoElement {
        let mut root = div()
            .flex()
            .flex_col()
            .gap(px(8.))
            .p(px(8.))
            .children(self.children);
        root.style().refine(&self.style);
        root
    }
}

/// A labeled section of menu items.
/// Sizing and shape overrides come from the caller via [`Styled`].
#[derive(IntoElement)]
pub struct SidebarGroup {
    label: Option<gpui::SharedString>,
    children: Vec<AnyElement>,
    style: StyleRefinement,
}

impl SidebarGroup {
    pub fn new() -> Self {
        Self {
            label: None,
            children: Vec::new(),
            style: StyleRefinement::default(),
        }
    }

    pub fn label(mut self, label: impl Into<gpui::SharedString>) -> Self {
        self.label = Some(label.into());
        self
    }
}

impl Default for SidebarGroup {
    fn default() -> Self {
        Self::new()
    }
}

impl ParentElement for SidebarGroup {
    fn extend(&mut self, elements: impl IntoIterator<Item = AnyElement>) {
        self.children.extend(elements);
    }
}

impl Styled for SidebarGroup {
    fn style(&mut self) -> &mut StyleRefinement {
        &mut self.style
    }
}

impl RenderOnce for SidebarGroup {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let theme = Theme::of(cx);
        let mut root = div()
            .flex()
            .flex_col()
            .gap(px(2.))
            .p(px(8.))
            .when_some(self.label, |el, label| {
                el.child(
                    div()
                        .px(px(8.))
                        .py(px(4.))
                        .text_size(px(12.))
                        .line_height(px(16.))
                        .font_weight(FontWeight::MEDIUM)
                        .text_color(theme.muted_foreground)
                        .child(label),
                )
            })
            .children(self.children);
        root.style().refine(&self.style);
        root
    }
}

/// One menu row: hover accent, active = accent bg + medium weight.
/// Sizing and shape overrides come from the caller via [`Styled`].
#[derive(IntoElement)]
pub struct SidebarMenuButton {
    id: ElementId,
    active: bool,
    on_click: Option<Box<dyn Fn(&ClickEvent, &mut Window, &mut App) + 'static>>,
    children: Vec<AnyElement>,
    style: StyleRefinement,
}

impl SidebarMenuButton {
    pub fn new(id: impl Into<ElementId>) -> Self {
        Self {
            id: id.into(),
            active: false,
            on_click: None,
            children: Vec::new(),
            style: StyleRefinement::default(),
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

impl ParentElement for SidebarMenuButton {
    fn extend(&mut self, elements: impl IntoIterator<Item = AnyElement>) {
        self.children.extend(elements);
    }
}

impl Styled for SidebarMenuButton {
    fn style(&mut self) -> &mut StyleRefinement {
        &mut self.style
    }
}

impl RenderOnce for SidebarMenuButton {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let theme = Theme::of(cx).clone();
        let mut root = div()
            .id(self.id)
            .flex()
            .flex_row()
            .items_center()
            .gap(px(8.))
            .rounded(theme.radius_md())
            .px(px(8.))
            .py(px(6.))
            .text_size(px(14.))
            .line_height(px(20.))
            .text_color(theme.foreground)
            .when(self.active, |el| {
                el.bg(theme.accent)
                    .text_color(theme.accent_foreground)
                    .font_weight(FontWeight::MEDIUM)
            })
            .map(|el| {
                let ring = motion::focus_ring(&theme);
                el.tab_index(0)
                    .focus_visible(move |s| s.border_color(theme.ring).shadow(ring.clone()))
            })
            .when(!self.active, |el| {
                el.hover(|s| s.bg(theme.accent).text_color(theme.accent_foreground))
            })
            .when_some(self.on_click, |el, on_click| el.on_click(on_click))
            .children(self.children);
        root.style().refine(&self.style);
        root
    }
}

/// The collapse/expand trigger (panel icon button).
/// Sizing and shape overrides come from the caller via [`Styled`].
#[derive(IntoElement)]
pub struct SidebarTrigger {
    id: ElementId,
    on_click: Option<Box<dyn Fn(&ClickEvent, &mut Window, &mut App) + 'static>>,
    style: StyleRefinement,
}

impl SidebarTrigger {
    pub fn new(id: impl Into<ElementId>) -> Self {
        Self {
            id: id.into(),
            on_click: None,
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

impl Styled for SidebarTrigger {
    fn style(&mut self) -> &mut StyleRefinement {
        &mut self.style
    }
}

impl RenderOnce for SidebarTrigger {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let theme = Theme::of(cx).clone();
        let mut root = div()
            .id(self.id)
            .flex()
            .size(px(28.))
            .items_center()
            .justify_center()
            .rounded(theme.radius_md())
            .hover(|s| s.bg(theme.muted))
            .when_some(self.on_click, |el, on_click| el.on_click(on_click))
            .child(
                svg()
                    .path(theme.icons.chevron_left())
                    .size(px(16.))
                    .text_color(theme.muted_foreground),
            );
        root.style().refine(&self.style);
        root
    }
}
