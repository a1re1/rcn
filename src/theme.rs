//! shadcn's design-token system, ported from the base-vega style's CSS
//! variables (ui.shadcn.com, neutral palette). Every color token from
//! `:root` / `.dark` in shadcn's globals.css maps to a field here; values are
//! sRGB conversions of the original oklch values.
//!
//! The active theme lives in the gpui `Global` store: read it in a component
//! with `Theme::of(cx)`, swap it with `cx.set_global(Theme::dark())`.

use gpui::{App, Global, Hsla, Pixels, Rgba, px, rgb, rgba};

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

/// Convert an oklch color (shadcn's native token space; hue in degrees) to a
/// gpui color, clamped into sRGB.
pub fn oklch(l: f32, c: f32, h_deg: f32) -> Hsla {
    let h = h_deg.to_radians();
    let (a, b) = (c * h.cos(), c * h.sin());
    let l_ = l + 0.3963377774 * a + 0.2158037573 * b;
    let m_ = l - 0.1055613458 * a - 0.0638541728 * b;
    let s_ = l - 0.0894841775 * a - 1.2914855480 * b;
    let (l3, m3, s3) = (l_.powi(3), m_.powi(3), s_.powi(3));
    let r = 4.0767416621 * l3 - 3.3077115913 * m3 + 0.2309699292 * s3;
    let g = -1.2684380046 * l3 + 2.6097574011 * m3 - 0.3413193965 * s3;
    let b = -0.0041960863 * l3 - 0.7034186147 * m3 + 1.7076147010 * s3;
    let enc = |x: f32| {
        let x = x.clamp(0., 1.);
        if x <= 0.0031308 {
            12.92 * x
        } else {
            1.055 * x.powf(1. / 2.4) - 0.055
        }
    };
    Rgba {
        r: enc(r),
        g: enc(g),
        b: enc(b),
        a: 1.,
    }
    .into()
}

/// The gray family behind the neutral tokens — shadcn's "base color" choice.
/// Each tints the whole background/border/muted ramp toward its hue.
#[derive(Clone, Copy, PartialEq, Eq, Debug, Default)]
pub enum BaseColor {
    #[default]
    Neutral,
    Stone,
    Zinc,
    Gray,
    Slate,
}

impl BaseColor {
    pub const ALL: [BaseColor; 5] = [
        BaseColor::Neutral,
        BaseColor::Stone,
        BaseColor::Zinc,
        BaseColor::Gray,
        BaseColor::Slate,
    ];

    pub fn label(self) -> &'static str {
        match self {
            BaseColor::Neutral => "neutral",
            BaseColor::Stone => "stone",
            BaseColor::Zinc => "zinc",
            BaseColor::Gray => "gray",
            BaseColor::Slate => "slate",
        }
    }

    /// The family's (chroma, hue) tint, approximating the tailwind ramps
    /// (e.g. slate-500 ≈ oklch(0.554 0.041 257)).
    fn tint(self) -> (f32, f32) {
        match self {
            BaseColor::Neutral => (0., 0.),
            BaseColor::Stone => (0.005, 58.),
            BaseColor::Zinc => (0.010, 286.),
            BaseColor::Gray => (0.020, 264.),
            BaseColor::Slate => (0.038, 257.),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn assert_close(color: Hsla, expected: Hsla) {
        let (a, b): (Rgba, Rgba) = (color.into(), expected.into());
        for (x, y) in [(a.r, b.r), (a.g, b.g), (a.b, b.b)] {
            assert!((x - y).abs() < 0.01, "{color:?} != {expected:?}");
        }
    }

    /// The oklch conversion must reproduce the hand-converted token values
    /// the themes are built from.
    #[test]
    fn oklch_matches_shadcn_tokens() {
        assert_close(oklch(0.577, 0.245, 27.325), rgb(0xe7000b).into()); // destructive
        assert_close(oklch(0.97, 0., 0.), rgb(0xf5f5f5).into()); // secondary
        assert_close(oklch(0.145, 0., 0.), rgb(0x0a0a0a).into()); // dark background
        assert_close(oklch(1., 0., 0.), rgb(0xffffff).into());
    }

    #[test]
    fn neutral_base_is_the_stock_theme() {
        for dark in [false, true] {
            let stock = if dark { Theme::dark() } else { Theme::light() };
            let based = Theme::with_base(BaseColor::Neutral, dark);
            assert_eq!(based.background, stock.background);
            assert_eq!(based.primary, stock.primary);
            assert_eq!(based.dark, stock.dark);
        }
    }

    /// Tinted bases keep the neutral lightness structure: light backgrounds
    /// stay white, dark backgrounds stay near-black.
    #[test]
    fn tinted_bases_preserve_lightness() {
        for base in BaseColor::ALL {
            let light = Theme::with_base(base, false);
            assert!(light.background.l > 0.95, "{base:?} light background");
            assert!(light.foreground.l < 0.25, "{base:?} light foreground");
            let dark = Theme::with_base(base, true);
            assert!(dark.background.l < 0.25, "{base:?} dark background");
            assert!(dark.foreground.l > 0.9, "{base:?} dark foreground");
        }
    }
}

impl Theme {
    /// The neutral theme re-tinted toward a base gray family, like choosing a
    /// base color in shadcn's init/create flow. `BaseColor::Neutral` returns
    /// [`Theme::light`]/[`Theme::dark`] exactly.
    pub fn with_base(base: BaseColor, dark: bool) -> Self {
        let neutral = if dark { Self::dark() } else { Self::light() };
        if base == BaseColor::Neutral {
            return neutral;
        }
        let (chroma, hue) = base.tint();
        // Tailwind's gray ramps hold chroma roughly constant except near
        // white, where it fades out.
        let shade = |l: f32| oklch(l, chroma * ((1. - l) * 8.).clamp(0., 1.), hue);
        if dark {
            Self {
                background: shade(0.141),
                foreground: shade(0.985),
                card: shade(0.205),
                card_foreground: shade(0.985),
                popover: shade(0.205),
                popover_foreground: shade(0.985),
                primary: shade(0.922),
                primary_foreground: shade(0.205),
                secondary: shade(0.269),
                secondary_foreground: shade(0.985),
                muted: shade(0.269),
                muted_foreground: shade(0.708),
                accent: shade(0.371),
                accent_foreground: shade(0.985),
                ring: shade(0.556),
                ..neutral
            }
        } else {
            Self {
                background: shade(1.),
                foreground: shade(0.141),
                card: shade(1.),
                card_foreground: shade(0.141),
                popover: shade(1.),
                popover_foreground: shade(0.141),
                primary: shade(0.205),
                primary_foreground: shade(0.985),
                secondary: shade(0.97),
                secondary_foreground: shade(0.205),
                muted: shade(0.97),
                muted_foreground: shade(0.556),
                accent: shade(0.97),
                accent_foreground: shade(0.205),
                destructive: neutral.destructive,
                destructive_foreground: neutral.destructive_foreground,
                border: shade(0.922),
                input: shade(0.922),
                ring: shade(0.708),
                ..neutral
            }
        }
    }
}
