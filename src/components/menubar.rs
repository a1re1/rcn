//! Menubar — port of shadcn base-vega `ui/menubar.tsx`.
//!
//! A horizontal row of menu triggers sharing the dropdown-menu panel.
//! Controlled: the caller owns which menu index is open (`None` = all
//! closed) and receives changes via `on_open_change`.

use std::rc::Rc;

use gpui::{
    App, ElementId, FontWeight, InteractiveElement as _, IntoElement, ParentElement as _,
    RenderOnce, StatefulInteractiveElement as _, Styled, Window, anchored, deferred, div,
    prelude::FluentBuilder as _, px, relative,
};

pub use crate::components::dropdown_menu::DropdownMenuItem as MenubarItem;
use crate::components::dropdown_menu::{DropdownMenuEntry, menu_panel};
use crate::theme::Theme;

type OpenChangeHandler = Rc<dyn Fn(&Option<usize>, &mut Window, &mut App) + 'static>;

/// One trigger + its menu.
pub struct MenubarMenu {
    label: gpui::SharedString,
    entries: Vec<DropdownMenuEntry>,
}

impl MenubarMenu {
    pub fn new(label: impl Into<gpui::SharedString>) -> Self {
        Self {
            label: label.into(),
            entries: Vec::new(),
        }
    }

    pub fn item(mut self, item: MenubarItem) -> Self {
        self.entries.push(DropdownMenuEntry::Item(item));
        self
    }

    pub fn separator(mut self) -> Self {
        self.entries.push(DropdownMenuEntry::Separator);
        self
    }
}

#[derive(IntoElement)]
pub struct Menubar {
    id: ElementId,
    open: Option<usize>,
    menus: Vec<MenubarMenu>,
    on_open_change: Option<OpenChangeHandler>,
}

impl Menubar {
    pub fn new(id: impl Into<ElementId>) -> Self {
        Self {
            id: id.into(),
            open: None,
            menus: Vec::new(),
            on_open_change: None,
        }
    }

    /// Which menu is open (`None` = closed).
    pub fn open(mut self, open: Option<usize>) -> Self {
        self.open = open;
        self
    }

    pub fn menu(mut self, menu: MenubarMenu) -> Self {
        self.menus.push(menu);
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

impl RenderOnce for Menubar {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let theme = Theme::of(cx).clone();
        let open = self.open;
        let on_open_change = self.on_open_change;

        // flex h-9 items-center gap-1 rounded-md border bg-background p-1 shadow-xs
        div()
            .id(self.id)
            .flex()
            .flex_row()
            .items_center()
            .gap(px(4.))
            .h(px(36.))
            .p(px(4.))
            .rounded(theme.radius_md())
            .border_1()
            .border_color(theme.border)
            .bg(theme.background)
            .shadow_xs()
            .children(self.menus.into_iter().enumerate().map(|(index, menu)| {
                let is_open = open == Some(index);
                let toggle = on_open_change.clone();
                let close = on_open_change.clone().map(|handler| {
                    let handler = handler.clone();
                    Rc::new(move |_open: &bool, window: &mut Window, cx: &mut App| {
                        handler(&None, window, cx)
                    }) as Rc<dyn Fn(&bool, &mut Window, &mut App)>
                });
                let panel = is_open.then(|| menu_panel(menu.entries, close, cx).into_any_element());

                // Trigger: rounded-sm px-2 py-1 text-sm font-medium,
                // open: bg-accent text-accent-foreground
                div()
                    .relative()
                    .child(
                        div()
                            .id(("menubar-trigger", index))
                            .rounded(theme.radius_sm())
                            .px(px(8.))
                            .py(px(4.))
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
                            .child(menu.label),
                    )
                    .when_some(panel, |el, panel| {
                        el.child(
                            div()
                                .absolute()
                                .left_0()
                                .top(relative(1.))
                                .pt(px(8.))
                                .child(deferred(
                                    anchored().snap_to_window_with_margin(px(8.)).child(panel),
                                )),
                        )
                    })
            }))
    }
}
