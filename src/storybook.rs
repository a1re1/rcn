//! The rcn storybook shell: a three-pane component explorer. The sidebar
//! lists every component; the canvas renders the selected one in isolation;
//! the controls panel binds each of the component's props (variant, size,
//! state flags) to interactive widgets — built from the library's own Button
//! and Switch.
//!
//! The Tokens story is special: its controls are global, modeled on
//! ui.shadcn.com/create — pick a base gray family, a brand color (presets or
//! custom hue/saturation/lightness sliders), and a radius, or shuffle the
//! whole thing — and every other story picks the changes up live through the
//! `Theme` global.

use std::time::{SystemTime, UNIX_EPOCH};

use gpui::{
    AnyElement, App, ClickEvent, Context, DragMoveEvent, Empty, FontWeight, Hsla, Window, div,
    hsla, prelude::*, px, relative, rgb,
};

use crate::assets::IconLibrary;
use crate::components::{
    Accordion, AccordionItem, Avatar, AvatarGroup, AvatarGroupCount, AvatarSize, Badge,
    BadgeVariant, Button, ButtonSize, ButtonVariant, Popover, PopoverDescription, PopoverHeader,
    PopoverTitle, Separator, Skeleton, Switch, SwitchSize,
};
use crate::theme::{BaseColor, Theme, oklch};

#[derive(Clone, Copy, PartialEq, Eq)]
enum Story {
    Tokens,
    Button,
    Badge,
    Avatar,
    Switch,
    Accordion,
    Popover,
    Separator,
    Skeleton,
}

impl Story {
    const ALL: [Story; 9] = [
        Story::Tokens,
        Story::Button,
        Story::Badge,
        Story::Avatar,
        Story::Switch,
        Story::Accordion,
        Story::Popover,
        Story::Separator,
        Story::Skeleton,
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
            Story::Separator => "Separator",
            Story::Skeleton => "Skeleton",
        }
    }

    fn description(self) -> &'static str {
        match self {
            Story::Tokens => {
                "Global design tokens. Tune the base palette, brand color, and radius — or \
                 shuffle — and every component picks the changes up live."
            }
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
            Story::Separator => "Visually or semantically separates content.",
            Story::Skeleton => "Use to show a placeholder while content is loading.",
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

/// Font choices: fonts bundled with macOS, so every pick resolves. `None`
/// is gpui's default UI font.
const FONTS: [(&str, Option<&str>); 8] = [
    ("system", None),
    ("Helvetica Neue", Some("Helvetica Neue")),
    ("Avenir Next", Some("Avenir Next")),
    ("Futura", Some("Futura")),
    ("Gill Sans", Some("Gill Sans")),
    ("Georgia", Some("Georgia")),
    ("Palatino", Some("Palatino")),
    ("Menlo", Some("Menlo")),
];

/// Theme presets — shadcn create's "Theme" picker — as oklch (l, c, h) at
/// the light-mode anchor (tailwind's 600-ish step). `None` is the neutral
/// default (black / near-white).
const THEME_PRESETS: [(&str, Option<(f32, f32, f32)>); 8] = [
    ("default", None),
    ("blue", Some((0.546, 0.245, 262.9))),
    ("green", Some((0.627, 0.194, 149.2))),
    ("orange", Some((0.646, 0.222, 41.1))),
    ("red", Some((0.577, 0.245, 27.3))),
    ("rose", Some((0.586, 0.253, 17.6))),
    ("violet", Some((0.541, 0.281, 293.0))),
    ("yellow", Some((0.795, 0.184, 86.0))),
];

/// The global token adjustments layered over the stock shadcn themes.
struct TokenSettings {
    base: BaseColor,
    /// Body font family; `None` is gpui's default.
    font_sans: Option<&'static str>,
    /// Heading font family; `None` falls back to the body font.
    font_heading: Option<&'static str>,
    icons: IconLibrary,
    /// When false, primary stays the stock neutral (black / near-white).
    custom_primary: bool,
    /// Custom brand color in HSL, each 0..1 (gpui's `Hsla` space).
    hue: f32,
    saturation: f32,
    lightness: f32,
    radius: f32,
}

impl Default for TokenSettings {
    fn default() -> Self {
        Self {
            base: BaseColor::Neutral,
            font_sans: None,
            font_heading: None,
            icons: IconLibrary::default(),
            custom_primary: false,
            hue: 0.6,
            saturation: 0.7,
            lightness: 0.5,
            radius: 10.,
        }
    }
}

impl TokenSettings {
    /// The brand primary for the given mode, if customized. Dark mode lifts
    /// the lightness a step, like shadcn's dark palettes do.
    fn primary(&self, dark: bool) -> Option<Hsla> {
        self.custom_primary.then(|| {
            let l = if dark {
                (self.lightness + 0.08).min(0.85)
            } else {
                self.lightness
            };
            hsla(self.hue, self.saturation, l, 1.)
        })
    }
}

/// Typed payload identifying which slider a drag belongs to.
struct SliderDrag(&'static str);

/// Invisible drag preview: sliders drag a value, not a visual.
struct DragPreview;

impl Render for DragPreview {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        Empty
    }
}

pub struct Storybook {
    story: Story,
    dark: bool,
    tokens: TokenSettings,
    /// A (light, dark) pair imported from shadcn theme CSS; overrides the
    /// generated tokens until another token control is touched.
    imported: Option<(Theme, Theme)>,
    /// Feedback from the last import attempt: (message, success).
    import_status: Option<(String, bool)>,
    rng: u64,
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
        let seed = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .map(|d| d.subsec_nanos() as u64 ^ d.as_secs())
            .unwrap_or(0x9e3779b9);
        Self {
            story: Story::Tokens,
            dark: false,
            tokens: TokenSettings::default(),
            imported: None,
            import_status: None,
            rng: seed | 1,
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

    /// Rebuild the global `Theme` from the current token settings and mode.
    fn apply_tokens(&self, cx: &mut Context<Self>) {
        let theme = if let Some((light, dark)) = &self.imported {
            if self.dark {
                dark.clone()
            } else {
                light.clone()
            }
        } else {
            let mut theme = Theme::with_base(self.tokens.base, self.dark);
            if let Some(primary) = self.tokens.primary(self.dark) {
                theme.primary = primary;
                theme.primary_foreground = if primary.l > 0.65 {
                    rgb(0x171717).into()
                } else {
                    rgb(0xfafafa).into()
                };
                theme.ring = primary;
            }
            theme.radius = px(self.tokens.radius);
            theme
        };
        // Fonts and icons layer over both generated and imported themes;
        // an imported theme's own fonts win unless the picker overrides them.
        let mut theme = theme;
        if let Some(font) = self.tokens.font_sans {
            theme.font_sans = Some(font.into());
        }
        if let Some(font) = self.tokens.font_heading {
            theme.font_heading = Some(font.into());
        }
        theme.icons = self.tokens.icons;
        cx.set_global(theme);
        cx.notify();
    }

    /// Drop an imported theme (called whenever a generated-token control is
    /// touched, so the controls always describe what's on screen).
    fn clear_import(&mut self) {
        self.imported = None;
        self.import_status = None;
    }

    /// Back-fill the token controls from an imported theme so the panel
    /// reflects it and later adjustments continue from the imported look
    /// instead of reverting to the previous settings.
    fn sync_settings_from(&mut self, light: &Theme) {
        let primary = light.primary;
        // A near-black primary is the stock neutral default; anything else is
        // a brand color the sliders should show.
        if primary.s < 0.02 && primary.l < 0.1 {
            self.tokens.custom_primary = false;
        } else {
            self.tokens.custom_primary = true;
            self.tokens.hue = primary.h;
            self.tokens.saturation = primary.s;
            self.tokens.lightness = primary.l;
        }
        self.tokens.radius = (light.radius / px(1.)).clamp(0., 24.);

        // Infer the closest base gray family from the tinted neutrals.
        let distance = |base: BaseColor| -> f32 {
            let candidate = Theme::with_base(base, false);
            [
                (candidate.secondary, light.secondary),
                (candidate.border, light.border),
                (candidate.muted_foreground, light.muted_foreground),
            ]
            .into_iter()
            .map(|(a, b)| {
                let (a, b): (gpui::Rgba, gpui::Rgba) = (a.into(), b.into());
                (a.r - b.r).powi(2) + (a.g - b.g).powi(2) + (a.b - b.b).powi(2)
            })
            .sum()
        };
        self.tokens.base = BaseColor::ALL
            .into_iter()
            .min_by(|a, b| distance(*a).total_cmp(&distance(*b)))
            .unwrap_or_default();

        // Fonts only sync when the imported family is in the picker list;
        // otherwise the imported theme keeps carrying them until edited.
        self.tokens.font_sans = FONTS
            .iter()
            .filter_map(|(_, family)| *family)
            .find(|family| Some(*family) == light.font_sans.as_deref());
        self.tokens.font_heading = FONTS
            .iter()
            .filter_map(|(_, family)| *family)
            .find(|family| Some(*family) == light.font_heading.as_deref());
    }

    fn import_from_clipboard(&mut self, cx: &mut Context<Self>) {
        let text = cx.read_from_clipboard().and_then(|item| item.text());
        match text.as_deref().and_then(Theme::from_shadcn_css) {
            Some(pair) => {
                self.sync_settings_from(&pair.0);
                self.imported = Some(pair);
                self.import_status = Some(("Theme imported — light + dark applied.".into(), true));
                self.apply_tokens(cx);
            }
            None => {
                self.import_status = Some((
                    "Clipboard doesn't look like shadcn theme CSS (:root { --token: … }).".into(),
                    false,
                ));
                cx.notify();
            }
        }
    }

    fn toggle_theme(&mut self, _: &ClickEvent, _window: &mut Window, cx: &mut Context<Self>) {
        self.dark = !self.dark;
        self.apply_tokens(cx);
    }

    /// xorshift64 — good enough to shuffle a palette, no dependency needed.
    fn rand(&mut self) -> f32 {
        let mut x = self.rng;
        x ^= x << 13;
        x ^= x >> 7;
        x ^= x << 17;
        self.rng = x;
        (x >> 40) as f32 / (1u64 << 24) as f32
    }

    fn shuffle(&mut self, cx: &mut Context<Self>) {
        self.clear_import();
        self.tokens.custom_primary = true;
        self.tokens.hue = self.rand();
        self.tokens.saturation = 0.5 + 0.45 * self.rand();
        self.tokens.lightness = 0.35 + 0.35 * self.rand();
        self.tokens.base = BaseColor::ALL[(self.rand() * 5.) as usize % 5];
        self.tokens.radius = [0., 4., 6., 8., 10., 12., 16., 20.][(self.rand() * 8.) as usize % 8];
        self.apply_tokens(cx);
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
                    .when_some(theme.heading_font(), |el, font| el.font_family(font))
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
            Story::Separator => Self::separator_preview(cx).into_any_element(),
            Story::Skeleton => Self::skeleton_preview().into_any_element(),
        };
        div()
            .id("canvas")
            .flex()
            .flex_col()
            .flex_1()
            // Shrinkable below its content's min-width, so a narrow window
            // squeezes the canvas instead of pushing the controls panel out.
            .min_w(px(0.))
            .h_full()
            .overflow_y_scroll()
            .overflow_x_hidden()
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
                            .when_some(theme.heading_font(), |el, font| el.font_family(font))
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
            Story::Tokens => self.token_controls(cx),
            Story::Button => vec![
                Self::control_row(
                    "variant",
                    Self::choices(
                        "button-variant",
                        &BUTTON_VARIANTS,
                        self.button_variant,
                        cx,
                        |this, v, cx| {
                            this.button_variant = v;
                            cx.notify();
                        },
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
                        |this, v, cx| {
                            this.button_size = v;
                            cx.notify();
                        },
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
                    |this, v, cx| {
                        this.badge_variant = v;
                        cx.notify();
                    },
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
                    |this, v, cx| {
                        this.avatar_size = v;
                        cx.notify();
                    },
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
                        |this, v, cx| {
                            this.switch_size = v;
                            cx.notify();
                        },
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
                    |this, v, cx| {
                        this.accordion_open = v;
                        cx.notify();
                    },
                ),
                &theme,
            )],
            Story::Separator => Vec::new(),
            Story::Skeleton => Vec::new(),
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
            .child(
                div()
                    .id("controls")
                    .flex()
                    .flex_col()
                    .flex_1()
                    .overflow_y_scroll()
                    .child(
                        div()
                            .flex()
                            .flex_col()
                            .gap(px(16.))
                            .p(px(16.))
                            .children(rows),
                    ),
            )
    }

    // ---- control widgets ---------------------------------------------------

    /// One labeled control row: prop name above the widget.
    fn control_row(label: impl Into<String>, control: AnyElement, theme: &Theme) -> AnyElement {
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
                    .child(label.into()),
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
        set: impl Fn(&mut Self, T, &mut Context<Self>) + Copy + 'static,
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
                    .on_click(cx.listener(move |this, _, _, cx| set(this, value, cx)))
                    .child(*label)
            }))
            .into_any_element()
    }

    /// A horizontal slider over 0..1. Dragging anywhere (even past the track)
    /// keeps updating; `set` receives the new fraction.
    fn slider(
        id: &'static str,
        fraction: f32,
        cx: &mut Context<Self>,
        set: impl Fn(&mut Self, f32) + Copy + 'static,
    ) -> AnyElement {
        let theme = Theme::of(cx).clone();
        let fraction = fraction.clamp(0., 1.);
        div()
            .id(id)
            .h(px(16.))
            .w_full()
            .flex()
            .items_center()
            .on_drag(SliderDrag(id), |_, _, _, cx| cx.new(|_| DragPreview))
            .on_drag_move(
                cx.listener(move |this, event: &DragMoveEvent<SliderDrag>, _, cx| {
                    if event.drag(cx).0 == id {
                        let f = ((event.event.position.x - event.bounds.origin.x)
                            / event.bounds.size.width)
                            .clamp(0., 1.);
                        set(this, f);
                        this.apply_tokens(cx);
                    }
                }),
            )
            .child(
                div()
                    .relative()
                    .w_full()
                    .h(px(4.))
                    .rounded_full()
                    .bg(theme.input)
                    .child(
                        div()
                            .h_full()
                            .w(relative(fraction))
                            .rounded_full()
                            .bg(theme.primary),
                    )
                    .child(Self::slider_thumb(fraction, &theme)),
            )
            .into_any_element()
    }

    /// The hue slider: same interaction as [`Self::slider`], but the track is
    /// a rainbow.
    fn hue_slider(
        id: &'static str,
        fraction: f32,
        cx: &mut Context<Self>,
        set: impl Fn(&mut Self, f32) + Copy + 'static,
    ) -> AnyElement {
        let theme = Theme::of(cx).clone();
        let fraction = fraction.clamp(0., 1.);
        const SLICES: usize = 16;
        div()
            .id(id)
            .h(px(16.))
            .w_full()
            .flex()
            .items_center()
            .on_drag(SliderDrag(id), |_, _, _, cx| cx.new(|_| DragPreview))
            .on_drag_move(
                cx.listener(move |this, event: &DragMoveEvent<SliderDrag>, _, cx| {
                    if event.drag(cx).0 == id {
                        let f = ((event.event.position.x - event.bounds.origin.x)
                            / event.bounds.size.width)
                            .clamp(0., 1.);
                        set(this, f);
                        this.apply_tokens(cx);
                    }
                }),
            )
            .child(
                div()
                    .relative()
                    .w_full()
                    .h(px(4.))
                    .flex()
                    .flex_row()
                    .children((0..SLICES).map(|i| {
                        div()
                            .flex_1()
                            .h_full()
                            .bg(hsla(i as f32 / SLICES as f32, 0.8, 0.55, 1.))
                            .when(i == 0, |el| el.rounded_l_full())
                            .when(i == SLICES - 1, |el| el.rounded_r_full())
                    }))
                    .child(Self::slider_thumb(fraction, &theme)),
            )
            .into_any_element()
    }

    fn slider_thumb(fraction: f32, theme: &Theme) -> AnyElement {
        div()
            .absolute()
            .top(px(-4.))
            .left(relative(fraction))
            .ml(px(-6.))
            .size(px(12.))
            .rounded_full()
            .bg(theme.background)
            .border_1()
            .border_color(theme.ring)
            .shadow_xs()
            .into_any_element()
    }

    // ---- token (global) controls -------------------------------------------

    fn token_controls(&self, cx: &mut Context<Self>) -> Vec<AnyElement> {
        let theme = Theme::of(cx).clone();

        let base_row = Self::choices(
            "base-color",
            &BaseColor::ALL.map(|b| (b.label(), b)),
            self.tokens.base,
            cx,
            |this, v, cx| {
                this.clear_import();
                this.tokens.base = v;
                this.apply_tokens(cx);
            },
        );

        let current_primary = self.tokens.primary(false);
        let preset_row = div()
            .flex()
            .flex_row()
            .flex_wrap()
            .gap(px(6.))
            .children(
                THEME_PRESETS
                    .iter()
                    .enumerate()
                    .map(|(index, (_name, preset))| {
                        let swatch: Hsla = match preset {
                            Some((l, c, h)) => oklch(*l, *c, *h),
                            None => {
                                if theme.dark {
                                    rgb(0xe5e5e5).into()
                                } else {
                                    rgb(0x000000).into()
                                }
                            }
                        };
                        let selected = match (preset, current_primary) {
                            (None, None) => true,
                            (Some((l, c, h)), Some(current)) => {
                                let p = oklch(*l, *c, *h);
                                (p.h - current.h).abs() < 0.01 && (p.l - current.l).abs() < 0.02
                            }
                            _ => false,
                        };
                        let value = *preset;
                        div()
                            .id(("theme-preset", index))
                            .size(px(22.))
                            .rounded_full()
                            .bg(swatch)
                            .border_2()
                            .border_color(if selected { theme.ring } else { theme.border })
                            .on_click(cx.listener(move |this, _, _, cx| {
                                this.clear_import();
                                match value {
                                    Some((l, c, h)) => {
                                        let p = oklch(l, c, h);
                                        this.tokens.custom_primary = true;
                                        this.tokens.hue = p.h;
                                        this.tokens.saturation = p.s;
                                        this.tokens.lightness = p.l;
                                    }
                                    None => this.tokens.custom_primary = false,
                                }
                                this.apply_tokens(cx);
                            }))
                    }),
            )
            .into_any_element();

        let hue = self.tokens.hue;
        let saturation = self.tokens.saturation;
        let lightness = self.tokens.lightness;
        let radius = self.tokens.radius;

        let import_row = div()
            .flex()
            .flex_col()
            .gap(px(6.))
            .child(
                Button::new("import-theme")
                    .variant(ButtonVariant::Outline)
                    .size(ButtonSize::Sm)
                    .on_click(cx.listener(|this, _, _, cx| this.import_from_clipboard(cx)))
                    .child("Paste from clipboard"),
            )
            .when_some(self.import_status.clone(), |el, (message, success)| {
                el.child(
                    div()
                        .text_size(px(11.))
                        .line_height(px(15.))
                        .text_color(if success {
                            theme.muted_foreground
                        } else {
                            theme.destructive
                        })
                        .child(message),
                )
            })
            .into_any_element();

        vec![
            Self::control_row("import theme css", import_row, &theme),
            Self::control_row("base color", base_row, &theme),
            Self::control_row("theme", preset_row, &theme),
            Self::control_row(
                format!("hue · {:.0}°", hue * 360.),
                Self::hue_slider("hue-slider", hue, cx, |this, f| {
                    this.clear_import();
                    this.tokens.custom_primary = true;
                    this.tokens.hue = f;
                }),
                &theme,
            ),
            Self::control_row(
                format!("saturation · {:.0}%", saturation * 100.),
                Self::slider("saturation-slider", saturation, cx, |this, f| {
                    this.clear_import();
                    this.tokens.custom_primary = true;
                    this.tokens.saturation = f;
                }),
                &theme,
            ),
            Self::control_row(
                format!("lightness · {:.0}%", lightness * 100.),
                Self::slider("lightness-slider", lightness, cx, |this, f| {
                    this.clear_import();
                    this.tokens.custom_primary = true;
                    this.tokens.lightness = f;
                }),
                &theme,
            ),
            Self::control_row(
                format!("radius · {:.0}px", radius),
                Self::slider("radius-slider", radius / 24., cx, |this, f| {
                    this.clear_import();
                    this.tokens.radius = (f * 24.).round();
                }),
                &theme,
            ),
            Self::control_row(
                "font",
                Self::choices(
                    "font-sans",
                    &FONTS,
                    self.tokens.font_sans,
                    cx,
                    |this, v, cx| {
                        this.tokens.font_sans = v;
                        this.apply_tokens(cx);
                    },
                ),
                &theme,
            ),
            Self::control_row(
                "heading font",
                Self::choices(
                    "font-heading",
                    &FONTS,
                    self.tokens.font_heading,
                    cx,
                    |this, v, cx| {
                        this.tokens.font_heading = v;
                        this.apply_tokens(cx);
                    },
                ),
                &theme,
            ),
            Self::control_row(
                "icon library",
                Self::choices(
                    "icon-library",
                    &IconLibrary::ALL.map(|lib| (lib.label(), lib)),
                    self.tokens.icons,
                    cx,
                    |this, v, cx| {
                        this.tokens.icons = v;
                        this.apply_tokens(cx);
                    },
                ),
                &theme,
            ),
            div()
                .flex()
                .flex_row()
                .gap(px(8.))
                .pt(px(4.))
                .child(
                    Button::new("shuffle-tokens")
                        .variant(ButtonVariant::Default)
                        .size(ButtonSize::Sm)
                        .on_click(cx.listener(|this, _, _, cx| this.shuffle(cx)))
                        .child("Shuffle"),
                )
                .child(
                    Button::new("reset-tokens")
                        .variant(ButtonVariant::Outline)
                        .size(ButtonSize::Sm)
                        .on_click(cx.listener(|this, _, _, cx| {
                            this.clear_import();
                            this.tokens = TokenSettings::default();
                            this.apply_tokens(cx);
                        }))
                        .child("Reset"),
                )
                .into_any_element(),
        ]
    }

    // ---- stories -----------------------------------------------------------

    fn tokens_preview(&self, cx: &mut Context<Self>) -> impl IntoElement + use<> {
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
            // Live sample so slider feedback is instant without switching
            // stories.
            .child(
                div()
                    .flex()
                    .flex_row()
                    .flex_wrap()
                    .items_center()
                    .gap(px(12.))
                    .pt(px(8.))
                    .border_t_1()
                    .border_color(theme.border)
                    .child(Button::new("tokens-button").child("Button"))
                    .child(
                        Button::new("tokens-outline")
                            .variant(ButtonVariant::Outline)
                            .child("Outline"),
                    )
                    .child(Badge::new().child("Badge"))
                    .child(
                        Switch::new("tokens-switch")
                            .checked(self.switch_checked)
                            .on_change(cx.listener(|this, checked: &bool, _, cx| {
                                this.switch_checked = *checked;
                                cx.notify();
                            })),
                    )
                    .child(Avatar::new("CN")),
            )
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
                    .path(theme.icons.chevron_right())
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
                    .size(self.avatar_size)
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

    fn separator_preview(cx: &App) -> impl IntoElement + use<> {
        let theme = Theme::of(cx).clone();
        div()
            .flex()
            .flex_col()
            .gap(px(16.))
            .w(px(288.))
            .child(
                div()
                    .flex()
                    .flex_col()
                    .gap(px(4.))
                    .child(
                        div()
                            .text_size(px(14.))
                            .line_height(px(20.))
                            .font_weight(FontWeight::MEDIUM)
                            .child("rcn"),
                    )
                    .child(
                        div()
                            .text_size(px(14.))
                            .line_height(px(20.))
                            .text_color(theme.muted_foreground)
                            .child("A copy-paste component library for gpui."),
                    ),
            )
            .child(Separator::new())
            .child(
                div()
                    .flex()
                    .flex_row()
                    .items_center()
                    .gap(px(16.))
                    .h(px(20.))
                    .text_size(px(14.))
                    .child("Blog")
                    .child(Separator::vertical())
                    .child("Docs")
                    .child(Separator::vertical())
                    .child("Source"),
            )
    }

    fn skeleton_preview() -> impl IntoElement + use<> {
        // Mirrors the shadcn docs example: avatar row + card-shaped block.
        div()
            .flex()
            .flex_col()
            .gap(px(24.))
            .child(
                div()
                    .flex()
                    .flex_row()
                    .items_center()
                    .gap(px(16.))
                    .child(Skeleton::new().w(px(48.)).h(px(48.)).rounded_full())
                    .child(
                        div()
                            .flex()
                            .flex_col()
                            .gap(px(8.))
                            .child(Skeleton::new().w(px(200.)).h(px(16.)))
                            .child(Skeleton::new().w(px(160.)).h(px(16.))),
                    ),
            )
            .child(
                div()
                    .flex()
                    .flex_col()
                    .gap(px(8.))
                    .child(Skeleton::new().w(px(200.)).h(px(100.)))
                    .child(Skeleton::new().w(px(200.)).h(px(16.)))
                    .child(Skeleton::new().w(px(160.)).h(px(16.))),
            )
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
            .when_some(theme.font_sans.clone(), |el, font| el.font_family(font))
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

#[cfg(test)]
mod tests {
    use super::*;

    /// Importing a theme must back-fill the controls: primary onto the
    /// sliders, radius onto its slider, and the nearest base gray family.
    #[test]
    fn import_syncs_controls() {
        let css = r#"
:root {
  --background: oklch(1 0 0);
  --foreground: oklch(0.147 0.004 49.3);
  --primary: oklch(0.553 0.195 38.402);
  --secondary: oklch(0.967 0.001 286.375);
  --muted-foreground: oklch(0.547 0.021 43.1);
  --border: oklch(0.922 0.005 34.3);
  --radius: 0;
}"#;
        let (light, _) = Theme::from_shadcn_css(css).expect("should parse");
        let mut storybook = Storybook::new();
        storybook.sync_settings_from(&light);
        assert!(storybook.tokens.custom_primary);
        assert!(
            storybook.tokens.hue < 0.12,
            "red-orange brand should land on a warm hue, got {}",
            storybook.tokens.hue
        );
        assert!(storybook.tokens.saturation > 0.3);
        assert_eq!(storybook.tokens.radius, 0.);
        // Warm-tinted neutrals sit closest to the stone family.
        assert_eq!(storybook.tokens.base, BaseColor::Stone);
    }

    /// A stock-neutral theme (black primary) must not flip the sliders into
    /// custom-brand mode.
    #[test]
    fn neutral_import_keeps_default_primary() {
        let mut storybook = Storybook::new();
        storybook.tokens.custom_primary = true;
        storybook.sync_settings_from(&Theme::light());
        assert!(!storybook.tokens.custom_primary);
        assert_eq!(storybook.tokens.base, BaseColor::Neutral);
    }
}
