//! Badge — port of shadcn base-vega `ui/badge.tsx`.
//!
//! Variants: Default, Secondary, Destructive, Outline, Ghost, Link.
//! Builders: `on_click`, `icon_inline_start`, `icon_inline_end`, `bg`,
//! `text_color`.
//!
//! The source's `rounded-4xl` renders as a pill at badge height, so the port
//! uses a full radius. Icons inside a badge render at 12px (`[&>svg]:size-3!`).
//! The hover transition (`transition-all`, 150ms) is omitted because gpui
//! hover styles are instant — same omission as Button. Aria-invalid styles
//! are omitted.

use gpui::{
    AnyElement, App, ClickEvent, ElementId, FontWeight, Hsla, InteractiveElement as _, IntoElement,
    ParentElement, RenderOnce, StatefulInteractiveElement as _, Styled, Window, div,
    prelude::FluentBuilder as _, px,
};

use crate::motion;
use crate::theme::{Theme, alpha};

#[derive(Clone, Copy, PartialEq, Eq, Default, Debug)]
pub enum BadgeVariant {
    #[default]
    Default,
    Secondary,
    Destructive,
    Outline,
    Ghost,
    Link,
}

type ClickHandler = Box<dyn Fn(&ClickEvent, &mut Window, &mut App) + 'static>;

#[derive(IntoElement)]
pub struct Badge {
    id: Option<ElementId>,
    variant: BadgeVariant,
    on_click: Option<ClickHandler>,
    icon_inline_start: bool,
    icon_inline_end: bool,
    bg: Option<Hsla>,
    text_color: Option<Hsla>,
    children: Vec<AnyElement>,
}

impl Badge {
    pub fn new() -> Self {
        Self {
            id: None,
            variant: BadgeVariant::default(),
            on_click: None,
            icon_inline_start: false,
            icon_inline_end: false,
            bg: None,
            text_color: None,
            children: Vec::new(),
        }
    }

    pub fn variant(mut self, variant: BadgeVariant) -> Self {
        self.variant = variant;
        self
    }

    /// Interactive/link badge (`render={<a/>}`). Sets the element id (required
    /// by gpui's StatefulInteractiveElement), makes the badge focusable, and
    /// attaches the click handler. Also enables focus-ring and `[a]:hover`
    /// styles for the active variant.
    pub fn on_click(
        mut self,
        id: impl Into<ElementId>,
        handler: impl Fn(&ClickEvent, &mut Window, &mut App) + 'static,
    ) -> Self {
        self.id = Some(id.into());
        self.on_click = Some(Box::new(handler));
        self
    }

    /// Child `data-icon="inline-start"` — trim start padding
    /// (`has-data-[icon=inline-start]:pl-1.5`).
    pub fn icon_inline_start(mut self) -> Self {
        self.icon_inline_start = true;
        self
    }

    /// Child `data-icon="inline-end"` — trim end padding
    /// (`has-data-[icon=inline-end]:pr-1.5`).
    pub fn icon_inline_end(mut self) -> Self {
        self.icon_inline_end = true;
        self
    }

    /// `className` color-override port — background color applied after the
    /// variant styles (e.g. `className="bg-blue-50"`).
    pub fn bg(mut self, color: Hsla) -> Self {
        self.bg = Some(color);
        self
    }

    /// `className` color-override port — text color applied after the variant
    /// styles (e.g. `className="text-blue-700"`).
    pub fn text_color(mut self, color: Hsla) -> Self {
        self.text_color = Some(color);
        self
    }
}

impl Default for Badge {
    fn default() -> Self {
        Self::new()
    }
}

impl ParentElement for Badge {
    fn extend(&mut self, elements: impl IntoIterator<Item = AnyElement>) {
        self.children.extend(elements);
    }
}

impl RenderOnce for Badge {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let theme = Theme::of(cx).clone();
        let interactive = self.on_click.is_some();

        // inline-flex h-5 w-fit items-center justify-center gap-1 rounded-4xl
        // border border-transparent px-2 py-0.5 text-xs font-medium
        let mut base = div()
            .flex()
            .flex_row()
            .flex_shrink_0()
            .h(px(20.))
            .items_center()
            .justify_center()
            .gap(px(4.))
            .overflow_hidden()
            .rounded_full()
            .border_1()
            .border_color(gpui::transparent_black())
            .px(px(8.))
            .text_size(px(12.))
            .line_height(px(16.))
            .font_weight(FontWeight::MEDIUM)
            .whitespace_nowrap();

        // has-data-[icon=inline-start]:pl-1.5 / has-data-[icon=inline-end]:pr-1.5
        if self.icon_inline_start {
            base = base.pl(px(6.));
        }
        if self.icon_inline_end {
            base = base.pr(px(6.));
        }

        let styled = match self.variant {
            // bg-primary text-primary-foreground [a]:hover:bg-primary/80
            BadgeVariant::Default => {
                let el = base.bg(theme.primary).text_color(theme.primary_foreground);
                if interactive {
                    el.hover(|s| s.bg(alpha(theme.primary, 0.8)))
                } else {
                    el
                }
            }
            // bg-secondary text-secondary-foreground [a]:hover:bg-secondary/80
            BadgeVariant::Secondary => {
                let el = base
                    .bg(theme.secondary)
                    .text_color(theme.secondary_foreground);
                if interactive {
                    el.hover(|s| s.bg(alpha(theme.secondary, 0.8)))
                } else {
                    el
                }
            }
            // bg-destructive/10 (dark: /20) text-destructive
            // [a]:hover:bg-destructive/20 (no dark override)
            BadgeVariant::Destructive => {
                let el = base
                    .bg(alpha(theme.destructive, if theme.dark { 0.2 } else { 0.1 }))
                    .text_color(theme.destructive);
                if interactive {
                    el.hover(|s| s.bg(alpha(theme.destructive, 0.2)))
                } else {
                    el
                }
            }
            // border-border text-foreground
            // [a]:hover:bg-muted [a]:hover:text-muted-foreground
            BadgeVariant::Outline => {
                let el = base.border_color(theme.border).text_color(theme.foreground);
                if interactive {
                    let muted = theme.muted;
                    let muted_foreground = theme.muted_foreground;
                    el.hover(move |s| s.bg(muted).text_color(muted_foreground))
                } else {
                    el
                }
            }
            // hover:bg-muted hover:text-muted-foreground dark:hover:bg-muted/50
            // (unconditional in source — not link-only)
            BadgeVariant::Ghost => {
                let dark = theme.dark;
                let muted = theme.muted;
                let muted_foreground = theme.muted_foreground;
                base.text_color(theme.foreground).hover(move |s| {
                    let bg = if dark { alpha(muted, 0.5) } else { muted };
                    s.bg(bg).text_color(muted_foreground)
                })
            }
            // text-primary underline-offset-4 hover:underline
            // (unconditional in source — not link-only)
            BadgeVariant::Link => base.text_color(theme.primary).hover(|s| s.underline()),
        };

        // className color overrides applied after variant styles
        let styled = styled
            .when_some(self.bg, |s, c| s.bg(c))
            .when_some(self.text_color, |s, c| s.text_color(c));

        // focus-visible:border-ring focus-visible:ring-[3px] focus-visible:ring-ring/50
        // destructive: focus-visible:ring-destructive/20 dark:focus-visible:ring-destructive/40
        // `.id()` turns Div into Stateful<Div>, so branch + into_any_element to unify types.
        if let (Some(id), Some(on_click)) = (self.id, self.on_click) {
            let (ring_border, ring_shadow) = if self.variant == BadgeVariant::Destructive {
                (
                    alpha(theme.destructive, 0.4),
                    motion::focus_ring_destructive(&theme),
                )
            } else {
                (theme.ring, motion::focus_ring(&theme))
            };
            styled
                .id(id)
                .tab_index(0)
                .focus_visible(move |s| s.border_color(ring_border).shadow(ring_shadow.clone()))
                .on_click(on_click)
                .children(self.children)
                .into_any_element()
        } else {
            styled.children(self.children).into_any_element()
        }
    }
}
