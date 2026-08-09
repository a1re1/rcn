//! shadcn's design-token system, ported from the base-vega style's CSS
//! variables (ui.shadcn.com, neutral palette). Every color token from
//! `:root` / `.dark` in shadcn's globals.css maps to a field here; values are
//! sRGB conversions of the original oklch values.
//!
//! The active theme lives in the gpui `Global` store: read it in a component
//! with `Theme::of(cx)`, swap it with `cx.set_global(Theme::dark())`.

use gpui::{App, Global, Hsla, Pixels, px, rgb, rgba};

/// Design tokens for one color scheme (shadcn `:root` or `.dark`).
#[derive(Clone, Debug)]
pub struct Theme {
    /// True when this is the dark scheme; components use it where shadcn
    /// styles carry `dark:` overrides that aren't pure token swaps.
    pub dark: bool,

    pub background: Hsla,
    pub foreground: Hsla,
    pub card: Hsla,
    pub card_foreground: Hsla,
    pub popover: Hsla,
    pub popover_foreground: Hsla,
    pub primary: Hsla,
    pub primary_foreground: Hsla,
    pub secondary: Hsla,
    pub secondary_foreground: Hsla,
    pub muted: Hsla,
    pub muted_foreground: Hsla,
    pub accent: Hsla,
    pub accent_foreground: Hsla,
    pub destructive: Hsla,
    pub destructive_foreground: Hsla,
    pub border: Hsla,
    pub input: Hsla,
    pub ring: Hsla,

    /// Base radius (shadcn `--radius: 0.625rem` = 10px). The sm/md/lg/xl
    /// scale derives from it, mirroring shadcn's calc() chain.
    pub radius: Pixels,
}

impl Global for Theme {}

impl Theme {
    pub fn of(cx: &App) -> &Theme {
        cx.global::<Theme>()
    }

    /// shadcn `--radius-sm` = radius × 0.6
    pub fn radius_sm(&self) -> Pixels {
        self.radius * 0.6
    }

    /// shadcn `--radius-md` = radius × 0.8
    pub fn radius_md(&self) -> Pixels {
        self.radius * 0.8
    }

    /// shadcn `--radius-lg` = radius × 1.0
    pub fn radius_lg(&self) -> Pixels {
        self.radius
    }

    /// shadcn `--radius-xl` = radius × 1.4
    pub fn radius_xl(&self) -> Pixels {
        self.radius * 1.4
    }

    /// shadcn `:root` (light), neutral palette.
    pub fn light() -> Self {
        Self {
            dark: false,
            background: rgb(0xffffff).into(),
            foreground: rgb(0x000000).into(),
            card: rgb(0xffffff).into(),
            card_foreground: rgb(0x000000).into(),
            popover: rgb(0xffffff).into(),
            popover_foreground: rgb(0x000000).into(),
            primary: rgb(0x000000).into(),
            primary_foreground: rgb(0xfafafa).into(),
            secondary: rgb(0xf5f5f5).into(),
            secondary_foreground: rgb(0x171717).into(),
            muted: rgb(0xf5f5f5).into(),
            muted_foreground: rgb(0x737373).into(),
            accent: rgb(0xf5f5f5).into(),
            accent_foreground: rgb(0x171717).into(),
            destructive: rgb(0xe7000b).into(),
            destructive_foreground: rgb(0xfcf3f3).into(),
            border: rgb(0xe5e5e5).into(),
            input: rgb(0xe5e5e5).into(),
            ring: rgb(0xa1a1a1).into(),
            radius: px(10.),
        }
    }

    /// shadcn `.dark`, neutral palette. Border and input are translucent
    /// white, exactly as in the source (`oklch(1 0 0 / 10%)` / `15%`).
    pub fn dark() -> Self {
        Self {
            dark: true,
            background: rgb(0x0a0a0a).into(),
            foreground: rgb(0xfafafa).into(),
            card: rgb(0x171717).into(),
            card_foreground: rgb(0xfafafa).into(),
            popover: rgb(0x171717).into(),
            popover_foreground: rgb(0xfafafa).into(),
            primary: rgb(0xe5e5e5).into(),
            primary_foreground: rgb(0x171717).into(),
            secondary: rgb(0x262626).into(),
            secondary_foreground: rgb(0xfafafa).into(),
            muted: rgb(0x262626).into(),
            muted_foreground: rgb(0xa1a1a1).into(),
            accent: rgb(0x404040).into(),
            accent_foreground: rgb(0xfafafa).into(),
            destructive: rgb(0xff6467).into(),
            destructive_foreground: rgb(0xdf2225).into(),
            border: rgba(0xffffff1a).into(),
            input: rgba(0xffffff26).into(),
            ring: rgb(0x737373).into(),
            radius: px(10.),
        }
    }
}

/// shadcn's `color/NN` opacity modifier: the token color at the given alpha.
pub fn alpha(mut color: Hsla, a: f32) -> Hsla {
    color.a = a;
    color
}
