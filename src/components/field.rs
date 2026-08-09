//! Field — port of shadcn base-vega `ui/field.tsx`.
//!
//! Form-field scaffolding: `Field` stacks a label, a control, and
//! description/error lines; `FieldGroup` stacks fields; `FieldSet` +
//! `FieldLegend` group related fields under a heading. The orientation
//! variants and checkbox/radio layouts are omitted.

use gpui::{
    AnyElement, App, FontWeight, IntoElement, ParentElement, RenderOnce, Styled, Window, div,
    prelude::FluentBuilder as _, px,
};

use crate::theme::Theme;

/// flex flex-col gap-2 — one labeled control.
#[derive(IntoElement)]
pub struct Field {
    children: Vec<AnyElement>,
}

impl Field {
    pub fn new() -> Self {
        Self {
            children: Vec::new(),
        }
    }
}

impl Default for Field {
    fn default() -> Self {
        Self::new()
    }
}

impl ParentElement for Field {
    fn extend(&mut self, elements: impl IntoIterator<Item = AnyElement>) {
        self.children.extend(elements);
    }
}

impl RenderOnce for Field {
    fn render(self, _window: &mut Window, _cx: &mut App) -> impl IntoElement {
        div().flex().flex_col().gap(px(8.)).children(self.children)
    }
}

/// text-sm text-muted-foreground.
#[derive(IntoElement)]
pub struct FieldDescription {
    children: Vec<AnyElement>,
}

impl FieldDescription {
    pub fn new() -> Self {
        Self {
            children: Vec::new(),
        }
    }
}

impl Default for FieldDescription {
    fn default() -> Self {
        Self::new()
    }
}

impl ParentElement for FieldDescription {
    fn extend(&mut self, elements: impl IntoIterator<Item = AnyElement>) {
        self.children.extend(elements);
    }
}

impl RenderOnce for FieldDescription {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let theme = Theme::of(cx);
        div()
            .text_size(px(14.))
            .line_height(px(20.))
            .text_color(theme.muted_foreground)
            .children(self.children)
    }
}

/// text-sm text-destructive — validation feedback.
#[derive(IntoElement)]
pub struct FieldError {
    children: Vec<AnyElement>,
}

impl FieldError {
    pub fn new() -> Self {
        Self {
            children: Vec::new(),
        }
    }
}

impl Default for FieldError {
    fn default() -> Self {
        Self::new()
    }
}

impl ParentElement for FieldError {
    fn extend(&mut self, elements: impl IntoIterator<Item = AnyElement>) {
        self.children.extend(elements);
    }
}

impl RenderOnce for FieldError {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let theme = Theme::of(cx);
        div()
            .text_size(px(14.))
            .line_height(px(20.))
            .text_color(theme.destructive)
            .children(self.children)
    }
}

/// flex flex-col gap-6 — stacks fields into a form section.
#[derive(IntoElement)]
pub struct FieldGroup {
    children: Vec<AnyElement>,
}

impl FieldGroup {
    pub fn new() -> Self {
        Self {
            children: Vec::new(),
        }
    }
}

impl Default for FieldGroup {
    fn default() -> Self {
        Self::new()
    }
}

impl ParentElement for FieldGroup {
    fn extend(&mut self, elements: impl IntoIterator<Item = AnyElement>) {
        self.children.extend(elements);
    }
}

impl RenderOnce for FieldGroup {
    fn render(self, _window: &mut Window, _cx: &mut App) -> impl IntoElement {
        div().flex().flex_col().gap(px(24.)).children(self.children)
    }
}

/// flex flex-col gap-4 rounded-lg border p-4 — a bordered group.
#[derive(IntoElement)]
pub struct FieldSet {
    children: Vec<AnyElement>,
}

impl FieldSet {
    pub fn new() -> Self {
        Self {
            children: Vec::new(),
        }
    }
}

impl Default for FieldSet {
    fn default() -> Self {
        Self::new()
    }
}

impl ParentElement for FieldSet {
    fn extend(&mut self, elements: impl IntoIterator<Item = AnyElement>) {
        self.children.extend(elements);
    }
}

impl RenderOnce for FieldSet {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let theme = Theme::of(cx);
        div()
            .flex()
            .flex_col()
            .gap(px(16.))
            .rounded(theme.radius_lg())
            .border_1()
            .border_color(theme.border)
            .p(px(16.))
            .children(self.children)
    }
}

/// text-sm font-medium — the FieldSet heading.
#[derive(IntoElement)]
pub struct FieldLegend {
    children: Vec<AnyElement>,
}

impl FieldLegend {
    pub fn new() -> Self {
        Self {
            children: Vec::new(),
        }
    }
}

impl Default for FieldLegend {
    fn default() -> Self {
        Self::new()
    }
}

impl ParentElement for FieldLegend {
    fn extend(&mut self, elements: impl IntoIterator<Item = AnyElement>) {
        self.children.extend(elements);
    }
}

impl RenderOnce for FieldLegend {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let theme = Theme::of(cx);
        div()
            .text_size(px(14.))
            .line_height(px(20.))
            .font_weight(FontWeight::MEDIUM)
            .when_some(theme.heading_font(), |el, font| el.font_family(font))
            .children(self.children)
    }
}
