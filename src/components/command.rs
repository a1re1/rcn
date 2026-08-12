//! Command — port of shadcn base-vega `ui/command.tsx` (cmdk upstream).
//!
//! A command palette: a search [`Input`] over grouped items, filtered by
//! substring match, with an empty state. The caller owns the input entity
//! (bare mode) and should `cx.observe` it so filtering re-renders live.
//! Keyboard selection (arrows/enter) and dialog mode are omitted.
//!
//! Sizing and shape overrides come from the caller via [`Styled`].

use gpui::{
    AnyElement, App, ClickEvent, ElementId, Entity, InteractiveElement as _, IntoElement,
    ParentElement as _, Refineable as _, RenderOnce, SharedString, StatefulInteractiveElement as _,
    StyleRefinement, Styled, Window, div, prelude::FluentBuilder as _, px, svg,
};

use crate::components::input::Input;
use crate::theme::Theme;

type SelectHandler = Box<dyn Fn(&ClickEvent, &mut Window, &mut App) + 'static>;

pub struct CommandItem {
    id: ElementId,
    label: SharedString,
    shortcut: Option<SharedString>,
    on_select: Option<SelectHandler>,
}

impl CommandItem {
    pub fn new(id: impl Into<ElementId>, label: impl Into<SharedString>) -> Self {
        Self {
            id: id.into(),
            label: label.into(),
            shortcut: None,
            on_select: None,
        }
    }

    pub fn shortcut(mut self, shortcut: impl Into<SharedString>) -> Self {
        self.shortcut = Some(shortcut.into());
        self
    }

    pub fn on_select(
        mut self,
        handler: impl Fn(&ClickEvent, &mut Window, &mut App) + 'static,
    ) -> Self {
        self.on_select = Some(Box::new(handler));
        self
    }
}

pub struct CommandGroup {
    heading: SharedString,
    items: Vec<CommandItem>,
}

impl CommandGroup {
    pub fn new(heading: impl Into<SharedString>) -> Self {
        Self {
            heading: heading.into(),
            items: Vec::new(),
        }
    }

    pub fn item(mut self, item: CommandItem) -> Self {
        self.items.push(item);
        self
    }
}

/// Command palette shell. Sizing and shape overrides come from the caller via [`Styled`].
#[derive(IntoElement)]
pub struct Command {
    input: Entity<Input>,
    groups: Vec<CommandGroup>,
    empty_message: SharedString,
    style: StyleRefinement,
}

impl Command {
    /// Wraps a bare [`Input`] used as the search box.
    pub fn new(input: Entity<Input>) -> Self {
        Self {
            input,
            groups: Vec::new(),
            empty_message: "No results found.".into(),
            style: StyleRefinement::default(),
        }
    }

    pub fn group(mut self, group: CommandGroup) -> Self {
        self.groups.push(group);
        self
    }

    pub fn empty_message(mut self, message: impl Into<SharedString>) -> Self {
        self.empty_message = message.into();
        self
    }
}

impl Styled for Command {
    fn style(&mut self) -> &mut StyleRefinement {
        &mut self.style
    }
}

impl RenderOnce for Command {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let theme = Theme::of(cx).clone();
        let query = self.input.read(cx).text().to_lowercase();

        let mut any_match = false;
        let groups: Vec<AnyElement> = self
            .groups
            .into_iter()
            .filter_map(|group| {
                let items: Vec<CommandItem> = group
                    .items
                    .into_iter()
                    .filter(|item| query.is_empty() || item.label.to_lowercase().contains(&query))
                    .collect();
                if items.is_empty() {
                    return None;
                }
                any_match = true;
                Some(
                    div()
                        .flex()
                        .flex_col()
                        .p(px(4.))
                        .child(
                            div()
                                .px(px(8.))
                                .py(px(6.))
                                .text_size(px(12.))
                                .line_height(px(16.))
                                .text_color(theme.muted_foreground)
                                .child(group.heading),
                        )
                        .children(items.into_iter().map(|item| {
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
                                .hover(|s| s.bg(theme.accent).text_color(theme.accent_foreground))
                                .when_some(item.on_select, |el, on_select| el.on_click(on_select))
                                .child(item.label)
                                .when_some(item.shortcut, |el, shortcut| {
                                    el.child(
                                        div()
                                            .ml_auto()
                                            .text_size(px(12.))
                                            .text_color(theme.muted_foreground)
                                            .child(shortcut),
                                    )
                                })
                        }))
                        .into_any_element(),
                )
            })
            .collect();

        // Shell: rounded-lg border bg-popover; search row with icon +
        // bordered bottom; then the scrollable list. Caller refinement
        // applied last so sizing/shape overrides win over component defaults.
        let mut root = div()
            .flex()
            .flex_col()
            .w_full()
            .rounded(theme.radius_lg())
            .border_1()
            .border_color(theme.border)
            .bg(theme.popover)
            .text_color(theme.popover_foreground)
            .overflow_hidden();
        root.style().refine(&self.style);
        root.child(
            div()
                .flex()
                .flex_row()
                .items_center()
                .gap(px(8.))
                .px(px(12.))
                .h(px(44.))
                .border_b_1()
                .border_color(theme.border)
                .child(
                    svg()
                        .path(theme.icons.chevron_right())
                        .size(px(16.))
                        .flex_shrink_0()
                        .text_color(theme.muted_foreground),
                )
                .child(div().flex_1().child(self.input.clone())),
        )
        .child(
            div()
                .id("command-list")
                .max_h(px(300.))
                .overflow_y_scroll()
                .map(|el| {
                    if any_match {
                        el.children(groups)
                    } else {
                        el.child(
                            div()
                                .py(px(24.))
                                .text_size(px(14.))
                                .text_color(theme.muted_foreground)
                                .flex()
                                .justify_center()
                                .child(self.empty_message),
                        )
                    }
                }),
        )
    }
}
