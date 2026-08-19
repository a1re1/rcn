//! Prototype: Button and Badge rebuilt from verbatim shadcn class strings via
//! the [`crate::tw`] parser, instead of hand-translated builder chains.
//!
//! The class strings below are the shadcn base-vega `button.tsx` / `badge.tsx`
//! cva strings (the same ones quoted in the handwritten ports' comments), with
//! two mechanical rewrites the parser requires:
//! - `relative` is added to Button's base so `active:translate-y-px` (mapped
//!   to a `top` inset — gpui has no transforms) can take effect.
//! - conditional selectors that Rust state already decides are appended as
//!   plain classes: `has-data-[icon=inline-start]:pl-2` becomes `pl-2` pushed
//!   when the builder flag is set; `[a]:hover:*` hover styles are pushed when
//!   the badge is interactive; `disabled:opacity-50` becomes `opacity-50`
//!   pushed when disabled.
//!
//! Not wired into the storybook yet — exercised by the unit tests, which
//! assert the parsed styles match the handwritten ports' builder output.
#![allow(dead_code)]

use gpui::{
    AnyElement, App, AppContext as _, ClickEvent, ElementId, InteractiveElement as _, IntoElement,
    ParentElement, RenderOnce, SharedString, StatefulInteractiveElement as _, Styled as _, Window,
    div, prelude::FluentBuilder as _, px,
};

use crate::components::badge::BadgeVariant;
use crate::components::button::{ButtonSize, ButtonVariant};
use crate::components::{Input, Textarea};
use crate::theme::Theme;
use crate::tw::{TwInteractiveExt as _, TwStatefulExt as _};

type ClickHandler = Box<dyn Fn(&ClickEvent, &mut Window, &mut App) + 'static>;

const FOCUS: &str = "focus-visible:border-ring focus-visible:ring-[3px] focus-visible:ring-ring/50";
const FOCUS_DESTRUCTIVE: &str = "focus-visible:border-destructive/40 focus-visible:ring-[3px] \
     focus-visible:ring-destructive/20 dark:focus-visible:ring-destructive/40";

// ---------------------------------------------------------------------------
// Button
// ---------------------------------------------------------------------------

const BUTTON_BASE: &str = "relative inline-flex items-center justify-center border \
     border-transparent text-sm font-medium whitespace-nowrap select-none";

fn button_size_classes(size: ButtonSize) -> &'static str {
    match size {
        ButtonSize::Default => "h-9 px-2.5 gap-1.5 rounded-md",
        ButtonSize::Xs => "h-6 px-2 gap-1 text-xs rounded-[min(var(--radius-md),8px)]",
        ButtonSize::Sm => "h-8 px-2.5 gap-1 rounded-[min(var(--radius-md),10px)]",
        ButtonSize::Lg => "h-10 px-2.5 gap-1.5 rounded-md",
        ButtonSize::Icon => "size-9 rounded-md",
        ButtonSize::IconXs => "size-6 text-xs rounded-[min(var(--radius-md),8px)]",
        ButtonSize::IconSm => "size-8 rounded-[min(var(--radius-md),10px)]",
        ButtonSize::IconLg => "size-10 rounded-md",
    }
}

fn button_variant_classes(variant: ButtonVariant, dark: bool) -> String {
    match variant {
        ButtonVariant::Default => "bg-primary text-primary-foreground hover:bg-primary/80".into(),
        ButtonVariant::Outline => "border-border bg-background shadow-xs text-foreground \
             hover:bg-muted hover:text-foreground \
             dark:border-input dark:bg-input/30 dark:hover:bg-input/50"
            .into(),
        // Source: hover:bg-[color-mix(in_oklch,var(--secondary),var(--foreground)_5%)];
        // the mix is precomputed per scheme like the handwritten port.
        ButtonVariant::Secondary => format!(
            "bg-secondary text-secondary-foreground hover:bg-[{}]",
            if dark { "#323232" } else { "#e9e9e9" }
        ),
        ButtonVariant::Ghost => {
            "text-foreground hover:bg-muted hover:text-foreground dark:hover:bg-muted/50".into()
        }
        ButtonVariant::Destructive => "bg-destructive/10 text-destructive \
             hover:bg-destructive/20 dark:bg-destructive/20 dark:hover:bg-destructive/30"
            .into(),
        ButtonVariant::Link => "text-primary underline-offset-4 hover:underline".into(),
    }
}

#[derive(IntoElement)]
pub struct TwButton {
    id: ElementId,
    variant: ButtonVariant,
    size: ButtonSize,
    disabled: bool,
    rounded_full: bool,
    icon_inline_start: bool,
    icon_inline_end: bool,
    /// `className` passthrough — appended last, so it overrides variant
    /// styles exactly like `cn(buttonVariants(...), className)`.
    class: Option<String>,
    on_click: Option<ClickHandler>,
    children: Vec<AnyElement>,
}

impl TwButton {
    pub fn new(id: impl Into<ElementId>) -> Self {
        Self {
            id: id.into(),
            variant: ButtonVariant::default(),
            size: ButtonSize::default(),
            disabled: false,
            rounded_full: false,
            icon_inline_start: false,
            icon_inline_end: false,
            class: None,
            on_click: None,
            children: Vec::new(),
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

    pub fn rounded_full(mut self) -> Self {
        self.rounded_full = true;
        self
    }

    pub fn icon_inline_start(mut self) -> Self {
        self.icon_inline_start = true;
        self
    }

    pub fn icon_inline_end(mut self) -> Self {
        self.icon_inline_end = true;
        self
    }

    pub fn class(mut self, classes: impl Into<String>) -> Self {
        self.class = Some(classes.into());
        self
    }

    pub fn on_click(
        mut self,
        handler: impl Fn(&ClickEvent, &mut Window, &mut App) + 'static,
    ) -> Self {
        self.on_click = Some(Box::new(handler));
        self
    }

    /// The full class string this button renders with — the tw-side analogue
    /// of `cn(buttonVariants({variant, size}), className)`.
    fn classes(&self, dark: bool) -> String {
        let mut parts: Vec<String> =
            vec![BUTTON_BASE.into(), button_size_classes(self.size).into()];
        // has-data-[icon=inline-start]:pl-2 / :pr-2 (pl-1.5/pr-1.5 on Xs/Sm);
        // no effect on icon sizes, which have no x padding to trim.
        let icon_pad = match self.size {
            ButtonSize::Default | ButtonSize::Lg => Some(("pl-2", "pr-2")),
            ButtonSize::Xs | ButtonSize::Sm => Some(("pl-1.5", "pr-1.5")),
            _ => None,
        };
        if let Some((pl, pr)) = icon_pad {
            if self.icon_inline_start {
                parts.push(pl.into());
            }
            if self.icon_inline_end {
                parts.push(pr.into());
            }
        }
        if self.rounded_full {
            parts.push("rounded-full".into());
        }
        parts.push(button_variant_classes(self.variant, dark));
        if self.disabled {
            parts.push("opacity-50".into());
        } else {
            parts.push("active:translate-y-px".into());
            parts.push(
                if self.variant == ButtonVariant::Destructive {
                    FOCUS_DESTRUCTIVE
                } else {
                    FOCUS
                }
                .into(),
            );
        }
        if let Some(class) = &self.class {
            parts.push(class.clone());
        }
        parts.join(" ")
    }
}

impl ParentElement for TwButton {
    fn extend(&mut self, elements: impl IntoIterator<Item = AnyElement>) {
        self.children.extend(elements);
    }
}

impl RenderOnce for TwButton {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let theme = Theme::of(cx).clone();
        let classes = self.classes(theme.dark);
        div()
            .id(self.id)
            .tw_stateful(&theme, &classes)
            .when(!self.disabled, |el| {
                el.tab_index(0)
                    .when_some(self.on_click, |el, on_click| el.on_click(on_click))
            })
            .children(self.children)
    }
}

// ---------------------------------------------------------------------------
// Badge
// ---------------------------------------------------------------------------

const BADGE_BASE: &str = "inline-flex h-5 w-fit shrink-0 items-center justify-center gap-1 \
     overflow-hidden rounded-4xl border border-transparent px-2 py-0.5 text-xs font-medium \
     whitespace-nowrap";

fn badge_variant_classes(variant: BadgeVariant, interactive: bool) -> String {
    // `[a]:hover:*` styles apply only to link badges — decided here in Rust,
    // so they join the string as plain hover classes when interactive.
    let (base, link_hover) = match variant {
        BadgeVariant::Default => ("bg-primary text-primary-foreground", "hover:bg-primary/80"),
        BadgeVariant::Secondary => (
            "bg-secondary text-secondary-foreground",
            "hover:bg-secondary/80",
        ),
        BadgeVariant::Destructive => (
            "bg-destructive/10 dark:bg-destructive/20 text-destructive",
            "hover:bg-destructive/20",
        ),
        BadgeVariant::Outline => (
            "border-border text-foreground",
            "hover:bg-muted hover:text-muted-foreground",
        ),
        // Ghost/Link hover is unconditional in the source.
        BadgeVariant::Ghost => {
            return "text-foreground hover:bg-muted hover:text-muted-foreground \
                 dark:hover:bg-muted/50"
                .into();
        }
        BadgeVariant::Link => {
            return "text-primary underline-offset-4 hover:underline".into();
        }
    };
    if interactive {
        format!("{base} {link_hover}")
    } else {
        base.into()
    }
}

#[derive(IntoElement)]
pub struct TwBadge {
    id: Option<ElementId>,
    variant: BadgeVariant,
    icon_inline_start: bool,
    icon_inline_end: bool,
    class: Option<String>,
    on_click: Option<ClickHandler>,
    children: Vec<AnyElement>,
}

impl TwBadge {
    pub fn new() -> Self {
        Self {
            id: None,
            variant: BadgeVariant::default(),
            icon_inline_start: false,
            icon_inline_end: false,
            class: None,
            on_click: None,
            children: Vec::new(),
        }
    }

    pub fn variant(mut self, variant: BadgeVariant) -> Self {
        self.variant = variant;
        self
    }

    pub fn icon_inline_start(mut self) -> Self {
        self.icon_inline_start = true;
        self
    }

    pub fn icon_inline_end(mut self) -> Self {
        self.icon_inline_end = true;
        self
    }

    /// `className` passthrough (e.g. `"bg-blue-50 text-blue-700"` in the
    /// docs examples becomes `.class("bg-[#eff6ff] text-[#1d4ed8]")`).
    pub fn class(mut self, classes: impl Into<String>) -> Self {
        self.class = Some(classes.into());
        self
    }

    pub fn on_click(
        mut self,
        id: impl Into<ElementId>,
        handler: impl Fn(&ClickEvent, &mut Window, &mut App) + 'static,
    ) -> Self {
        self.id = Some(id.into());
        self.on_click = Some(Box::new(handler));
        self
    }

    fn classes(&self) -> String {
        let interactive = self.on_click.is_some();
        let mut parts: Vec<String> = vec![BADGE_BASE.into()];
        if self.icon_inline_start {
            parts.push("pl-1.5".into());
        }
        if self.icon_inline_end {
            parts.push("pr-1.5".into());
        }
        parts.push(badge_variant_classes(self.variant, interactive));
        if interactive {
            parts.push(
                if self.variant == BadgeVariant::Destructive {
                    FOCUS_DESTRUCTIVE
                } else {
                    FOCUS
                }
                .into(),
            );
        }
        if let Some(class) = &self.class {
            parts.push(class.clone());
        }
        parts.join(" ")
    }
}

impl Default for TwBadge {
    fn default() -> Self {
        Self::new()
    }
}

impl ParentElement for TwBadge {
    fn extend(&mut self, elements: impl IntoIterator<Item = AnyElement>) {
        self.children.extend(elements);
    }
}

impl RenderOnce for TwBadge {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let theme = Theme::of(cx).clone();
        let classes = self.classes();
        if let (Some(id), Some(on_click)) = (self.id, self.on_click) {
            div()
                .id(id)
                .tw_stateful(&theme, &classes)
                .tab_index(0)
                .on_click(on_click)
                .children(self.children)
                .into_any_element()
        } else {
            div()
                .tw(&theme, &classes)
                .children(self.children)
                .into_any_element()
        }
    }
}

// ---------------------------------------------------------------------------
// Demo window (`RCN_TW_DEMO=1 cargo run`)
// ---------------------------------------------------------------------------

/// Seed class strings for the playground editors.
const PLAYGROUND_BUTTON_SEED: &str = "relative inline-flex items-center justify-center h-9 \
     px-2.5 gap-1.5 rounded-md border border-transparent text-sm font-medium whitespace-nowrap \
     bg-primary text-primary-foreground hover:bg-primary/80 active:translate-y-px \
     focus-visible:border-ring focus-visible:ring-[3px] focus-visible:ring-ring/50";
const PLAYGROUND_BADGE_SEED: &str = "inline-flex h-5 shrink-0 items-center justify-center gap-1 \
     rounded-4xl border border-transparent px-2 py-0.5 text-xs font-medium whitespace-nowrap \
     bg-destructive/10 text-destructive hover:bg-destructive/20";
const PLAYGROUND_CARD_SEED: &str = "flex flex-col items-start gap-1 w-[300px] p-4 rounded-lg \
     border border-border bg-card text-card-foreground shadow-sm hover:shadow-lg \
     hover:border-ring";

/// One editable playground: a class-string editor and the element it styles.
struct Playground {
    title: &'static str,
    children_text: &'static [&'static str],
    input: gpui::Entity<Input>,
}

/// Side-by-side comparison view: every row renders the tw-string-driven
/// component directly above the handwritten port — they should be pixel
/// identical (modulo the badge pill radius note in [`crate::tw`]). The
/// playground section at the top re-parses its class strings on every
/// keystroke and re-styles the preview elements live.
pub struct TwDemoView {
    playgrounds: Vec<Playground>,
}

impl TwDemoView {
    pub fn new(cx: &mut gpui::Context<Self>) -> Self {
        let editor = |cx: &mut gpui::Context<Self>, seed: &str| {
            let input = cx.new(|cx| {
                let mut input = Input::new(cx);
                // Bare: the Textarea shell provides the chrome, and bare
                // inputs soft-wrap, so long class strings stay fully visible.
                input.set_bare(true);
                input.set_text(seed.to_string(), cx);
                input
            });
            // Re-render (and re-parse) on every edit.
            cx.observe(&input, |_, _, cx| cx.notify()).detach();
            input
        };
        let playgrounds = vec![
            Playground {
                title: "Button (shadcn default variant)",
                children_text: &["Button"],
                input: editor(cx, PLAYGROUND_BUTTON_SEED),
            },
            Playground {
                title: "Badge (shadcn destructive variant)",
                children_text: &["Badge"],
                input: editor(cx, PLAYGROUND_BADGE_SEED),
            },
            Playground {
                title: "Freeform",
                children_text: &[
                    "Tailwind, live",
                    "Edit the classes above and watch this re-style.",
                ],
                input: editor(cx, PLAYGROUND_CARD_SEED),
            },
        ];
        Self { playgrounds }
    }

    /// Editor + diagnostics + live preview for one playground entry.
    fn playground_row(
        &self,
        idx: usize,
        theme: &Theme,
        window: &Window,
        cx: &gpui::Context<Self>,
    ) -> AnyElement {
        let pg = &self.playgrounds[idx];
        let classes = pg.input.read(cx).text().to_string();
        let parsed = crate::tw::parse_at(theme, window.viewport_size(), &classes);

        let mut diagnostics: Vec<AnyElement> = Vec::new();
        if !parsed.unknown.is_empty() {
            diagnostics.push(
                div()
                    .text_size(px(11.))
                    .text_color(theme.destructive)
                    .child(format!("unknown: {}", parsed.unknown.join("  ")))
                    .into_any_element(),
            );
        }
        if !parsed.skipped.is_empty() {
            diagnostics.push(
                div()
                    .text_size(px(11.))
                    .text_color(theme.muted_foreground)
                    .child(format!(
                        "skipped (no gpui equivalent): {}",
                        parsed.skipped.join("  ")
                    ))
                    .into_any_element(),
            );
        }

        div()
            .flex()
            .flex_col()
            .gap(px(6.))
            .child(
                div()
                    .text_size(px(11.))
                    .text_color(theme.muted_foreground)
                    .child(pg.title),
            )
            .child(Textarea::new(pg.input.clone()).rows(2).resizable(true))
            .children(diagnostics)
            .child(
                // Neutral stage around the styled element, so the element's
                // own box (bg, border, shadow, radius) reads clearly.
                div()
                    .min_h(px(64.))
                    .p(px(16.))
                    .rounded(px(8.))
                    .border_1()
                    .border_color(theme.border)
                    .flex()
                    .items_center()
                    .child(
                        div()
                            .id(SharedString::from(format!("tw-playground-{idx}")))
                            .tw_stateful_at(theme, window, &classes)
                            .tab_index(0)
                            .children(pg.children_text.iter().copied()),
                    ),
            )
            .into_any_element()
    }

    fn section(title: &str, theme: &Theme, rows: Vec<AnyElement>) -> AnyElement {
        div()
            .flex()
            .flex_col()
            .gap(px(8.))
            .child(
                div()
                    .text_size(px(12.))
                    .text_color(theme.muted_foreground)
                    .child(title.to_string()),
            )
            .children(rows)
            .into_any_element()
    }

    fn labeled_row(label: &str, theme: &Theme, items: Vec<AnyElement>) -> AnyElement {
        div()
            .flex()
            .flex_row()
            .items_center()
            .gap(px(12.))
            .child(
                div()
                    .w(px(36.))
                    .text_size(px(11.))
                    .text_color(theme.muted_foreground)
                    .child(label.to_string()),
            )
            .children(items)
            .into_any_element()
    }
}

impl gpui::Render for TwDemoView {
    fn render(&mut self, window: &mut Window, cx: &mut gpui::Context<Self>) -> impl IntoElement {
        use crate::components::badge::Badge;
        use crate::components::button::Button;

        let theme = Theme::of(cx).clone();

        const VARIANTS: [(&str, ButtonVariant); 6] = [
            ("Default", ButtonVariant::Default),
            ("Outline", ButtonVariant::Outline),
            ("Secondary", ButtonVariant::Secondary),
            ("Ghost", ButtonVariant::Ghost),
            ("Destructive", ButtonVariant::Destructive),
            ("Link", ButtonVariant::Link),
        ];
        const SIZES: [(&str, ButtonSize); 4] = [
            ("Xs", ButtonSize::Xs),
            ("Sm", ButtonSize::Sm),
            ("Default", ButtonSize::Default),
            ("Lg", ButtonSize::Lg),
        ];
        const BADGES: [(&str, BadgeVariant); 6] = [
            ("Default", BadgeVariant::Default),
            ("Secondary", BadgeVariant::Secondary),
            ("Destructive", BadgeVariant::Destructive),
            ("Outline", BadgeVariant::Outline),
            ("Ghost", BadgeVariant::Ghost),
            ("Link", BadgeVariant::Link),
        ];

        let tw_variant_buttons = VARIANTS
            .iter()
            .map(|&(label, v)| {
                TwButton::new(SharedString::from(format!("tw-var-{label}")))
                    .variant(v)
                    .on_click(|_, _, _| {})
                    .child(label)
                    .into_any_element()
            })
            .collect::<Vec<_>>();
        let port_variant_buttons = VARIANTS
            .iter()
            .map(|&(label, v)| {
                Button::new(SharedString::from(format!("port-var-{label}")))
                    .variant(v)
                    .on_click(|_, _, _| {})
                    .child(label)
                    .into_any_element()
            })
            .collect::<Vec<_>>();

        let tw_size_buttons = SIZES
            .iter()
            .map(|&(label, s)| {
                TwButton::new(SharedString::from(format!("tw-size-{label}")))
                    .size(s)
                    .on_click(|_, _, _| {})
                    .child(label)
                    .into_any_element()
            })
            .collect::<Vec<_>>();
        let port_size_buttons = SIZES
            .iter()
            .map(|&(label, s)| {
                Button::new(SharedString::from(format!("port-size-{label}")))
                    .size(s)
                    .on_click(|_, _, _| {})
                    .child(label)
                    .into_any_element()
            })
            .collect::<Vec<_>>();

        let tw_badges = BADGES
            .iter()
            .map(|&(label, v)| {
                TwBadge::new()
                    .variant(v)
                    .on_click(
                        SharedString::from(format!("tw-badge-{label}")),
                        |_, _, _| {},
                    )
                    .child(label)
                    .into_any_element()
            })
            .collect::<Vec<_>>();
        let port_badges = BADGES
            .iter()
            .map(|&(label, v)| {
                Badge::new()
                    .variant(v)
                    .on_click(
                        SharedString::from(format!("port-badge-{label}")),
                        |_, _, _| {},
                    )
                    .child(label)
                    .into_any_element()
            })
            .collect::<Vec<_>>();

        let extras = vec![
            TwButton::new("tw-disabled")
                .disabled(true)
                .child("Disabled")
                .into_any_element(),
            TwButton::new("tw-pill")
                .rounded_full()
                .on_click(|_, _, _| {})
                .child("Pill")
                .into_any_element(),
            // className passthrough: docs' `bg-blue-50 text-blue-700` badge.
            TwBadge::new()
                .class("bg-[#eff6ff] text-[#1d4ed8]")
                .child("class override")
                .into_any_element(),
            crate::tw::element::tw_div(
                "flex items-center rounded-md border border-border px-3 py-2 bg-background \
                 transition-colors duration-300 hover:bg-primary hover:text-primary-foreground \
                 hover:border-primary",
            )
            .id("tw-transition-demo")
            .child("hover me — colors fade over 300ms")
            .into_any_element(),
            crate::tw::element::tw_div(
                "flex flex-row items-center divide-x divide-border rounded-md border border-border",
            )
            .child(div().p(px(8.)).child("divide"))
            .child(div().p(px(8.)).child("between"))
            .child(div().p(px(8.)).child("children"))
            .into_any_element(),
            TwButton::new("tw-arbitrary")
                .class("h-[52px] px-[26px] rounded-[26px] bg-[#7c3aed] text-[#ffffff]")
                .on_click(|_, _, _| {})
                .child("arbitrary values")
                .into_any_element(),
        ];

        let playground_rows = (0..self.playgrounds.len())
            .map(|idx| self.playground_row(idx, &theme, window, cx))
            .collect::<Vec<_>>();

        div()
            .id("tw-demo-root")
            .size_full()
            .overflow_y_scroll()
            .bg(theme.background)
            .text_color(theme.foreground)
            .when_some(theme.font_sans.clone(), |el, font| el.font_family(font))
            .flex()
            .flex_col()
            .gap(px(20.))
            .p(px(32.))
            .child(
                div()
                    .flex()
                    .flex_row()
                    .items_center()
                    .justify_between()
                    .child(
                        div()
                            .text_size(px(16.))
                            .child("tw parser demo — tw row above, handwritten port below"),
                    )
                    .child(
                        TwButton::new("toggle-theme")
                            .variant(ButtonVariant::Outline)
                            .on_click(cx.listener(|_, _, _, cx| {
                                let dark = Theme::of(cx).dark;
                                cx.set_global(if dark { Theme::light() } else { Theme::dark() });
                                cx.notify();
                            }))
                            .child("Toggle dark"),
                    ),
            )
            .child(Self::section(
                "Playground — edit the classes, the element re-styles on every keystroke",
                &theme,
                playground_rows,
            ))
            .child(Self::section(
                "Button variants",
                &theme,
                vec![
                    Self::labeled_row("tw", &theme, tw_variant_buttons),
                    Self::labeled_row("port", &theme, port_variant_buttons),
                ],
            ))
            .child(Self::section(
                "Button sizes",
                &theme,
                vec![
                    Self::labeled_row("tw", &theme, tw_size_buttons),
                    Self::labeled_row("port", &theme, port_size_buttons),
                ],
            ))
            .child(Self::section(
                "Badges (interactive: hover + focus ring live)",
                &theme,
                vec![
                    Self::labeled_row("tw", &theme, tw_badges),
                    Self::labeled_row("port", &theme, port_badges),
                ],
            ))
            .child(Self::section(
                "tw-only extras: disabled, pill, className passthrough, arbitrary values",
                &theme,
                vec![Self::labeled_row("tw", &theme, extras)],
            ))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::motion;
    use crate::theme::alpha;
    use crate::tw::parse;
    use gpui::{FontWeight, Hsla, StyleRefinement, Styled, px, transparent_black};

    fn assert_style_eq(got: &StyleRefinement, expected: &StyleRefinement, what: &str) {
        assert_eq!(format!("{got:?}"), format!("{expected:?}"), "{what}");
    }

    fn button(variant: ButtonVariant, size: ButtonSize) -> TwButton {
        TwButton::new("test").variant(variant).size(size)
    }

    /// Every variant × size × scheme × disabled state parses with zero
    /// unknown tokens, and skips only the expected unmappables.
    #[test]
    fn full_button_and_badge_coverage() {
        const KNOWN_SKIPS: [&str; 3] = ["select-none", "underline-offset-4", "w-fit"];
        for theme in [Theme::light(), Theme::dark()] {
            for variant in [
                ButtonVariant::Default,
                ButtonVariant::Outline,
                ButtonVariant::Secondary,
                ButtonVariant::Ghost,
                ButtonVariant::Destructive,
                ButtonVariant::Link,
            ] {
                for size in [
                    ButtonSize::Default,
                    ButtonSize::Xs,
                    ButtonSize::Sm,
                    ButtonSize::Lg,
                    ButtonSize::Icon,
                    ButtonSize::IconXs,
                    ButtonSize::IconSm,
                    ButtonSize::IconLg,
                ] {
                    for disabled in [false, true] {
                        let classes = button(variant, size).disabled(disabled).classes(theme.dark);
                        let styles = parse(&theme, &classes);
                        assert!(
                            styles.unknown.is_empty(),
                            "unknown classes for {variant:?}/{size:?}: {:?}",
                            styles.unknown
                        );
                        assert!(
                            styles
                                .skipped
                                .iter()
                                .all(|c| KNOWN_SKIPS.contains(&c.as_str())),
                            "unexpected skips: {:?}",
                            styles.skipped
                        );
                    }
                }
            }
            for variant in [
                BadgeVariant::Default,
                BadgeVariant::Secondary,
                BadgeVariant::Destructive,
                BadgeVariant::Outline,
                BadgeVariant::Ghost,
                BadgeVariant::Link,
            ] {
                for interactive in [false, true] {
                    let mut badge = TwBadge::new().variant(variant);
                    if interactive {
                        badge = badge.on_click("b", |_, _, _| {});
                    }
                    let styles = parse(&theme, &badge.classes());
                    assert!(
                        styles.unknown.is_empty(),
                        "unknown classes for badge {variant:?}: {:?}",
                        styles.unknown
                    );
                }
            }
        }
    }

    /// Default button, light scheme — parsed buckets match the handwritten
    /// port's builder chains (`src/components/button.rs`).
    #[test]
    fn button_default_matches_handwritten_port() {
        let theme = Theme::light();
        let classes = button(ButtonVariant::Default, ButtonSize::Default).classes(theme.dark);
        let styles = parse(&theme, &classes);

        let expected_base = StyleRefinement::default()
            .relative()
            .flex()
            .items_center()
            .justify_center()
            .whitespace_nowrap()
            .border_1()
            .border_color(transparent_black())
            .text_size(px(14.))
            .line_height(px(20.))
            .font_weight(FontWeight::MEDIUM)
            .h(px(36.))
            .px(px(10.))
            .gap(px(6.))
            .rounded(theme.radius_md())
            .bg(theme.primary)
            .text_color(theme.primary_foreground);
        assert_style_eq(&styles.base, &expected_base, "base");

        let expected_hover = StyleRefinement::default().bg(alpha(theme.primary, 0.8));
        assert_style_eq(styles.hover.as_ref().unwrap(), &expected_hover, "hover");

        let expected_active = StyleRefinement::default().top(px(1.));
        assert_style_eq(styles.active.as_ref().unwrap(), &expected_active, "active");

        let expected_focus = StyleRefinement::default()
            .border_color(theme.ring)
            .shadow(motion::focus_ring(&theme));
        assert_style_eq(
            styles.focus_visible.as_ref().unwrap(),
            &expected_focus,
            "focus-visible",
        );
    }

    /// Xs size — the `rounded-[min(var(--radius-md),8px)]` clamp and the
    /// text-xs size/line-height override.
    #[test]
    fn button_xs_radius_clamp() {
        let theme = Theme::light();
        let classes = button(ButtonVariant::Default, ButtonSize::Xs).classes(theme.dark);
        let styles = parse(&theme, &classes);
        let expected = StyleRefinement::default()
            .relative()
            .flex()
            .items_center()
            .justify_center()
            .whitespace_nowrap()
            .border_1()
            .border_color(transparent_black())
            .font_weight(FontWeight::MEDIUM)
            .h(px(24.))
            .px(px(8.))
            .gap(px(4.))
            .text_size(px(12.))
            .line_height(px(16.))
            .rounded(theme.radius_md().min(px(8.)))
            .bg(theme.primary)
            .text_color(theme.primary_foreground);
        assert_style_eq(&styles.base, &expected, "base");
    }

    /// Outline in dark — `dark:` overrides replace the light border/bg, and
    /// dark hover wins over the light hover class.
    #[test]
    fn button_outline_dark_overrides() {
        let theme = Theme::dark();
        let classes = button(ButtonVariant::Outline, ButtonSize::Default).classes(theme.dark);
        let styles = parse(&theme, &classes);

        let expected_base = StyleRefinement::default()
            .relative()
            .flex()
            .items_center()
            .justify_center()
            .whitespace_nowrap()
            .border_1()
            .text_size(px(14.))
            .line_height(px(20.))
            .font_weight(FontWeight::MEDIUM)
            .h(px(36.))
            .px(px(10.))
            .gap(px(6.))
            .rounded(theme.radius_md())
            .shadow_xs()
            .text_color(theme.foreground)
            .border_color(theme.input)
            .bg(alpha(theme.input, 0.3));
        assert_style_eq(&styles.base, &expected_base, "base");

        // Port's dark hover only sets bg; the class string also re-asserts
        // text-foreground, which base already sets — visually identical.
        let expected_hover = StyleRefinement::default()
            .text_color(theme.foreground)
            .bg(alpha(theme.input, 0.5));
        assert_style_eq(styles.hover.as_ref().unwrap(), &expected_hover, "hover");
    }

    /// Secondary badge (interactive) — pill chrome plus the `[a]:hover` bg.
    #[test]
    fn badge_secondary_interactive_matches_port() {
        let theme = Theme::light();
        let badge = TwBadge::new()
            .variant(BadgeVariant::Secondary)
            .on_click("b", |_, _, _| {});
        let styles = parse(&theme, &badge.classes());

        let expected_base = StyleRefinement::default()
            .flex()
            .h(px(20.))
            .flex_shrink_0()
            .items_center()
            .justify_center()
            .gap(px(4.))
            .overflow_hidden()
            .rounded(px(32.))
            .border_1()
            .border_color(transparent_black())
            .px(px(8.))
            .py(px(2.))
            .text_size(px(12.))
            .line_height(px(16.))
            .font_weight(FontWeight::MEDIUM)
            .whitespace_nowrap()
            .bg(theme.secondary)
            .text_color(theme.secondary_foreground);
        assert_style_eq(&styles.base, &expected_base, "base");

        let expected_hover = StyleRefinement::default().bg(alpha(theme.secondary, 0.8));
        assert_style_eq(styles.hover.as_ref().unwrap(), &expected_hover, "hover");

        let expected_focus = StyleRefinement::default()
            .border_color(theme.ring)
            .shadow(motion::focus_ring(&theme));
        assert_style_eq(
            styles.focus_visible.as_ref().unwrap(),
            &expected_focus,
            "focus-visible",
        );
    }

    /// `className` passthrough overrides variant styles, last-wins.
    #[test]
    fn class_passthrough_overrides_variant() {
        let theme = Theme::light();
        let badge = TwBadge::new().class("bg-[#eff6ff] text-[#1d4ed8]");
        let styles = parse(&theme, &badge.classes());
        let expected_bg = StyleRefinement::default().bg(Hsla::from(gpui::rgb(0xeff6ff)));
        assert_eq!(
            format!("{:?}", styles.base.background),
            format!("{:?}", expected_bg.background),
        );
        assert_eq!(
            styles.base.text.color,
            Some(Hsla::from(gpui::rgb(0x1d4ed8)))
        );
    }
}
