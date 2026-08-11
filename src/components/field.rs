//! Field — port of shadcn base-nova `ui/field.tsx`.
//!
//! Form-field scaffolding: [`Field`] lays out a label, a control, and
//! description/error lines in three orientations; [`FieldGroup`] stacks
//! fields; [`FieldSet`] + [`FieldLegend`] group related fields under a
//! heading; [`FieldLabel`] doubles as a selectable choice card when it wraps
//! a nested `Field`; [`FieldSeparator`] divides sections, optionally with
//! centered inline content.
//!
//! Portability notes vs the TSX source:
//! - CSS `:has()` auto-styling becomes explicit builders: horizontal fields
//!   take `.content(..)` for the start-aligned FieldContent layout, and
//!   choice cards are `FieldLabel::new().choice_card(checked)`.
//! - `orientation="responsive"` swaps at a measured container width
//!   ([`crate::container_query`]) instead of a `@container` query; the
//!   breakpoint defaults to shadcn's `@md` (448px).
//! - `role="group"`/`role="alert"` semantics and the RTL variants have no
//!   gpui analog and are omitted, as is `FieldError`'s Standard Schema
//!   `issues` overload (pass message strings to `.errors(..)`).

use gpui::{
    AnyElement, App, ElementId, FontWeight, IntoElement, ParentElement, Pixels, RenderOnce,
    SharedString, Styled, Window, div, prelude::FluentBuilder as _, px,
};

use crate::container_query::container_query;
use crate::theme::Theme;

/// shadcn `@md` container breakpoint (28rem): responsive fields render
/// horizontally at or above this container width.
const RESPONSIVE_BREAKPOINT: Pixels = px(448.);

/// `orientation` variant of [`Field`].
#[derive(Clone, Copy, PartialEq, Eq, Default, Debug)]
pub enum FieldOrientation {
    /// flex-col; children span the field's width.
    #[default]
    Vertical,
    /// flex-row items-center; the label flexes to fill.
    Horizontal,
    /// Vertical below the container breakpoint, horizontal at or above it.
    Responsive,
}

/// group/field flex w-full gap-2 data-[invalid=true]:text-destructive —
/// one labeled control.
#[derive(IntoElement)]
pub struct Field {
    id: ElementId,
    orientation: FieldOrientation,
    breakpoint: Pixels,
    invalid: bool,
    has_content: bool,
    children: Vec<AnyElement>,
}

impl Field {
    pub fn new() -> Self {
        Self {
            id: ElementId::from("field-responsive"),
            orientation: FieldOrientation::default(),
            breakpoint: RESPONSIVE_BREAKPOINT,
            invalid: false,
            has_content: false,
            children: Vec::new(),
        }
    }

    /// Distinguishes this field's measured width state from siblings'.
    /// Only responsive fields with different widths under one parent need it.
    pub fn id(mut self, id: impl Into<ElementId>) -> Self {
        self.id = id.into();
        self
    }

    pub fn orientation(mut self, orientation: FieldOrientation) -> Self {
        self.orientation = orientation;
        self
    }

    /// Container width at which a responsive field turns horizontal
    /// (default 448px, shadcn's `@md`).
    // Intentional API surface; the storybook demos the default breakpoint.
    #[allow(dead_code)]
    pub fn breakpoint(mut self, breakpoint: Pixels) -> Self {
        self.breakpoint = breakpoint;
        self
    }

    /// data-invalid — paints inherited text (the label) destructive.
    pub fn invalid(mut self, invalid: bool) -> Self {
        self.invalid = invalid;
        self
    }

    /// Add a [`FieldContent`] child. In horizontal/responsive-horizontal
    /// layouts this also start-aligns the row, the ported
    /// `has-[>[data-slot=field-content]]:items-start`.
    pub fn content(mut self, content: FieldContent) -> Self {
        self.has_content = true;
        self.children.push(content.into_any_element());
        self
    }

    fn render_oriented(self, horizontal: bool, cx: &mut App) -> AnyElement {
        let theme = Theme::of(cx);
        div()
            .flex()
            .w_full()
            .gap(px(8.))
            .map(|el| {
                if horizontal {
                    // items-center, items-start when a FieldContent is present
                    let el = el.flex_row();
                    if self.has_content {
                        el.items_start()
                    } else {
                        el.items_center()
                    }
                } else {
                    // *:w-full comes free: column cross-axis default is stretch
                    el.flex_col()
                }
            })
            .when(self.invalid, |el| el.text_color(theme.destructive))
            .children(self.children)
            .into_any_element()
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
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        match self.orientation {
            FieldOrientation::Vertical => self.render_oriented(false, cx),
            FieldOrientation::Horizontal => self.render_oriented(true, cx),
            FieldOrientation::Responsive => {
                let id = self.id.clone();
                let breakpoint = self.breakpoint;
                container_query(id, move |width, _window, cx| {
                    let horizontal = width.is_some_and(|w| w >= breakpoint);
                    self.render_oriented(horizontal, cx)
                })
                .into_any_element()
            }
        }
    }
}

/// flex w-fit gap-2 items-center text-sm leading-snug font-medium — the
/// field's label line. Text color inherits so an invalid [`Field`] cascades
/// destructive onto it. With `.choice_card(checked)` it becomes the
/// selectable bordered card the docs build by nesting a `Field` inside a
/// `FieldLabel`.
#[derive(IntoElement)]
pub struct FieldLabel {
    choice_card: bool,
    checked: bool,
    font_normal: bool,
    disabled: bool,
    children: Vec<AnyElement>,
}

impl FieldLabel {
    pub fn new() -> Self {
        Self {
            choice_card: false,
            checked: false,
            font_normal: false,
            disabled: false,
            children: Vec::new(),
        }
    }

    /// has-[>[data-slot=field]] — render as a selectable choice card:
    /// bordered, rounded-lg, padded, tinted primary while checked.
    pub fn choice_card(mut self, checked: bool) -> Self {
        self.choice_card = true;
        self.checked = checked;
        self
    }

    /// The docs' `className="font-normal"` on checkbox/radio row labels.
    pub fn font_normal(mut self) -> Self {
        self.font_normal = true;
        self
    }

    /// group-data-[disabled=true]/field:opacity-50.
    // Mirrors Label::disabled; not yet exercised by the storybook.
    #[allow(dead_code)]
    pub fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }
}

impl Default for FieldLabel {
    fn default() -> Self {
        Self::new()
    }
}

impl ParentElement for FieldLabel {
    fn extend(&mut self, elements: impl IntoIterator<Item = AnyElement>) {
        self.children.extend(elements);
    }
}

impl RenderOnce for FieldLabel {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let theme = Theme::of(cx);
        // leading-snug: 1.375 × 14px
        let base = div()
            .flex()
            .flex_row()
            .items_center()
            .gap(px(8.))
            .flex_grow(1.)
            .text_size(px(14.))
            .line_height(px(19.25))
            .font_weight(if self.font_normal {
                FontWeight::NORMAL
            } else {
                FontWeight::MEDIUM
            })
            .when(self.disabled, |el| el.opacity(0.5));
        let base = if self.choice_card {
            // has-[>[data-slot=field]]:w-full flex-col rounded-lg border;
            // the source pads the nested field (*:data-[slot=field]:p-2.5) —
            // padding the card is the same box.
            let (border_alpha, bg_alpha) = if theme.dark { (0.2, 0.1) } else { (0.3, 0.05) };
            base.w_full()
                .flex_col()
                .items_start()
                .rounded(theme.radius_lg())
                .border_1()
                .p(px(10.))
                .map(|el| {
                    if self.checked {
                        el.border_color(theme.primary.opacity(border_alpha))
                            .bg(theme.primary.opacity(bg_alpha))
                    } else {
                        el.border_color(theme.border)
                    }
                })
        } else {
            base
        };
        base.children(self.children)
    }
}

/// group/field-content flex flex-1 flex-col gap-0.5 — stacks a title/label
/// and description beside a control.
#[derive(IntoElement)]
pub struct FieldContent {
    children: Vec<AnyElement>,
}

impl FieldContent {
    pub fn new() -> Self {
        Self {
            children: Vec::new(),
        }
    }
}

impl Default for FieldContent {
    fn default() -> Self {
        Self::new()
    }
}

impl ParentElement for FieldContent {
    fn extend(&mut self, elements: impl IntoIterator<Item = AnyElement>) {
        self.children.extend(elements);
    }
}

impl RenderOnce for FieldContent {
    fn render(self, _window: &mut Window, _cx: &mut App) -> impl IntoElement {
        div()
            .flex()
            .flex_col()
            .flex_1()
            .gap(px(2.))
            .children(self.children)
    }
}

/// flex w-fit items-center gap-2 text-sm font-medium — label-styled title
/// inside a [`FieldContent`] (used when the row's real label is elsewhere,
/// e.g. choice cards). Inherits text color for the invalid cascade.
#[derive(IntoElement)]
pub struct FieldTitle {
    children: Vec<AnyElement>,
}

impl FieldTitle {
    pub fn new() -> Self {
        Self {
            children: Vec::new(),
        }
    }
}

impl Default for FieldTitle {
    fn default() -> Self {
        Self::new()
    }
}

impl ParentElement for FieldTitle {
    fn extend(&mut self, elements: impl IntoIterator<Item = AnyElement>) {
        self.children.extend(elements);
    }
}

impl RenderOnce for FieldTitle {
    fn render(self, _window: &mut Window, _cx: &mut App) -> impl IntoElement {
        div()
            .flex()
            .flex_row()
            .items_center()
            .gap(px(8.))
            .text_size(px(14.))
            .line_height(px(20.))
            .font_weight(FontWeight::MEDIUM)
            .children(self.children)
    }
}

/// text-sm leading-normal font-normal text-muted-foreground.
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
        // leading-normal: 1.5 × 14px
        div()
            .text_size(px(14.))
            .line_height(px(21.))
            .font_weight(FontWeight::NORMAL)
            .text_color(theme.muted_foreground)
            .children(self.children)
    }
}

/// text-sm font-normal text-destructive — validation feedback. One error
/// renders as a line; several render as a disc list (duplicates dropped),
/// matching the TSX `errors` array handling.
#[derive(IntoElement)]
pub struct FieldError {
    errors: Vec<SharedString>,
    children: Vec<AnyElement>,
}

impl FieldError {
    pub fn new() -> Self {
        Self {
            errors: Vec::new(),
            children: Vec::new(),
        }
    }

    /// The TSX `errors={...}` prop: explicit children win over it, one
    /// unique message renders plain, several render as a bullet list.
    pub fn errors(mut self, errors: impl IntoIterator<Item = impl Into<SharedString>>) -> Self {
        self.errors = errors.into_iter().map(Into::into).collect();
        self
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
        let base = div()
            .text_size(px(14.))
            .line_height(px(20.))
            .font_weight(FontWeight::NORMAL)
            .text_color(theme.destructive);
        if !self.children.is_empty() || self.errors.is_empty() {
            return base.children(self.children);
        }
        let mut unique: Vec<SharedString> = Vec::new();
        for error in self.errors {
            if !unique.contains(&error) {
                unique.push(error);
            }
        }
        if unique.len() == 1 {
            return base.child(unique.remove(0));
        }
        // ml-4 flex list-disc flex-col gap-1
        base.child(
            div()
                .ml(px(16.))
                .flex()
                .flex_col()
                .gap(px(4.))
                .children(unique.into_iter().map(|message| {
                    div()
                        .flex()
                        .flex_row()
                        .gap(px(8.))
                        .child("•")
                        .child(message)
                })),
        )
    }
}

/// group/field-group flex w-full flex-col gap-5 — stacks fields into a form
/// section. `.gap(..)` covers the docs' tighter checkbox-group stacks.
#[derive(IntoElement)]
pub struct FieldGroup {
    gap: Pixels,
    children: Vec<AnyElement>,
}

impl FieldGroup {
    pub fn new() -> Self {
        Self {
            gap: px(20.),
            children: Vec::new(),
        }
    }

    /// Overrides the 20px stack gap (the docs use 12px —
    /// `data-[slot=checkbox-group]:gap-3` — for checkbox stacks).
    pub fn gap(mut self, gap: Pixels) -> Self {
        self.gap = gap;
        self
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
        div()
            .flex()
            .flex_col()
            .w_full()
            .gap(self.gap)
            .children(self.children)
    }
}

/// flex flex-col gap-4 — semantic grouping under a legend. Unlike base-vega
/// this carries no border or padding. The browser floats a `<legend>` out of
/// the fieldset's flex flow, so the measured layout is legend → description
/// at 0px (legend variant) or 2px (label variant), then the 16px gap to
/// everything else; `.legend(..)`/`.description(..)` reproduce that.
#[derive(IntoElement)]
pub struct FieldSet {
    legend: Option<FieldLegend>,
    description: Option<FieldDescription>,
    gap: Pixels,
    children: Vec<AnyElement>,
}

impl FieldSet {
    pub fn new() -> Self {
        Self {
            legend: None,
            description: None,
            gap: px(16.),
            children: Vec::new(),
        }
    }

    pub fn legend(mut self, legend: FieldLegend) -> Self {
        self.legend = Some(legend);
        self
    }

    /// The description line under the legend.
    pub fn description(mut self, description: FieldDescription) -> Self {
        self.description = Some(description);
        self
    }

    /// Overrides the 16px gap (the docs' checkbox/radio fieldsets tighten to
    /// 12px — `has-[>[data-slot=radio-group]]:gap-3`).
    pub fn gap(mut self, gap: Pixels) -> Self {
        self.gap = gap;
        self
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
    fn render(self, _window: &mut Window, _cx: &mut App) -> impl IntoElement {
        let header_gap = match &self.legend {
            Some(legend) if legend.variant == FieldLegendVariant::Label => px(2.),
            _ => px(0.),
        };
        let header = match (self.legend, self.description) {
            (None, None) => None,
            (legend, description) => Some(
                div()
                    .flex()
                    .flex_col()
                    .gap(header_gap)
                    .children(legend.map(IntoElement::into_any_element))
                    .children(description.map(IntoElement::into_any_element)),
            ),
        };
        div()
            .flex()
            .flex_col()
            .gap(self.gap)
            .children(header)
            .children(self.children)
    }
}

/// `variant` of [`FieldLegend`].
#[derive(Clone, Copy, PartialEq, Eq, Default, Debug)]
pub enum FieldLegendVariant {
    /// text-base — the fieldset heading size.
    #[default]
    Legend,
    /// text-sm — label sizing, for nested fieldsets.
    Label,
}

/// font-medium, text-base or text-sm by variant — the [`FieldSet`] heading.
#[derive(IntoElement)]
pub struct FieldLegend {
    variant: FieldLegendVariant,
    children: Vec<AnyElement>,
}

impl FieldLegend {
    pub fn new() -> Self {
        Self {
            variant: FieldLegendVariant::default(),
            children: Vec::new(),
        }
    }

    pub fn variant(mut self, variant: FieldLegendVariant) -> Self {
        self.variant = variant;
        self
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
        let (size, line_height) = match self.variant {
            FieldLegendVariant::Legend => (px(16.), px(24.)),
            FieldLegendVariant::Label => (px(14.), px(20.)),
        };
        div()
            .text_size(size)
            .line_height(line_height)
            .font_weight(FontWeight::MEDIUM)
            .when_some(theme.heading_font(), |el, font| el.font_family(font))
            .children(self.children)
    }
}

/// relative -my-2 h-5 — a rule between [`FieldGroup`] sections; children
/// render centered over it on a background chip ("Or continue with").
#[derive(IntoElement)]
pub struct FieldSeparator {
    children: Vec<AnyElement>,
}

impl FieldSeparator {
    pub fn new() -> Self {
        Self {
            children: Vec::new(),
        }
    }
}

impl Default for FieldSeparator {
    fn default() -> Self {
        Self::new()
    }
}

impl ParentElement for FieldSeparator {
    fn extend(&mut self, elements: impl IntoIterator<Item = AnyElement>) {
        self.children.extend(elements);
    }
}

impl RenderOnce for FieldSeparator {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let theme = Theme::of(cx);
        div()
            .relative()
            .h(px(20.))
            .mt(px(-8.))
            .mb(px(-8.))
            .w_full()
            // Separator absolute inset-0 top-1/2
            .child(
                div()
                    .absolute()
                    .top(px(10.))
                    .left_0()
                    .right_0()
                    .h(px(1.))
                    .bg(theme.border),
            )
            .when(!self.children.is_empty(), |el| {
                // relative mx-auto block w-fit bg-background px-2
                el.child(
                    div().relative().flex().justify_center().child(
                        div()
                            .bg(theme.background)
                            .px(px(8.))
                            .text_size(px(14.))
                            .line_height(px(20.))
                            .text_color(theme.muted_foreground)
                            .children(self.children),
                    ),
                )
            })
    }
}
