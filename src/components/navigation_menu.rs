//! NavigationMenu — port of shadcn base-vega `ui/navigation-menu.tsx`.
//!
//! A horizontal set of triggers, each opening a free-form content panel
//! below (site-nav style). Controlled: the caller owns which index is
//! open. The shared viewport animation and indicator arrow are omitted.
//! `NavigationMenuLink` is the styled row used inside panels.
//! Sizing and shape overrides come from the caller via [`Styled`].

use std::rc::Rc;

use gpui::{
    AnyElement, App, ClickEvent, ElementId, FontWeight, InteractiveElement as _, IntoElement,
    ParentElement, Refineable as _, RenderOnce, StatefulInteractiveElement as _, StyleRefinement,
    Styled, Window, anchored, deferred, div, prelude::FluentBuilder as _, px, relative, svg,
};

use crate::theme::{Theme, alpha};

type OpenChangeHandler = Rc<dyn Fn(&Option<usize>, &mut Window, &mut App) + 'static>;

/// One trigger + its panel content.
pub struct NavigationMenuEntry {
    label: gpui::SharedString,
    content: Option<AnyElement>,
}

impl NavigationMenuEntry {
    pub fn new(label: impl Into<gpui::SharedString>) -> Self {
        Self {
            label: label.into(),
            content: None,
        }
    }

    /// The panel shown while this entry is open.
    pub fn content(mut self, content: impl IntoElement) -> Self {
        self.content = Some(content.into_any_element());
        self
    }
}

/// Horizontal navigation menu bar. Sizing and shape overrides come from the
/// caller via [`Styled`].
#[derive(IntoElement)]
pub struct NavigationMenu {
    id: ElementId,
    open: Option<usize>,
    entries: Vec<NavigationMenuEntry>,
    on_open_change: Option<OpenChangeHandler>,
    style: StyleRefinement,
}

impl NavigationMenu {
    pub fn new(id: impl Into<ElementId>) -> Self {
        Self {
            id: id.into(),
            open: None,
            entries: Vec::new(),
            on_open_change: None,
            style: StyleRefinement::default(),
        }
    }

    pub fn open(mut self, open: Option<usize>) -> Self {
        self.open = open;
        self
    }

    pub fn entry(mut self, entry: NavigationMenuEntry) -> Self {
        self.entries.push(entry);
        self
    }

    pub fn on_open_change(
        mut self,
        handler: impl Fn(&Option<usize>, &mut Window, &mut App) + 'static,
    ) -> Self {
        self.on_open_change = Some(Rc::new(handler));
        self
    }
}

impl Styled for NavigationMenu {
    fn style(&mut self) -> &mut StyleRefinement {
        &mut self.style
    }
}

impl RenderOnce for NavigationMenu {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let theme = Theme::of(cx).clone();
        let open = self.open;
        let on_open_change = self.on_open_change;

        let mut root = div()
            .id(self.id)
            .flex()
            .flex_row()
            .items_center()
            .gap(px(4.));
        root.style().refine(&self.style);
        root.children(self.entries.into_iter().enumerate().map(|(index, entry)| {
            let is_open = open == Some(index);
            let has_content = entry.content.is_some();
            let toggle = on_open_change.clone();
            let close_out = on_open_change.clone();
            let panel = (is_open && has_content).then(|| {
                div()
                    .occlude()
                    .rounded(theme.radius_md())
                    .bg(theme.popover)
                    .text_color(theme.popover_foreground)
                    .p(px(8.))
                    .shadow_md()
                    .border_1()
                    .border_color(alpha(theme.foreground, 0.1))
                    .when_some(close_out, |el, close| {
                        el.on_mouse_down_out(move |_, window, cx| close(&None, window, cx))
                    })
                    .children(entry.content)
                    .into_any_element()
            });

            // Trigger: h-9 rounded-md px-4 py-2 text-sm font-medium,
            // hover/open: bg-accent text-accent-foreground, with a
            // chevron when it has a panel.
            div()
                .relative()
                .child(
                    div()
                        .id(("nav-menu-trigger", index))
                        .flex()
                        .flex_row()
                        .items_center()
                        .gap(px(4.))
                        .h(px(36.))
                        .rounded(theme.radius_md())
                        .px(px(16.))
                        .py(px(8.))
                        .text_size(px(14.))
                        .line_height(px(20.))
                        .font_weight(FontWeight::MEDIUM)
                        .when(is_open, |el| {
                            el.bg(theme.accent).text_color(theme.accent_foreground)
                        })
                        .when(!is_open, |el| {
                            el.hover(|s| s.bg(theme.accent).text_color(theme.accent_foreground))
                        })
                        .when_some(toggle, |el, toggle| {
                            el.on_click(move |_, window, cx| {
                                let next = if is_open { None } else { Some(index) };
                                toggle(&next, window, cx)
                            })
                        })
                        .child(entry.label)
                        .when(has_content, |el| {
                            el.child(
                                svg()
                                    .path(if is_open {
                                        theme.icons.chevron_up()
                                    } else {
                                        theme.icons.chevron_down()
                                    })
                                    .size(px(12.))
                                    .text_color(theme.muted_foreground),
                            )
                        }),
                )
                .when_some(panel, |el, panel| {
                    el.child(
                        div()
                            .absolute()
                            .left_0()
                            .top(relative(1.))
                            .pt(px(6.))
                            .child(deferred(
                                anchored().snap_to_window_with_margin(px(8.)).child(
                                    crate::motion::pop_in("navmenu-in", gpui::div().child(panel)),
                                ),
                            )),
                    )
                })
        }))
    }
}

/// A styled link row for panel content: block rounded-sm p-2, title +
/// optional muted description, hover accent.
/// Sizing and shape overrides come from the caller via [`Styled`].
#[derive(IntoElement)]
pub struct NavigationMenuLink {
    id: ElementId,
    title: gpui::SharedString,
    description: Option<gpui::SharedString>,
    on_click: Option<Box<dyn Fn(&ClickEvent, &mut Window, &mut App) + 'static>>,
    style: StyleRefinement,
}

impl NavigationMenuLink {
    pub fn new(id: impl Into<ElementId>, title: impl Into<gpui::SharedString>) -> Self {
        Self {
            id: id.into(),
            title: title.into(),
            description: None,
            on_click: None,
            style: StyleRefinement::default(),
        }
    }

    pub fn description(mut self, description: impl Into<gpui::SharedString>) -> Self {
        self.description = Some(description.into());
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

impl Styled for NavigationMenuLink {
    fn style(&mut self) -> &mut StyleRefinement {
        &mut self.style
    }
}

impl RenderOnce for NavigationMenuLink {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let theme = Theme::of(cx).clone();
        let mut root = div()
            .id(self.id)
            .flex()
            .flex_col()
            .gap(px(2.))
            .rounded(theme.radius_sm())
            .p(px(8.))
            .hover(|s| s.bg(theme.accent).text_color(theme.accent_foreground))
            .when_some(self.on_click, |el, on_click| el.on_click(on_click));
        root.style().refine(&self.style);
        root.child(
            div()
                .text_size(px(14.))
                .line_height(px(20.))
                .font_weight(FontWeight::MEDIUM)
                .child(self.title),
        )
        .when_some(self.description, |el, description| {
            el.child(
                div()
                    .text_size(px(13.))
                    .line_height(px(18.))
                    .text_color(theme.muted_foreground)
                    .child(description),
            )
        })
    }
}
