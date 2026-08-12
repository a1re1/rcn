//! Combobox — port of the shadcn base-vega combobox (Popover + Command
//! composition).
//!
//! A select-style trigger opening a searchable option list: typing in the
//! embedded [`Input`] filters; picking an option reports its index and
//! closes. The caller owns value/open state and the search input entity
//! (bare mode, observed for live filtering).
//!
//! Sizing and shape overrides come from the caller via [`Styled`] (applied
//! to the component root wrapper).

use std::rc::Rc;

use gpui::{
    App, ElementId, Entity, InteractiveElement as _, IntoElement, ParentElement as _,
    Refineable as _, RenderOnce, SharedString, StatefulInteractiveElement as _, StyleRefinement,
    Styled, Window, anchored, deferred, div, prelude::FluentBuilder as _, px, relative, svg,
};

use crate::components::input::Input;
use crate::theme::{Theme, alpha};

type ChangeHandler = Rc<dyn Fn(&usize, &mut Window, &mut App) + 'static>;
type OpenChangeHandler = Rc<dyn Fn(&bool, &mut Window, &mut App) + 'static>;

#[derive(IntoElement)]
pub struct Combobox {
    id: ElementId,
    search: Entity<Input>,
    options: Vec<SharedString>,
    value: Option<usize>,
    placeholder: SharedString,
    empty_message: SharedString,
    open: bool,
    on_change: Option<ChangeHandler>,
    on_open_change: Option<OpenChangeHandler>,
    style: StyleRefinement,
}

impl Combobox {
    /// `search` is a bare [`Input`] entity used for filtering.
    pub fn new(id: impl Into<ElementId>, search: Entity<Input>) -> Self {
        Self {
            id: id.into(),
            search,
            options: Vec::new(),
            value: None,
            placeholder: "Select an option...".into(),
            empty_message: "No results found.".into(),
            open: false,
            on_change: None,
            on_open_change: None,
            style: StyleRefinement::default(),
        }
    }

    pub fn options(mut self, options: impl IntoIterator<Item = impl Into<SharedString>>) -> Self {
        self.options.extend(options.into_iter().map(Into::into));
        self
    }

    pub fn value(mut self, value: Option<usize>) -> Self {
        self.value = value;
        self
    }

    pub fn placeholder(mut self, placeholder: impl Into<SharedString>) -> Self {
        self.placeholder = placeholder.into();
        self
    }

    pub fn empty_message(mut self, message: impl Into<SharedString>) -> Self {
        self.empty_message = message.into();
        self
    }

    pub fn open(mut self, open: bool) -> Self {
        self.open = open;
        self
    }

    pub fn on_change(mut self, handler: impl Fn(&usize, &mut Window, &mut App) + 'static) -> Self {
        self.on_change = Some(Rc::new(handler));
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

impl Styled for Combobox {
    fn style(&mut self) -> &mut StyleRefinement {
        &mut self.style
    }
}

impl RenderOnce for Combobox {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let theme = Theme::of(cx).clone();
        let open = self.open;
        let toggle = self.on_open_change.clone();
        let close = self.on_open_change.clone();
        let selected_label = self
            .value
            .and_then(|index| self.options.get(index).cloned());
        let query = self.search.read(cx).text().to_lowercase();

        // Trigger mirrors the shadcn example: outline-button styling with
        // the value (or placeholder) and a chevron.
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
            .child(match selected_label {
                Some(label) => div().text_color(theme.foreground).child(label),
                None => div()
                    .text_color(theme.muted_foreground)
                    .child(self.placeholder.clone()),
            })
            .child(
                svg()
                    .path(theme.icons.chevron_down())
                    .size(px(16.))
                    .flex_shrink_0()
                    .text_color(theme.muted_foreground),
            );

        let panel = open.then(|| {
            let matches: Vec<(usize, SharedString)> = self
                .options
                .iter()
                .enumerate()
                .filter(|(_, label)| query.is_empty() || label.to_lowercase().contains(&query))
                .map(|(index, label)| (index, label.clone()))
                .collect();

            div()
                .occlude()
                .flex()
                .flex_col()
                .w_full()
                .rounded(theme.radius_md())
                .bg(theme.popover)
                .text_color(theme.popover_foreground)
                .shadow_md()
                .border_1()
                .border_color(alpha(theme.foreground, 0.1))
                .overflow_hidden()
                .when_some(close, |el, close| {
                    el.on_mouse_down_out(move |_, window, cx| close(&false, window, cx))
                })
                .child(
                    div()
                        .flex()
                        .flex_row()
                        .items_center()
                        .gap(px(8.))
                        .px(px(12.))
                        .h(px(36.))
                        .border_b_1()
                        .border_color(theme.border)
                        .child(div().flex_1().child(self.search.clone())),
                )
                .child(
                    div()
                        .id("combobox-list")
                        .max_h(px(240.))
                        .overflow_y_scroll()
                        .p(px(4.))
                        .map(|el| {
                            if matches.is_empty() {
                                el.child(
                                    div()
                                        .py(px(20.))
                                        .flex()
                                        .justify_center()
                                        .text_size(px(14.))
                                        .text_color(theme.muted_foreground)
                                        .child(self.empty_message),
                                )
                            } else {
                                el.children(matches.into_iter().map(|(index, label)| {
                                    let selected = self.value == Some(index);
                                    let on_change = self.on_change.clone();
                                    let finish = self.on_open_change.clone();
                                    div()
                                        .id(("combobox-option", index))
                                        .flex()
                                        .flex_row()
                                        .items_center()
                                        .gap(px(8.))
                                        .rounded(theme.radius_sm())
                                        .px(px(8.))
                                        .py(px(6.))
                                        .text_size(px(14.))
                                        .line_height(px(20.))
                                        .hover(|s| {
                                            s.bg(theme.accent).text_color(theme.accent_foreground)
                                        })
                                        .on_click(move |_, window, cx| {
                                            if let Some(on_change) = &on_change {
                                                on_change(&index, window, cx);
                                            }
                                            if let Some(finish) = &finish {
                                                finish(&false, window, cx);
                                            }
                                        })
                                        .child(div().w(px(14.)).flex_shrink_0().when(
                                            selected,
                                            |el| {
                                                el.child(
                                                    svg()
                                                        .path(theme.icons.check())
                                                        .size(px(14.))
                                                        .text_color(theme.popover_foreground),
                                                )
                                            },
                                        ))
                                        .child(label)
                                }))
                            }
                        }),
                )
        });

        let mut root = div().child(trigger).when_some(panel, |el, panel| {
            el.child(
                div()
                    .absolute()
                    .left_0()
                    .top(relative(1.))
                    .pt(px(4.))
                    .w_full()
                    .child(deferred(
                        anchored()
                            .snap_to_window_with_margin(px(8.))
                            .child(crate::motion::pop_in("combobox-in", panel)),
                    )),
            )
        });
        root.style().refine(&self.style);
        root
    }
}
