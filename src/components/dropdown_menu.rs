//! DropdownMenu — port of shadcn base-vega `ui/dropdown-menu.tsx`.
//!
//! A menu panel opened from a trigger, positioned below it (Popover
//! pattern): outside clicks close it, item clicks fire `on_select` and
//! close. Items are typed so the panel can style hover/destructive/
//! disabled states. Submenus and typeahead are omitted.

use std::rc::Rc;

use gpui::{
    AnyElement, App, ClickEvent, ElementId, FontWeight, InteractiveElement as _, IntoElement,
    ParentElement as _, RenderOnce, StatefulInteractiveElement as _, Styled, Window, anchored,
    deferred, div, prelude::FluentBuilder as _, px, relative, svg,
};

use crate::theme::{Theme, alpha};

pub type OpenChangeHandler = Rc<dyn Fn(&bool, &mut Window, &mut App) + 'static>;
type SelectHandler = Rc<dyn Fn(&ClickEvent, &mut Window, &mut App) + 'static>;

/// One entry in the menu.
pub enum DropdownMenuEntry {
    Item(DropdownMenuItem),
    Label(AnyElement),
    Separator,
}

pub struct DropdownMenuItem {
    id: ElementId,
    destructive: bool,
    disabled: bool,
    checked: Option<bool>,
    shortcut: Option<gpui::SharedString>,
    on_select: Option<SelectHandler>,
    children: Vec<AnyElement>,
}

impl DropdownMenuItem {
    pub fn new(id: impl Into<ElementId>) -> Self {
        Self {
            id: id.into(),
            destructive: false,
            disabled: false,
            checked: None,
            shortcut: None,
            on_select: None,
            children: Vec::new(),
        }
    }

    /// The source's `variant="destructive"`.
    pub fn destructive(mut self, destructive: bool) -> Self {
        self.destructive = destructive;
        self
    }

    pub fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }

    /// Renders as a checkbox item with a leading check slot.
    pub fn checked(mut self, checked: bool) -> Self {
        self.checked = Some(checked);
        self
    }

    /// Right-aligned muted shortcut text (`DropdownMenuShortcut`).
    pub fn shortcut(mut self, shortcut: impl Into<gpui::SharedString>) -> Self {
        self.shortcut = Some(shortcut.into());
        self
    }

    pub fn on_select(
        mut self,
        handler: impl Fn(&ClickEvent, &mut Window, &mut App) + 'static,
    ) -> Self {
        self.on_select = Some(Rc::new(handler));
        self
    }

    pub fn child(mut self, child: impl IntoElement) -> Self {
        self.children.push(child.into_any_element());
        self
    }
}

#[derive(IntoElement)]
pub struct DropdownMenu {
    id: ElementId,
    open: bool,
    trigger: Option<AnyElement>,
    entries: Vec<DropdownMenuEntry>,
    on_open_change: Option<OpenChangeHandler>,
}

impl DropdownMenu {
    pub fn new(id: impl Into<ElementId>) -> Self {
        Self {
            id: id.into(),
            open: false,
            trigger: None,
            entries: Vec::new(),
            on_open_change: None,
        }
    }

    pub fn open(mut self, open: bool) -> Self {
        self.open = open;
        self
    }

    pub fn trigger(mut self, trigger: impl IntoElement) -> Self {
        self.trigger = Some(trigger.into_any_element());
        self
    }

    pub fn item(mut self, item: DropdownMenuItem) -> Self {
        self.entries.push(DropdownMenuEntry::Item(item));
        self
    }

    /// px-2 py-1.5 text-sm font-medium — a non-interactive heading.
    pub fn label(mut self, label: impl IntoElement) -> Self {
        self.entries
            .push(DropdownMenuEntry::Label(label.into_any_element()));
        self
    }

    pub fn separator(mut self) -> Self {
        self.entries.push(DropdownMenuEntry::Separator);
        self
    }

    pub fn on_open_change(
        mut self,
        handler: impl Fn(&bool, &mut Window, &mut App) + 'static,
    ) -> Self {
        self.on_open_change = Some(Rc::new(handler));
        self
    }
}

/// Renders the shared menu panel; also used by context-menu and menubar.
pub(crate) fn menu_panel(
    entries: Vec<DropdownMenuEntry>,
    on_open_change: Option<OpenChangeHandler>,
    cx: &App,
) -> impl IntoElement + use<> {
    let theme = Theme::of(cx).clone();
    let close_on_out = on_open_change.clone();
    div()
        .occlude()
        .flex()
        .flex_col()
        .min_w(px(128.))
        .max_h(px(400.))
        .rounded(theme.radius_md())
        .bg(theme.popover)
        .text_color(theme.popover_foreground)
        .p(px(4.))
        .shadow_md()
        .border_1()
        .border_color(alpha(theme.foreground, 0.1))
        .when_some(close_on_out, |el, close| {
            el.on_mouse_down_out(move |_, window, cx| close(&false, window, cx))
        })
        .children(entries.into_iter().map(|entry| {
            match entry {
                DropdownMenuEntry::Separator => div()
                    .my(px(4.))
                    .mx(px(-4.))
                    .h(px(1.))
                    .bg(theme.border)
                    .into_any_element(),
                DropdownMenuEntry::Label(label) => div()
                    .px(px(8.))
                    .py(px(6.))
                    .text_size(px(14.))
                    .line_height(px(20.))
                    .font_weight(FontWeight::MEDIUM)
                    .child(label)
                    .into_any_element(),
                DropdownMenuEntry::Item(item) => {
                    let close = on_open_change.clone();
                    let text_color = if item.destructive {
                        theme.destructive
                    } else {
                        theme.popover_foreground
                    };
                    let hover_bg = if item.destructive {
                        alpha(theme.destructive, 0.1)
                    } else {
                        theme.accent
                    };
                    let hover_text = if item.destructive {
                        theme.destructive
                    } else {
                        theme.accent_foreground
                    };
                    div()
                        .id(item.id)
                        .flex()
                        .flex_row()
                        .items_center()
                        .gap(px(8.))
                        .rounded(theme.radius_sm())
                        .px(px(8.))
                        .py(px(6.))
                        .text_size(px(14.))
                        .line_height(px(20.))
                        .text_color(text_color)
                        .whitespace_nowrap()
                        .when(item.disabled, |el| el.opacity(0.5))
                        .when(!item.disabled, |el| {
                            el.hover(move |s| s.bg(hover_bg).text_color(hover_text))
                                .on_click(move |event, window, cx| {
                                    if let Some(on_select) = &item.on_select {
                                        on_select(event, window, cx);
                                    }
                                    if let Some(close) = &close {
                                        close(&false, window, cx);
                                    }
                                })
                        })
                        .when_some(item.checked, |el, checked| {
                            el.child(div().w(px(14.)).flex_shrink_0().when(checked, |el| {
                                el.child(
                                    svg()
                                        .path(theme.icons.check())
                                        .size(px(14.))
                                        .text_color(text_color),
                                )
                            }))
                        })
                        .children(item.children)
                        .when_some(item.shortcut, |el, shortcut| {
                            el.child(
                                div()
                                    .ml_auto()
                                    .pl(px(16.))
                                    .text_size(px(12.))
                                    .text_color(theme.muted_foreground)
                                    .child(shortcut),
                            )
                        })
                        .into_any_element()
                }
            }
        }))
}

impl RenderOnce for DropdownMenu {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let open = self.open;
        let toggle = self.on_open_change.clone();
        let panel = if open {
            Some(menu_panel(self.entries, self.on_open_change, cx).into_any_element())
        } else {
            None
        };

        div()
            .relative()
            .flex()
            .flex_col()
            .items_start()
            .child(
                div()
                    .id(self.id)
                    .when_some(toggle, |el, toggle| {
                        el.on_click(move |_, window, cx| toggle(&!open, window, cx))
                    })
                    .children(self.trigger),
            )
            .when_some(panel, |el, panel| {
                el.child(
                    div()
                        .absolute()
                        .left_0()
                        .top(relative(1.))
                        .pt(px(4.))
                        .child(deferred(
                            anchored().snap_to_window_with_margin(px(8.)).child(
                                crate::motion::pop_in("dropdown-in", gpui::div().child(panel)),
                            ),
                        )),
                )
            })
    }
}
