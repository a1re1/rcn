//! DatePicker — port of the shadcn date-picker (Popover + Calendar
//! composition; no standalone registry item upstream).
//!
//! A select-style trigger showing the chosen date, opening a [`Calendar`]
//! in a popover panel. Controlled: the caller owns the value, the visible
//! month, and the open flag.

use std::rc::Rc;

use gpui::{
    App, ElementId, InteractiveElement as _, IntoElement, ParentElement as _, RenderOnce,
    SharedString, StatefulInteractiveElement as _, Styled, Window, anchored, deferred, div,
    prelude::FluentBuilder as _, px, relative, svg,
};

use crate::components::calendar::{Calendar, CalendarDate};
use crate::theme::{Theme, alpha};

type OpenChangeHandler = Rc<dyn Fn(&bool, &mut Window, &mut App) + 'static>;
type SelectHandler = Rc<dyn Fn(&CalendarDate, &mut Window, &mut App) + 'static>;
type MonthChangeHandler = Rc<dyn Fn(&(i32, u32), &mut Window, &mut App) + 'static>;

#[derive(IntoElement)]
pub struct DatePicker {
    id: ElementId,
    value: Option<CalendarDate>,
    month: (i32, u32),
    placeholder: SharedString,
    open: bool,
    on_select: Option<SelectHandler>,
    on_month_change: Option<MonthChangeHandler>,
    on_open_change: Option<OpenChangeHandler>,
}

impl DatePicker {
    pub fn new(id: impl Into<ElementId>, month: (i32, u32)) -> Self {
        Self {
            id: id.into(),
            value: None,
            month,
            placeholder: "Pick a date".into(),
            open: false,
            on_select: None,
            on_month_change: None,
            on_open_change: None,
        }
    }

    pub fn value(mut self, value: Option<CalendarDate>) -> Self {
        self.value = value;
        self
    }

    pub fn placeholder(mut self, placeholder: impl Into<SharedString>) -> Self {
        self.placeholder = placeholder.into();
        self
    }

    pub fn open(mut self, open: bool) -> Self {
        self.open = open;
        self
    }

    pub fn on_select(
        mut self,
        handler: impl Fn(&CalendarDate, &mut Window, &mut App) + 'static,
    ) -> Self {
        self.on_select = Some(Rc::new(handler));
        self
    }

    pub fn on_month_change(
        mut self,
        handler: impl Fn(&(i32, u32), &mut Window, &mut App) + 'static,
    ) -> Self {
        self.on_month_change = Some(Rc::new(handler));
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

impl RenderOnce for DatePicker {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let theme = Theme::of(cx).clone();
        let open = self.open;
        let toggle = self.on_open_change.clone();
        let close = self.on_open_change.clone();

        let label: SharedString = match self.value {
            Some(date) => format!("{:04}-{:02}-{:02}", date.year, date.month, date.day).into(),
            None => self.placeholder.clone(),
        };
        let has_value = self.value.is_some();

        let trigger = div()
            .id(self.id)
            .flex()
            .flex_row()
            .items_center()
            .justify_between()
            .gap(px(8.))
            .h(px(36.))
            .w_full()
            .rounded(theme.radius_md())
            .border_1()
            .border_color(theme.input)
            .when(theme.dark, |el| el.bg(alpha(theme.input, 0.3)))
            .px(px(12.))
            .text_size(px(14.))
            .line_height(px(20.))
            .shadow_xs()
            .when_some(toggle, |el, toggle| {
                el.on_click(move |_, window, cx| toggle(&!open, window, cx))
            })
            .child(
                div()
                    .text_color(if has_value {
                        theme.foreground
                    } else {
                        theme.muted_foreground
                    })
                    .child(label),
            )
            .child(
                svg()
                    .path(theme.icons.chevron_down())
                    .size(px(16.))
                    .flex_shrink_0()
                    .text_color(theme.muted_foreground),
            );

        let panel = open.then(|| {
            let on_select = self.on_select.clone();
            let finish = self.on_open_change.clone();
            div()
                .occlude()
                .when_some(close, |el, close| {
                    el.on_mouse_down_out(move |_, window, cx| close(&false, window, cx))
                })
                .child(
                    Calendar::new(self.month.0, self.month.1)
                        .selected(self.value)
                        .when_some(self.on_month_change, |el, handler| {
                            el.on_month_change(move |month, window, cx| handler(month, window, cx))
                        })
                        .when_some(on_select, |el, on_select| {
                            el.on_select(move |date, window, cx| {
                                on_select(date, window, cx);
                                if let Some(finish) = &finish {
                                    finish(&false, window, cx);
                                }
                            })
                        }),
                )
        });

        div()
            .relative()
            .flex()
            .flex_col()
            .w(px(220.))
            .child(trigger)
            .when_some(panel, |el, panel| {
                el.child(
                    div()
                        .absolute()
                        .left_0()
                        .top(relative(1.))
                        .pt(px(4.))
                        .child(deferred(
                            anchored().snap_to_window_with_margin(px(8.)).child(panel),
                        )),
                )
            })
    }
}
