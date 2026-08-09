//! Calendar — port of shadcn base-vega `ui/calendar.tsx` (react-day-picker
//! upstream).
//!
//! A month grid with previous/next navigation and single-day selection.
//! Controlled: the caller owns the visible month and the selected day.
//! Range selection, multiple months, and dropdown navigation are omitted.

use gpui::{
    App, FontWeight, InteractiveElement as _, IntoElement, ParentElement as _, RenderOnce,
    StatefulInteractiveElement as _, Styled, Window, div, prelude::FluentBuilder as _, px, svg,
};
use std::rc::Rc;

use crate::motion;
use crate::theme::Theme;

/// A calendar date (proleptic Gregorian).
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct CalendarDate {
    pub year: i32,
    /// 1..=12
    pub month: u32,
    /// 1..=31
    pub day: u32,
}

impl CalendarDate {
    pub fn new(year: i32, month: u32, day: u32) -> Self {
        Self { year, month, day }
    }
}

/// Days in a Gregorian month.
pub(crate) fn days_in_month(year: i32, month: u32) -> u32 {
    match month {
        1 | 3 | 5 | 7 | 8 | 10 | 12 => 31,
        4 | 6 | 9 | 11 => 30,
        2 => {
            if (year % 4 == 0 && year % 100 != 0) || year % 400 == 0 {
                29
            } else {
                28
            }
        }
        _ => 30,
    }
}

/// Weekday of the first day of a month; 0 = Sunday (Zeller's congruence).
pub(crate) fn first_weekday(year: i32, month: u32) -> u32 {
    let (m, y) = if month < 3 {
        (month + 12, year - 1)
    } else {
        (month, year)
    };
    let k = y.rem_euclid(100);
    let j = y.div_euclid(100);
    let h = (1 + (13 * (m as i32 + 1)) / 5 + k + k / 4 + j / 4 + 5 * j).rem_euclid(7);
    // Zeller: 0 = Saturday; convert to 0 = Sunday.
    ((h + 6) % 7) as u32
}

const MONTH_NAMES: [&str; 12] = [
    "January",
    "February",
    "March",
    "April",
    "May",
    "June",
    "July",
    "August",
    "September",
    "October",
    "November",
    "December",
];

type MonthChangeHandler = Rc<dyn Fn(&(i32, u32), &mut Window, &mut App) + 'static>;
type SelectHandler = Rc<dyn Fn(&CalendarDate, &mut Window, &mut App) + 'static>;

#[derive(IntoElement)]
pub struct Calendar {
    /// Visible (year, month).
    month: (i32, u32),
    selected: Option<CalendarDate>,
    on_month_change: Option<MonthChangeHandler>,
    on_select: Option<SelectHandler>,
}

impl Calendar {
    pub fn new(year: i32, month: u32) -> Self {
        Self {
            month: (year, month.clamp(1, 12)),
            selected: None,
            on_month_change: None,
            on_select: None,
        }
    }

    pub fn selected(mut self, selected: Option<CalendarDate>) -> Self {
        self.selected = selected;
        self
    }

    /// Fired by the prev/next buttons with the new (year, month).
    pub fn on_month_change(
        mut self,
        handler: impl Fn(&(i32, u32), &mut Window, &mut App) + 'static,
    ) -> Self {
        self.on_month_change = Some(Rc::new(handler));
        self
    }

    pub fn on_select(
        mut self,
        handler: impl Fn(&CalendarDate, &mut Window, &mut App) + 'static,
    ) -> Self {
        self.on_select = Some(Rc::new(handler));
        self
    }
}

impl RenderOnce for Calendar {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let theme = Theme::of(cx).clone();
        let (year, month) = self.month;
        let total_days = days_in_month(year, month);
        let lead = first_weekday(year, month);
        let prev = if month == 1 {
            (year - 1, 12)
        } else {
            (year, month - 1)
        };
        let next = if month == 12 {
            (year + 1, 1)
        } else {
            (year, month + 1)
        };

        let nav_button = |id: &'static str, icon: String, target: (i32, u32)| {
            let handler = self.on_month_change.clone();
            let ring = motion::focus_ring(&theme);
            div()
                .id(id)
                .flex()
                .size(px(28.))
                .items_center()
                .justify_center()
                .rounded(theme.radius_md())
                .border_1()
                .border_color(theme.border)
                .tab_index(0)
                .focus_visible(move |s| s.border_color(theme.ring).shadow(ring.clone()))
                .hover(|s| s.bg(theme.muted))
                .when_some(handler, |el, handler| {
                    el.on_click(move |_, window, cx| handler(&target, window, cx))
                })
                .child(svg().path(icon).size(px(14.)).text_color(theme.foreground))
        };

        // p-3 panel: header (prev / month year / next), weekday row, grid.
        div()
            .flex()
            .flex_col()
            .gap(px(12.))
            .p(px(12.))
            .rounded(theme.radius_lg())
            .border_1()
            .border_color(theme.border)
            .bg(theme.background)
            .child(
                div()
                    .flex()
                    .flex_row()
                    .items_center()
                    .justify_between()
                    .child(nav_button(
                        "calendar-prev",
                        theme.icons.chevron_left(),
                        prev,
                    ))
                    .child(
                        div()
                            .text_size(px(14.))
                            .line_height(px(20.))
                            .font_weight(FontWeight::MEDIUM)
                            .child(format!("{} {year}", MONTH_NAMES[(month - 1) as usize])),
                    )
                    .child(nav_button(
                        "calendar-next",
                        theme.icons.chevron_right(),
                        next,
                    )),
            )
            .child(div().flex().flex_row().children(
                ["Su", "Mo", "Tu", "We", "Th", "Fr", "Sa"].map(|name| {
                    div()
                        .flex()
                        .w(px(32.))
                        .h(px(28.))
                        .items_center()
                        .justify_center()
                        .text_size(px(12.))
                        .text_color(theme.muted_foreground)
                        .child(name)
                }),
            ))
            .children({
                // Six week rows cover every month layout.
                (0u32..6).map(move |week| {
                    let theme = theme.clone();
                    let selected = self.selected;
                    let on_select = self.on_select.clone();
                    div()
                        .flex()
                        .flex_row()
                        .children((0u32..7).map(move |weekday| {
                            let cell: u32 = week * 7 + weekday;
                            let day = (cell + 1).checked_sub(lead + 1).map(|d| d + 1);
                            let day = day.filter(|d| *d <= total_days);
                            let base = div()
                                .flex()
                                .w(px(32.))
                                .h(px(32.))
                                .items_center()
                                .justify_center()
                                .text_size(px(14.))
                                .line_height(px(20.));
                            match day {
                                None => base.into_any_element(),
                                Some(day) => {
                                    let date = CalendarDate::new(year, month, day);
                                    let is_selected = selected == Some(date);
                                    let on_select = on_select.clone();
                                    let ring = motion::focus_ring(&theme);
                                    base.id(("calendar-day", cell as usize))
                                        .rounded(theme.radius_md())
                                        .tab_index(0)
                                        .focus_visible(move |s| {
                                            s.border_color(theme.ring).shadow(ring.clone())
                                        })
                                        .map(|el| {
                                            if is_selected {
                                                el.bg(theme.primary)
                                                    .text_color(theme.primary_foreground)
                                            } else {
                                                el.text_color(theme.foreground)
                                                    .hover(|s| s.bg(theme.muted))
                                            }
                                        })
                                        .when_some(on_select, |el, on_select| {
                                            el.on_click(move |_, window, cx| {
                                                on_select(&date, window, cx)
                                            })
                                        })
                                        .child(day.to_string())
                                        .into_any_element()
                                }
                            }
                        }))
                })
            })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn month_math() {
        assert_eq!(days_in_month(2024, 2), 29);
        assert_eq!(days_in_month(2025, 2), 28);
        assert_eq!(days_in_month(2000, 2), 29);
        assert_eq!(days_in_month(1900, 2), 28);
        assert_eq!(days_in_month(2026, 8), 31);
        // 2026-08-01 is a Saturday.
        assert_eq!(first_weekday(2026, 8), 6);
        // 2026-01-01 is a Thursday.
        assert_eq!(first_weekday(2026, 1), 4);
        // 2024-02-01 was a Thursday.
        assert_eq!(first_weekday(2024, 2), 4);
    }
}
