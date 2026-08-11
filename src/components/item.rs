//! Item — port of shadcn base-nova `ui/item.tsx`.
//!
//! A flexible list row: Media (plain / icon / image tile) + Content
//! (Title, Description) + Actions, with Header/Footer spanning rows,
//! grouped by ItemGroup with ItemSeparator between rows.
//!
//! Interactive/link mode (parity with shadcn `render={<a/>}`): set
//! [`Item::id`] to make the row focusable and clickable. Hover uses an
//! instant `theme.muted` swap — source `[a]:hover:bg-muted` /
//! `transition-colors duration-100` cannot animate under gpui hover
//! styles (same TODO convention as Button).

use gpui::{
    AnyElement, App, ClickEvent, ElementId, FontWeight, InteractiveElement as _, IntoElement,
    ParentElement, RenderOnce, StatefulInteractiveElement as _, Styled, Window, div,
    prelude::FluentBuilder as _, px,
};

use crate::components::separator::Separator;
use crate::motion;
use crate::theme::{Theme, alpha};

#[derive(Clone, Copy, PartialEq, Eq, Default, Debug)]
pub enum ItemVariant {
    #[default]
    Default,
    Outline,
    Muted,
}

#[derive(Clone, Copy, PartialEq, Eq, Default, Debug)]
pub enum ItemSize {
    #[default]
    Default,
    Sm,
    Xs,
}

impl ItemSize {
    /// gap / horizontal padding / vertical padding per size.
    ///
    /// base-nova `itemVariants` sizes: default and sm both use
    /// `gap-2.5 px-3 py-2.5` — arms kept separate for clarity.
    fn metrics(self) -> (f32, f32, f32) {
        match self {
            ItemSize::Default => (10., 12., 10.),
            // Identical to Default in base-nova (`gap-2.5 px-3 py-2.5`).
            ItemSize::Sm => (10., 12., 10.),
            ItemSize::Xs => (8., 10., 8.),
        }
    }
}

/// flex w-full flex-wrap items-center rounded-lg border text-sm
#[derive(IntoElement)]
pub struct Item {
    variant: ItemVariant,
    size: ItemSize,
    /// When true, zero padding — source `in-data-[slot=dropdown-menu-content]:p-0`.
    flush: bool,
    /// When set, the item is interactive (link/button parity with shadcn `render={<a/>}`).
    id: Option<ElementId>,
    on_click: Option<Box<dyn Fn(&ClickEvent, &mut Window, &mut App) + 'static>>,
    children: Vec<AnyElement>,
}

impl Item {
    pub fn new() -> Self {
        Self {
            variant: ItemVariant::default(),
            size: ItemSize::default(),
            flush: false,
            id: None,
            on_click: None,
            children: Vec::new(),
        }
    }

    pub fn variant(mut self, variant: ItemVariant) -> Self {
        self.variant = variant;
        self
    }

    pub fn size(mut self, size: ItemSize) -> Self {
        self.size = size;
        self
    }

    /// Zero out padding — source class `in-data-[slot=dropdown-menu-content]:p-0`.
    /// Used by the dropdown storybook example where the item sits inside a menu.
    pub fn flush(mut self, flush: bool) -> Self {
        self.flush = flush;
        self
    }

    /// Make the item interactive (focusable + clickable). Source: shadcn
    /// `render={<a/>}` / `[a]:hover:bg-muted` + focus-visible ring.
    pub fn id(mut self, id: impl Into<ElementId>) -> Self {
        self.id = Some(id.into());
        self
    }

    pub fn on_click(
        mut self,
        handler: impl Fn(&ClickEvent, &mut Window, &mut App) + 'static,
    ) -> Self {
        self.on_click = Some(Box::new(handler));
        self
    }
}

impl Default for Item {
    fn default() -> Self {
        Self::new()
    }
}

impl ParentElement for Item {
    fn extend(&mut self, elements: impl IntoIterator<Item = AnyElement>) {
        self.children.extend(elements);
    }
}

impl RenderOnce for Item {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let theme = Theme::of(cx).clone();
        let (gap, padding_x, padding_y) = self.size.metrics();
        let (padding_x, padding_y) = if self.flush {
            (0., 0.)
        } else {
            (padding_x, padding_y)
        };

        // Shared layout: group/item flex w-full flex-wrap items-center rounded-lg
        // border text-sm. Root radius is radius_lg (base-nova).
        let base = div()
            .flex()
            .flex_row()
            .flex_wrap()
            .w_full()
            .items_center()
            .rounded(theme.radius_lg())
            .border_1()
            .border_color(gpui::transparent_black())
            .gap(px(gap))
            .px(px(padding_x))
            .py(px(padding_y))
            .text_size(px(14.))
            .line_height(px(20.));

        let base = match self.variant {
            ItemVariant::Default => base,
            ItemVariant::Outline => base.border_color(theme.border),
            ItemVariant::Muted => base.bg(alpha(theme.muted, 0.5)),
        };

        // Interactive/link mode (shadcn `render={<a/>}`): `.id()` turns the
        // Div into Stateful<Div>, so the interactive branch is separate.
        // Pattern from Button / SidebarMenuButton: id + tab_index + focus
        // ring + cursor + hover + on_click. Enter/Space activate via gpui's
        // default focused-element click handling once tab_index is set
        // (same as Button — no explicit key handler).
        // TODO: source `transition-colors duration-100` — gpui hover styles
        // swap instantly; cannot animate color transitions.
        if let Some(id) = self.id {
            let ring = motion::focus_ring(&theme);
            let ring_border = theme.ring;
            let muted = theme.muted;
            base.id(id)
                .tab_index(0)
                .cursor_pointer()
                .focus_visible(move |s| s.border_color(ring_border).shadow(ring.clone()))
                .hover(move |s| s.bg(muted))
                .when_some(self.on_click, |el, on_click| el.on_click(on_click))
                .children(self.children)
                .into_any_element()
        } else {
            base.children(self.children).into_any_element()
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Default, Debug)]
pub enum ItemMediaVariant {
    #[default]
    Default,
    /// Icon slot. Source sizes unsized SVGs via
    /// `[&_svg:not([class*='size-'])]:size-4` — unportable descendant
    /// selector; callers must size their SVGs explicitly (typically
    /// `px(16.)`, or `px(20.)` where the source uses `size-5`).
    Icon,
    /// size-10 rounded-sm image tile (sm → size-8, xs → size-6 via
    /// [`ItemMedia::size`]).
    Image,
}

/// flex shrink-0 items-center justify-center gap-2
#[derive(IntoElement)]
pub struct ItemMedia {
    variant: ItemMediaVariant,
    /// Item size — drives image-tile dimensions
    /// (`size-10` / `group-data-[size=sm]/item:size-8` /
    /// `group-data-[size=xs]/item:size-6`).
    size: ItemSize,
    /// shadcn auto-applies `group-has-data-[slot=item-description]/item:self-start`
    /// + `translate-y-0.5` when a description is present. CSS context
    /// selectors are unportable and children are opaque `AnyElement`s,
    /// so callers set this explicitly (storybook does so wherever an
    /// item has both media and a description).
    top_align: bool,
    children: Vec<AnyElement>,
}

impl ItemMedia {
    pub fn new() -> Self {
        Self {
            variant: ItemMediaVariant::default(),
            size: ItemSize::default(),
            top_align: false,
            children: Vec::new(),
        }
    }

    pub fn variant(mut self, variant: ItemMediaVariant) -> Self {
        self.variant = variant;
        self
    }

    /// Image-tile size from the parent item size. Source:
    /// `size-10 ... group-data-[size=sm]/item:size-8 group-data-[size=xs]/item:size-6`.
    /// Only affects [`ItemMediaVariant::Image`]; default is [`ItemSize::Default`].
    pub fn size(mut self, size: ItemSize) -> Self {
        self.size = size;
        self
    }

    /// Top-align media when the item has a description.
    /// Source: `group-has-data-[slot=item-description]/item:self-start`
    /// + `translate-y-0.5` — applied as `.self_start().mt(px(2.))`.
    /// shadcn does this automatically; storybook sets it wherever a
    /// description is present.
    pub fn top_align(mut self, top_align: bool) -> Self {
        self.top_align = top_align;
        self
    }
}

impl Default for ItemMedia {
    fn default() -> Self {
        Self::new()
    }
}

impl ParentElement for ItemMedia {
    fn extend(&mut self, elements: impl IntoIterator<Item = AnyElement>) {
        self.children.extend(elements);
    }
}

impl RenderOnce for ItemMedia {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let theme = Theme::of(cx);
        // Image tile sizes: Default 40, Sm 32, Xs 24 (size-10/8/6).
        let image_size = match self.size {
            ItemSize::Default => px(40.),
            ItemSize::Sm => px(32.),
            ItemSize::Xs => px(24.),
        };
        div()
            .flex()
            .flex_shrink_0()
            .items_center()
            .justify_center()
            .gap(px(8.))
            .when(self.top_align, |el| el.self_start().mt(px(2.)))
            .when(self.variant == ItemMediaVariant::Image, |el| {
                el.size(image_size)
                    .overflow_hidden()
                    .rounded(theme.radius_sm())
            })
            .children(self.children)
    }
}

/// flex flex-1 flex-col gap-1 group-data-[size=xs]/item:gap-0
/// [&+[data-slot=item-content]]:flex-none
#[derive(IntoElement)]
pub struct ItemContent {
    /// Parent item size — Xs collapses gap to 0
    /// (`group-data-[size=xs]/item:gap-0`).
    size: ItemSize,
    /// Source `[&+[data-slot=item-content]]:flex-none` — a trailing
    /// sibling content column (e.g. duration) opts out of flex-1.
    flex_none: bool,
    children: Vec<AnyElement>,
}

impl ItemContent {
    pub fn new() -> Self {
        Self {
            size: ItemSize::default(),
            flex_none: false,
            children: Vec::new(),
        }
    }

    /// Parent item size. Source: `group-data-[size=xs]/item:gap-0`
    /// (Xs → gap 0, else gap 4px / `gap-1`).
    pub fn size(mut self, size: ItemSize) -> Self {
        self.size = size;
        self
    }

    /// Replace `flex-1` with `flex-none` for trailing content columns.
    /// Source: `[&+[data-slot=item-content]]:flex-none`.
    pub fn flex_none(mut self, flex_none: bool) -> Self {
        self.flex_none = flex_none;
        self
    }
}

impl Default for ItemContent {
    fn default() -> Self {
        Self::new()
    }
}

impl ParentElement for ItemContent {
    fn extend(&mut self, elements: impl IntoIterator<Item = AnyElement>) {
        self.children.extend(elements);
    }
}

impl RenderOnce for ItemContent {
    fn render(self, _window: &mut Window, _cx: &mut App) -> impl IntoElement {
        // Xs → gap-0; else gap-1 (4px). Source:
        // `flex flex-1 flex-col gap-1 group-data-[size=xs]/item:gap-0`.
        let gap = match self.size {
            ItemSize::Xs => px(0.),
            _ => px(4.),
        };
        div()
            .flex()
            .flex_col()
            .when(self.flex_none, |el| el.flex_none())
            // min_w(0): let the column shrink below its content so long
            // titles truncate instead of pushing siblings onto the item's
            // next flex-wrap line (CSS gets this via line-clamp overflow).
            .when(!self.flex_none, |el| el.flex_1().min_w(px(0.)))
            .gap(gap)
            .children(self.children)
    }
}

/// line-clamp-1 flex w-fit items-center gap-2 text-sm font-medium
#[derive(IntoElement)]
pub struct ItemTitle {
    children: Vec<AnyElement>,
}

impl ItemTitle {
    pub fn new() -> Self {
        Self {
            children: Vec::new(),
        }
    }
}

impl Default for ItemTitle {
    fn default() -> Self {
        Self::new()
    }
}

impl ParentElement for ItemTitle {
    fn extend(&mut self, elements: impl IntoIterator<Item = AnyElement>) {
        self.children.extend(elements);
    }
}

impl RenderOnce for ItemTitle {
    fn render(self, _window: &mut Window, _cx: &mut App) -> impl IntoElement {
        // Source `line-clamp-1`. Applied on the flex-row title container.
        div()
            .flex()
            .flex_row()
            .items_center()
            .gap(px(8.))
            .text_size(px(14.))
            .line_height(px(19.))
            .font_weight(FontWeight::MEDIUM)
            .line_clamp(1)
            .children(self.children)
    }
}

/// line-clamp-2 text-sm text-muted-foreground
/// group-data-[size=xs]/item:text-xs
#[derive(IntoElement)]
pub struct ItemDescription {
    /// Parent item size — Xs uses text-xs (12/16). Source:
    /// `group-data-[size=xs]/item:text-xs`.
    size: ItemSize,
    children: Vec<AnyElement>,
}

impl ItemDescription {
    pub fn new() -> Self {
        Self {
            size: ItemSize::default(),
            children: Vec::new(),
        }
    }

    /// Parent item size. Source: `group-data-[size=xs]/item:text-xs`
    /// (Xs → 12px/16px line-height; else 14px/21px).
    pub fn size(mut self, size: ItemSize) -> Self {
        self.size = size;
        self
    }
}

impl Default for ItemDescription {
    fn default() -> Self {
        Self::new()
    }
}

impl ParentElement for ItemDescription {
    fn extend(&mut self, elements: impl IntoIterator<Item = AnyElement>) {
        self.children.extend(elements);
    }
}

impl RenderOnce for ItemDescription {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let theme = Theme::of(cx);
        // Xs → text-xs (12/16); else text-sm (14/21).
        let (text_size, line_height) = match self.size {
            ItemSize::Xs => (px(12.), px(16.)),
            _ => (px(14.), px(21.)),
        };
        // Source `line-clamp-2`.
        div()
            .text_size(text_size)
            .line_height(line_height)
            .text_color(theme.muted_foreground)
            .line_clamp(2)
            .children(self.children)
    }
}

/// flex items-center gap-2 — trailing action cluster.
#[derive(IntoElement)]
pub struct ItemActions {
    children: Vec<AnyElement>,
}

impl ItemActions {
    pub fn new() -> Self {
        Self {
            children: Vec::new(),
        }
    }
}

impl Default for ItemActions {
    fn default() -> Self {
        Self::new()
    }
}

impl ParentElement for ItemActions {
    fn extend(&mut self, elements: impl IntoIterator<Item = AnyElement>) {
        self.children.extend(elements);
    }
}

impl RenderOnce for ItemActions {
    fn render(self, _window: &mut Window, _cx: &mut App) -> impl IntoElement {
        div()
            .flex()
            .flex_row()
            .items_center()
            .gap(px(8.))
            .children(self.children)
    }
}

/// flex basis-full items-center justify-between gap-2
#[derive(IntoElement)]
pub struct ItemHeader {
    children: Vec<AnyElement>,
}

impl ItemHeader {
    pub fn new() -> Self {
        Self {
            children: Vec::new(),
        }
    }
}

impl Default for ItemHeader {
    fn default() -> Self {
        Self::new()
    }
}

impl ParentElement for ItemHeader {
    fn extend(&mut self, elements: impl IntoIterator<Item = AnyElement>) {
        self.children.extend(elements);
    }
}

impl RenderOnce for ItemHeader {
    fn render(self, _window: &mut Window, _cx: &mut App) -> impl IntoElement {
        div()
            .flex()
            .flex_row()
            .w_full()
            .items_center()
            .justify_between()
            .gap(px(8.))
            .children(self.children)
    }
}

/// flex basis-full items-center justify-between gap-2
/// Public API surface matching shadcn `ItemFooter`; not used by the current
/// docs examples (kept for parity).
#[allow(dead_code)]
#[derive(IntoElement)]
pub struct ItemFooter {
    children: Vec<AnyElement>,
}

#[allow(dead_code)]
impl ItemFooter {
    pub fn new() -> Self {
        Self {
            children: Vec::new(),
        }
    }
}

impl Default for ItemFooter {
    fn default() -> Self {
        Self::new()
    }
}

impl ParentElement for ItemFooter {
    fn extend(&mut self, elements: impl IntoIterator<Item = AnyElement>) {
        self.children.extend(elements);
    }
}

impl RenderOnce for ItemFooter {
    fn render(self, _window: &mut Window, _cx: &mut App) -> impl IntoElement {
        div()
            .flex()
            .flex_row()
            .w_full()
            .items_center()
            .justify_between()
            .gap(px(8.))
            .children(self.children)
    }
}

/// flex w-full flex-col gap-4 — the list container.
/// Source also has `has-data-[size=sm]:gap-2.5 has-data-[size=xs]:gap-2`;
/// shadcn derives gap from child item sizes automatically via context
/// selectors, which are unportable — callers set [`ItemGroup::size`]
/// explicitly to match the child item size.
#[derive(IntoElement)]
pub struct ItemGroup {
    /// Child item size — drives inter-item gap
    /// (`gap-4` / `has-data-[size=sm]:gap-2.5` / `has-data-[size=xs]:gap-2`).
    size: ItemSize,
    children: Vec<AnyElement>,
}

impl ItemGroup {
    pub fn new() -> Self {
        Self {
            size: ItemSize::default(),
            children: Vec::new(),
        }
    }

    /// Inter-item gap from child item size. Source:
    /// `gap-4 has-data-[size=sm]:gap-2.5 has-data-[size=xs]:gap-2`
    /// (Default 16px, Sm 10px, Xs 8px). shadcn derives this from child
    /// sizes automatically; set explicitly here.
    pub fn size(mut self, size: ItemSize) -> Self {
        self.size = size;
        self
    }
}

impl Default for ItemGroup {
    fn default() -> Self {
        Self::new()
    }
}

impl ParentElement for ItemGroup {
    fn extend(&mut self, elements: impl IntoIterator<Item = AnyElement>) {
        self.children.extend(elements);
    }
}

impl RenderOnce for ItemGroup {
    fn render(self, _window: &mut Window, _cx: &mut App) -> impl IntoElement {
        // Default gap-4 (16); Sm gap-2.5 (10); Xs gap-2 (8).
        let gap = match self.size {
            ItemSize::Default => px(16.),
            ItemSize::Sm => px(10.),
            ItemSize::Xs => px(8.),
        };
        div()
            .flex()
            .flex_col()
            .w_full()
            .gap(gap)
            .children(self.children)
    }
}

/// A horizontal separator with my-2, for use between items.
/// Public API surface matching shadcn `ItemSeparator`; not used by the current
/// docs examples (kept for parity).
#[allow(dead_code)]
#[derive(IntoElement)]
pub struct ItemSeparator;

#[allow(dead_code)]
impl ItemSeparator {
    pub fn new() -> Self {
        Self
    }
}

impl Default for ItemSeparator {
    fn default() -> Self {
        Self::new()
    }
}

impl RenderOnce for ItemSeparator {
    fn render(self, _window: &mut Window, _cx: &mut App) -> impl IntoElement {
        div().my(px(8.)).w_full().child(Separator::new())
    }
}
