//! Button — port of shadcn base-vega `ui/button.tsx`.
//!
//! Variants: Default, Outline, Secondary, Ghost, Destructive, Link.
//! Sizes: Default, Xs, Sm, Lg and the square Icon, IconXs, IconSm, IconLg.
//! Builders: `rounded_full`, `icon_inline_start`, `icon_inline_end`,
//! `tooltip_rich`. Additional sizing/shape overrides come from the caller via
//! [`Styled`] (gpui's equivalent of shadcn's `className` passthrough).
//!
//! Omitted from the source (no gpui equivalent yet): focus-visible ring,
//! aria-invalid styles.

use std::rc::Rc;

use gpui::{
    AnyElement, App, ClickEvent, ElementId, FontWeight, InteractiveElement as _, IntoElement,
    ParentElement, Refineable as _, RenderOnce, StatefulInteractiveElement as _, StyleRefinement,
    Styled, Window, div, prelude::FluentBuilder as _, px,
};

use crate::components::tooltip::attach_tooltip;
use crate::motion;
use crate::theme::{Theme, alpha};

#[derive(Clone, Copy, PartialEq, Eq, Default, Debug)]
pub enum ButtonVariant {
    #[default]
    Default,
    Outline,
    Secondary,
    Ghost,
    Destructive,
    Link,
}

/// Position of a button inside a [`crate::components::ButtonGroup`]:
/// ends keep their outer rounding, middles go square, and non-first
/// buttons collapse their left border.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum GroupPosition {
    First,
    Middle,
    Last,
    Only,
}

#[derive(Clone, Copy, PartialEq, Eq, Default, Debug)]
pub enum ButtonSize {
    #[default]
    Default,
    Xs,
    Sm,
    Lg,
    Icon,
    IconXs,
    IconSm,
    IconLg,
}

type ClickHandler = Box<dyn Fn(&ClickEvent, &mut Window, &mut App) + 'static>;
type TooltipContent = Rc<dyn Fn(&mut Window, &mut App) -> AnyElement>;

#[derive(IntoElement)]
pub struct Button {
    id: ElementId,
    variant: ButtonVariant,
    size: ButtonSize,
    disabled: bool,
    rounded_full: bool,
    icon_inline_start: bool,
    icon_inline_end: bool,
    group_position: Option<GroupPosition>,
    on_click: Option<ClickHandler>,
    tooltip: Option<TooltipContent>,
    children: Vec<AnyElement>,
    style: StyleRefinement,
}

impl Button {
    pub fn new(id: impl Into<ElementId>) -> Self {
        Self {
            id: id.into(),
            variant: ButtonVariant::default(),
            size: ButtonSize::default(),
            disabled: false,
            rounded_full: false,
            icon_inline_start: false,
            icon_inline_end: false,
            group_position: None,
            on_click: None,
            tooltip: None,
            children: Vec::new(),
            style: StyleRefinement::default(),
        }
    }

    pub fn variant(mut self, variant: ButtonVariant) -> Self {
        self.variant = variant;
        self
    }

    pub fn size(mut self, size: ButtonSize) -> Self {
        self.size = size;
        self
    }

    pub fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }

    pub fn on_click(
        mut self,
        handler: impl Fn(&ClickEvent, &mut Window, &mut App) + 'static,
    ) -> Self {
        self.on_click = Some(Box::new(handler));
        self
    }

    /// `rounded-full` — pill corners (px(9999.)).
    pub fn rounded_full(mut self) -> Self {
        self.rounded_full = true;
        self
    }

    /// Child `data-icon="inline-start"` — trim start padding
    /// (`has-data-[icon=inline-start]:pl-2` / `pl-1.5` for Xs/Sm).
    pub fn icon_inline_start(mut self) -> Self {
        self.icon_inline_start = true;
        self
    }

    /// Child `data-icon="inline-end"` — trim end padding
    /// (`has-data-[icon=inline-end]:pr-2` / `pr-1.5` for Xs/Sm).
    pub fn icon_inline_end(mut self) -> Self {
        self.icon_inline_end = true;
        self
    }

    /// Attach a rich hover tooltip (mirrors shadcn's
    /// `<TooltipTrigger render={<Button/>}>` composition). Needed because
    /// [`crate::components::ButtonGroup`] takes typed [`Button`] items, so a
    /// wrapping [`crate::components::Tooltip`] div cannot sit inside a group.
    pub fn tooltip_rich(
        mut self,
        content: impl Fn(&mut Window, &mut App) -> AnyElement + 'static,
    ) -> Self {
        self.tooltip = Some(Rc::new(content));
        self
    }

    /// Used by ButtonGroup to join neighboring buttons.
    pub(crate) fn group_position(mut self, position: GroupPosition) -> Self {
        self.group_position = Some(position);
        self
    }
}

impl ParentElement for Button {
    fn extend(&mut self, elements: impl IntoIterator<Item = AnyElement>) {
        self.children.extend(elements);
    }
}

impl Styled for Button {
    fn style(&mut self) -> &mut StyleRefinement {
        &mut self.style
    }
}

impl RenderOnce for Button {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let theme = Theme::of(cx).clone();
        let tooltip_id = (self.id.clone(), "tt");

        // Base: inline-flex items-center justify-center rounded-md border
        // border-transparent text-sm font-medium whitespace-nowrap select-none
        let base = div()
            .id(self.id)
            .relative()
            .flex()
            .flex_row()
            .flex_shrink_0()
            .items_center()
            .justify_center()
            .whitespace_nowrap()
            .border_1()
            .border_color(gpui::transparent_black())
            .text_size(px(14.))
            .line_height(px(20.))
            .font_weight(FontWeight::MEDIUM);

        // Size: h-9 px-2.5 gap-1.5 (default) etc.; icon sizes are square with
        // no padding. Xs/Sm clamp the radius like the source's
        // rounded-[min(var(--radius-md),8px)].
        let radius_md = theme.radius_md();
        let base = match self.size {
            ButtonSize::Default => base.h(px(36.)).px(px(10.)).gap(px(6.)).rounded(radius_md),
            ButtonSize::Xs => base
                .h(px(24.))
                .px(px(8.))
                .gap(px(4.))
                .text_size(px(12.))
                .line_height(px(16.))
                .rounded(radius_md.min(px(8.))),
            ButtonSize::Sm => base
                .h(px(32.))
                .px(px(10.))
                .gap(px(4.))
                .rounded(radius_md.min(px(10.))),
            ButtonSize::Lg => base.h(px(40.)).px(px(10.)).gap(px(6.)).rounded(radius_md),
            ButtonSize::Icon => base.size(px(36.)).rounded(radius_md),
            ButtonSize::IconXs => base
                .size(px(24.))
                .text_size(px(12.))
                .line_height(px(16.))
                .rounded(radius_md.min(px(8.))),
            ButtonSize::IconSm => base.size(px(32.)).rounded(radius_md.min(px(10.))),
            ButtonSize::IconLg => base.size(px(40.)).rounded(radius_md),
        };

        // has-data-[icon=inline-start]:pl-2 / inline-end:pr-2 (Default/Lg);
        // pl-1.5 / pr-1.5 (Xs/Sm). No effect on Icon* sizes.
        let base = match self.size {
            ButtonSize::Default | ButtonSize::Lg => {
                let mut b = base;
                if self.icon_inline_start {
                    b = b.pl(px(8.));
                }
                if self.icon_inline_end {
                    b = b.pr(px(8.));
                }
                b
            }
            ButtonSize::Xs | ButtonSize::Sm => {
                let mut b = base;
                if self.icon_inline_start {
                    b = b.pl(px(6.));
                }
                if self.icon_inline_end {
                    b = b.pr(px(6.));
                }
                b
            }
            ButtonSize::Icon | ButtonSize::IconXs | ButtonSize::IconSm | ButtonSize::IconLg => base,
        };

        // rounded-full — applied after size-based rounding, before group_position.
        let base = if self.rounded_full {
            base.rounded(px(9999.))
        } else {
            base
        };

        let dark = theme.dark;
        let styled = match self.variant {
            // bg-primary text-primary-foreground hover:bg-primary/80
            ButtonVariant::Default => base
                .bg(theme.primary)
                .text_color(theme.primary_foreground)
                .hover(|s| s.bg(alpha(theme.primary, 0.8))),
            // border-border bg-background shadow-xs hover:bg-muted
            // dark:border-input dark:bg-input/30 dark:hover:bg-input/50
            ButtonVariant::Outline => if dark {
                base.border_color(theme.input)
                    .bg(alpha(theme.input, 0.3))
                    .hover(|s| s.bg(alpha(theme.input, 0.5)))
            } else {
                base.border_color(theme.border)
                    .bg(theme.background)
                    .hover(|s| s.bg(theme.muted).text_color(theme.foreground))
            }
            .shadow_xs()
            .text_color(theme.foreground),
            // bg-secondary text-secondary-foreground
            // hover:bg-[color-mix(in oklch, secondary, foreground 5%)]
            ButtonVariant::Secondary => {
                let hover_bg: gpui::Hsla = if dark {
                    gpui::rgb(0x323232).into()
                } else {
                    gpui::rgb(0xe9e9e9).into()
                };
                base.bg(theme.secondary)
                    .text_color(theme.secondary_foreground)
                    .hover(move |s| s.bg(hover_bg))
            }
            // hover:bg-muted hover:text-foreground dark:hover:bg-muted/50
            ButtonVariant::Ghost => base.text_color(theme.foreground).hover(move |s| {
                let bg = if dark {
                    alpha(theme.muted, 0.5)
                } else {
                    theme.muted
                };
                s.bg(bg).text_color(theme.foreground)
            }),
            // bg-destructive/10 text-destructive hover:bg-destructive/20
            // dark:bg-destructive/20 dark:hover:bg-destructive/30
            ButtonVariant::Destructive => {
                let (bg_a, hover_a) = if dark { (0.2, 0.3) } else { (0.1, 0.2) };
                base.bg(alpha(theme.destructive, bg_a))
                    .text_color(theme.destructive)
                    .hover(move |s| s.bg(alpha(theme.destructive, hover_a)))
            }
            // text-primary underline-offset-4 hover:underline
            ButtonVariant::Link => base.text_color(theme.primary).hover(|s| s.underline()),
        };

        let styled = match self.group_position {
            None | Some(GroupPosition::Only) => styled,
            Some(GroupPosition::First) => styled.rounded_r(px(0.)),
            Some(GroupPosition::Middle) => styled.rounded(px(0.)).ml(px(-1.)),
            Some(GroupPosition::Last) => styled.rounded_l(px(0.)).ml(px(-1.)),
        };

        // focus-visible:border-ring focus-visible:ring-3 ring-ring/50
        // (destructive: border-destructive/40 ring-destructive/20)
        let (ring_border, ring_shadow) = if self.variant == ButtonVariant::Destructive {
            (
                alpha(theme.destructive, 0.4),
                motion::focus_ring_destructive(&theme),
            )
        } else {
            (theme.ring, motion::focus_ring(&theme))
        };

        // active:translate-y-px; disabled:opacity-50 + no pointer events.
        // Optional rich tooltip via anchored attach_tooltip (side Top, instant).
        // Caller refinement applied last so Styled overrides win over defaults.
        let mut button = styled
            .when(self.disabled, |s| s.opacity(0.5))
            .when(!self.disabled, |s| {
                s.tab_index(0)
                    .focus_visible(move |s| s.border_color(ring_border).shadow(ring_shadow.clone()))
                    .active(|s| s.top(px(1.)))
                    .when_some(self.on_click, |s, on_click| s.on_click(on_click))
            })
            .children(self.children);
        button.style().refine(&self.style);

        match self.tooltip {
            Some(content) => {
                attach_tooltip(tooltip_id, button, move |window, cx| content(window, cx))
                    .into_any_element()
            }
            None => button.into_any_element(),
        }
    }
}
