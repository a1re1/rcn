//! Sidebar — port of shadcn base-vega `ui/sidebar.tsx` (core subset).
//!
//! The app-shell sidebar: provider row (rail + inset content), header/
//! content/footer stack, labeled groups, and menu buttons with active
//! state. Controlled collapse via `open` + a trigger button. The
//! dedicated `--sidebar` token family, icon-rail collapse mode, mobile
//! sheet mode, and submenu machinery are omitted (backgrounds approximate
//! with the base tokens).

use gpui::{
    AnyElement, App, ClickEvent, ElementId, FontWeight, InteractiveElement as _, IntoElement,
    ParentElement, RenderOnce, StatefulInteractiveElement as _, Styled, Window, div,
    prelude::FluentBuilder as _, px, svg,
};

use crate::motion;
use crate::theme::Theme;

/// The shell: sidebar rail + inset main content, filling its container.
#[derive(IntoElement)]
pub struct SidebarProvider {
    open: bool,
    sidebar: Option<AnyElement>,
    inset: Option<AnyElement>,
}

impl SidebarProvider {
    pub fn new() -> Self {
        Self {
            open: true,
            sidebar: None,
            inset: None,
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

impl RenderOnce for SidebarProvider {
    fn render(self, _window: &mut Window, _cx: &mut App) -> impl IntoElement {
        div()
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
            )
    }
}

/// The rail: w-64 full-height column on a muted background.
#[derive(IntoElement)]
pub struct Sidebar {
    children: Vec<AnyElement>,
}

impl Sidebar {
    pub fn new() -> Self {
        Self {
            children: Vec::new(),
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

impl RenderOnce for Sidebar {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let theme = Theme::of(cx);
        div()
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
            .children(self.children)
    }
}

/// p-2 stack pinned to the top of the rail.
#[derive(IntoElement)]
pub struct SidebarHeader {
    children: Vec<AnyElement>,
}

impl SidebarHeader {
    pub fn new() -> Self {
        Self {
            children: Vec::new(),
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

impl RenderOnce for SidebarHeader {
    fn render(self, _window: &mut Window, _cx: &mut App) -> impl IntoElement {
        div()
            .flex()
            .flex_col()
            .gap(px(8.))
            .p(px(8.))
            .children(self.children)
    }
}

/// flex-1 scrollable middle of the rail.
#[derive(IntoElement)]
pub struct SidebarContent {
    children: Vec<AnyElement>,
}

impl SidebarContent {
    pub fn new() -> Self {
        Self {
            children: Vec::new(),
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

impl RenderOnce for SidebarContent {
    fn render(self, _window: &mut Window, _cx: &mut App) -> impl IntoElement {
        div()
            .id("sidebar-content")
            .flex()
            .flex_col()
            .flex_1()
            .min_h(px(0.))
            .gap(px(8.))
            .overflow_y_scroll()
            .children(self.children)
    }
}

/// p-2 stack pinned to the bottom of the rail.
#[derive(IntoElement)]
pub struct SidebarFooter {
    children: Vec<AnyElement>,
}

impl SidebarFooter {
    pub fn new() -> Self {
        Self {
            children: Vec::new(),
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

impl RenderOnce for SidebarFooter {
    fn render(self, _window: &mut Window, _cx: &mut App) -> impl IntoElement {
        div()
            .flex()
            .flex_col()
            .gap(px(8.))
            .p(px(8.))
            .children(self.children)
    }
}

/// A labeled section of menu items.
#[derive(IntoElement)]
pub struct SidebarGroup {
    label: Option<gpui::SharedString>,
    children: Vec<AnyElement>,
}

impl SidebarGroup {
    pub fn new() -> Self {
        Self {
            label: None,
            children: Vec::new(),
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

impl RenderOnce for SidebarGroup {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let theme = Theme::of(cx);
        div()
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
            .children(self.children)
    }
}

/// One menu row: hover accent, active = accent bg + medium weight.
#[derive(IntoElement)]
pub struct SidebarMenuButton {
    id: ElementId,
    active: bool,
    on_click: Option<Box<dyn Fn(&ClickEvent, &mut Window, &mut App) + 'static>>,
    children: Vec<AnyElement>,
}

impl SidebarMenuButton {
    pub fn new(id: impl Into<ElementId>) -> Self {
        Self {
            id: id.into(),
            active: false,
            on_click: None,
            children: Vec::new(),
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

impl RenderOnce for SidebarMenuButton {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let theme = Theme::of(cx).clone();
        div()
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
            .children(self.children)
    }
}

/// The collapse/expand trigger (panel icon button).
#[derive(IntoElement)]
pub struct SidebarTrigger {
    id: ElementId,
    on_click: Option<Box<dyn Fn(&ClickEvent, &mut Window, &mut App) + 'static>>,
}

impl SidebarTrigger {
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

impl RenderOnce for SidebarTrigger {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let theme = Theme::of(cx).clone();
        div()
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
            )
    }
}
