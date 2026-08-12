//! Table — port of shadcn base-vega `ui/table.tsx`.
//!
//! gpui has no HTML table layout, so rows are flex rows and column sizing
//! comes from the cells: `TableHead`/`TableCell` default to `flex: 1` and
//! take an optional fixed `.w(px)` — give the same widths to matching
//! header/body cells to keep columns aligned. Selected/hover row states
//! mirror the source's `data-[state=selected]` and `hover:` styles.
//!
//! Sizing and shape overrides come from the caller via [`Styled`] (gpui's
//! equivalent of shadcn's `className` passthrough) on each table part.

use gpui::{
    AnyElement, App, FontWeight, InteractiveElement as _, IntoElement, ParentElement, Pixels,
    Refineable as _, RenderOnce, StyleRefinement, Styled, Window, div, prelude::FluentBuilder as _,
    px,
};

use crate::theme::{Theme, alpha};

/// w-full text-sm — the outer table container.
///
/// Sizing and shape overrides come from the caller via [`Styled`].
#[derive(IntoElement)]
pub struct Table {
    style: StyleRefinement,
    children: Vec<AnyElement>,
}

impl Table {
    pub fn new() -> Self {
        Self {
            style: StyleRefinement::default(),
            children: Vec::new(),
        }
    }
}

impl Default for Table {
    fn default() -> Self {
        Self::new()
    }
}

impl Styled for Table {
    fn style(&mut self) -> &mut StyleRefinement {
        &mut self.style
    }
}

impl ParentElement for Table {
    fn extend(&mut self, elements: impl IntoIterator<Item = AnyElement>) {
        self.children.extend(elements);
    }
}

impl RenderOnce for Table {
    fn render(self, _window: &mut Window, _cx: &mut App) -> impl IntoElement {
        let mut root = div()
            .flex()
            .flex_col()
            .w_full()
            .text_size(px(14.))
            .line_height(px(20.))
            .children(self.children);
        root.style().refine(&self.style);
        root
    }
}

/// thead: rows keep their bottom border.
///
/// Sizing and shape overrides come from the caller via [`Styled`].
#[derive(IntoElement)]
pub struct TableHeader {
    style: StyleRefinement,
    children: Vec<AnyElement>,
}

impl TableHeader {
    pub fn new() -> Self {
        Self {
            style: StyleRefinement::default(),
            children: Vec::new(),
        }
    }
}

impl Default for TableHeader {
    fn default() -> Self {
        Self::new()
    }
}

impl Styled for TableHeader {
    fn style(&mut self) -> &mut StyleRefinement {
        &mut self.style
    }
}

impl ParentElement for TableHeader {
    fn extend(&mut self, elements: impl IntoIterator<Item = AnyElement>) {
        self.children.extend(elements);
    }
}

impl RenderOnce for TableHeader {
    fn render(self, _window: &mut Window, _cx: &mut App) -> impl IntoElement {
        let mut root = div().flex().flex_col().children(self.children);
        root.style().refine(&self.style);
        root
    }
}

/// tbody.
///
/// Sizing and shape overrides come from the caller via [`Styled`].
#[derive(IntoElement)]
pub struct TableBody {
    style: StyleRefinement,
    children: Vec<AnyElement>,
}

impl TableBody {
    pub fn new() -> Self {
        Self {
            style: StyleRefinement::default(),
            children: Vec::new(),
        }
    }
}

impl Default for TableBody {
    fn default() -> Self {
        Self::new()
    }
}

impl Styled for TableBody {
    fn style(&mut self) -> &mut StyleRefinement {
        &mut self.style
    }
}

impl ParentElement for TableBody {
    fn extend(&mut self, elements: impl IntoIterator<Item = AnyElement>) {
        self.children.extend(elements);
    }
}

impl RenderOnce for TableBody {
    fn render(self, _window: &mut Window, _cx: &mut App) -> impl IntoElement {
        let mut root = div().flex().flex_col().children(self.children);
        root.style().refine(&self.style);
        root
    }
}

/// tfoot: border-t bg-muted/50 font-medium.
///
/// Sizing and shape overrides come from the caller via [`Styled`].
#[derive(IntoElement)]
pub struct TableFooter {
    style: StyleRefinement,
    children: Vec<AnyElement>,
}

impl TableFooter {
    pub fn new() -> Self {
        Self {
            style: StyleRefinement::default(),
            children: Vec::new(),
        }
    }
}

impl Default for TableFooter {
    fn default() -> Self {
        Self::new()
    }
}

impl Styled for TableFooter {
    fn style(&mut self) -> &mut StyleRefinement {
        &mut self.style
    }
}

impl ParentElement for TableFooter {
    fn extend(&mut self, elements: impl IntoIterator<Item = AnyElement>) {
        self.children.extend(elements);
    }
}

impl RenderOnce for TableFooter {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let theme = Theme::of(cx);
        let mut root = div()
            .flex()
            .flex_col()
            .border_t_1()
            .border_color(theme.border)
            .bg(alpha(theme.muted, 0.5))
            .font_weight(FontWeight::MEDIUM)
            .children(self.children);
        root.style().refine(&self.style);
        root
    }
}

/// tr: border-b hover:bg-muted/50 data-[state=selected]:bg-muted.
///
/// Sizing and shape overrides come from the caller via [`Styled`].
#[derive(IntoElement)]
pub struct TableRow {
    id: Option<gpui::ElementId>,
    selected: bool,
    last: bool,
    style: StyleRefinement,
    children: Vec<AnyElement>,
}

impl TableRow {
    pub fn new() -> Self {
        Self {
            id: None,
            selected: false,
            last: false,
            style: StyleRefinement::default(),
            children: Vec::new(),
        }
    }

    /// Give the row an id to enable the hover background.
    pub fn id(mut self, id: impl Into<gpui::ElementId>) -> Self {
        self.id = Some(id.into());
        self
    }

    pub fn selected(mut self, selected: bool) -> Self {
        self.selected = selected;
        self
    }

    /// The source drops the border on `tbody tr:last-child`.
    pub fn last(mut self, last: bool) -> Self {
        self.last = last;
        self
    }
}

impl Default for TableRow {
    fn default() -> Self {
        Self::new()
    }
}

impl Styled for TableRow {
    fn style(&mut self) -> &mut StyleRefinement {
        &mut self.style
    }
}

impl ParentElement for TableRow {
    fn extend(&mut self, elements: impl IntoIterator<Item = AnyElement>) {
        self.children.extend(elements);
    }
}

impl RenderOnce for TableRow {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let theme = Theme::of(cx).clone();
        let base = div()
            .flex()
            .flex_row()
            .w_full()
            .items_center()
            .when(!self.last, |el| el.border_b_1().border_color(theme.border))
            .when(self.selected, |el| el.bg(theme.muted));
        match self.id {
            Some(id) => {
                let hover_bg = alpha(theme.muted, 0.5);
                let mut root = base
                    .id(id)
                    .hover(move |s| s.bg(hover_bg))
                    .children(self.children);
                root.style().refine(&self.style);
                root.into_any_element()
            }
            None => {
                let mut root = base.children(self.children);
                root.style().refine(&self.style);
                root.into_any_element()
            }
        }
    }
}

/// th: h-10 px-2 text-left font-medium text-foreground.
///
/// Sizing and shape overrides come from the caller via [`Styled`].
/// The inherent [`TableHead::w`] builder still sets fixed column width.
#[derive(IntoElement)]
pub struct TableHead {
    width: Option<Pixels>,
    style: StyleRefinement,
    children: Vec<AnyElement>,
}

impl TableHead {
    pub fn new() -> Self {
        Self {
            width: None,
            style: StyleRefinement::default(),
            children: Vec::new(),
        }
    }

    /// Fixed column width; unset columns share remaining space equally.
    pub fn w(mut self, width: Pixels) -> Self {
        self.width = Some(width);
        self
    }
}

impl Default for TableHead {
    fn default() -> Self {
        Self::new()
    }
}

impl Styled for TableHead {
    fn style(&mut self) -> &mut StyleRefinement {
        &mut self.style
    }
}

impl ParentElement for TableHead {
    fn extend(&mut self, elements: impl IntoIterator<Item = AnyElement>) {
        self.children.extend(elements);
    }
}

impl RenderOnce for TableHead {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let theme = Theme::of(cx);
        let mut root = div()
            .flex()
            .flex_row()
            .items_center()
            .h(px(40.))
            .px(px(8.))
            .whitespace_nowrap()
            .font_weight(FontWeight::MEDIUM)
            .text_color(theme.foreground)
            .map(|el| match self.width {
                Some(width) => el.w(width).flex_shrink_0(),
                None => el.flex_1(),
            })
            .children(self.children);
        root.style().refine(&self.style);
        root
    }
}

/// td: p-2 align-middle.
///
/// Sizing and shape overrides come from the caller via [`Styled`].
/// The inherent [`TableCell::w`] builder still sets fixed column width.
#[derive(IntoElement)]
pub struct TableCell {
    width: Option<Pixels>,
    style: StyleRefinement,
    children: Vec<AnyElement>,
}

impl TableCell {
    pub fn new() -> Self {
        Self {
            width: None,
            style: StyleRefinement::default(),
            children: Vec::new(),
        }
    }

    /// Fixed column width matching the corresponding [`TableHead`].
    pub fn w(mut self, width: Pixels) -> Self {
        self.width = Some(width);
        self
    }
}

impl Default for TableCell {
    fn default() -> Self {
        Self::new()
    }
}

impl Styled for TableCell {
    fn style(&mut self) -> &mut StyleRefinement {
        &mut self.style
    }
}

impl ParentElement for TableCell {
    fn extend(&mut self, elements: impl IntoIterator<Item = AnyElement>) {
        self.children.extend(elements);
    }
}

impl RenderOnce for TableCell {
    fn render(self, _window: &mut Window, _cx: &mut App) -> impl IntoElement {
        let mut root = div()
            .flex()
            .flex_row()
            .items_center()
            .p(px(8.))
            .whitespace_nowrap()
            .map(|el| match self.width {
                Some(width) => el.w(width).flex_shrink_0(),
                None => el.flex_1(),
            })
            .children(self.children);
        root.style().refine(&self.style);
        root
    }
}

/// caption: mt-4 text-sm text-muted-foreground (rendered below the table).
///
/// Sizing and shape overrides come from the caller via [`Styled`].
#[derive(IntoElement)]
pub struct TableCaption {
    style: StyleRefinement,
    children: Vec<AnyElement>,
}

impl TableCaption {
    pub fn new() -> Self {
        Self {
            style: StyleRefinement::default(),
            children: Vec::new(),
        }
    }
}

impl Default for TableCaption {
    fn default() -> Self {
        Self::new()
    }
}

impl Styled for TableCaption {
    fn style(&mut self) -> &mut StyleRefinement {
        &mut self.style
    }
}

impl ParentElement for TableCaption {
    fn extend(&mut self, elements: impl IntoIterator<Item = AnyElement>) {
        self.children.extend(elements);
    }
}

impl RenderOnce for TableCaption {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let theme = Theme::of(cx);
        let mut root = div()
            .mt(px(16.))
            .text_size(px(14.))
            .line_height(px(20.))
            .text_color(theme.muted_foreground)
            .children(self.children);
        root.style().refine(&self.style);
        root
    }
}
