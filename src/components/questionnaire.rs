//! Questionnaire — port of shadcn base-vega `ui/questionnaire.tsx` (core
//! subset).
//!
//! A form-flow step: progress line, title/description, selectable choice
//! rows (radio-style indicator, checked = primary border on muted), and
//! an actions row (Previous/Skip/Next built from Buttons). Keyboard
//! shortcuts, text-input steps, and multi-step orchestration are omitted
//! — the caller owns the selected choice.

use gpui::{
    AnyElement, App, ClickEvent, ElementId, FontWeight, InteractiveElement as _, IntoElement,
    ParentElement, RenderOnce, SharedString, StatefulInteractiveElement as _, Styled, Window, div,
    prelude::FluentBuilder as _, px,
};

use crate::theme::{Theme, alpha};

/// The step container: grid gap-6.
#[derive(IntoElement)]
pub struct Questionnaire {
    children: Vec<AnyElement>,
}

impl Questionnaire {
    pub fn new() -> Self {
        Self {
            children: Vec::new(),
        }
    }
}

impl Default for Questionnaire {
    fn default() -> Self {
        Self::new()
    }
}

impl ParentElement for Questionnaire {
    fn extend(&mut self, elements: impl IntoIterator<Item = AnyElement>) {
        self.children.extend(elements);
    }
}

impl RenderOnce for Questionnaire {
    fn render(self, _window: &mut Window, _cx: &mut App) -> impl IntoElement {
        div().flex().flex_col().gap(px(24.)).children(self.children)
    }
}

/// text-xs font-medium text-muted-foreground tabular-nums progress line.
#[derive(IntoElement)]
pub struct QuestionnaireProgress {
    current: usize,
    total: usize,
}

impl QuestionnaireProgress {
    pub fn new(current: usize, total: usize) -> Self {
        Self { current, total }
    }
}

impl RenderOnce for QuestionnaireProgress {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let theme = Theme::of(cx);
        div()
            .text_size(px(12.))
            .line_height(px(16.))
            .font_weight(FontWeight::MEDIUM)
            .text_color(theme.muted_foreground)
            .child(format!("Question {} of {}", self.current, self.total))
    }
}

/// The question heading (heading font, text-lg medium).
#[derive(IntoElement)]
pub struct QuestionnaireTitle {
    children: Vec<AnyElement>,
}

impl QuestionnaireTitle {
    pub fn new() -> Self {
        Self {
            children: Vec::new(),
        }
    }
}

impl Default for QuestionnaireTitle {
    fn default() -> Self {
        Self::new()
    }
}

impl ParentElement for QuestionnaireTitle {
    fn extend(&mut self, elements: impl IntoIterator<Item = AnyElement>) {
        self.children.extend(elements);
    }
}

impl RenderOnce for QuestionnaireTitle {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let theme = Theme::of(cx);
        div()
            .text_size(px(18.))
            .line_height(px(26.))
            .font_weight(FontWeight::MEDIUM)
            .when_some(theme.heading_font(), |el, font| el.font_family(font))
            .children(self.children)
    }
}

/// text-sm text-muted-foreground under the title.
#[derive(IntoElement)]
pub struct QuestionnaireDescription {
    children: Vec<AnyElement>,
}

impl QuestionnaireDescription {
    pub fn new() -> Self {
        Self {
            children: Vec::new(),
        }
    }
}

impl Default for QuestionnaireDescription {
    fn default() -> Self {
        Self::new()
    }
}

impl ParentElement for QuestionnaireDescription {
    fn extend(&mut self, elements: impl IntoIterator<Item = AnyElement>) {
        self.children.extend(elements);
    }
}

impl RenderOnce for QuestionnaireDescription {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let theme = Theme::of(cx);
        div()
            .text_size(px(14.))
            .line_height(px(20.))
            .text_color(theme.muted_foreground)
            .children(self.children)
    }
}

/// grid gap-3 — the option list.
#[derive(IntoElement)]
pub struct QuestionnaireChoices {
    children: Vec<AnyElement>,
}

impl QuestionnaireChoices {
    pub fn new() -> Self {
        Self {
            children: Vec::new(),
        }
    }
}

impl Default for QuestionnaireChoices {
    fn default() -> Self {
        Self::new()
    }
}

impl ParentElement for QuestionnaireChoices {
    fn extend(&mut self, elements: impl IntoIterator<Item = AnyElement>) {
        self.children.extend(elements);
    }
}

impl RenderOnce for QuestionnaireChoices {
    fn render(self, _window: &mut Window, _cx: &mut App) -> impl IntoElement {
        div().flex().flex_col().gap(px(12.)).children(self.children)
    }
}

/// One selectable option row with a radio indicator.
#[derive(IntoElement)]
pub struct QuestionnaireChoice {
    id: ElementId,
    label: SharedString,
    description: Option<SharedString>,
    checked: bool,
    on_select: Option<Box<dyn Fn(&ClickEvent, &mut Window, &mut App) + 'static>>,
}

impl QuestionnaireChoice {
    pub fn new(id: impl Into<ElementId>, label: impl Into<SharedString>) -> Self {
        Self {
            id: id.into(),
            label: label.into(),
            description: None,
            checked: false,
            on_select: None,
        }
    }

    pub fn description(mut self, description: impl Into<SharedString>) -> Self {
        self.description = Some(description.into());
        self
    }

    pub fn checked(mut self, checked: bool) -> Self {
        self.checked = checked;
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

impl RenderOnce for QuestionnaireChoice {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let theme = Theme::of(cx).clone();
        let checked = self.checked;

        // Indicator: size-4 rounded-full border; checked: primary + dot.
        let indicator = div()
            .flex()
            .flex_shrink_0()
            .size(px(16.))
            .mt(px(2.))
            .items_center()
            .justify_center()
            .rounded_full()
            .border_1()
            .map(|el| {
                if checked {
                    el.border_color(theme.primary).bg(theme.primary)
                } else {
                    el.border_color(theme.input)
                }
            })
            .when(checked, |el| {
                el.child(
                    div()
                        .size(px(8.))
                        .rounded_full()
                        .bg(theme.primary_foreground),
                )
            });

        div()
            .id(self.id)
            .flex()
            .flex_row()
            .items_start()
            .gap(px(12.))
            .min_h(px(44.))
            .rounded(theme.radius_md())
            .border_1()
            .px(px(16.))
            .py(px(14.))
            .text_size(px(14.))
            .line_height(px(19.))
            .shadow_xs()
            .map(|el| {
                if checked {
                    // data-checked: border-primary/40 bg-muted
                    el.border_color(alpha(theme.primary, 0.4)).bg(theme.muted)
                } else {
                    el.border_color(theme.input)
                }
            })
            .hover(move |s| s.bg(alpha(theme.muted, 0.5)))
            .when_some(self.on_select, |el, on_select| el.on_click(on_select))
            .child(indicator)
            .child(
                div()
                    .flex()
                    .flex_col()
                    .flex_1()
                    .min_w(px(0.))
                    .gap(px(4.))
                    .child(self.label)
                    .when_some(self.description, |el, description| {
                        el.child(div().text_color(theme.muted_foreground).child(description))
                    }),
            )
    }
}

/// The actions row: previous at the start, skip/next at the end.
#[derive(IntoElement)]
pub struct QuestionnaireActions {
    previous: Option<AnyElement>,
    trailing: Vec<AnyElement>,
}

impl QuestionnaireActions {
    pub fn new() -> Self {
        Self {
            previous: None,
            trailing: Vec::new(),
        }
    }

    pub fn previous(mut self, previous: impl IntoElement) -> Self {
        self.previous = Some(previous.into_any_element());
        self
    }

    pub fn action(mut self, action: impl IntoElement) -> Self {
        self.trailing.push(action.into_any_element());
        self
    }
}

impl Default for QuestionnaireActions {
    fn default() -> Self {
        Self::new()
    }
}

impl RenderOnce for QuestionnaireActions {
    fn render(self, _window: &mut Window, _cx: &mut App) -> impl IntoElement {
        div()
            .flex()
            .flex_row()
            .items_center()
            .gap(px(8.))
            .w_full()
            .children(self.previous)
            .child(div().flex_1())
            .children(self.trailing)
    }
}
