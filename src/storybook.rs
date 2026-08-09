//! The rcn storybook shell: a three-pane component explorer. The sidebar
//! lists every component; the canvas renders the selected one in isolation;
//! the controls panel binds each of the component's props (variant, size,
//! state flags) to interactive widgets — built from the library's own Button
//! and Switch.

use gpui::{AnyElement, App, ClickEvent, Context, FontWeight, Window, div, prelude::*, px};

use crate::components::{
    Accordion, AccordionItem, Avatar, AvatarGroup, AvatarGroupCount, AvatarSize, Badge,
    BadgeVariant, Button, ButtonSize, ButtonVariant, Popover, PopoverDescription, PopoverHeader,
    PopoverTitle, Switch, SwitchSize,
};
use crate::theme::Theme;

#[derive(Clone, Copy, PartialEq, Eq)]
enum Story {
    Tokens,
    Button,
    Badge,
    Avatar,
    Switch,
    Accordion,
    Popover,
}

impl Story {
    const ALL: [Story; 7] = [
        Story::Tokens,
        Story::Button,
        Story::Badge,
        Story::Avatar,
        Story::Switch,
        Story::Accordion,
        Story::Popover,
    ];

    fn label(self) -> &'static str {
        match self {
            Story::Tokens => "Tokens",
            Story::Button => "Button",
            Story::Badge => "Badge",
            Story::Avatar => "Avatar",
            Story::Switch => "Switch",
            Story::Accordion => "Accordion",
            Story::Popover => "Popover",
        }
    }

    fn description(self) -> &'static str {
        match self {
            Story::Tokens => "The shadcn design tokens: every color variable and the radius scale.",
            Story::Button => "Displays a button or a component that looks like a button.",
            Story::Badge => "Displays a badge or a component that looks like a badge.",
            Story::Avatar => "An image element with a fallback for representing the user.",
            Story::Switch => {
                "A control that allows the user to toggle between checked and not checked."
            }
            Story::Accordion => {
                "A vertically stacked set of interactive headings that each reveal a section of content."
            }
            Story::Popover => "Displays rich content in a portal, triggered by a button.",
        }
    }
}

const BUTTON_VARIANTS: [(&str, ButtonVariant); 6] = [
    ("default", ButtonVariant::Default),
    ("outline", ButtonVariant::Outline),
    ("secondary", ButtonVariant::Secondary),
    ("ghost", ButtonVariant::Ghost),
    ("destructive", ButtonVariant::Destructive),
    ("link", ButtonVariant::Link),
];

const BUTTON_SIZES: [(&str, ButtonSize); 8] = [
    ("xs", ButtonSize::Xs),
    ("sm", ButtonSize::Sm),
    ("default", ButtonSize::Default),
    ("lg", ButtonSize::Lg),
    ("icon-xs", ButtonSize::IconXs),
    ("icon-sm", ButtonSize::IconSm),
    ("icon", ButtonSize::Icon),
    ("icon-lg", ButtonSize::IconLg),
];

const BADGE_VARIANTS: [(&str, BadgeVariant); 6] = [
    ("default", BadgeVariant::Default),
    ("secondary", BadgeVariant::Secondary),
    ("destructive", BadgeVariant::Destructive),
    ("outline", BadgeVariant::Outline),
    ("ghost", BadgeVariant::Ghost),
    ("link", BadgeVariant::Link),
];

const AVATAR_SIZES: [(&str, AvatarSize); 3] = [
    ("sm", AvatarSize::Sm),
    ("default", AvatarSize::Default),
    ("lg", AvatarSize::Lg),
];

const SWITCH_SIZES: [(&str, SwitchSize); 2] =
    [("sm", SwitchSize::Sm), ("default", SwitchSize::Default)];

pub struct Storybook {
    story: Story,
    // Button controls
    button_variant: ButtonVariant,
    button_size: ButtonSize,
    button_disabled: bool,
    // Badge controls
    badge_variant: BadgeVariant,
    // Avatar controls
    avatar_size: AvatarSize,
    // Switch controls
    switch_checked: bool,
    switch_size: SwitchSize,
    switch_disabled: bool,
    // Accordion / Popover state
    accordion_open: Option<usize>,
    popover_open: bool,
}

impl Storybook {
    pub fn new() -> Self {
        Self {
            story: Story::Button,
            button_variant: ButtonVariant::Default,
            button_size: ButtonSize::Default,
            button_disabled: false,
            badge_variant: BadgeVariant::Default,
            avatar_size: AvatarSize::Default,
            switch_checked: true,
            switch_size: SwitchSize::Default,
            switch_disabled: false,
            accordion_open: Some(0),
            popover_open: false,
        }
    }

    fn toggle_theme(&mut self, _: &ClickEvent, _window: &mut Window, cx: &mut Context<Self>) {
        let next = if Theme::of(cx).dark {
            Theme::light()
        } else {
            Theme::dark()
        };
        cx.set_global(next);
        cx.notify();
    }

    // ---- chrome ------------------------------------------------------------

    fn sidebar(&self, cx: &mut Context<Self>) -> impl IntoElement + use<> {
        let theme = Theme::of(cx).clone();
        div()
            .flex()
            .flex_col()
            .w(px(200.))
            .flex_shrink_0()
            .h_full()
            .border_r_1()
            .border_color(theme.border)
            .bg(if theme.dark {
                theme.card
            } else {
                theme.secondary
            })
            .child(
                div()
                    .px(px(16.))
                    .py(px(14.))
                    .text_size(px(15.))
                    .font_weight(FontWeight::SEMIBOLD)
                    .child("rcn"),
            )
            .child(
                div()
                    .flex()
                    .flex_col()
                    .flex_1()
                    .gap(px(2.))
                    .px(px(8.))
                    .children(Story::ALL.into_iter().enumerate().map(|(index, story)| {
                        let selected = self.story == story;
                        div()
                            .id(("nav", index))
                            .px(px(8.))
                            .py(px(5.))
                            .rounded(theme.radius_sm())
                            .text_size(px(13.))
                            .line_height(px(18.))
                            .map(|el| {
                                if selected {
                                    el.bg(theme.primary)
                                        .text_color(theme.primary_foreground)
                                        .font_weight(FontWeight::MEDIUM)
                                } else {
                                    el.text_color(theme.foreground).hover(|s| s.bg(theme.muted))
                                }
                            })
                            .on_click(cx.listener(move |this, _, _, cx| {
                                this.story = story;
                                cx.notify();
                            }))
                            .child(story.label())
                    })),
            )
            .child(
                div().p(px(12.)).child(
                    Button::new("theme-toggle")
                        .variant(ButtonVariant::Outline)
                        .size(ButtonSize::Sm)
                        .on_click(cx.listener(Self::toggle_theme))
                        .child(if theme.dark {
                            "Light mode"
                        } else {
                            "Dark mode"
                        }),
                ),
            )
    }

    fn canvas(&self, cx: &mut Context<Self>) -> impl IntoElement + use<> {
        let theme = Theme::of(cx).clone();
        let preview: AnyElement = match self.story {
            Story::Tokens => self.tokens_preview(cx).into_any_element(),
            Story::Button => self.button_preview(cx).into_any_element(),
            Story::Badge => self.badge_preview().into_any_element(),
            Story::Avatar => self.avatar_preview().into_any_element(),
            Story::Switch => self.switch_preview(cx).into_any_element(),
            Story::Accordion => self.accordion_preview(cx).into_any_element(),
            Story::Popover => self.popover_preview(cx).into_any_element(),
        };
        div()
            .id("canvas")
            .flex()
            .flex_col()
            .flex_1()
            .h_full()
            .overflow_y_scroll()
            .child(
                div()
                    .flex()
                    .flex_col()
                    .gap(px(4.))
                    .px(px(28.))
                    .pt(px(24.))
                    .child(
                        div()
                            .text_size(px(18.))
                            .line_height(px(24.))
                            .font_weight(FontWeight::SEMIBOLD)
                            .child(self.story.label()),
                    )
                    .child(
                        div()
                            .text_size(px(13.))
                            .line_height(px(18.))
                            .text_color(theme.muted_foreground)
                            .child(self.story.description()),
                    ),
            )
            .child(
                div().flex().flex_1().p(px(28.)).child(
                    div()
                        .flex()
                        .flex_1()
                        .min_h(px(280.))
                        .items_center()
                        .justify_center()
                        .rounded(theme.radius_lg())
                        .border_1()
                        .border_color(theme.border)
                        .p(px(32.))
                        .child(preview),
                ),
            )
    }

    fn controls_panel(&self, cx: &mut Context<Self>) -> impl IntoElement + use<> {
        let theme = Theme::of(cx).clone();
        let rows: Vec<AnyElement> = match self.story {
            Story::Tokens => Vec::new(),
            Story::Button => vec![
                Self::control_row(
                    "variant",
                    Self::choices(
                        "button-variant",
                        &BUTTON_VARIANTS,
                        self.button_variant,
                        cx,
                        |this, v| this.button_variant = v,
                    ),
                    &theme,
                ),
                Self::control_row(
                    "size",
                    Self::choices(
                        "button-size",
                        &BUTTON_SIZES,
                        self.button_size,
                        cx,
                        |this, v| this.button_size = v,
                    ),
                    &theme,
                ),
                Self::control_row(
                    "disabled",
                    Switch::new("button-disabled")
                        .checked(self.button_disabled)
                        .size(SwitchSize::Sm)
                        .on_change(cx.listener(|this, checked: &bool, _, cx| {
                            this.button_disabled = *checked;
                            cx.notify();
                        }))
                        .into_any_element(),
                    &theme,
                ),
            ],
            Story::Badge => vec![Self::control_row(
                "variant",
                Self::choices(
                    "badge-variant",
                    &BADGE_VARIANTS,
                    self.badge_variant,
                    cx,
                    |this, v| this.badge_variant = v,
                ),
                &theme,
            )],
            Story::Avatar => vec![Self::control_row(
                "size",
                Self::choices(
                    "avatar-size",
                    &AVATAR_SIZES,
                    self.avatar_size,
                    cx,
                    |this, v| this.avatar_size = v,
                ),
                &theme,
            )],
            Story::Switch => vec![
                Self::control_row(
                    "checked",
                    Switch::new("ctl-switch-checked")
                        .checked(self.switch_checked)
                        .size(SwitchSize::Sm)
                        .on_change(cx.listener(|this, checked: &bool, _, cx| {
                            this.switch_checked = *checked;
                            cx.notify();
                        }))
                        .into_any_element(),
                    &theme,
                ),
                Self::control_row(
                    "size",
                    Self::choices(
                        "switch-size",
                        &SWITCH_SIZES,
                        self.switch_size,
                        cx,
                        |this, v| this.switch_size = v,
                    ),
                    &theme,
                ),
                Self::control_row(
                    "disabled",
                    Switch::new("ctl-switch-disabled")
                        .checked(self.switch_disabled)
                        .size(SwitchSize::Sm)
                        .on_change(cx.listener(|this, checked: &bool, _, cx| {
                            this.switch_disabled = *checked;
                            cx.notify();
                        }))
                        .into_any_element(),
                    &theme,
                ),
            ],
            Story::Accordion => vec![Self::control_row(
                "open item",
                Self::choices(
                    "accordion-open",
                    &[
                        ("none", None),
                        ("first", Some(0)),
                        ("second", Some(1)),
                        ("third", Some(2)),
                    ],
                    self.accordion_open,
                    cx,
                    |this, v| this.accordion_open = v,
                ),
                &theme,
            )],
            Story::Popover => vec![Self::control_row(
                "open",
                Switch::new("ctl-popover-open")
                    .checked(self.popover_open)
                    .size(SwitchSize::Sm)
                    .on_change(cx.listener(|this, open: &bool, _, cx| {
                        this.popover_open = *open;
                        cx.notify();
                    }))
                    .into_any_element(),
                &theme,
            )],
        };

        div()
            .flex()
            .flex_col()
            .w(px(280.))
            .flex_shrink_0()
            .h_full()
            .border_l_1()
            .border_color(theme.border)
            .child(
                div()
                    .px(px(16.))
                    .py(px(14.))
                    .border_b_1()
                    .border_color(theme.border)
                    .text_size(px(13.))
                    .font_weight(FontWeight::MEDIUM)
                    .child("Controls"),
            )
            .map(|el| {
                if rows.is_empty() {
                    el.child(
                        div()
                            .p(px(16.))
                            .text_size(px(13.))
                            .text_color(theme.muted_foreground)
                            .child("This story has no controls."),
                    )
                } else {
                    el.child(
                        div()
                            .flex()
                            .flex_col()
                            .gap(px(16.))
                            .p(px(16.))
                            .children(rows),
                    )
                }
            })
    }

    /// One labeled control row: prop name above the widget.
    fn control_row(label: &'static str, control: AnyElement, theme: &Theme) -> AnyElement {
        div()
            .flex()
            .flex_col()
            .gap(px(6.))
            .child(
                div()
                    .text_size(px(12.))
                    .line_height(px(16.))
                    .font_weight(FontWeight::MEDIUM)
                    .text_color(theme.muted_foreground)
                    .child(label),
            )
            .child(control)
            .into_any_element()
    }

    /// A segmented value picker: one xs button per option, the current value
    /// rendered filled.
    fn choices<T: Copy + PartialEq + 'static>(
        id: &'static str,
        options: &[(&'static str, T)],
        current: T,
        cx: &mut Context<Self>,
        set: impl Fn(&mut Self, T) + Copy + 'static,
    ) -> AnyElement {
        div()
            .flex()
            .flex_row()
            .flex_wrap()
            .gap(px(4.))
            .children(options.iter().enumerate().map(|(index, (label, value))| {
                let value = *value;
                let selected = value == current;
                Button::new((id, index))
                    .variant(if selected {
                        ButtonVariant::Default
                    } else {
                        ButtonVariant::Outline
                    })
                    .size(ButtonSize::Xs)
                    .on_click(cx.listener(move |this, _, _, cx| {
                        set(this, value);
                        cx.notify();
                    }))
                    .child(*label)
            }))
            .into_any_element()
    }

    // ---- stories -----------------------------------------------------------

    fn tokens_preview(&self, cx: &App) -> impl IntoElement + use<> {
        let theme = Theme::of(cx).clone();
        let swatches = [
            ("background", theme.background),
            ("foreground", theme.foreground),
            ("card", theme.card),
            ("card-foreground", theme.card_foreground),
            ("popover", theme.popover),
            ("popover-foreground", theme.popover_foreground),
            ("primary", theme.primary),
            ("primary-foreground", theme.primary_foreground),
            ("secondary", theme.secondary),
            ("secondary-foreground", theme.secondary_foreground),
            ("muted", theme.muted),
            ("muted-foreground", theme.muted_foreground),
            ("accent", theme.accent),
            ("accent-foreground", theme.accent_foreground),
            ("destructive", theme.destructive),
            ("destructive-foreground", theme.destructive_foreground),
            ("border", theme.border),
            ("input", theme.input),
            ("ring", theme.ring),
        ];
        let radii = [
            ("sm", theme.radius_sm()),
            ("md", theme.radius_md()),
            ("lg", theme.radius_lg()),
            ("xl", theme.radius_xl()),
        ];
        div()
            .flex()
            .flex_col()
            .gap(px(24.))
            .max_w(px(480.))
            .child(div().flex().flex_row().flex_wrap().gap(px(8.)).children(
                swatches.into_iter().map(|(name, color)| {
                    div()
                        .flex()
                        .flex_col()
                        .items_center()
                        .gap(px(4.))
                        .w(px(72.))
                        .child(
                            div()
                                .size(px(32.))
                                .rounded(theme.radius_sm())
                                .border_1()
                                .border_color(theme.border)
                                .bg(color),
                        )
                        .child(
                            div()
                                .text_size(px(10.))
                                .line_height(px(14.))
                                .text_color(theme.muted_foreground)
                                .child(name),
                        )
                }),
            ))
            .child(div().flex().flex_row().items_end().gap(px(12.)).children(
                radii.into_iter().map(|(name, radius)| {
                    div()
                        .flex()
                        .flex_col()
                        .items_center()
                        .gap(px(4.))
                        .child(
                            div()
                                .size(px(40.))
                                .rounded(radius)
                                .border_1()
                                .border_color(theme.ring)
                                .bg(theme.muted),
                        )
                        .child(
                            div()
                                .text_size(px(11.))
                                .text_color(theme.muted_foreground)
                                .child(name),
                        )
                }),
            ))
    }

    fn button_preview(&self, cx: &App) -> impl IntoElement + use<> {
        let theme = Theme::of(cx);
        let icon_only = matches!(
            self.button_size,
            ButtonSize::Icon | ButtonSize::IconXs | ButtonSize::IconSm | ButtonSize::IconLg
        );
        let button = Button::new("preview-button")
            .variant(self.button_variant)
            .size(self.button_size)
            .disabled(self.button_disabled);
        if icon_only {
            button.child(
                gpui::svg()
                    .path(crate::assets::ICON_CHEVRON_RIGHT)
                    .size(px(16.))
                    .text_color(theme.foreground),
            )
        } else {
            button.child("Button")
        }
    }

    fn badge_preview(&self) -> impl IntoElement + use<> {
        Badge::new().variant(self.badge_variant).child("Badge")
    }

    fn avatar_preview(&self) -> impl IntoElement + use<> {
        div()
            .flex()
            .flex_col()
            .items_center()
            .gap(px(24.))
            .child(Avatar::new("CN").size(self.avatar_size))
            .child(
                AvatarGroup::new()
                    .child(Avatar::new("CN").size(self.avatar_size))
                    .child(Avatar::new("ER").size(self.avatar_size))
                    .child(Avatar::new("LR").size(self.avatar_size))
                    .child(AvatarGroupCount::new(3).size(self.avatar_size)),
            )
    }

    fn switch_preview(&self, cx: &mut Context<Self>) -> impl IntoElement + use<> {
        Switch::new("preview-switch")
            .checked(self.switch_checked)
            .size(self.switch_size)
            .disabled(self.switch_disabled)
            .on_change(cx.listener(|this, checked: &bool, _, cx| {
                this.switch_checked = *checked;
                cx.notify();
            }))
    }

    fn accordion_preview(&self, cx: &mut Context<Self>) -> impl IntoElement + use<> {
        let items = [
            (
                "Product Information",
                "Our flagship product combines cutting-edge technology with sleek design. \
                 Built with premium materials, it offers unparalleled performance and \
                 reliability.",
            ),
            (
                "Shipping Details",
                "We offer worldwide shipping through trusted courier partners. Standard \
                 delivery takes 3-5 business days, while express shipping ensures delivery \
                 within 1-2 business days.",
            ),
            (
                "Return Policy",
                "We stand behind our products with a comprehensive 30-day return policy. If \
                 you're not completely satisfied, simply return the item in its original \
                 condition.",
            ),
        ];
        let count = items.len();
        div()
            .w(px(384.))
            .child(Accordion::new().children(items.into_iter().enumerate().map(
                |(index, (title, body))| {
                    AccordionItem::new(("accordion-item", index))
                        .trigger(title)
                        .content(body)
                        .open(self.accordion_open == Some(index))
                        .last(index + 1 == count)
                        .on_toggle(cx.listener(move |this, _: &ClickEvent, _, cx| {
                            this.accordion_open = if this.accordion_open == Some(index) {
                                None
                            } else {
                                Some(index)
                            };
                            cx.notify();
                        }))
                },
            )))
    }

    fn popover_preview(&self, cx: &mut Context<Self>) -> impl IntoElement + use<> {
        Popover::new("preview-popover")
            .trigger(
                Button::new("popover-trigger")
                    .variant(ButtonVariant::Outline)
                    .child("Open popover"),
            )
            .open(self.popover_open)
            .on_open_change(cx.listener(|this, open: &bool, _, cx| {
                this.popover_open = *open;
                cx.notify();
            }))
            .content(
                PopoverHeader::new()
                    .child(PopoverTitle::new().child("Dimensions"))
                    .child(PopoverDescription::new().child("Set the dimensions for the layer.")),
            )
    }
}

impl Render for Storybook {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let theme = Theme::of(cx).clone();
        div()
            .flex()
            .flex_row()
            .size_full()
            .bg(theme.background)
            .text_color(theme.foreground)
            .child(self.sidebar(cx))
            .child(self.canvas(cx))
            .child(self.controls_panel(cx))
    }
}

impl Default for Storybook {
    fn default() -> Self {
        Self::new()
    }
}
