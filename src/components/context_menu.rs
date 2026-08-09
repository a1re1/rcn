//! ContextMenu — port of shadcn base-vega `ui/context-menu.tsx`.
//!
//! The dropdown-menu panel opened at the pointer on right-click over the
//! trigger area. Controlled: the caller stores the open position from
//! `on_request_open` and clears it via `on_open_change(false)`.

use gpui::{
    AnyElement, App, ElementId, InteractiveElement as _, IntoElement, MouseButton, ParentElement,
    Pixels, Point, RenderOnce, Window, anchored, deferred, div, prelude::FluentBuilder as _,
};

pub use crate::components::dropdown_menu::DropdownMenuItem as ContextMenuItem;
use crate::components::dropdown_menu::{DropdownMenuEntry, OpenChangeHandler, menu_panel};

type RequestOpenHandler = Box<dyn Fn(&Point<Pixels>, &mut Window, &mut App) + 'static>;

#[derive(IntoElement)]
pub struct ContextMenu {
    id: ElementId,
    open_at: Option<Point<Pixels>>,
    trigger: Option<AnyElement>,
    entries: Vec<DropdownMenuEntry>,
    on_request_open: Option<RequestOpenHandler>,
    on_open_change: Option<OpenChangeHandler>,
}

impl ContextMenu {
    pub fn new(id: impl Into<ElementId>) -> Self {
        Self {
            id: id.into(),
            open_at: None,
            trigger: None,
            entries: Vec::new(),
            on_request_open: None,
            on_open_change: None,
        }
    }

    /// The window position to show the menu at (`None` = closed).
    pub fn open_at(mut self, position: Option<Point<Pixels>>) -> Self {
        self.open_at = position;
        self
    }

    /// The right-clickable area.
    pub fn trigger(mut self, trigger: impl IntoElement) -> Self {
        self.trigger = Some(trigger.into_any_element());
        self
    }

    pub fn item(mut self, item: ContextMenuItem) -> Self {
        self.entries.push(DropdownMenuEntry::Item(item));
        self
    }

    pub fn label(mut self, label: impl IntoElement) -> Self {
        self.entries
            .push(DropdownMenuEntry::Label(label.into_any_element()));
        self
    }

    pub fn separator(mut self) -> Self {
        self.entries.push(DropdownMenuEntry::Separator);
        self
    }

    /// Fired on right-click with the pointer position.
    pub fn on_request_open(
        mut self,
        handler: impl Fn(&Point<Pixels>, &mut Window, &mut App) + 'static,
    ) -> Self {
        self.on_request_open = Some(Box::new(handler));
        self
    }

    /// Fired with `false` when the menu wants to close.
    pub fn on_open_change(
        mut self,
        handler: impl Fn(&bool, &mut Window, &mut App) + 'static,
    ) -> Self {
        self.on_open_change = Some(std::rc::Rc::new(handler));
        self
    }
}

impl RenderOnce for ContextMenu {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let panel = self.open_at.map(|position| {
            deferred(
                anchored()
                    .position(position)
                    .snap_to_window_with_margin(gpui::px(8.))
                    .child(crate::motion::pop_in(
                        "contextmenu-in",
                        gpui::div().child(menu_panel(self.entries, self.on_open_change, cx)),
                    )),
            )
        });

        div()
            .id(self.id)
            .when_some(self.on_request_open, |el, on_open| {
                el.on_mouse_down(MouseButton::Right, move |event, window, cx| {
                    on_open(&event.position, window, cx)
                })
            })
            .children(self.trigger)
            .children(panel)
    }
}
