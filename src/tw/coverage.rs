//! tw coverage ledger — every Tailwind v4 utility root, classified.
//!
//! Scaffolded by scripts (see scripts/pull-tailwind.py for the manifest
//! side); curated by hand from here on. The enforcement tests at the
//! bottom keep this ledger honest in both directions:
//! - the ledger must cover exactly the roots in manifest.json,
//! - every `Supported` sample must parse cleanly (no unknown/skipped),
//! - every `Todo` sample must still be unknown (implementing a utility
//!   forces flipping its status),
//! - every `NoEquivalent` sample must be reported as skipped.
//!
//! Coverage % (printed by the `coverage_floor` test) counts
//! Supported / (Supported + Todo); NoEquivalent utilities are tracked
//! but excluded from the denominator by agreement.

/// How the tw parser relates to one Tailwind utility root.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(super) enum Status {
    /// A handler maps this root onto gpui styles.
    Supported,
    /// Mappable onto gpui Style, but no handler yet.
    Todo,
    /// No gpui equivalent — the parser reports these as skipped.
    NoEquivalent(&'static str),
}

#[allow(dead_code)] // `sample` is read by the enforcement tests
pub(super) struct Entry {
    pub name: &'static str,
    pub status: Status,
    /// Representative class the enforcement tests parse.
    pub sample: &'static str,
}

pub(super) const LEDGER: &[Entry] = &[
    Entry {
        name: "@container",
        status: Status::NoEquivalent(
            "container queries are component-level (crate::container_query)",
        ),
        sample: "@container-2",
    },
    Entry {
        name: "absolute",
        status: Status::Supported,
        sample: "absolute",
    },
    Entry {
        name: "accent",
        status: Status::NoEquivalent("input internals are component-level in rcn"),
        sample: "accent-2",
    },
    Entry {
        name: "accent-auto",
        status: Status::NoEquivalent("CSS default/reset; omit the class instead"),
        sample: "accent-auto",
    },
    Entry {
        name: "align",
        status: Status::NoEquivalent("no inline vertical-align in gpui"),
        sample: "align-2",
    },
    Entry {
        name: "align-baseline",
        status: Status::NoEquivalent("no inline vertical-align in gpui"),
        sample: "align-baseline",
    },
    Entry {
        name: "align-bottom",
        status: Status::NoEquivalent("no inline vertical-align in gpui"),
        sample: "align-bottom",
    },
    Entry {
        name: "align-middle",
        status: Status::NoEquivalent("no inline vertical-align in gpui"),
        sample: "align-middle",
    },
    Entry {
        name: "align-sub",
        status: Status::NoEquivalent("no inline vertical-align in gpui"),
        sample: "align-sub",
    },
    Entry {
        name: "align-super",
        status: Status::NoEquivalent("no inline vertical-align in gpui"),
        sample: "align-super",
    },
    Entry {
        name: "align-text-bottom",
        status: Status::NoEquivalent("no inline vertical-align in gpui"),
        sample: "align-text-bottom",
    },
    Entry {
        name: "align-text-top",
        status: Status::NoEquivalent("no inline vertical-align in gpui"),
        sample: "align-text-top",
    },
    Entry {
        name: "align-top",
        status: Status::NoEquivalent("no inline vertical-align in gpui"),
        sample: "align-top",
    },
    Entry {
        name: "animate",
        status: Status::NoEquivalent("no style transitions; motion timings live in crate::motion"),
        sample: "animate-2",
    },
    Entry {
        name: "animate-none",
        status: Status::NoEquivalent("no style transitions; motion timings live in crate::motion"),
        sample: "animate-none",
    },
    Entry {
        name: "antialiased",
        status: Status::NoEquivalent("text rasterization is platform-level"),
        sample: "antialiased",
    },
    Entry {
        name: "appearance-auto",
        status: Status::NoEquivalent("no native-widget appearance in gpui"),
        sample: "appearance-auto",
    },
    Entry {
        name: "appearance-none",
        status: Status::NoEquivalent("no native-widget appearance in gpui"),
        sample: "appearance-none",
    },
    Entry {
        name: "aspect",
        status: Status::Supported,
        sample: "aspect-video",
    },
    Entry {
        name: "aspect-auto",
        status: Status::NoEquivalent("CSS default/reset; omit the class instead"),
        sample: "aspect-auto",
    },
    Entry {
        name: "aspect-square",
        status: Status::Supported,
        sample: "aspect-square",
    },
    Entry {
        name: "auto-cols",
        status: Status::NoEquivalent("grid auto tracks not exposed by gpui Style"),
        sample: "auto-cols-2",
    },
    Entry {
        name: "auto-cols-auto",
        status: Status::NoEquivalent("grid auto tracks not exposed by gpui Style"),
        sample: "auto-cols-auto",
    },
    Entry {
        name: "auto-cols-fr",
        status: Status::NoEquivalent("grid auto tracks not exposed by gpui Style"),
        sample: "auto-cols-fr",
    },
    Entry {
        name: "auto-cols-max",
        status: Status::NoEquivalent("grid auto tracks not exposed by gpui Style"),
        sample: "auto-cols-max",
    },
    Entry {
        name: "auto-cols-min",
        status: Status::NoEquivalent("grid auto tracks not exposed by gpui Style"),
        sample: "auto-cols-min",
    },
    Entry {
        name: "auto-rows",
        status: Status::NoEquivalent("grid auto tracks not exposed by gpui Style"),
        sample: "auto-rows-2",
    },
    Entry {
        name: "auto-rows-auto",
        status: Status::NoEquivalent("grid auto tracks not exposed by gpui Style"),
        sample: "auto-rows-auto",
    },
    Entry {
        name: "auto-rows-fr",
        status: Status::NoEquivalent("grid auto tracks not exposed by gpui Style"),
        sample: "auto-rows-fr",
    },
    Entry {
        name: "auto-rows-max",
        status: Status::NoEquivalent("grid auto tracks not exposed by gpui Style"),
        sample: "auto-rows-max",
    },
    Entry {
        name: "auto-rows-min",
        status: Status::NoEquivalent("grid auto tracks not exposed by gpui Style"),
        sample: "auto-rows-min",
    },
    Entry {
        name: "backdrop-blur",
        status: Status::NoEquivalent("no filter pipeline in gpui"),
        sample: "backdrop-blur-2",
    },
    Entry {
        name: "backdrop-blur-none",
        status: Status::NoEquivalent("no filter pipeline in gpui"),
        sample: "backdrop-blur-none",
    },
    Entry {
        name: "backdrop-brightness",
        status: Status::NoEquivalent("no filter pipeline in gpui"),
        sample: "backdrop-brightness-2",
    },
    Entry {
        name: "backdrop-contrast",
        status: Status::NoEquivalent("no filter pipeline in gpui"),
        sample: "backdrop-contrast-2",
    },
    Entry {
        name: "backdrop-filter",
        status: Status::NoEquivalent("no filter pipeline in gpui"),
        sample: "backdrop-filter-2",
    },
    Entry {
        name: "backdrop-grayscale",
        status: Status::NoEquivalent("no filter pipeline in gpui"),
        sample: "backdrop-grayscale-2",
    },
    Entry {
        name: "backdrop-hue-rotate",
        status: Status::NoEquivalent("no filter pipeline in gpui"),
        sample: "backdrop-hue-rotate-2",
    },
    Entry {
        name: "backdrop-invert",
        status: Status::NoEquivalent("no filter pipeline in gpui"),
        sample: "backdrop-invert-2",
    },
    Entry {
        name: "backdrop-opacity",
        status: Status::NoEquivalent("no filter pipeline in gpui"),
        sample: "backdrop-opacity-2",
    },
    Entry {
        name: "backdrop-saturate",
        status: Status::NoEquivalent("no filter pipeline in gpui"),
        sample: "backdrop-saturate-2",
    },
    Entry {
        name: "backdrop-sepia",
        status: Status::NoEquivalent("no filter pipeline in gpui"),
        sample: "backdrop-sepia-2",
    },
    Entry {
        name: "backface-hidden",
        status: Status::NoEquivalent("no transforms in gpui styles"),
        sample: "backface-hidden",
    },
    Entry {
        name: "backface-visible",
        status: Status::NoEquivalent("no transforms in gpui styles"),
        sample: "backface-visible",
    },
    Entry {
        name: "basis",
        status: Status::Supported,
        sample: "basis-4",
    },
    Entry {
        name: "basis-auto",
        status: Status::Supported,
        sample: "basis-auto",
    },
    Entry {
        name: "basis-full",
        status: Status::Supported,
        sample: "basis-full",
    },
    Entry {
        name: "bg",
        status: Status::Supported,
        sample: "bg-red-500",
    },
    Entry {
        name: "bg-auto",
        status: Status::NoEquivalent("background images/attachment/blending unsupported in gpui"),
        sample: "bg-auto",
    },
    Entry {
        name: "bg-blend-color",
        status: Status::NoEquivalent("background images/attachment/blending unsupported in gpui"),
        sample: "bg-blend-color",
    },
    Entry {
        name: "bg-blend-color-burn",
        status: Status::NoEquivalent("background images/attachment/blending unsupported in gpui"),
        sample: "bg-blend-color-burn",
    },
    Entry {
        name: "bg-blend-color-dodge",
        status: Status::NoEquivalent("background images/attachment/blending unsupported in gpui"),
        sample: "bg-blend-color-dodge",
    },
    Entry {
        name: "bg-blend-darken",
        status: Status::NoEquivalent("background images/attachment/blending unsupported in gpui"),
        sample: "bg-blend-darken",
    },
    Entry {
        name: "bg-blend-difference",
        status: Status::NoEquivalent("background images/attachment/blending unsupported in gpui"),
        sample: "bg-blend-difference",
    },
    Entry {
        name: "bg-blend-exclusion",
        status: Status::NoEquivalent("background images/attachment/blending unsupported in gpui"),
        sample: "bg-blend-exclusion",
    },
    Entry {
        name: "bg-blend-hard-light",
        status: Status::NoEquivalent("background images/attachment/blending unsupported in gpui"),
        sample: "bg-blend-hard-light",
    },
    Entry {
        name: "bg-blend-hue",
        status: Status::NoEquivalent("background images/attachment/blending unsupported in gpui"),
        sample: "bg-blend-hue",
    },
    Entry {
        name: "bg-blend-lighten",
        status: Status::NoEquivalent("background images/attachment/blending unsupported in gpui"),
        sample: "bg-blend-lighten",
    },
    Entry {
        name: "bg-blend-luminosity",
        status: Status::NoEquivalent("background images/attachment/blending unsupported in gpui"),
        sample: "bg-blend-luminosity",
    },
    Entry {
        name: "bg-blend-multiply",
        status: Status::NoEquivalent("background images/attachment/blending unsupported in gpui"),
        sample: "bg-blend-multiply",
    },
    Entry {
        name: "bg-blend-normal",
        status: Status::NoEquivalent("background images/attachment/blending unsupported in gpui"),
        sample: "bg-blend-normal",
    },
    Entry {
        name: "bg-blend-overlay",
        status: Status::NoEquivalent("background images/attachment/blending unsupported in gpui"),
        sample: "bg-blend-overlay",
    },
    Entry {
        name: "bg-blend-saturation",
        status: Status::NoEquivalent("background images/attachment/blending unsupported in gpui"),
        sample: "bg-blend-saturation",
    },
    Entry {
        name: "bg-blend-screen",
        status: Status::NoEquivalent("background images/attachment/blending unsupported in gpui"),
        sample: "bg-blend-screen",
    },
    Entry {
        name: "bg-blend-soft-light",
        status: Status::NoEquivalent("background images/attachment/blending unsupported in gpui"),
        sample: "bg-blend-soft-light",
    },
    Entry {
        name: "bg-bottom",
        status: Status::NoEquivalent("background images/attachment/blending unsupported in gpui"),
        sample: "bg-bottom",
    },
    Entry {
        name: "bg-bottom-left",
        status: Status::NoEquivalent("background images/attachment/blending unsupported in gpui"),
        sample: "bg-bottom-left",
    },
    Entry {
        name: "bg-bottom-right",
        status: Status::NoEquivalent("background images/attachment/blending unsupported in gpui"),
        sample: "bg-bottom-right",
    },
    Entry {
        name: "bg-center",
        status: Status::NoEquivalent("background images/attachment/blending unsupported in gpui"),
        sample: "bg-center",
    },
    Entry {
        name: "bg-clip-border",
        status: Status::NoEquivalent("background images/attachment/blending unsupported in gpui"),
        sample: "bg-clip-border",
    },
    Entry {
        name: "bg-clip-content",
        status: Status::NoEquivalent("background images/attachment/blending unsupported in gpui"),
        sample: "bg-clip-content",
    },
    Entry {
        name: "bg-clip-padding",
        status: Status::NoEquivalent("background images/attachment/blending unsupported in gpui"),
        sample: "bg-clip-padding",
    },
    Entry {
        name: "bg-clip-text",
        status: Status::NoEquivalent("background images/attachment/blending unsupported in gpui"),
        sample: "bg-clip-text",
    },
    Entry {
        name: "bg-conic",
        status: Status::NoEquivalent("background images/attachment/blending unsupported in gpui"),
        sample: "bg-conic-2",
    },
    Entry {
        name: "bg-contain",
        status: Status::NoEquivalent("background images/attachment/blending unsupported in gpui"),
        sample: "bg-contain",
    },
    Entry {
        name: "bg-cover",
        status: Status::NoEquivalent("background images/attachment/blending unsupported in gpui"),
        sample: "bg-cover",
    },
    Entry {
        name: "bg-fixed",
        status: Status::NoEquivalent("background images/attachment/blending unsupported in gpui"),
        sample: "bg-fixed",
    },
    Entry {
        name: "bg-left",
        status: Status::NoEquivalent("background images/attachment/blending unsupported in gpui"),
        sample: "bg-left",
    },
    Entry {
        name: "bg-linear",
        status: Status::Supported,
        sample: "bg-linear-to-r",
    },
    Entry {
        name: "bg-local",
        status: Status::NoEquivalent("background images/attachment/blending unsupported in gpui"),
        sample: "bg-local",
    },
    Entry {
        name: "bg-no-repeat",
        status: Status::NoEquivalent("background images/attachment/blending unsupported in gpui"),
        sample: "bg-no-repeat",
    },
    Entry {
        name: "bg-none",
        status: Status::NoEquivalent("background images/attachment/blending unsupported in gpui"),
        sample: "bg-none",
    },
    Entry {
        name: "bg-origin-border",
        status: Status::NoEquivalent("background images/attachment/blending unsupported in gpui"),
        sample: "bg-origin-border",
    },
    Entry {
        name: "bg-origin-content",
        status: Status::NoEquivalent("background images/attachment/blending unsupported in gpui"),
        sample: "bg-origin-content",
    },
    Entry {
        name: "bg-origin-padding",
        status: Status::NoEquivalent("background images/attachment/blending unsupported in gpui"),
        sample: "bg-origin-padding",
    },
    Entry {
        name: "bg-position",
        status: Status::NoEquivalent("background images/attachment/blending unsupported in gpui"),
        sample: "bg-position-2",
    },
    Entry {
        name: "bg-radial",
        status: Status::NoEquivalent("background images/attachment/blending unsupported in gpui"),
        sample: "bg-radial-2",
    },
    Entry {
        name: "bg-repeat",
        status: Status::NoEquivalent("background images/attachment/blending unsupported in gpui"),
        sample: "bg-repeat",
    },
    Entry {
        name: "bg-repeat-round",
        status: Status::NoEquivalent("background images/attachment/blending unsupported in gpui"),
        sample: "bg-repeat-round",
    },
    Entry {
        name: "bg-repeat-space",
        status: Status::NoEquivalent("background images/attachment/blending unsupported in gpui"),
        sample: "bg-repeat-space",
    },
    Entry {
        name: "bg-repeat-x",
        status: Status::NoEquivalent("background images/attachment/blending unsupported in gpui"),
        sample: "bg-repeat-x",
    },
    Entry {
        name: "bg-repeat-y",
        status: Status::NoEquivalent("background images/attachment/blending unsupported in gpui"),
        sample: "bg-repeat-y",
    },
    Entry {
        name: "bg-right",
        status: Status::NoEquivalent("background images/attachment/blending unsupported in gpui"),
        sample: "bg-right",
    },
    Entry {
        name: "bg-scroll",
        status: Status::NoEquivalent("background images/attachment/blending unsupported in gpui"),
        sample: "bg-scroll",
    },
    Entry {
        name: "bg-size",
        status: Status::NoEquivalent("background images/attachment/blending unsupported in gpui"),
        sample: "bg-size-2",
    },
    Entry {
        name: "bg-top",
        status: Status::NoEquivalent("background images/attachment/blending unsupported in gpui"),
        sample: "bg-top",
    },
    Entry {
        name: "bg-top-left",
        status: Status::NoEquivalent("background images/attachment/blending unsupported in gpui"),
        sample: "bg-top-left",
    },
    Entry {
        name: "bg-top-right",
        status: Status::NoEquivalent("background images/attachment/blending unsupported in gpui"),
        sample: "bg-top-right",
    },
    Entry {
        name: "block",
        status: Status::Supported,
        sample: "block",
    },
    Entry {
        name: "blur",
        status: Status::NoEquivalent("no filter pipeline in gpui"),
        sample: "blur-2",
    },
    Entry {
        name: "blur-none",
        status: Status::NoEquivalent("no filter pipeline in gpui"),
        sample: "blur-none",
    },
    Entry {
        name: "border",
        status: Status::Supported,
        sample: "border-2",
    },
    Entry {
        name: "border-b",
        status: Status::Supported,
        sample: "border-b-2",
    },
    Entry {
        name: "border-collapse",
        status: Status::NoEquivalent("no table layout in gpui"),
        sample: "border-collapse",
    },
    Entry {
        name: "border-dashed",
        status: Status::Supported,
        sample: "border-dashed",
    },
    Entry {
        name: "border-dotted",
        status: Status::NoEquivalent("gpui BorderStyle is solid/dashed"),
        sample: "border-dotted",
    },
    Entry {
        name: "border-double",
        status: Status::NoEquivalent("gpui BorderStyle is solid/dashed"),
        sample: "border-double",
    },
    Entry {
        name: "border-e",
        status: Status::Supported,
        sample: "border-e-2",
    },
    Entry {
        name: "border-hidden",
        status: Status::NoEquivalent("CSS default/reset; omit the class instead"),
        sample: "border-hidden",
    },
    Entry {
        name: "border-l",
        status: Status::Supported,
        sample: "border-l-2",
    },
    Entry {
        name: "border-none",
        status: Status::Supported,
        sample: "border-none",
    },
    Entry {
        name: "border-r",
        status: Status::Supported,
        sample: "border-r-2",
    },
    Entry {
        name: "border-s",
        status: Status::Supported,
        sample: "border-s-2",
    },
    Entry {
        name: "border-separate",
        status: Status::NoEquivalent("no table layout in gpui"),
        sample: "border-separate",
    },
    Entry {
        name: "border-solid",
        status: Status::Supported,
        sample: "border-solid",
    },
    Entry {
        name: "border-spacing",
        status: Status::NoEquivalent("no table layout in gpui"),
        sample: "border-spacing-2",
    },
    Entry {
        name: "border-spacing-x",
        status: Status::NoEquivalent("no table layout in gpui"),
        sample: "border-spacing-x-2",
    },
    Entry {
        name: "border-spacing-y",
        status: Status::NoEquivalent("no table layout in gpui"),
        sample: "border-spacing-y-2",
    },
    Entry {
        name: "border-t",
        status: Status::Supported,
        sample: "border-t-2",
    },
    Entry {
        name: "border-x",
        status: Status::Supported,
        sample: "border-x-2",
    },
    Entry {
        name: "border-y",
        status: Status::Supported,
        sample: "border-y-2",
    },
    Entry {
        name: "bottom",
        status: Status::Supported,
        sample: "bottom-2",
    },
    Entry {
        name: "bottom-auto",
        status: Status::Supported,
        sample: "bottom-auto",
    },
    Entry {
        name: "bottom-full",
        status: Status::Supported,
        sample: "bottom-full",
    },
    Entry {
        name: "box-border",
        status: Status::NoEquivalent("gpui is border-box only"),
        sample: "box-border",
    },
    Entry {
        name: "box-content",
        status: Status::NoEquivalent("gpui is border-box only"),
        sample: "box-content",
    },
    Entry {
        name: "box-decoration-clone",
        status: Status::NoEquivalent("no fragmentation in gpui"),
        sample: "box-decoration-clone",
    },
    Entry {
        name: "box-decoration-slice",
        status: Status::NoEquivalent("no fragmentation in gpui"),
        sample: "box-decoration-slice",
    },
    Entry {
        name: "break-after-all",
        status: Status::NoEquivalent("no fragmentation or word-break control in gpui"),
        sample: "break-after-all",
    },
    Entry {
        name: "break-after-auto",
        status: Status::NoEquivalent("no fragmentation or word-break control in gpui"),
        sample: "break-after-auto",
    },
    Entry {
        name: "break-after-avoid",
        status: Status::NoEquivalent("no fragmentation or word-break control in gpui"),
        sample: "break-after-avoid",
    },
    Entry {
        name: "break-after-avoid-page",
        status: Status::NoEquivalent("no fragmentation or word-break control in gpui"),
        sample: "break-after-avoid-page",
    },
    Entry {
        name: "break-after-column",
        status: Status::NoEquivalent("no fragmentation or word-break control in gpui"),
        sample: "break-after-column",
    },
    Entry {
        name: "break-after-left",
        status: Status::NoEquivalent("no fragmentation or word-break control in gpui"),
        sample: "break-after-left",
    },
    Entry {
        name: "break-after-page",
        status: Status::NoEquivalent("no fragmentation or word-break control in gpui"),
        sample: "break-after-page",
    },
    Entry {
        name: "break-after-right",
        status: Status::NoEquivalent("no fragmentation or word-break control in gpui"),
        sample: "break-after-right",
    },
    Entry {
        name: "break-all",
        status: Status::NoEquivalent("no fragmentation or word-break control in gpui"),
        sample: "break-all",
    },
    Entry {
        name: "break-before-all",
        status: Status::NoEquivalent("no fragmentation or word-break control in gpui"),
        sample: "break-before-all",
    },
    Entry {
        name: "break-before-auto",
        status: Status::NoEquivalent("no fragmentation or word-break control in gpui"),
        sample: "break-before-auto",
    },
    Entry {
        name: "break-before-avoid",
        status: Status::NoEquivalent("no fragmentation or word-break control in gpui"),
        sample: "break-before-avoid",
    },
    Entry {
        name: "break-before-avoid-page",
        status: Status::NoEquivalent("no fragmentation or word-break control in gpui"),
        sample: "break-before-avoid-page",
    },
    Entry {
        name: "break-before-column",
        status: Status::NoEquivalent("no fragmentation or word-break control in gpui"),
        sample: "break-before-column",
    },
    Entry {
        name: "break-before-left",
        status: Status::NoEquivalent("no fragmentation or word-break control in gpui"),
        sample: "break-before-left",
    },
    Entry {
        name: "break-before-page",
        status: Status::NoEquivalent("no fragmentation or word-break control in gpui"),
        sample: "break-before-page",
    },
    Entry {
        name: "break-before-right",
        status: Status::NoEquivalent("no fragmentation or word-break control in gpui"),
        sample: "break-before-right",
    },
    Entry {
        name: "break-inside-auto",
        status: Status::NoEquivalent("no fragmentation or word-break control in gpui"),
        sample: "break-inside-auto",
    },
    Entry {
        name: "break-inside-avoid",
        status: Status::NoEquivalent("no fragmentation or word-break control in gpui"),
        sample: "break-inside-avoid",
    },
    Entry {
        name: "break-inside-avoid-column",
        status: Status::NoEquivalent("no fragmentation or word-break control in gpui"),
        sample: "break-inside-avoid-column",
    },
    Entry {
        name: "break-inside-avoid-page",
        status: Status::NoEquivalent("no fragmentation or word-break control in gpui"),
        sample: "break-inside-avoid-page",
    },
    Entry {
        name: "break-keep",
        status: Status::NoEquivalent("no fragmentation or word-break control in gpui"),
        sample: "break-keep",
    },
    Entry {
        name: "break-normal",
        status: Status::NoEquivalent("no fragmentation or word-break control in gpui"),
        sample: "break-normal",
    },
    Entry {
        name: "break-words",
        status: Status::NoEquivalent("no fragmentation or word-break control in gpui"),
        sample: "break-words",
    },
    Entry {
        name: "brightness",
        status: Status::NoEquivalent("no filter pipeline in gpui"),
        sample: "brightness-2",
    },
    Entry {
        name: "capitalize",
        status: Status::NoEquivalent("no text-transform in gpui"),
        sample: "capitalize",
    },
    Entry {
        name: "caption-bottom",
        status: Status::NoEquivalent("no table layout in gpui"),
        sample: "caption-bottom",
    },
    Entry {
        name: "caption-top",
        status: Status::NoEquivalent("no table layout in gpui"),
        sample: "caption-top",
    },
    Entry {
        name: "caret",
        status: Status::NoEquivalent("input internals are component-level in rcn"),
        sample: "caret-2",
    },
    Entry {
        name: "clear-both",
        status: Status::NoEquivalent("no float layout in gpui"),
        sample: "clear-both",
    },
    Entry {
        name: "clear-end",
        status: Status::NoEquivalent("no float layout in gpui"),
        sample: "clear-end",
    },
    Entry {
        name: "clear-left",
        status: Status::NoEquivalent("no float layout in gpui"),
        sample: "clear-left",
    },
    Entry {
        name: "clear-none",
        status: Status::NoEquivalent("no float layout in gpui"),
        sample: "clear-none",
    },
    Entry {
        name: "clear-right",
        status: Status::NoEquivalent("no float layout in gpui"),
        sample: "clear-right",
    },
    Entry {
        name: "clear-start",
        status: Status::NoEquivalent("no float layout in gpui"),
        sample: "clear-start",
    },
    Entry {
        name: "col",
        status: Status::Todo,
        sample: "col-2",
    },
    Entry {
        name: "col-auto",
        status: Status::Todo,
        sample: "col-auto",
    },
    Entry {
        name: "col-end",
        status: Status::Todo,
        sample: "col-end-1",
    },
    Entry {
        name: "col-end-auto",
        status: Status::Todo,
        sample: "col-end-auto",
    },
    Entry {
        name: "col-span",
        status: Status::Todo,
        sample: "col-span-2",
    },
    Entry {
        name: "col-span-full",
        status: Status::Todo,
        sample: "col-span-full",
    },
    Entry {
        name: "col-start",
        status: Status::Todo,
        sample: "col-start-1",
    },
    Entry {
        name: "col-start-auto",
        status: Status::Todo,
        sample: "col-start-auto",
    },
    Entry {
        name: "collapse",
        status: Status::NoEquivalent("no table layout in gpui"),
        sample: "collapse",
    },
    Entry {
        name: "columns",
        status: Status::NoEquivalent("no multi-column layout in gpui"),
        sample: "columns-2",
    },
    Entry {
        name: "columns-auto",
        status: Status::NoEquivalent("no multi-column layout in gpui"),
        sample: "columns-auto",
    },
    Entry {
        name: "contain",
        status: Status::NoEquivalent("CSS containment; nothing to map"),
        sample: "contain-2",
    },
    Entry {
        name: "contain-content",
        status: Status::NoEquivalent("CSS containment; nothing to map"),
        sample: "contain-content",
    },
    Entry {
        name: "contain-inline-size",
        status: Status::NoEquivalent("CSS containment; nothing to map"),
        sample: "contain-inline-size",
    },
    Entry {
        name: "contain-layout",
        status: Status::NoEquivalent("CSS containment; nothing to map"),
        sample: "contain-layout",
    },
    Entry {
        name: "contain-none",
        status: Status::NoEquivalent("CSS containment; nothing to map"),
        sample: "contain-none",
    },
    Entry {
        name: "contain-paint",
        status: Status::NoEquivalent("CSS containment; nothing to map"),
        sample: "contain-paint",
    },
    Entry {
        name: "contain-size",
        status: Status::NoEquivalent("CSS containment; nothing to map"),
        sample: "contain-size",
    },
    Entry {
        name: "contain-strict",
        status: Status::NoEquivalent("CSS containment; nothing to map"),
        sample: "contain-strict",
    },
    Entry {
        name: "contain-style",
        status: Status::NoEquivalent("CSS containment; nothing to map"),
        sample: "contain-style",
    },
    Entry {
        name: "container",
        status: Status::NoEquivalent("CSS containment; nothing to map"),
        sample: "container",
    },
    Entry {
        name: "content",
        status: Status::NoEquivalent("pseudo-element content; no equivalent"),
        sample: "content-2",
    },
    Entry {
        name: "content-around",
        status: Status::Supported,
        sample: "content-around",
    },
    Entry {
        name: "content-baseline",
        status: Status::NoEquivalent("safe/baseline-last alignment not in taffy"),
        sample: "content-baseline",
    },
    Entry {
        name: "content-between",
        status: Status::Supported,
        sample: "content-between",
    },
    Entry {
        name: "content-center",
        status: Status::Supported,
        sample: "content-center",
    },
    Entry {
        name: "content-center-safe",
        status: Status::NoEquivalent("safe/baseline-last alignment not in taffy"),
        sample: "content-center-safe",
    },
    Entry {
        name: "content-end",
        status: Status::Supported,
        sample: "content-end",
    },
    Entry {
        name: "content-end-safe",
        status: Status::NoEquivalent("safe/baseline-last alignment not in taffy"),
        sample: "content-end-safe",
    },
    Entry {
        name: "content-evenly",
        status: Status::Supported,
        sample: "content-evenly",
    },
    Entry {
        name: "content-none",
        status: Status::NoEquivalent("CSS default/reset; omit the class instead"),
        sample: "content-none",
    },
    Entry {
        name: "content-normal",
        status: Status::NoEquivalent("CSS default/reset; omit the class instead"),
        sample: "content-normal",
    },
    Entry {
        name: "content-start",
        status: Status::Supported,
        sample: "content-start",
    },
    Entry {
        name: "content-stretch",
        status: Status::Supported,
        sample: "content-stretch",
    },
    Entry {
        name: "contents",
        status: Status::NoEquivalent("no inline formatting context in taffy"),
        sample: "contents",
    },
    Entry {
        name: "contrast",
        status: Status::NoEquivalent("no filter pipeline in gpui"),
        sample: "contrast-2",
    },
    Entry {
        name: "cursor",
        status: Status::Supported,
        sample: "cursor-pointer",
    },
    Entry {
        name: "decoration",
        status: Status::Supported,
        sample: "decoration-red-500",
    },
    Entry {
        name: "decoration-auto",
        status: Status::NoEquivalent("gpui underline is plain or wavy only"),
        sample: "decoration-auto",
    },
    Entry {
        name: "decoration-dashed",
        status: Status::NoEquivalent("gpui underline is plain or wavy only"),
        sample: "decoration-dashed",
    },
    Entry {
        name: "decoration-dotted",
        status: Status::NoEquivalent("gpui underline is plain or wavy only"),
        sample: "decoration-dotted",
    },
    Entry {
        name: "decoration-double",
        status: Status::NoEquivalent("gpui underline is plain or wavy only"),
        sample: "decoration-double",
    },
    Entry {
        name: "decoration-from-font",
        status: Status::NoEquivalent("gpui underline is plain or wavy only"),
        sample: "decoration-from-font",
    },
    Entry {
        name: "decoration-solid",
        status: Status::NoEquivalent("gpui underline is plain or wavy only"),
        sample: "decoration-solid",
    },
    Entry {
        name: "decoration-wavy",
        status: Status::Supported,
        sample: "decoration-wavy",
    },
    Entry {
        name: "delay",
        status: Status::NoEquivalent("no style transitions; motion timings live in crate::motion"),
        sample: "delay-2",
    },
    Entry {
        name: "diagonal-fractions",
        status: Status::Supported,
        sample: "diagonal-fractions",
    },
    Entry {
        name: "divide",
        status: Status::NoEquivalent(
            "child-combinator borders; use per-child borders or Separator",
        ),
        sample: "divide-2",
    },
    Entry {
        name: "divide-x",
        status: Status::NoEquivalent(
            "child-combinator borders; use per-child borders or Separator",
        ),
        sample: "divide-x-2",
    },
    Entry {
        name: "divide-x-reverse",
        status: Status::NoEquivalent(
            "child-combinator borders; use per-child borders or Separator",
        ),
        sample: "divide-x-reverse",
    },
    Entry {
        name: "divide-y",
        status: Status::NoEquivalent(
            "child-combinator borders; use per-child borders or Separator",
        ),
        sample: "divide-y-2",
    },
    Entry {
        name: "divide-y-reverse",
        status: Status::NoEquivalent(
            "child-combinator borders; use per-child borders or Separator",
        ),
        sample: "divide-y-reverse",
    },
    Entry {
        name: "drop-shadow",
        status: Status::NoEquivalent("no filter pipeline in gpui"),
        sample: "drop-shadow-2",
    },
    Entry {
        name: "drop-shadow-none",
        status: Status::NoEquivalent("no filter pipeline in gpui"),
        sample: "drop-shadow-none",
    },
    Entry {
        name: "duration",
        status: Status::NoEquivalent("no style transitions; motion timings live in crate::motion"),
        sample: "duration-2",
    },
    Entry {
        name: "duration-initial",
        status: Status::NoEquivalent("no style transitions; motion timings live in crate::motion"),
        sample: "duration-initial",
    },
    Entry {
        name: "ease",
        status: Status::NoEquivalent("no style transitions; motion timings live in crate::motion"),
        sample: "ease-2",
    },
    Entry {
        name: "ease-initial",
        status: Status::NoEquivalent("no style transitions; motion timings live in crate::motion"),
        sample: "ease-initial",
    },
    Entry {
        name: "ease-linear",
        status: Status::NoEquivalent("no style transitions; motion timings live in crate::motion"),
        sample: "ease-linear",
    },
    Entry {
        name: "end",
        status: Status::Supported,
        sample: "end-2",
    },
    Entry {
        name: "end-auto",
        status: Status::Supported,
        sample: "end-auto",
    },
    Entry {
        name: "end-full",
        status: Status::Supported,
        sample: "end-full",
    },
    Entry {
        name: "field-sizing-content",
        status: Status::NoEquivalent("input sizing is component-level in rcn"),
        sample: "field-sizing-content",
    },
    Entry {
        name: "field-sizing-fixed",
        status: Status::NoEquivalent("input sizing is component-level in rcn"),
        sample: "field-sizing-fixed",
    },
    Entry {
        name: "fill",
        status: Status::Supported,
        sample: "fill-red-500",
    },
    Entry {
        name: "fill-none",
        status: Status::NoEquivalent("svg stroke/fill resets are element-level in gpui"),
        sample: "fill-none",
    },
    Entry {
        name: "filter",
        status: Status::NoEquivalent("no filter pipeline in gpui"),
        sample: "filter-2",
    },
    Entry {
        name: "fixed",
        status: Status::NoEquivalent("gpui positions are relative/absolute only"),
        sample: "fixed",
    },
    Entry {
        name: "flex",
        status: Status::Supported,
        sample: "flex",
    },
    Entry {
        name: "flex-auto",
        status: Status::Supported,
        sample: "flex-auto",
    },
    Entry {
        name: "flex-col",
        status: Status::Supported,
        sample: "flex-col",
    },
    Entry {
        name: "flex-col-reverse",
        status: Status::Supported,
        sample: "flex-col-reverse",
    },
    Entry {
        name: "flex-initial",
        status: Status::Supported,
        sample: "flex-initial",
    },
    Entry {
        name: "flex-none",
        status: Status::Supported,
        sample: "flex-none",
    },
    Entry {
        name: "flex-nowrap",
        status: Status::Supported,
        sample: "flex-nowrap",
    },
    Entry {
        name: "flex-row",
        status: Status::Supported,
        sample: "flex-row",
    },
    Entry {
        name: "flex-row-reverse",
        status: Status::Supported,
        sample: "flex-row-reverse",
    },
    Entry {
        name: "flex-wrap",
        status: Status::Supported,
        sample: "flex-wrap",
    },
    Entry {
        name: "flex-wrap-reverse",
        status: Status::Supported,
        sample: "flex-wrap-reverse",
    },
    Entry {
        name: "float-end",
        status: Status::NoEquivalent("no float layout in gpui"),
        sample: "float-end",
    },
    Entry {
        name: "float-left",
        status: Status::NoEquivalent("no float layout in gpui"),
        sample: "float-left",
    },
    Entry {
        name: "float-none",
        status: Status::NoEquivalent("no float layout in gpui"),
        sample: "float-none",
    },
    Entry {
        name: "float-right",
        status: Status::NoEquivalent("no float layout in gpui"),
        sample: "float-right",
    },
    Entry {
        name: "float-start",
        status: Status::NoEquivalent("no float layout in gpui"),
        sample: "float-start",
    },
    Entry {
        name: "flow-root",
        status: Status::NoEquivalent("no inline formatting context in taffy"),
        sample: "flow-root",
    },
    Entry {
        name: "font",
        status: Status::Supported,
        sample: "font-medium",
    },
    Entry {
        name: "font-stretch",
        status: Status::NoEquivalent("no font-stretch in gpui"),
        sample: "font-stretch-2",
    },
    Entry {
        name: "font-stretch-condensed",
        status: Status::NoEquivalent("no font-stretch in gpui"),
        sample: "font-stretch-condensed",
    },
    Entry {
        name: "font-stretch-expanded",
        status: Status::NoEquivalent("no font-stretch in gpui"),
        sample: "font-stretch-expanded",
    },
    Entry {
        name: "font-stretch-extra-condensed",
        status: Status::NoEquivalent("no font-stretch in gpui"),
        sample: "font-stretch-extra-condensed",
    },
    Entry {
        name: "font-stretch-extra-expanded",
        status: Status::NoEquivalent("no font-stretch in gpui"),
        sample: "font-stretch-extra-expanded",
    },
    Entry {
        name: "font-stretch-normal",
        status: Status::NoEquivalent("no font-stretch in gpui"),
        sample: "font-stretch-normal",
    },
    Entry {
        name: "font-stretch-semi-condensed",
        status: Status::NoEquivalent("no font-stretch in gpui"),
        sample: "font-stretch-semi-condensed",
    },
    Entry {
        name: "font-stretch-semi-expanded",
        status: Status::NoEquivalent("no font-stretch in gpui"),
        sample: "font-stretch-semi-expanded",
    },
    Entry {
        name: "font-stretch-ultra-condensed",
        status: Status::NoEquivalent("no font-stretch in gpui"),
        sample: "font-stretch-ultra-condensed",
    },
    Entry {
        name: "font-stretch-ultra-expanded",
        status: Status::NoEquivalent("no font-stretch in gpui"),
        sample: "font-stretch-ultra-expanded",
    },
    Entry {
        name: "forced-color-adjust-auto",
        status: Status::NoEquivalent("no forced-colors mode in gpui"),
        sample: "forced-color-adjust-auto",
    },
    Entry {
        name: "forced-color-adjust-none",
        status: Status::NoEquivalent("no forced-colors mode in gpui"),
        sample: "forced-color-adjust-none",
    },
    Entry {
        name: "from",
        status: Status::Supported,
        sample: "from-red-500",
    },
    Entry {
        name: "gap",
        status: Status::Supported,
        sample: "gap-2",
    },
    Entry {
        name: "gap-x",
        status: Status::Supported,
        sample: "gap-x-2",
    },
    Entry {
        name: "gap-y",
        status: Status::Supported,
        sample: "gap-y-2",
    },
    Entry {
        name: "grayscale",
        status: Status::NoEquivalent("no filter pipeline in gpui"),
        sample: "grayscale-2",
    },
    Entry {
        name: "grid",
        status: Status::Supported,
        sample: "grid",
    },
    Entry {
        name: "grid-cols",
        status: Status::Todo,
        sample: "grid-cols-3",
    },
    Entry {
        name: "grid-cols-none",
        status: Status::NoEquivalent("subgrid/none resets not expressible in gpui"),
        sample: "grid-cols-none",
    },
    Entry {
        name: "grid-cols-subgrid",
        status: Status::NoEquivalent("subgrid/none resets not expressible in gpui"),
        sample: "grid-cols-subgrid",
    },
    Entry {
        name: "grid-flow-col",
        status: Status::NoEquivalent("grid auto flow not exposed by gpui Style"),
        sample: "grid-flow-col",
    },
    Entry {
        name: "grid-flow-col-dense",
        status: Status::NoEquivalent("grid auto flow not exposed by gpui Style"),
        sample: "grid-flow-col-dense",
    },
    Entry {
        name: "grid-flow-dense",
        status: Status::NoEquivalent("grid auto flow not exposed by gpui Style"),
        sample: "grid-flow-dense",
    },
    Entry {
        name: "grid-flow-row",
        status: Status::NoEquivalent("grid auto flow not exposed by gpui Style"),
        sample: "grid-flow-row",
    },
    Entry {
        name: "grid-flow-row-dense",
        status: Status::NoEquivalent("grid auto flow not exposed by gpui Style"),
        sample: "grid-flow-row-dense",
    },
    Entry {
        name: "grid-rows",
        status: Status::Todo,
        sample: "grid-rows-3",
    },
    Entry {
        name: "grid-rows-none",
        status: Status::NoEquivalent("subgrid/none resets not expressible in gpui"),
        sample: "grid-rows-none",
    },
    Entry {
        name: "grid-rows-subgrid",
        status: Status::NoEquivalent("subgrid/none resets not expressible in gpui"),
        sample: "grid-rows-subgrid",
    },
    Entry {
        name: "grow",
        status: Status::Supported,
        sample: "grow",
    },
    Entry {
        name: "h",
        status: Status::Supported,
        sample: "h-8",
    },
    Entry {
        name: "h-auto",
        status: Status::Supported,
        sample: "h-auto",
    },
    Entry {
        name: "h-dvh",
        status: Status::NoEquivalent("viewport units are not in gpui styles"),
        sample: "h-dvh",
    },
    Entry {
        name: "h-dvw",
        status: Status::NoEquivalent("viewport units are not in gpui styles"),
        sample: "h-dvw",
    },
    Entry {
        name: "h-fit",
        status: Status::NoEquivalent("min/max/fit-content sizing is not in taffy"),
        sample: "h-fit",
    },
    Entry {
        name: "h-full",
        status: Status::Supported,
        sample: "h-full",
    },
    Entry {
        name: "h-lh",
        status: Status::NoEquivalent("lh units are not in gpui styles"),
        sample: "h-lh",
    },
    Entry {
        name: "h-lvh",
        status: Status::NoEquivalent("viewport units are not in gpui styles"),
        sample: "h-lvh",
    },
    Entry {
        name: "h-lvw",
        status: Status::NoEquivalent("viewport units are not in gpui styles"),
        sample: "h-lvw",
    },
    Entry {
        name: "h-max",
        status: Status::NoEquivalent("min/max/fit-content sizing is not in taffy"),
        sample: "h-max",
    },
    Entry {
        name: "h-min",
        status: Status::NoEquivalent("min/max/fit-content sizing is not in taffy"),
        sample: "h-min",
    },
    Entry {
        name: "h-screen",
        status: Status::NoEquivalent("viewport units are not in gpui styles"),
        sample: "h-screen",
    },
    Entry {
        name: "h-svh",
        status: Status::NoEquivalent("viewport units are not in gpui styles"),
        sample: "h-svh",
    },
    Entry {
        name: "h-svw",
        status: Status::NoEquivalent("viewport units are not in gpui styles"),
        sample: "h-svw",
    },
    Entry {
        name: "hidden",
        status: Status::Supported,
        sample: "hidden",
    },
    Entry {
        name: "hue-rotate",
        status: Status::NoEquivalent("no filter pipeline in gpui"),
        sample: "hue-rotate-2",
    },
    Entry {
        name: "hyphens-auto",
        status: Status::NoEquivalent("no hyphenation control in gpui"),
        sample: "hyphens-auto",
    },
    Entry {
        name: "hyphens-manual",
        status: Status::NoEquivalent("no hyphenation control in gpui"),
        sample: "hyphens-manual",
    },
    Entry {
        name: "hyphens-none",
        status: Status::NoEquivalent("no hyphenation control in gpui"),
        sample: "hyphens-none",
    },
    Entry {
        name: "indent",
        status: Status::NoEquivalent("no text-indent in gpui"),
        sample: "indent-2",
    },
    Entry {
        name: "inline",
        status: Status::NoEquivalent("no inline formatting context in taffy"),
        sample: "inline",
    },
    Entry {
        name: "inline-block",
        status: Status::NoEquivalent("no inline formatting context in taffy"),
        sample: "inline-block",
    },
    Entry {
        name: "inline-flex",
        status: Status::Supported,
        sample: "inline-flex",
    },
    Entry {
        name: "inline-grid",
        status: Status::Supported,
        sample: "inline-grid",
    },
    Entry {
        name: "inline-table",
        status: Status::NoEquivalent("no table layout in gpui"),
        sample: "inline-table",
    },
    Entry {
        name: "inset",
        status: Status::Supported,
        sample: "inset-2",
    },
    Entry {
        name: "inset-auto",
        status: Status::Supported,
        sample: "inset-auto",
    },
    Entry {
        name: "inset-full",
        status: Status::Supported,
        sample: "inset-full",
    },
    Entry {
        name: "inset-ring",
        status: Status::Supported,
        sample: "inset-ring-2",
    },
    Entry {
        name: "inset-shadow",
        status: Status::Supported,
        sample: "inset-shadow-sm",
    },
    Entry {
        name: "inset-shadow-initial",
        status: Status::NoEquivalent("CSS default/reset; omit the class instead"),
        sample: "inset-shadow-initial",
    },
    Entry {
        name: "inset-x",
        status: Status::Supported,
        sample: "inset-x-2",
    },
    Entry {
        name: "inset-x-auto",
        status: Status::Supported,
        sample: "inset-x-auto",
    },
    Entry {
        name: "inset-x-full",
        status: Status::Supported,
        sample: "inset-x-full",
    },
    Entry {
        name: "inset-y",
        status: Status::Supported,
        sample: "inset-y-2",
    },
    Entry {
        name: "inset-y-auto",
        status: Status::Supported,
        sample: "inset-y-auto",
    },
    Entry {
        name: "inset-y-full",
        status: Status::Supported,
        sample: "inset-y-full",
    },
    Entry {
        name: "invert",
        status: Status::NoEquivalent("no filter pipeline in gpui"),
        sample: "invert-2",
    },
    Entry {
        name: "invisible",
        status: Status::Supported,
        sample: "invisible",
    },
    Entry {
        name: "isolate",
        status: Status::NoEquivalent("no stacking-context control in gpui"),
        sample: "isolate",
    },
    Entry {
        name: "isolation-auto",
        status: Status::NoEquivalent("no stacking-context control in gpui"),
        sample: "isolation-auto",
    },
    Entry {
        name: "italic",
        status: Status::Supported,
        sample: "italic",
    },
    Entry {
        name: "items-baseline",
        status: Status::Supported,
        sample: "items-baseline",
    },
    Entry {
        name: "items-baseline-last",
        status: Status::NoEquivalent("safe/baseline-last alignment not in taffy"),
        sample: "items-baseline-last",
    },
    Entry {
        name: "items-center",
        status: Status::Supported,
        sample: "items-center",
    },
    Entry {
        name: "items-center-safe",
        status: Status::NoEquivalent("safe/baseline-last alignment not in taffy"),
        sample: "items-center-safe",
    },
    Entry {
        name: "items-end",
        status: Status::Supported,
        sample: "items-end",
    },
    Entry {
        name: "items-end-safe",
        status: Status::NoEquivalent("safe/baseline-last alignment not in taffy"),
        sample: "items-end-safe",
    },
    Entry {
        name: "items-start",
        status: Status::Supported,
        sample: "items-start",
    },
    Entry {
        name: "items-stretch",
        status: Status::Supported,
        sample: "items-stretch",
    },
    Entry {
        name: "justify-around",
        status: Status::Supported,
        sample: "justify-around",
    },
    Entry {
        name: "justify-baseline",
        status: Status::NoEquivalent("safe/baseline-last alignment not in taffy"),
        sample: "justify-baseline",
    },
    Entry {
        name: "justify-between",
        status: Status::Supported,
        sample: "justify-between",
    },
    Entry {
        name: "justify-center",
        status: Status::Supported,
        sample: "justify-center",
    },
    Entry {
        name: "justify-center-safe",
        status: Status::NoEquivalent("safe/baseline-last alignment not in taffy"),
        sample: "justify-center-safe",
    },
    Entry {
        name: "justify-end",
        status: Status::Supported,
        sample: "justify-end",
    },
    Entry {
        name: "justify-end-safe",
        status: Status::NoEquivalent("safe/baseline-last alignment not in taffy"),
        sample: "justify-end-safe",
    },
    Entry {
        name: "justify-evenly",
        status: Status::Supported,
        sample: "justify-evenly",
    },
    Entry {
        name: "justify-items-center",
        status: Status::NoEquivalent("justify-items not exposed by gpui Style"),
        sample: "justify-items-center",
    },
    Entry {
        name: "justify-items-center-safe",
        status: Status::NoEquivalent("justify-items not exposed by gpui Style"),
        sample: "justify-items-center-safe",
    },
    Entry {
        name: "justify-items-end",
        status: Status::NoEquivalent("justify-items not exposed by gpui Style"),
        sample: "justify-items-end",
    },
    Entry {
        name: "justify-items-end-safe",
        status: Status::NoEquivalent("justify-items not exposed by gpui Style"),
        sample: "justify-items-end-safe",
    },
    Entry {
        name: "justify-items-normal",
        status: Status::NoEquivalent("justify-items not exposed by gpui Style"),
        sample: "justify-items-normal",
    },
    Entry {
        name: "justify-items-start",
        status: Status::NoEquivalent("justify-items not exposed by gpui Style"),
        sample: "justify-items-start",
    },
    Entry {
        name: "justify-items-stretch",
        status: Status::NoEquivalent("justify-items not exposed by gpui Style"),
        sample: "justify-items-stretch",
    },
    Entry {
        name: "justify-normal",
        status: Status::NoEquivalent("CSS default/reset; omit the class instead"),
        sample: "justify-normal",
    },
    Entry {
        name: "justify-self-auto",
        status: Status::NoEquivalent("justify-self not exposed by gpui Style"),
        sample: "justify-self-auto",
    },
    Entry {
        name: "justify-self-center",
        status: Status::NoEquivalent("justify-self not exposed by gpui Style"),
        sample: "justify-self-center",
    },
    Entry {
        name: "justify-self-center-safe",
        status: Status::NoEquivalent("justify-self not exposed by gpui Style"),
        sample: "justify-self-center-safe",
    },
    Entry {
        name: "justify-self-end",
        status: Status::NoEquivalent("justify-self not exposed by gpui Style"),
        sample: "justify-self-end",
    },
    Entry {
        name: "justify-self-end-safe",
        status: Status::NoEquivalent("justify-self not exposed by gpui Style"),
        sample: "justify-self-end-safe",
    },
    Entry {
        name: "justify-self-start",
        status: Status::NoEquivalent("justify-self not exposed by gpui Style"),
        sample: "justify-self-start",
    },
    Entry {
        name: "justify-self-stretch",
        status: Status::NoEquivalent("justify-self not exposed by gpui Style"),
        sample: "justify-self-stretch",
    },
    Entry {
        name: "justify-start",
        status: Status::Supported,
        sample: "justify-start",
    },
    Entry {
        name: "justify-stretch",
        status: Status::Supported,
        sample: "justify-stretch",
    },
    Entry {
        name: "leading",
        status: Status::Supported,
        sample: "leading-6",
    },
    Entry {
        name: "leading-none",
        status: Status::Supported,
        sample: "leading-none",
    },
    Entry {
        name: "left",
        status: Status::Supported,
        sample: "left-2",
    },
    Entry {
        name: "left-auto",
        status: Status::Supported,
        sample: "left-auto",
    },
    Entry {
        name: "left-full",
        status: Status::Supported,
        sample: "left-full",
    },
    Entry {
        name: "line-clamp",
        status: Status::Supported,
        sample: "line-clamp-2",
    },
    Entry {
        name: "line-clamp-none",
        status: Status::NoEquivalent("CSS default/reset; omit the class instead"),
        sample: "line-clamp-none",
    },
    Entry {
        name: "line-through",
        status: Status::Supported,
        sample: "line-through",
    },
    Entry {
        name: "lining-nums",
        status: Status::Supported,
        sample: "lining-nums",
    },
    Entry {
        name: "list",
        status: Status::NoEquivalent("no list markers in gpui"),
        sample: "list-2",
    },
    Entry {
        name: "list-decimal",
        status: Status::NoEquivalent("no list markers in gpui"),
        sample: "list-decimal",
    },
    Entry {
        name: "list-disc",
        status: Status::NoEquivalent("no list markers in gpui"),
        sample: "list-disc",
    },
    Entry {
        name: "list-image",
        status: Status::NoEquivalent("no list markers in gpui"),
        sample: "list-image-2",
    },
    Entry {
        name: "list-image-none",
        status: Status::NoEquivalent("no list markers in gpui"),
        sample: "list-image-none",
    },
    Entry {
        name: "list-inside",
        status: Status::NoEquivalent("no list markers in gpui"),
        sample: "list-inside",
    },
    Entry {
        name: "list-item",
        status: Status::NoEquivalent("no list markers in gpui"),
        sample: "list-item",
    },
    Entry {
        name: "list-none",
        status: Status::NoEquivalent("no list markers in gpui"),
        sample: "list-none",
    },
    Entry {
        name: "list-outside",
        status: Status::NoEquivalent("no list markers in gpui"),
        sample: "list-outside",
    },
    Entry {
        name: "lowercase",
        status: Status::NoEquivalent("no text-transform in gpui"),
        sample: "lowercase",
    },
    Entry {
        name: "m",
        status: Status::Supported,
        sample: "m-4",
    },
    Entry {
        name: "m-auto",
        status: Status::Supported,
        sample: "m-auto",
    },
    Entry {
        name: "mask",
        status: Status::NoEquivalent("no mask support in gpui"),
        sample: "mask-2",
    },
    Entry {
        name: "mask-add",
        status: Status::NoEquivalent("no mask support in gpui"),
        sample: "mask-add",
    },
    Entry {
        name: "mask-alpha",
        status: Status::NoEquivalent("no mask support in gpui"),
        sample: "mask-alpha",
    },
    Entry {
        name: "mask-auto",
        status: Status::NoEquivalent("no mask support in gpui"),
        sample: "mask-auto",
    },
    Entry {
        name: "mask-bottom",
        status: Status::NoEquivalent("no mask support in gpui"),
        sample: "mask-bottom",
    },
    Entry {
        name: "mask-bottom-left",
        status: Status::NoEquivalent("no mask support in gpui"),
        sample: "mask-bottom-left",
    },
    Entry {
        name: "mask-bottom-right",
        status: Status::NoEquivalent("no mask support in gpui"),
        sample: "mask-bottom-right",
    },
    Entry {
        name: "mask-center",
        status: Status::NoEquivalent("no mask support in gpui"),
        sample: "mask-center",
    },
    Entry {
        name: "mask-circle",
        status: Status::NoEquivalent("no mask support in gpui"),
        sample: "mask-circle",
    },
    Entry {
        name: "mask-clip-border",
        status: Status::NoEquivalent("no mask support in gpui"),
        sample: "mask-clip-border",
    },
    Entry {
        name: "mask-clip-content",
        status: Status::NoEquivalent("no mask support in gpui"),
        sample: "mask-clip-content",
    },
    Entry {
        name: "mask-clip-fill",
        status: Status::NoEquivalent("no mask support in gpui"),
        sample: "mask-clip-fill",
    },
    Entry {
        name: "mask-clip-padding",
        status: Status::NoEquivalent("no mask support in gpui"),
        sample: "mask-clip-padding",
    },
    Entry {
        name: "mask-clip-stroke",
        status: Status::NoEquivalent("no mask support in gpui"),
        sample: "mask-clip-stroke",
    },
    Entry {
        name: "mask-clip-view",
        status: Status::NoEquivalent("no mask support in gpui"),
        sample: "mask-clip-view",
    },
    Entry {
        name: "mask-conic",
        status: Status::NoEquivalent("no mask support in gpui"),
        sample: "mask-conic-2",
    },
    Entry {
        name: "mask-contain",
        status: Status::NoEquivalent("no mask support in gpui"),
        sample: "mask-contain",
    },
    Entry {
        name: "mask-cover",
        status: Status::NoEquivalent("no mask support in gpui"),
        sample: "mask-cover",
    },
    Entry {
        name: "mask-ellipse",
        status: Status::NoEquivalent("no mask support in gpui"),
        sample: "mask-ellipse",
    },
    Entry {
        name: "mask-exclude",
        status: Status::NoEquivalent("no mask support in gpui"),
        sample: "mask-exclude",
    },
    Entry {
        name: "mask-intersect",
        status: Status::NoEquivalent("no mask support in gpui"),
        sample: "mask-intersect",
    },
    Entry {
        name: "mask-left",
        status: Status::NoEquivalent("no mask support in gpui"),
        sample: "mask-left",
    },
    Entry {
        name: "mask-linear",
        status: Status::NoEquivalent("no mask support in gpui"),
        sample: "mask-linear-2",
    },
    Entry {
        name: "mask-luminance",
        status: Status::NoEquivalent("no mask support in gpui"),
        sample: "mask-luminance",
    },
    Entry {
        name: "mask-match",
        status: Status::NoEquivalent("no mask support in gpui"),
        sample: "mask-match",
    },
    Entry {
        name: "mask-no-clip",
        status: Status::NoEquivalent("no mask support in gpui"),
        sample: "mask-no-clip",
    },
    Entry {
        name: "mask-no-repeat",
        status: Status::NoEquivalent("no mask support in gpui"),
        sample: "mask-no-repeat",
    },
    Entry {
        name: "mask-none",
        status: Status::NoEquivalent("no mask support in gpui"),
        sample: "mask-none",
    },
    Entry {
        name: "mask-origin-border",
        status: Status::NoEquivalent("no mask support in gpui"),
        sample: "mask-origin-border",
    },
    Entry {
        name: "mask-origin-content",
        status: Status::NoEquivalent("no mask support in gpui"),
        sample: "mask-origin-content",
    },
    Entry {
        name: "mask-origin-fill",
        status: Status::NoEquivalent("no mask support in gpui"),
        sample: "mask-origin-fill",
    },
    Entry {
        name: "mask-origin-padding",
        status: Status::NoEquivalent("no mask support in gpui"),
        sample: "mask-origin-padding",
    },
    Entry {
        name: "mask-origin-stroke",
        status: Status::NoEquivalent("no mask support in gpui"),
        sample: "mask-origin-stroke",
    },
    Entry {
        name: "mask-origin-view",
        status: Status::NoEquivalent("no mask support in gpui"),
        sample: "mask-origin-view",
    },
    Entry {
        name: "mask-position",
        status: Status::NoEquivalent("no mask support in gpui"),
        sample: "mask-position-2",
    },
    Entry {
        name: "mask-radial",
        status: Status::NoEquivalent("no mask support in gpui"),
        sample: "mask-radial-2",
    },
    Entry {
        name: "mask-radial-at",
        status: Status::NoEquivalent("no mask support in gpui"),
        sample: "mask-radial-at-2",
    },
    Entry {
        name: "mask-radial-at-bottom",
        status: Status::NoEquivalent("no mask support in gpui"),
        sample: "mask-radial-at-bottom",
    },
    Entry {
        name: "mask-radial-at-bottom-left",
        status: Status::NoEquivalent("no mask support in gpui"),
        sample: "mask-radial-at-bottom-left",
    },
    Entry {
        name: "mask-radial-at-bottom-right",
        status: Status::NoEquivalent("no mask support in gpui"),
        sample: "mask-radial-at-bottom-right",
    },
    Entry {
        name: "mask-radial-at-center",
        status: Status::NoEquivalent("no mask support in gpui"),
        sample: "mask-radial-at-center",
    },
    Entry {
        name: "mask-radial-at-left",
        status: Status::NoEquivalent("no mask support in gpui"),
        sample: "mask-radial-at-left",
    },
    Entry {
        name: "mask-radial-at-right",
        status: Status::NoEquivalent("no mask support in gpui"),
        sample: "mask-radial-at-right",
    },
    Entry {
        name: "mask-radial-at-top",
        status: Status::NoEquivalent("no mask support in gpui"),
        sample: "mask-radial-at-top",
    },
    Entry {
        name: "mask-radial-at-top-left",
        status: Status::NoEquivalent("no mask support in gpui"),
        sample: "mask-radial-at-top-left",
    },
    Entry {
        name: "mask-radial-at-top-right",
        status: Status::NoEquivalent("no mask support in gpui"),
        sample: "mask-radial-at-top-right",
    },
    Entry {
        name: "mask-radial-closest-corner",
        status: Status::NoEquivalent("no mask support in gpui"),
        sample: "mask-radial-closest-corner",
    },
    Entry {
        name: "mask-radial-closest-side",
        status: Status::NoEquivalent("no mask support in gpui"),
        sample: "mask-radial-closest-side",
    },
    Entry {
        name: "mask-radial-farthest-corner",
        status: Status::NoEquivalent("no mask support in gpui"),
        sample: "mask-radial-farthest-corner",
    },
    Entry {
        name: "mask-radial-farthest-side",
        status: Status::NoEquivalent("no mask support in gpui"),
        sample: "mask-radial-farthest-side",
    },
    Entry {
        name: "mask-repeat",
        status: Status::NoEquivalent("no mask support in gpui"),
        sample: "mask-repeat",
    },
    Entry {
        name: "mask-repeat-round",
        status: Status::NoEquivalent("no mask support in gpui"),
        sample: "mask-repeat-round",
    },
    Entry {
        name: "mask-repeat-space",
        status: Status::NoEquivalent("no mask support in gpui"),
        sample: "mask-repeat-space",
    },
    Entry {
        name: "mask-repeat-x",
        status: Status::NoEquivalent("no mask support in gpui"),
        sample: "mask-repeat-x",
    },
    Entry {
        name: "mask-repeat-y",
        status: Status::NoEquivalent("no mask support in gpui"),
        sample: "mask-repeat-y",
    },
    Entry {
        name: "mask-right",
        status: Status::NoEquivalent("no mask support in gpui"),
        sample: "mask-right",
    },
    Entry {
        name: "mask-size",
        status: Status::NoEquivalent("no mask support in gpui"),
        sample: "mask-size-2",
    },
    Entry {
        name: "mask-subtract",
        status: Status::NoEquivalent("no mask support in gpui"),
        sample: "mask-subtract",
    },
    Entry {
        name: "mask-top",
        status: Status::NoEquivalent("no mask support in gpui"),
        sample: "mask-top",
    },
    Entry {
        name: "mask-top-left",
        status: Status::NoEquivalent("no mask support in gpui"),
        sample: "mask-top-left",
    },
    Entry {
        name: "mask-top-right",
        status: Status::NoEquivalent("no mask support in gpui"),
        sample: "mask-top-right",
    },
    Entry {
        name: "mask-type-alpha",
        status: Status::NoEquivalent("no mask support in gpui"),
        sample: "mask-type-alpha",
    },
    Entry {
        name: "mask-type-luminance",
        status: Status::NoEquivalent("no mask support in gpui"),
        sample: "mask-type-luminance",
    },
    Entry {
        name: "max-h",
        status: Status::Supported,
        sample: "max-h-8",
    },
    Entry {
        name: "max-h-dvh",
        status: Status::NoEquivalent("viewport units are not in gpui styles"),
        sample: "max-h-dvh",
    },
    Entry {
        name: "max-h-dvw",
        status: Status::NoEquivalent("viewport units are not in gpui styles"),
        sample: "max-h-dvw",
    },
    Entry {
        name: "max-h-fit",
        status: Status::NoEquivalent("min/max/fit-content sizing is not in taffy"),
        sample: "max-h-fit",
    },
    Entry {
        name: "max-h-full",
        status: Status::Supported,
        sample: "max-h-full",
    },
    Entry {
        name: "max-h-lh",
        status: Status::NoEquivalent("lh units are not in gpui styles"),
        sample: "max-h-lh",
    },
    Entry {
        name: "max-h-lvh",
        status: Status::NoEquivalent("viewport units are not in gpui styles"),
        sample: "max-h-lvh",
    },
    Entry {
        name: "max-h-lvw",
        status: Status::NoEquivalent("viewport units are not in gpui styles"),
        sample: "max-h-lvw",
    },
    Entry {
        name: "max-h-max",
        status: Status::NoEquivalent("min/max/fit-content sizing is not in taffy"),
        sample: "max-h-max",
    },
    Entry {
        name: "max-h-min",
        status: Status::NoEquivalent("min/max/fit-content sizing is not in taffy"),
        sample: "max-h-min",
    },
    Entry {
        name: "max-h-none",
        status: Status::Supported,
        sample: "max-h-none",
    },
    Entry {
        name: "max-h-screen",
        status: Status::NoEquivalent("viewport units are not in gpui styles"),
        sample: "max-h-screen",
    },
    Entry {
        name: "max-h-svh",
        status: Status::NoEquivalent("viewport units are not in gpui styles"),
        sample: "max-h-svh",
    },
    Entry {
        name: "max-h-svw",
        status: Status::NoEquivalent("viewport units are not in gpui styles"),
        sample: "max-h-svw",
    },
    Entry {
        name: "max-w",
        status: Status::Supported,
        sample: "max-w-8",
    },
    Entry {
        name: "max-w-dvh",
        status: Status::NoEquivalent("viewport units are not in gpui styles"),
        sample: "max-w-dvh",
    },
    Entry {
        name: "max-w-dvw",
        status: Status::NoEquivalent("viewport units are not in gpui styles"),
        sample: "max-w-dvw",
    },
    Entry {
        name: "max-w-fit",
        status: Status::NoEquivalent("min/max/fit-content sizing is not in taffy"),
        sample: "max-w-fit",
    },
    Entry {
        name: "max-w-full",
        status: Status::Supported,
        sample: "max-w-full",
    },
    Entry {
        name: "max-w-lvh",
        status: Status::NoEquivalent("viewport units are not in gpui styles"),
        sample: "max-w-lvh",
    },
    Entry {
        name: "max-w-lvw",
        status: Status::NoEquivalent("viewport units are not in gpui styles"),
        sample: "max-w-lvw",
    },
    Entry {
        name: "max-w-max",
        status: Status::NoEquivalent("min/max/fit-content sizing is not in taffy"),
        sample: "max-w-max",
    },
    Entry {
        name: "max-w-min",
        status: Status::NoEquivalent("min/max/fit-content sizing is not in taffy"),
        sample: "max-w-min",
    },
    Entry {
        name: "max-w-none",
        status: Status::Supported,
        sample: "max-w-none",
    },
    Entry {
        name: "max-w-screen",
        status: Status::NoEquivalent("viewport units are not in gpui styles"),
        sample: "max-w-screen",
    },
    Entry {
        name: "max-w-svh",
        status: Status::NoEquivalent("viewport units are not in gpui styles"),
        sample: "max-w-svh",
    },
    Entry {
        name: "max-w-svw",
        status: Status::NoEquivalent("viewport units are not in gpui styles"),
        sample: "max-w-svw",
    },
    Entry {
        name: "mb",
        status: Status::Supported,
        sample: "mb-4",
    },
    Entry {
        name: "mb-auto",
        status: Status::Supported,
        sample: "mb-auto",
    },
    Entry {
        name: "me",
        status: Status::Supported,
        sample: "me-4",
    },
    Entry {
        name: "me-auto",
        status: Status::Supported,
        sample: "me-auto",
    },
    Entry {
        name: "min-h",
        status: Status::Supported,
        sample: "min-h-8",
    },
    Entry {
        name: "min-h-auto",
        status: Status::Supported,
        sample: "min-h-auto",
    },
    Entry {
        name: "min-h-dvh",
        status: Status::NoEquivalent("viewport units are not in gpui styles"),
        sample: "min-h-dvh",
    },
    Entry {
        name: "min-h-dvw",
        status: Status::NoEquivalent("viewport units are not in gpui styles"),
        sample: "min-h-dvw",
    },
    Entry {
        name: "min-h-fit",
        status: Status::NoEquivalent("min/max/fit-content sizing is not in taffy"),
        sample: "min-h-fit",
    },
    Entry {
        name: "min-h-full",
        status: Status::Supported,
        sample: "min-h-full",
    },
    Entry {
        name: "min-h-lh",
        status: Status::NoEquivalent("lh units are not in gpui styles"),
        sample: "min-h-lh",
    },
    Entry {
        name: "min-h-lvh",
        status: Status::NoEquivalent("viewport units are not in gpui styles"),
        sample: "min-h-lvh",
    },
    Entry {
        name: "min-h-lvw",
        status: Status::NoEquivalent("viewport units are not in gpui styles"),
        sample: "min-h-lvw",
    },
    Entry {
        name: "min-h-max",
        status: Status::NoEquivalent("min/max/fit-content sizing is not in taffy"),
        sample: "min-h-max",
    },
    Entry {
        name: "min-h-min",
        status: Status::NoEquivalent("min/max/fit-content sizing is not in taffy"),
        sample: "min-h-min",
    },
    Entry {
        name: "min-h-screen",
        status: Status::NoEquivalent("viewport units are not in gpui styles"),
        sample: "min-h-screen",
    },
    Entry {
        name: "min-h-svh",
        status: Status::NoEquivalent("viewport units are not in gpui styles"),
        sample: "min-h-svh",
    },
    Entry {
        name: "min-h-svw",
        status: Status::NoEquivalent("viewport units are not in gpui styles"),
        sample: "min-h-svw",
    },
    Entry {
        name: "min-w",
        status: Status::Supported,
        sample: "min-w-8",
    },
    Entry {
        name: "min-w-auto",
        status: Status::Supported,
        sample: "min-w-auto",
    },
    Entry {
        name: "min-w-dvh",
        status: Status::NoEquivalent("viewport units are not in gpui styles"),
        sample: "min-w-dvh",
    },
    Entry {
        name: "min-w-dvw",
        status: Status::NoEquivalent("viewport units are not in gpui styles"),
        sample: "min-w-dvw",
    },
    Entry {
        name: "min-w-fit",
        status: Status::NoEquivalent("min/max/fit-content sizing is not in taffy"),
        sample: "min-w-fit",
    },
    Entry {
        name: "min-w-full",
        status: Status::Supported,
        sample: "min-w-full",
    },
    Entry {
        name: "min-w-lvh",
        status: Status::NoEquivalent("viewport units are not in gpui styles"),
        sample: "min-w-lvh",
    },
    Entry {
        name: "min-w-lvw",
        status: Status::NoEquivalent("viewport units are not in gpui styles"),
        sample: "min-w-lvw",
    },
    Entry {
        name: "min-w-max",
        status: Status::NoEquivalent("min/max/fit-content sizing is not in taffy"),
        sample: "min-w-max",
    },
    Entry {
        name: "min-w-min",
        status: Status::NoEquivalent("min/max/fit-content sizing is not in taffy"),
        sample: "min-w-min",
    },
    Entry {
        name: "min-w-screen",
        status: Status::NoEquivalent("viewport units are not in gpui styles"),
        sample: "min-w-screen",
    },
    Entry {
        name: "min-w-svh",
        status: Status::NoEquivalent("viewport units are not in gpui styles"),
        sample: "min-w-svh",
    },
    Entry {
        name: "min-w-svw",
        status: Status::NoEquivalent("viewport units are not in gpui styles"),
        sample: "min-w-svw",
    },
    Entry {
        name: "mix-blend-color",
        status: Status::NoEquivalent("no blend modes in gpui"),
        sample: "mix-blend-color",
    },
    Entry {
        name: "mix-blend-color-burn",
        status: Status::NoEquivalent("no blend modes in gpui"),
        sample: "mix-blend-color-burn",
    },
    Entry {
        name: "mix-blend-color-dodge",
        status: Status::NoEquivalent("no blend modes in gpui"),
        sample: "mix-blend-color-dodge",
    },
    Entry {
        name: "mix-blend-darken",
        status: Status::NoEquivalent("no blend modes in gpui"),
        sample: "mix-blend-darken",
    },
    Entry {
        name: "mix-blend-difference",
        status: Status::NoEquivalent("no blend modes in gpui"),
        sample: "mix-blend-difference",
    },
    Entry {
        name: "mix-blend-exclusion",
        status: Status::NoEquivalent("no blend modes in gpui"),
        sample: "mix-blend-exclusion",
    },
    Entry {
        name: "mix-blend-hard-light",
        status: Status::NoEquivalent("no blend modes in gpui"),
        sample: "mix-blend-hard-light",
    },
    Entry {
        name: "mix-blend-hue",
        status: Status::NoEquivalent("no blend modes in gpui"),
        sample: "mix-blend-hue",
    },
    Entry {
        name: "mix-blend-lighten",
        status: Status::NoEquivalent("no blend modes in gpui"),
        sample: "mix-blend-lighten",
    },
    Entry {
        name: "mix-blend-luminosity",
        status: Status::NoEquivalent("no blend modes in gpui"),
        sample: "mix-blend-luminosity",
    },
    Entry {
        name: "mix-blend-multiply",
        status: Status::NoEquivalent("no blend modes in gpui"),
        sample: "mix-blend-multiply",
    },
    Entry {
        name: "mix-blend-normal",
        status: Status::NoEquivalent("no blend modes in gpui"),
        sample: "mix-blend-normal",
    },
    Entry {
        name: "mix-blend-overlay",
        status: Status::NoEquivalent("no blend modes in gpui"),
        sample: "mix-blend-overlay",
    },
    Entry {
        name: "mix-blend-plus-darker",
        status: Status::NoEquivalent("no blend modes in gpui"),
        sample: "mix-blend-plus-darker",
    },
    Entry {
        name: "mix-blend-plus-lighter",
        status: Status::NoEquivalent("no blend modes in gpui"),
        sample: "mix-blend-plus-lighter",
    },
    Entry {
        name: "mix-blend-saturation",
        status: Status::NoEquivalent("no blend modes in gpui"),
        sample: "mix-blend-saturation",
    },
    Entry {
        name: "mix-blend-screen",
        status: Status::NoEquivalent("no blend modes in gpui"),
        sample: "mix-blend-screen",
    },
    Entry {
        name: "mix-blend-soft-light",
        status: Status::NoEquivalent("no blend modes in gpui"),
        sample: "mix-blend-soft-light",
    },
    Entry {
        name: "ml",
        status: Status::Supported,
        sample: "ml-4",
    },
    Entry {
        name: "ml-auto",
        status: Status::Supported,
        sample: "ml-auto",
    },
    Entry {
        name: "mr",
        status: Status::Supported,
        sample: "mr-4",
    },
    Entry {
        name: "mr-auto",
        status: Status::Supported,
        sample: "mr-auto",
    },
    Entry {
        name: "ms",
        status: Status::Supported,
        sample: "ms-4",
    },
    Entry {
        name: "ms-auto",
        status: Status::Supported,
        sample: "ms-auto",
    },
    Entry {
        name: "mt",
        status: Status::Supported,
        sample: "mt-4",
    },
    Entry {
        name: "mt-auto",
        status: Status::Supported,
        sample: "mt-auto",
    },
    Entry {
        name: "mx",
        status: Status::Supported,
        sample: "mx-4",
    },
    Entry {
        name: "mx-auto",
        status: Status::Supported,
        sample: "mx-auto",
    },
    Entry {
        name: "my",
        status: Status::Supported,
        sample: "my-4",
    },
    Entry {
        name: "my-auto",
        status: Status::Supported,
        sample: "my-auto",
    },
    Entry {
        name: "no-underline",
        status: Status::Supported,
        sample: "no-underline",
    },
    Entry {
        name: "normal-case",
        status: Status::NoEquivalent("no text-transform in gpui"),
        sample: "normal-case",
    },
    Entry {
        name: "normal-nums",
        status: Status::NoEquivalent("CSS default/reset; omit the class instead"),
        sample: "normal-nums",
    },
    Entry {
        name: "not-italic",
        status: Status::Supported,
        sample: "not-italic",
    },
    Entry {
        name: "not-sr-only",
        status: Status::NoEquivalent("screen-reader positioning; gpui a11y works via roles"),
        sample: "not-sr-only",
    },
    Entry {
        name: "object",
        status: Status::NoEquivalent("object-fit is an img-element property in gpui, not a style"),
        sample: "object-2",
    },
    Entry {
        name: "object-bottom",
        status: Status::NoEquivalent("object-fit is an img-element property in gpui, not a style"),
        sample: "object-bottom",
    },
    Entry {
        name: "object-bottom-left",
        status: Status::NoEquivalent("object-fit is an img-element property in gpui, not a style"),
        sample: "object-bottom-left",
    },
    Entry {
        name: "object-bottom-right",
        status: Status::NoEquivalent("object-fit is an img-element property in gpui, not a style"),
        sample: "object-bottom-right",
    },
    Entry {
        name: "object-center",
        status: Status::NoEquivalent("object-fit is an img-element property in gpui, not a style"),
        sample: "object-center",
    },
    Entry {
        name: "object-contain",
        status: Status::NoEquivalent("object-fit is an img-element property in gpui, not a style"),
        sample: "object-contain",
    },
    Entry {
        name: "object-cover",
        status: Status::NoEquivalent("object-fit is an img-element property in gpui, not a style"),
        sample: "object-cover",
    },
    Entry {
        name: "object-fill",
        status: Status::NoEquivalent("object-fit is an img-element property in gpui, not a style"),
        sample: "object-fill",
    },
    Entry {
        name: "object-left",
        status: Status::NoEquivalent("object-fit is an img-element property in gpui, not a style"),
        sample: "object-left",
    },
    Entry {
        name: "object-none",
        status: Status::NoEquivalent("object-fit is an img-element property in gpui, not a style"),
        sample: "object-none",
    },
    Entry {
        name: "object-right",
        status: Status::NoEquivalent("object-fit is an img-element property in gpui, not a style"),
        sample: "object-right",
    },
    Entry {
        name: "object-scale-down",
        status: Status::NoEquivalent("object-fit is an img-element property in gpui, not a style"),
        sample: "object-scale-down",
    },
    Entry {
        name: "object-top",
        status: Status::NoEquivalent("object-fit is an img-element property in gpui, not a style"),
        sample: "object-top",
    },
    Entry {
        name: "object-top-left",
        status: Status::NoEquivalent("object-fit is an img-element property in gpui, not a style"),
        sample: "object-top-left",
    },
    Entry {
        name: "object-top-right",
        status: Status::NoEquivalent("object-fit is an img-element property in gpui, not a style"),
        sample: "object-top-right",
    },
    Entry {
        name: "oldstyle-nums",
        status: Status::Supported,
        sample: "oldstyle-nums",
    },
    Entry {
        name: "opacity",
        status: Status::Supported,
        sample: "opacity-50",
    },
    Entry {
        name: "order",
        status: Status::NoEquivalent("paint/layout order is element-tree order in gpui"),
        sample: "order-2",
    },
    Entry {
        name: "order-first",
        status: Status::NoEquivalent("paint/layout order is element-tree order in gpui"),
        sample: "order-first",
    },
    Entry {
        name: "order-last",
        status: Status::NoEquivalent("paint/layout order is element-tree order in gpui"),
        sample: "order-last",
    },
    Entry {
        name: "ordinal",
        status: Status::Supported,
        sample: "ordinal",
    },
    Entry {
        name: "origin",
        status: Status::NoEquivalent("no transforms in gpui styles"),
        sample: "origin-2",
    },
    Entry {
        name: "origin-bottom",
        status: Status::NoEquivalent("no transforms in gpui styles"),
        sample: "origin-bottom",
    },
    Entry {
        name: "origin-bottom-left",
        status: Status::NoEquivalent("no transforms in gpui styles"),
        sample: "origin-bottom-left",
    },
    Entry {
        name: "origin-bottom-right",
        status: Status::NoEquivalent("no transforms in gpui styles"),
        sample: "origin-bottom-right",
    },
    Entry {
        name: "origin-center",
        status: Status::NoEquivalent("no transforms in gpui styles"),
        sample: "origin-center",
    },
    Entry {
        name: "origin-left",
        status: Status::NoEquivalent("no transforms in gpui styles"),
        sample: "origin-left",
    },
    Entry {
        name: "origin-right",
        status: Status::NoEquivalent("no transforms in gpui styles"),
        sample: "origin-right",
    },
    Entry {
        name: "origin-top",
        status: Status::NoEquivalent("no transforms in gpui styles"),
        sample: "origin-top",
    },
    Entry {
        name: "origin-top-left",
        status: Status::NoEquivalent("no transforms in gpui styles"),
        sample: "origin-top-left",
    },
    Entry {
        name: "origin-top-right",
        status: Status::NoEquivalent("no transforms in gpui styles"),
        sample: "origin-top-right",
    },
    Entry {
        name: "outline",
        status: Status::NoEquivalent("no outline in gpui; use ring-* for focus rings"),
        sample: "outline-2",
    },
    Entry {
        name: "outline-dashed",
        status: Status::NoEquivalent("no outline in gpui; use ring-* for focus rings"),
        sample: "outline-dashed",
    },
    Entry {
        name: "outline-dotted",
        status: Status::NoEquivalent("no outline in gpui; use ring-* for focus rings"),
        sample: "outline-dotted",
    },
    Entry {
        name: "outline-double",
        status: Status::NoEquivalent("no outline in gpui; use ring-* for focus rings"),
        sample: "outline-double",
    },
    Entry {
        name: "outline-hidden",
        status: Status::NoEquivalent("no outline in gpui; use ring-* for focus rings"),
        sample: "outline-hidden",
    },
    Entry {
        name: "outline-none",
        status: Status::NoEquivalent("no outline in gpui; use ring-* for focus rings"),
        sample: "outline-none",
    },
    Entry {
        name: "outline-offset",
        status: Status::NoEquivalent("no outline in gpui; use ring-* for focus rings"),
        sample: "outline-offset-2",
    },
    Entry {
        name: "outline-solid",
        status: Status::NoEquivalent("no outline in gpui; use ring-* for focus rings"),
        sample: "outline-solid",
    },
    Entry {
        name: "overflow-auto",
        status: Status::Supported,
        sample: "overflow-auto",
    },
    Entry {
        name: "overflow-clip",
        status: Status::Supported,
        sample: "overflow-clip",
    },
    Entry {
        name: "overflow-hidden",
        status: Status::Supported,
        sample: "overflow-hidden",
    },
    Entry {
        name: "overflow-scroll",
        status: Status::Supported,
        sample: "overflow-scroll",
    },
    Entry {
        name: "overflow-visible",
        status: Status::Supported,
        sample: "overflow-visible",
    },
    Entry {
        name: "overflow-x-auto",
        status: Status::Supported,
        sample: "overflow-x-auto",
    },
    Entry {
        name: "overflow-x-clip",
        status: Status::Supported,
        sample: "overflow-x-clip",
    },
    Entry {
        name: "overflow-x-hidden",
        status: Status::Supported,
        sample: "overflow-x-hidden",
    },
    Entry {
        name: "overflow-x-scroll",
        status: Status::Supported,
        sample: "overflow-x-scroll",
    },
    Entry {
        name: "overflow-x-visible",
        status: Status::Supported,
        sample: "overflow-x-visible",
    },
    Entry {
        name: "overflow-y-auto",
        status: Status::Supported,
        sample: "overflow-y-auto",
    },
    Entry {
        name: "overflow-y-clip",
        status: Status::Supported,
        sample: "overflow-y-clip",
    },
    Entry {
        name: "overflow-y-hidden",
        status: Status::Supported,
        sample: "overflow-y-hidden",
    },
    Entry {
        name: "overflow-y-scroll",
        status: Status::Supported,
        sample: "overflow-y-scroll",
    },
    Entry {
        name: "overflow-y-visible",
        status: Status::Supported,
        sample: "overflow-y-visible",
    },
    Entry {
        name: "overline",
        status: Status::NoEquivalent("gpui text decorations are underline/strikethrough"),
        sample: "overline",
    },
    Entry {
        name: "overscroll-auto",
        status: Status::NoEquivalent("overscroll behavior is not a gpui style"),
        sample: "overscroll-auto",
    },
    Entry {
        name: "overscroll-contain",
        status: Status::NoEquivalent("overscroll behavior is not a gpui style"),
        sample: "overscroll-contain",
    },
    Entry {
        name: "overscroll-none",
        status: Status::NoEquivalent("overscroll behavior is not a gpui style"),
        sample: "overscroll-none",
    },
    Entry {
        name: "overscroll-x-auto",
        status: Status::NoEquivalent("overscroll behavior is not a gpui style"),
        sample: "overscroll-x-auto",
    },
    Entry {
        name: "overscroll-x-contain",
        status: Status::NoEquivalent("overscroll behavior is not a gpui style"),
        sample: "overscroll-x-contain",
    },
    Entry {
        name: "overscroll-x-none",
        status: Status::NoEquivalent("overscroll behavior is not a gpui style"),
        sample: "overscroll-x-none",
    },
    Entry {
        name: "overscroll-y-auto",
        status: Status::NoEquivalent("overscroll behavior is not a gpui style"),
        sample: "overscroll-y-auto",
    },
    Entry {
        name: "overscroll-y-contain",
        status: Status::NoEquivalent("overscroll behavior is not a gpui style"),
        sample: "overscroll-y-contain",
    },
    Entry {
        name: "overscroll-y-none",
        status: Status::NoEquivalent("overscroll behavior is not a gpui style"),
        sample: "overscroll-y-none",
    },
    Entry {
        name: "p",
        status: Status::Supported,
        sample: "p-4",
    },
    Entry {
        name: "pb",
        status: Status::Supported,
        sample: "pb-4",
    },
    Entry {
        name: "pe",
        status: Status::Supported,
        sample: "pe-4",
    },
    Entry {
        name: "perspective",
        status: Status::NoEquivalent("no transforms in gpui styles"),
        sample: "perspective-2",
    },
    Entry {
        name: "perspective-none",
        status: Status::NoEquivalent("no transforms in gpui styles"),
        sample: "perspective-none",
    },
    Entry {
        name: "perspective-origin",
        status: Status::NoEquivalent("no transforms in gpui styles"),
        sample: "perspective-origin-2",
    },
    Entry {
        name: "perspective-origin-bottom",
        status: Status::NoEquivalent("no transforms in gpui styles"),
        sample: "perspective-origin-bottom",
    },
    Entry {
        name: "perspective-origin-bottom-left",
        status: Status::NoEquivalent("no transforms in gpui styles"),
        sample: "perspective-origin-bottom-left",
    },
    Entry {
        name: "perspective-origin-bottom-right",
        status: Status::NoEquivalent("no transforms in gpui styles"),
        sample: "perspective-origin-bottom-right",
    },
    Entry {
        name: "perspective-origin-center",
        status: Status::NoEquivalent("no transforms in gpui styles"),
        sample: "perspective-origin-center",
    },
    Entry {
        name: "perspective-origin-left",
        status: Status::NoEquivalent("no transforms in gpui styles"),
        sample: "perspective-origin-left",
    },
    Entry {
        name: "perspective-origin-right",
        status: Status::NoEquivalent("no transforms in gpui styles"),
        sample: "perspective-origin-right",
    },
    Entry {
        name: "perspective-origin-top",
        status: Status::NoEquivalent("no transforms in gpui styles"),
        sample: "perspective-origin-top",
    },
    Entry {
        name: "perspective-origin-top-left",
        status: Status::NoEquivalent("no transforms in gpui styles"),
        sample: "perspective-origin-top-left",
    },
    Entry {
        name: "perspective-origin-top-right",
        status: Status::NoEquivalent("no transforms in gpui styles"),
        sample: "perspective-origin-top-right",
    },
    Entry {
        name: "pl",
        status: Status::Supported,
        sample: "pl-4",
    },
    Entry {
        name: "place-content-around",
        status: Status::NoEquivalent("grid place-* not exposed by gpui Style"),
        sample: "place-content-around",
    },
    Entry {
        name: "place-content-baseline",
        status: Status::NoEquivalent("grid place-* not exposed by gpui Style"),
        sample: "place-content-baseline",
    },
    Entry {
        name: "place-content-between",
        status: Status::NoEquivalent("grid place-* not exposed by gpui Style"),
        sample: "place-content-between",
    },
    Entry {
        name: "place-content-center",
        status: Status::NoEquivalent("grid place-* not exposed by gpui Style"),
        sample: "place-content-center",
    },
    Entry {
        name: "place-content-center-safe",
        status: Status::NoEquivalent("grid place-* not exposed by gpui Style"),
        sample: "place-content-center-safe",
    },
    Entry {
        name: "place-content-end",
        status: Status::NoEquivalent("grid place-* not exposed by gpui Style"),
        sample: "place-content-end",
    },
    Entry {
        name: "place-content-end-safe",
        status: Status::NoEquivalent("grid place-* not exposed by gpui Style"),
        sample: "place-content-end-safe",
    },
    Entry {
        name: "place-content-evenly",
        status: Status::NoEquivalent("grid place-* not exposed by gpui Style"),
        sample: "place-content-evenly",
    },
    Entry {
        name: "place-content-start",
        status: Status::NoEquivalent("grid place-* not exposed by gpui Style"),
        sample: "place-content-start",
    },
    Entry {
        name: "place-content-stretch",
        status: Status::NoEquivalent("grid place-* not exposed by gpui Style"),
        sample: "place-content-stretch",
    },
    Entry {
        name: "place-items-baseline",
        status: Status::NoEquivalent("grid place-* not exposed by gpui Style"),
        sample: "place-items-baseline",
    },
    Entry {
        name: "place-items-center",
        status: Status::NoEquivalent("grid place-* not exposed by gpui Style"),
        sample: "place-items-center",
    },
    Entry {
        name: "place-items-center-safe",
        status: Status::NoEquivalent("grid place-* not exposed by gpui Style"),
        sample: "place-items-center-safe",
    },
    Entry {
        name: "place-items-end",
        status: Status::NoEquivalent("grid place-* not exposed by gpui Style"),
        sample: "place-items-end",
    },
    Entry {
        name: "place-items-end-safe",
        status: Status::NoEquivalent("grid place-* not exposed by gpui Style"),
        sample: "place-items-end-safe",
    },
    Entry {
        name: "place-items-start",
        status: Status::NoEquivalent("grid place-* not exposed by gpui Style"),
        sample: "place-items-start",
    },
    Entry {
        name: "place-items-stretch",
        status: Status::NoEquivalent("grid place-* not exposed by gpui Style"),
        sample: "place-items-stretch",
    },
    Entry {
        name: "place-self-auto",
        status: Status::NoEquivalent("grid place-* not exposed by gpui Style"),
        sample: "place-self-auto",
    },
    Entry {
        name: "place-self-center",
        status: Status::NoEquivalent("grid place-* not exposed by gpui Style"),
        sample: "place-self-center",
    },
    Entry {
        name: "place-self-center-safe",
        status: Status::NoEquivalent("grid place-* not exposed by gpui Style"),
        sample: "place-self-center-safe",
    },
    Entry {
        name: "place-self-end",
        status: Status::NoEquivalent("grid place-* not exposed by gpui Style"),
        sample: "place-self-end",
    },
    Entry {
        name: "place-self-end-safe",
        status: Status::NoEquivalent("grid place-* not exposed by gpui Style"),
        sample: "place-self-end-safe",
    },
    Entry {
        name: "place-self-start",
        status: Status::NoEquivalent("grid place-* not exposed by gpui Style"),
        sample: "place-self-start",
    },
    Entry {
        name: "place-self-stretch",
        status: Status::NoEquivalent("grid place-* not exposed by gpui Style"),
        sample: "place-self-stretch",
    },
    Entry {
        name: "placeholder",
        status: Status::NoEquivalent("input internals are component-level in rcn"),
        sample: "placeholder-2",
    },
    Entry {
        name: "pointer-events-auto",
        status: Status::NoEquivalent("hit-testing is behavioral in gpui, not a style"),
        sample: "pointer-events-auto",
    },
    Entry {
        name: "pointer-events-none",
        status: Status::NoEquivalent("hit-testing is behavioral in gpui, not a style"),
        sample: "pointer-events-none",
    },
    Entry {
        name: "pr",
        status: Status::Supported,
        sample: "pr-4",
    },
    Entry {
        name: "proportional-nums",
        status: Status::Supported,
        sample: "proportional-nums",
    },
    Entry {
        name: "ps",
        status: Status::Supported,
        sample: "ps-4",
    },
    Entry {
        name: "pt",
        status: Status::Supported,
        sample: "pt-4",
    },
    Entry {
        name: "px",
        status: Status::Supported,
        sample: "px-4",
    },
    Entry {
        name: "py",
        status: Status::Supported,
        sample: "py-4",
    },
    Entry {
        name: "relative",
        status: Status::Supported,
        sample: "relative",
    },
    Entry {
        name: "resize",
        status: Status::NoEquivalent("resizing is component behavior (see Textarea)"),
        sample: "resize",
    },
    Entry {
        name: "resize-none",
        status: Status::NoEquivalent("resizing is component behavior (see Textarea)"),
        sample: "resize-none",
    },
    Entry {
        name: "resize-x",
        status: Status::NoEquivalent("resizing is component behavior (see Textarea)"),
        sample: "resize-x",
    },
    Entry {
        name: "resize-y",
        status: Status::NoEquivalent("resizing is component behavior (see Textarea)"),
        sample: "resize-y",
    },
    Entry {
        name: "right",
        status: Status::Supported,
        sample: "right-2",
    },
    Entry {
        name: "right-auto",
        status: Status::Supported,
        sample: "right-auto",
    },
    Entry {
        name: "right-full",
        status: Status::Supported,
        sample: "right-full",
    },
    Entry {
        name: "ring",
        status: Status::Supported,
        sample: "ring-2",
    },
    Entry {
        name: "ring-inset",
        status: Status::Supported,
        sample: "ring-inset",
    },
    Entry {
        name: "ring-offset",
        status: Status::NoEquivalent("ring offset requires compositing gpui lacks"),
        sample: "ring-offset-2",
    },
    Entry {
        name: "rotate",
        status: Status::NoEquivalent("no transforms in gpui styles"),
        sample: "rotate-2",
    },
    Entry {
        name: "rotate-none",
        status: Status::NoEquivalent("no transforms in gpui styles"),
        sample: "rotate-none",
    },
    Entry {
        name: "rounded",
        status: Status::Supported,
        sample: "rounded-lg",
    },
    Entry {
        name: "rounded-b",
        status: Status::Supported,
        sample: "rounded-b-lg",
    },
    Entry {
        name: "rounded-b-full",
        status: Status::Supported,
        sample: "rounded-b-full",
    },
    Entry {
        name: "rounded-b-none",
        status: Status::Supported,
        sample: "rounded-b-none",
    },
    Entry {
        name: "rounded-bl",
        status: Status::Supported,
        sample: "rounded-bl-lg",
    },
    Entry {
        name: "rounded-bl-full",
        status: Status::Supported,
        sample: "rounded-bl-full",
    },
    Entry {
        name: "rounded-bl-none",
        status: Status::Supported,
        sample: "rounded-bl-none",
    },
    Entry {
        name: "rounded-br",
        status: Status::Supported,
        sample: "rounded-br-lg",
    },
    Entry {
        name: "rounded-br-full",
        status: Status::Supported,
        sample: "rounded-br-full",
    },
    Entry {
        name: "rounded-br-none",
        status: Status::Supported,
        sample: "rounded-br-none",
    },
    Entry {
        name: "rounded-e",
        status: Status::Supported,
        sample: "rounded-e-lg",
    },
    Entry {
        name: "rounded-e-full",
        status: Status::Supported,
        sample: "rounded-e-full",
    },
    Entry {
        name: "rounded-e-none",
        status: Status::Supported,
        sample: "rounded-e-none",
    },
    Entry {
        name: "rounded-ee",
        status: Status::Supported,
        sample: "rounded-ee-lg",
    },
    Entry {
        name: "rounded-ee-full",
        status: Status::Supported,
        sample: "rounded-ee-full",
    },
    Entry {
        name: "rounded-ee-none",
        status: Status::Supported,
        sample: "rounded-ee-none",
    },
    Entry {
        name: "rounded-es",
        status: Status::Supported,
        sample: "rounded-es-lg",
    },
    Entry {
        name: "rounded-es-full",
        status: Status::Supported,
        sample: "rounded-es-full",
    },
    Entry {
        name: "rounded-es-none",
        status: Status::Supported,
        sample: "rounded-es-none",
    },
    Entry {
        name: "rounded-full",
        status: Status::Supported,
        sample: "rounded-full",
    },
    Entry {
        name: "rounded-l",
        status: Status::Supported,
        sample: "rounded-l-lg",
    },
    Entry {
        name: "rounded-l-full",
        status: Status::Supported,
        sample: "rounded-l-full",
    },
    Entry {
        name: "rounded-l-none",
        status: Status::Supported,
        sample: "rounded-l-none",
    },
    Entry {
        name: "rounded-none",
        status: Status::Supported,
        sample: "rounded-none",
    },
    Entry {
        name: "rounded-r",
        status: Status::Supported,
        sample: "rounded-r-lg",
    },
    Entry {
        name: "rounded-r-full",
        status: Status::Supported,
        sample: "rounded-r-full",
    },
    Entry {
        name: "rounded-r-none",
        status: Status::Supported,
        sample: "rounded-r-none",
    },
    Entry {
        name: "rounded-s",
        status: Status::Supported,
        sample: "rounded-s-lg",
    },
    Entry {
        name: "rounded-s-full",
        status: Status::Supported,
        sample: "rounded-s-full",
    },
    Entry {
        name: "rounded-s-none",
        status: Status::Supported,
        sample: "rounded-s-none",
    },
    Entry {
        name: "rounded-se",
        status: Status::Supported,
        sample: "rounded-se-lg",
    },
    Entry {
        name: "rounded-se-full",
        status: Status::Supported,
        sample: "rounded-se-full",
    },
    Entry {
        name: "rounded-se-none",
        status: Status::Supported,
        sample: "rounded-se-none",
    },
    Entry {
        name: "rounded-ss",
        status: Status::Supported,
        sample: "rounded-ss-lg",
    },
    Entry {
        name: "rounded-ss-full",
        status: Status::Supported,
        sample: "rounded-ss-full",
    },
    Entry {
        name: "rounded-ss-none",
        status: Status::Supported,
        sample: "rounded-ss-none",
    },
    Entry {
        name: "rounded-t",
        status: Status::Supported,
        sample: "rounded-t-lg",
    },
    Entry {
        name: "rounded-t-full",
        status: Status::Supported,
        sample: "rounded-t-full",
    },
    Entry {
        name: "rounded-t-none",
        status: Status::Supported,
        sample: "rounded-t-none",
    },
    Entry {
        name: "rounded-tl",
        status: Status::Supported,
        sample: "rounded-tl-lg",
    },
    Entry {
        name: "rounded-tl-full",
        status: Status::Supported,
        sample: "rounded-tl-full",
    },
    Entry {
        name: "rounded-tl-none",
        status: Status::Supported,
        sample: "rounded-tl-none",
    },
    Entry {
        name: "rounded-tr",
        status: Status::Supported,
        sample: "rounded-tr-lg",
    },
    Entry {
        name: "rounded-tr-full",
        status: Status::Supported,
        sample: "rounded-tr-full",
    },
    Entry {
        name: "rounded-tr-none",
        status: Status::Supported,
        sample: "rounded-tr-none",
    },
    Entry {
        name: "row",
        status: Status::Todo,
        sample: "row-2",
    },
    Entry {
        name: "row-auto",
        status: Status::Todo,
        sample: "row-auto",
    },
    Entry {
        name: "row-end",
        status: Status::Todo,
        sample: "row-end-1",
    },
    Entry {
        name: "row-end-auto",
        status: Status::Todo,
        sample: "row-end-auto",
    },
    Entry {
        name: "row-span",
        status: Status::Todo,
        sample: "row-span-2",
    },
    Entry {
        name: "row-span-full",
        status: Status::Todo,
        sample: "row-span-full",
    },
    Entry {
        name: "row-start",
        status: Status::Todo,
        sample: "row-start-1",
    },
    Entry {
        name: "row-start-auto",
        status: Status::Todo,
        sample: "row-start-auto",
    },
    Entry {
        name: "saturate",
        status: Status::NoEquivalent("no filter pipeline in gpui"),
        sample: "saturate-2",
    },
    Entry {
        name: "scale",
        status: Status::NoEquivalent("no transforms in gpui styles"),
        sample: "scale-2",
    },
    Entry {
        name: "scale-3d",
        status: Status::NoEquivalent("no transforms in gpui styles"),
        sample: "scale-3d",
    },
    Entry {
        name: "scale-none",
        status: Status::NoEquivalent("no transforms in gpui styles"),
        sample: "scale-none",
    },
    Entry {
        name: "scheme-dark",
        status: Status::NoEquivalent("color scheme is theme-level in rcn"),
        sample: "scheme-dark",
    },
    Entry {
        name: "scheme-light",
        status: Status::NoEquivalent("color scheme is theme-level in rcn"),
        sample: "scheme-light",
    },
    Entry {
        name: "scheme-light-dark",
        status: Status::NoEquivalent("color scheme is theme-level in rcn"),
        sample: "scheme-light-dark",
    },
    Entry {
        name: "scheme-normal",
        status: Status::NoEquivalent("color scheme is theme-level in rcn"),
        sample: "scheme-normal",
    },
    Entry {
        name: "scheme-only-dark",
        status: Status::NoEquivalent("color scheme is theme-level in rcn"),
        sample: "scheme-only-dark",
    },
    Entry {
        name: "scheme-only-light",
        status: Status::NoEquivalent("color scheme is theme-level in rcn"),
        sample: "scheme-only-light",
    },
    Entry {
        name: "scroll-auto",
        status: Status::NoEquivalent("scroll behavior/margins are not gpui styles"),
        sample: "scroll-auto",
    },
    Entry {
        name: "scroll-m",
        status: Status::NoEquivalent("scroll behavior/margins are not gpui styles"),
        sample: "scroll-m-2",
    },
    Entry {
        name: "scroll-mb",
        status: Status::NoEquivalent("scroll behavior/margins are not gpui styles"),
        sample: "scroll-mb-2",
    },
    Entry {
        name: "scroll-me",
        status: Status::NoEquivalent("scroll behavior/margins are not gpui styles"),
        sample: "scroll-me-2",
    },
    Entry {
        name: "scroll-ml",
        status: Status::NoEquivalent("scroll behavior/margins are not gpui styles"),
        sample: "scroll-ml-2",
    },
    Entry {
        name: "scroll-mr",
        status: Status::NoEquivalent("scroll behavior/margins are not gpui styles"),
        sample: "scroll-mr-2",
    },
    Entry {
        name: "scroll-ms",
        status: Status::NoEquivalent("scroll behavior/margins are not gpui styles"),
        sample: "scroll-ms-2",
    },
    Entry {
        name: "scroll-mt",
        status: Status::NoEquivalent("scroll behavior/margins are not gpui styles"),
        sample: "scroll-mt-2",
    },
    Entry {
        name: "scroll-mx",
        status: Status::NoEquivalent("scroll behavior/margins are not gpui styles"),
        sample: "scroll-mx-2",
    },
    Entry {
        name: "scroll-my",
        status: Status::NoEquivalent("scroll behavior/margins are not gpui styles"),
        sample: "scroll-my-2",
    },
    Entry {
        name: "scroll-p",
        status: Status::NoEquivalent("scroll behavior/margins are not gpui styles"),
        sample: "scroll-p-2",
    },
    Entry {
        name: "scroll-pb",
        status: Status::NoEquivalent("scroll behavior/margins are not gpui styles"),
        sample: "scroll-pb-2",
    },
    Entry {
        name: "scroll-pe",
        status: Status::NoEquivalent("scroll behavior/margins are not gpui styles"),
        sample: "scroll-pe-2",
    },
    Entry {
        name: "scroll-pl",
        status: Status::NoEquivalent("scroll behavior/margins are not gpui styles"),
        sample: "scroll-pl-2",
    },
    Entry {
        name: "scroll-pr",
        status: Status::NoEquivalent("scroll behavior/margins are not gpui styles"),
        sample: "scroll-pr-2",
    },
    Entry {
        name: "scroll-ps",
        status: Status::NoEquivalent("scroll behavior/margins are not gpui styles"),
        sample: "scroll-ps-2",
    },
    Entry {
        name: "scroll-pt",
        status: Status::NoEquivalent("scroll behavior/margins are not gpui styles"),
        sample: "scroll-pt-2",
    },
    Entry {
        name: "scroll-px",
        status: Status::NoEquivalent("scroll behavior/margins are not gpui styles"),
        sample: "scroll-px-2",
    },
    Entry {
        name: "scroll-py",
        status: Status::NoEquivalent("scroll behavior/margins are not gpui styles"),
        sample: "scroll-py-2",
    },
    Entry {
        name: "scroll-smooth",
        status: Status::NoEquivalent("scroll behavior/margins are not gpui styles"),
        sample: "scroll-smooth",
    },
    Entry {
        name: "select-all",
        status: Status::NoEquivalent("text selection is component behavior in gpui"),
        sample: "select-all",
    },
    Entry {
        name: "select-auto",
        status: Status::NoEquivalent("text selection is component behavior in gpui"),
        sample: "select-auto",
    },
    Entry {
        name: "select-none",
        status: Status::NoEquivalent("text selection is component behavior in gpui"),
        sample: "select-none",
    },
    Entry {
        name: "select-text",
        status: Status::NoEquivalent("text selection is component behavior in gpui"),
        sample: "select-text",
    },
    Entry {
        name: "self-auto",
        status: Status::NoEquivalent("CSS default/reset; omit the class instead"),
        sample: "self-auto",
    },
    Entry {
        name: "self-baseline",
        status: Status::Supported,
        sample: "self-baseline",
    },
    Entry {
        name: "self-baseline-last",
        status: Status::NoEquivalent("safe/baseline-last alignment not in taffy"),
        sample: "self-baseline-last",
    },
    Entry {
        name: "self-center",
        status: Status::Supported,
        sample: "self-center",
    },
    Entry {
        name: "self-center-safe",
        status: Status::NoEquivalent("safe/baseline-last alignment not in taffy"),
        sample: "self-center-safe",
    },
    Entry {
        name: "self-end",
        status: Status::Supported,
        sample: "self-end",
    },
    Entry {
        name: "self-end-safe",
        status: Status::NoEquivalent("safe/baseline-last alignment not in taffy"),
        sample: "self-end-safe",
    },
    Entry {
        name: "self-start",
        status: Status::Supported,
        sample: "self-start",
    },
    Entry {
        name: "self-stretch",
        status: Status::Supported,
        sample: "self-stretch",
    },
    Entry {
        name: "sepia",
        status: Status::NoEquivalent("no filter pipeline in gpui"),
        sample: "sepia-2",
    },
    Entry {
        name: "shadow",
        status: Status::Supported,
        sample: "shadow-lg",
    },
    Entry {
        name: "shadow-initial",
        status: Status::NoEquivalent("CSS default/reset; omit the class instead"),
        sample: "shadow-initial",
    },
    Entry {
        name: "shrink",
        status: Status::Supported,
        sample: "shrink-0",
    },
    Entry {
        name: "size",
        status: Status::Supported,
        sample: "size-8",
    },
    Entry {
        name: "size-auto",
        status: Status::Supported,
        sample: "size-auto",
    },
    Entry {
        name: "size-dvh",
        status: Status::NoEquivalent("viewport units are not in gpui styles"),
        sample: "size-dvh",
    },
    Entry {
        name: "size-dvw",
        status: Status::NoEquivalent("viewport units are not in gpui styles"),
        sample: "size-dvw",
    },
    Entry {
        name: "size-fit",
        status: Status::NoEquivalent("min/max/fit-content sizing is not in taffy"),
        sample: "size-fit",
    },
    Entry {
        name: "size-full",
        status: Status::Supported,
        sample: "size-full",
    },
    Entry {
        name: "size-lvh",
        status: Status::NoEquivalent("viewport units are not in gpui styles"),
        sample: "size-lvh",
    },
    Entry {
        name: "size-lvw",
        status: Status::NoEquivalent("viewport units are not in gpui styles"),
        sample: "size-lvw",
    },
    Entry {
        name: "size-max",
        status: Status::NoEquivalent("min/max/fit-content sizing is not in taffy"),
        sample: "size-max",
    },
    Entry {
        name: "size-min",
        status: Status::NoEquivalent("min/max/fit-content sizing is not in taffy"),
        sample: "size-min",
    },
    Entry {
        name: "size-svh",
        status: Status::NoEquivalent("viewport units are not in gpui styles"),
        sample: "size-svh",
    },
    Entry {
        name: "size-svw",
        status: Status::NoEquivalent("viewport units are not in gpui styles"),
        sample: "size-svw",
    },
    Entry {
        name: "skew",
        status: Status::NoEquivalent("no transforms in gpui styles"),
        sample: "skew-2",
    },
    Entry {
        name: "skew-x",
        status: Status::NoEquivalent("no transforms in gpui styles"),
        sample: "skew-x-2",
    },
    Entry {
        name: "skew-y",
        status: Status::NoEquivalent("no transforms in gpui styles"),
        sample: "skew-y-2",
    },
    Entry {
        name: "slashed-zero",
        status: Status::Supported,
        sample: "slashed-zero",
    },
    Entry {
        name: "snap-align-none",
        status: Status::NoEquivalent("scroll snapping is not a gpui style"),
        sample: "snap-align-none",
    },
    Entry {
        name: "snap-always",
        status: Status::NoEquivalent("scroll snapping is not a gpui style"),
        sample: "snap-always",
    },
    Entry {
        name: "snap-both",
        status: Status::NoEquivalent("scroll snapping is not a gpui style"),
        sample: "snap-both",
    },
    Entry {
        name: "snap-center",
        status: Status::NoEquivalent("scroll snapping is not a gpui style"),
        sample: "snap-center",
    },
    Entry {
        name: "snap-end",
        status: Status::NoEquivalent("scroll snapping is not a gpui style"),
        sample: "snap-end",
    },
    Entry {
        name: "snap-mandatory",
        status: Status::NoEquivalent("scroll snapping is not a gpui style"),
        sample: "snap-mandatory",
    },
    Entry {
        name: "snap-none",
        status: Status::NoEquivalent("scroll snapping is not a gpui style"),
        sample: "snap-none",
    },
    Entry {
        name: "snap-normal",
        status: Status::NoEquivalent("scroll snapping is not a gpui style"),
        sample: "snap-normal",
    },
    Entry {
        name: "snap-proximity",
        status: Status::NoEquivalent("scroll snapping is not a gpui style"),
        sample: "snap-proximity",
    },
    Entry {
        name: "snap-start",
        status: Status::NoEquivalent("scroll snapping is not a gpui style"),
        sample: "snap-start",
    },
    Entry {
        name: "snap-x",
        status: Status::NoEquivalent("scroll snapping is not a gpui style"),
        sample: "snap-x",
    },
    Entry {
        name: "snap-y",
        status: Status::NoEquivalent("scroll snapping is not a gpui style"),
        sample: "snap-y",
    },
    Entry {
        name: "space-x",
        status: Status::NoEquivalent("child-combinator margins; use gap"),
        sample: "space-x-2",
    },
    Entry {
        name: "space-x-reverse",
        status: Status::NoEquivalent("child-combinator margins; use gap"),
        sample: "space-x-reverse",
    },
    Entry {
        name: "space-y",
        status: Status::NoEquivalent("child-combinator margins; use gap"),
        sample: "space-y-2",
    },
    Entry {
        name: "space-y-reverse",
        status: Status::NoEquivalent("child-combinator margins; use gap"),
        sample: "space-y-reverse",
    },
    Entry {
        name: "sr-only",
        status: Status::NoEquivalent("screen-reader positioning; gpui a11y works via roles"),
        sample: "sr-only",
    },
    Entry {
        name: "stacked-fractions",
        status: Status::Supported,
        sample: "stacked-fractions",
    },
    Entry {
        name: "start",
        status: Status::Supported,
        sample: "start-2",
    },
    Entry {
        name: "start-auto",
        status: Status::Supported,
        sample: "start-auto",
    },
    Entry {
        name: "start-full",
        status: Status::Supported,
        sample: "start-full",
    },
    Entry {
        name: "static",
        status: Status::NoEquivalent("gpui positions are relative/absolute only"),
        sample: "static",
    },
    Entry {
        name: "sticky",
        status: Status::NoEquivalent("gpui positions are relative/absolute only"),
        sample: "sticky",
    },
    Entry {
        name: "stroke",
        status: Status::NoEquivalent("svg stroke/fill resets are element-level in gpui"),
        sample: "stroke-2",
    },
    Entry {
        name: "stroke-none",
        status: Status::NoEquivalent("svg stroke/fill resets are element-level in gpui"),
        sample: "stroke-none",
    },
    Entry {
        name: "subpixel-antialiased",
        status: Status::NoEquivalent("text rasterization is platform-level"),
        sample: "subpixel-antialiased",
    },
    Entry {
        name: "table",
        status: Status::NoEquivalent("no table layout in gpui"),
        sample: "table",
    },
    Entry {
        name: "table-auto",
        status: Status::NoEquivalent("no table layout in gpui"),
        sample: "table-auto",
    },
    Entry {
        name: "table-caption",
        status: Status::NoEquivalent("no table layout in gpui"),
        sample: "table-caption",
    },
    Entry {
        name: "table-cell",
        status: Status::NoEquivalent("no table layout in gpui"),
        sample: "table-cell",
    },
    Entry {
        name: "table-column",
        status: Status::NoEquivalent("no table layout in gpui"),
        sample: "table-column",
    },
    Entry {
        name: "table-column-group",
        status: Status::NoEquivalent("no table layout in gpui"),
        sample: "table-column-group",
    },
    Entry {
        name: "table-fixed",
        status: Status::NoEquivalent("no table layout in gpui"),
        sample: "table-fixed",
    },
    Entry {
        name: "table-footer-group",
        status: Status::NoEquivalent("no table layout in gpui"),
        sample: "table-footer-group",
    },
    Entry {
        name: "table-header-group",
        status: Status::NoEquivalent("no table layout in gpui"),
        sample: "table-header-group",
    },
    Entry {
        name: "table-row",
        status: Status::NoEquivalent("no table layout in gpui"),
        sample: "table-row",
    },
    Entry {
        name: "table-row-group",
        status: Status::NoEquivalent("no table layout in gpui"),
        sample: "table-row-group",
    },
    Entry {
        name: "tabular-nums",
        status: Status::Supported,
        sample: "tabular-nums",
    },
    Entry {
        name: "text",
        status: Status::Supported,
        sample: "text-sm",
    },
    Entry {
        name: "text-balance",
        status: Status::NoEquivalent("no wrap balancing in gpui"),
        sample: "text-balance",
    },
    Entry {
        name: "text-center",
        status: Status::Supported,
        sample: "text-center",
    },
    Entry {
        name: "text-clip",
        status: Status::NoEquivalent("CSS default/reset; omit the class instead"),
        sample: "text-clip",
    },
    Entry {
        name: "text-ellipsis",
        status: Status::Supported,
        sample: "text-ellipsis",
    },
    Entry {
        name: "text-end",
        status: Status::Supported,
        sample: "text-end",
    },
    Entry {
        name: "text-justify",
        status: Status::NoEquivalent("gpui TextAlign is left/center/right"),
        sample: "text-justify",
    },
    Entry {
        name: "text-left",
        status: Status::Supported,
        sample: "text-left",
    },
    Entry {
        name: "text-nowrap",
        status: Status::Supported,
        sample: "text-nowrap",
    },
    Entry {
        name: "text-pretty",
        status: Status::NoEquivalent("no wrap balancing in gpui"),
        sample: "text-pretty",
    },
    Entry {
        name: "text-right",
        status: Status::Supported,
        sample: "text-right",
    },
    Entry {
        name: "text-shadow",
        status: Status::NoEquivalent("no text shadows in gpui"),
        sample: "text-shadow-2",
    },
    Entry {
        name: "text-shadow-initial",
        status: Status::NoEquivalent("CSS default/reset; omit the class instead"),
        sample: "text-shadow-initial",
    },
    Entry {
        name: "text-start",
        status: Status::Supported,
        sample: "text-start",
    },
    Entry {
        name: "text-wrap",
        status: Status::Supported,
        sample: "text-wrap",
    },
    Entry {
        name: "to",
        status: Status::Supported,
        sample: "to-red-500",
    },
    Entry {
        name: "top",
        status: Status::Supported,
        sample: "top-2",
    },
    Entry {
        name: "top-auto",
        status: Status::Supported,
        sample: "top-auto",
    },
    Entry {
        name: "top-full",
        status: Status::Supported,
        sample: "top-full",
    },
    Entry {
        name: "touch-auto",
        status: Status::NoEquivalent("touch-action is not a gpui style"),
        sample: "touch-auto",
    },
    Entry {
        name: "touch-manipulation",
        status: Status::NoEquivalent("touch-action is not a gpui style"),
        sample: "touch-manipulation",
    },
    Entry {
        name: "touch-none",
        status: Status::NoEquivalent("touch-action is not a gpui style"),
        sample: "touch-none",
    },
    Entry {
        name: "touch-pan-down",
        status: Status::NoEquivalent("touch-action is not a gpui style"),
        sample: "touch-pan-down",
    },
    Entry {
        name: "touch-pan-left",
        status: Status::NoEquivalent("touch-action is not a gpui style"),
        sample: "touch-pan-left",
    },
    Entry {
        name: "touch-pan-right",
        status: Status::NoEquivalent("touch-action is not a gpui style"),
        sample: "touch-pan-right",
    },
    Entry {
        name: "touch-pan-up",
        status: Status::NoEquivalent("touch-action is not a gpui style"),
        sample: "touch-pan-up",
    },
    Entry {
        name: "touch-pan-x",
        status: Status::NoEquivalent("touch-action is not a gpui style"),
        sample: "touch-pan-x",
    },
    Entry {
        name: "touch-pan-y",
        status: Status::NoEquivalent("touch-action is not a gpui style"),
        sample: "touch-pan-y",
    },
    Entry {
        name: "touch-pinch-zoom",
        status: Status::NoEquivalent("touch-action is not a gpui style"),
        sample: "touch-pinch-zoom",
    },
    Entry {
        name: "tracking",
        status: Status::NoEquivalent("no letter-spacing in gpui text system"),
        sample: "tracking-2",
    },
    Entry {
        name: "transform",
        status: Status::NoEquivalent("no transforms in gpui styles"),
        sample: "transform-2",
    },
    Entry {
        name: "transform-3d",
        status: Status::NoEquivalent("no transforms in gpui styles"),
        sample: "transform-3d",
    },
    Entry {
        name: "transform-border",
        status: Status::NoEquivalent("no transforms in gpui styles"),
        sample: "transform-border",
    },
    Entry {
        name: "transform-content",
        status: Status::NoEquivalent("no transforms in gpui styles"),
        sample: "transform-content",
    },
    Entry {
        name: "transform-cpu",
        status: Status::NoEquivalent("no transforms in gpui styles"),
        sample: "transform-cpu",
    },
    Entry {
        name: "transform-fill",
        status: Status::NoEquivalent("no transforms in gpui styles"),
        sample: "transform-fill",
    },
    Entry {
        name: "transform-flat",
        status: Status::NoEquivalent("no transforms in gpui styles"),
        sample: "transform-flat",
    },
    Entry {
        name: "transform-gpu",
        status: Status::NoEquivalent("no transforms in gpui styles"),
        sample: "transform-gpu",
    },
    Entry {
        name: "transform-none",
        status: Status::NoEquivalent("no transforms in gpui styles"),
        sample: "transform-none",
    },
    Entry {
        name: "transform-stroke",
        status: Status::NoEquivalent("no transforms in gpui styles"),
        sample: "transform-stroke",
    },
    Entry {
        name: "transform-view",
        status: Status::NoEquivalent("no transforms in gpui styles"),
        sample: "transform-view",
    },
    Entry {
        name: "transition",
        status: Status::NoEquivalent("no style transitions; motion timings live in crate::motion"),
        sample: "transition-2",
    },
    Entry {
        name: "transition-all",
        status: Status::NoEquivalent("no style transitions; motion timings live in crate::motion"),
        sample: "transition-all",
    },
    Entry {
        name: "transition-colors",
        status: Status::NoEquivalent("no style transitions; motion timings live in crate::motion"),
        sample: "transition-colors",
    },
    Entry {
        name: "transition-discrete",
        status: Status::NoEquivalent("no style transitions; motion timings live in crate::motion"),
        sample: "transition-discrete",
    },
    Entry {
        name: "transition-none",
        status: Status::NoEquivalent("no style transitions; motion timings live in crate::motion"),
        sample: "transition-none",
    },
    Entry {
        name: "transition-normal",
        status: Status::NoEquivalent("no style transitions; motion timings live in crate::motion"),
        sample: "transition-normal",
    },
    Entry {
        name: "transition-opacity",
        status: Status::NoEquivalent("no style transitions; motion timings live in crate::motion"),
        sample: "transition-opacity",
    },
    Entry {
        name: "transition-shadow",
        status: Status::NoEquivalent("no style transitions; motion timings live in crate::motion"),
        sample: "transition-shadow",
    },
    Entry {
        name: "transition-transform",
        status: Status::NoEquivalent("no style transitions; motion timings live in crate::motion"),
        sample: "transition-transform",
    },
    Entry {
        name: "translate",
        status: Status::Supported,
        sample: "translate-1",
    },
    Entry {
        name: "translate-3d",
        status: Status::NoEquivalent("no 3D transforms in gpui"),
        sample: "translate-3d",
    },
    Entry {
        name: "translate-full",
        status: Status::NoEquivalent(
            "percentage translate is of own size; gpui insets are parent-relative",
        ),
        sample: "translate-full",
    },
    Entry {
        name: "translate-none",
        status: Status::NoEquivalent("no 3D transforms in gpui"),
        sample: "translate-none",
    },
    Entry {
        name: "translate-x",
        status: Status::Supported,
        sample: "translate-x-1",
    },
    Entry {
        name: "translate-y",
        status: Status::Supported,
        sample: "translate-y-px",
    },
    Entry {
        name: "translate-z",
        status: Status::NoEquivalent("no 3D transforms in gpui"),
        sample: "translate-z-2",
    },
    Entry {
        name: "truncate",
        status: Status::Supported,
        sample: "truncate",
    },
    Entry {
        name: "underline",
        status: Status::Supported,
        sample: "underline",
    },
    Entry {
        name: "underline-offset",
        status: Status::NoEquivalent("gpui UnderlineStyle has no offset"),
        sample: "underline-offset-2",
    },
    Entry {
        name: "underline-offset-auto",
        status: Status::NoEquivalent("gpui UnderlineStyle has no offset"),
        sample: "underline-offset-auto",
    },
    Entry {
        name: "uppercase",
        status: Status::NoEquivalent("no text-transform in gpui"),
        sample: "uppercase",
    },
    Entry {
        name: "via",
        status: Status::NoEquivalent("gpui gradients are two-stop (from/to only)"),
        sample: "via-2",
    },
    Entry {
        name: "via-none",
        status: Status::NoEquivalent("CSS default/reset; omit the class instead"),
        sample: "via-none",
    },
    Entry {
        name: "visible",
        status: Status::Supported,
        sample: "visible",
    },
    Entry {
        name: "w",
        status: Status::Supported,
        sample: "w-8",
    },
    Entry {
        name: "w-auto",
        status: Status::Supported,
        sample: "w-auto",
    },
    Entry {
        name: "w-dvh",
        status: Status::NoEquivalent("viewport units are not in gpui styles"),
        sample: "w-dvh",
    },
    Entry {
        name: "w-dvw",
        status: Status::NoEquivalent("viewport units are not in gpui styles"),
        sample: "w-dvw",
    },
    Entry {
        name: "w-fit",
        status: Status::NoEquivalent("min/max/fit-content sizing is not in taffy"),
        sample: "w-fit",
    },
    Entry {
        name: "w-full",
        status: Status::Supported,
        sample: "w-full",
    },
    Entry {
        name: "w-lvh",
        status: Status::NoEquivalent("viewport units are not in gpui styles"),
        sample: "w-lvh",
    },
    Entry {
        name: "w-lvw",
        status: Status::NoEquivalent("viewport units are not in gpui styles"),
        sample: "w-lvw",
    },
    Entry {
        name: "w-max",
        status: Status::NoEquivalent("min/max/fit-content sizing is not in taffy"),
        sample: "w-max",
    },
    Entry {
        name: "w-min",
        status: Status::NoEquivalent("min/max/fit-content sizing is not in taffy"),
        sample: "w-min",
    },
    Entry {
        name: "w-screen",
        status: Status::NoEquivalent("viewport units are not in gpui styles"),
        sample: "w-screen",
    },
    Entry {
        name: "w-svh",
        status: Status::NoEquivalent("viewport units are not in gpui styles"),
        sample: "w-svh",
    },
    Entry {
        name: "w-svw",
        status: Status::NoEquivalent("viewport units are not in gpui styles"),
        sample: "w-svw",
    },
    Entry {
        name: "whitespace-break-spaces",
        status: Status::NoEquivalent("gpui WhiteSpace supports normal/nowrap only"),
        sample: "whitespace-break-spaces",
    },
    Entry {
        name: "whitespace-normal",
        status: Status::Supported,
        sample: "whitespace-normal",
    },
    Entry {
        name: "whitespace-nowrap",
        status: Status::Supported,
        sample: "whitespace-nowrap",
    },
    Entry {
        name: "whitespace-pre",
        status: Status::NoEquivalent("gpui WhiteSpace supports normal/nowrap only"),
        sample: "whitespace-pre",
    },
    Entry {
        name: "whitespace-pre-line",
        status: Status::NoEquivalent("gpui WhiteSpace supports normal/nowrap only"),
        sample: "whitespace-pre-line",
    },
    Entry {
        name: "whitespace-pre-wrap",
        status: Status::NoEquivalent("gpui WhiteSpace supports normal/nowrap only"),
        sample: "whitespace-pre-wrap",
    },
    Entry {
        name: "will-change",
        status: Status::NoEquivalent("rendering hint; nothing to map"),
        sample: "will-change-2",
    },
    Entry {
        name: "will-change-auto",
        status: Status::NoEquivalent("rendering hint; nothing to map"),
        sample: "will-change-auto",
    },
    Entry {
        name: "will-change-contents",
        status: Status::NoEquivalent("rendering hint; nothing to map"),
        sample: "will-change-contents",
    },
    Entry {
        name: "will-change-scroll",
        status: Status::NoEquivalent("rendering hint; nothing to map"),
        sample: "will-change-scroll",
    },
    Entry {
        name: "will-change-transform",
        status: Status::NoEquivalent("rendering hint; nothing to map"),
        sample: "will-change-transform",
    },
    Entry {
        name: "wrap-anywhere",
        status: Status::NoEquivalent("no overflow-wrap control in gpui"),
        sample: "wrap-anywhere",
    },
    Entry {
        name: "wrap-break-word",
        status: Status::NoEquivalent("no overflow-wrap control in gpui"),
        sample: "wrap-break-word",
    },
    Entry {
        name: "wrap-normal",
        status: Status::NoEquivalent("no overflow-wrap control in gpui"),
        sample: "wrap-normal",
    },
    Entry {
        name: "z",
        status: Status::NoEquivalent("paint/layout order is element-tree order in gpui"),
        sample: "z-2",
    },
    Entry {
        name: "z-auto",
        status: Status::NoEquivalent("CSS default/reset; omit the class instead"),
        sample: "z-auto",
    },
];

/// Longest-matching-root lookup for a (variant-stripped) token: try the full
/// token, then strip trailing `-segment`s until a ledger entry matches.
pub(super) fn lookup(token: &str) -> Option<Status> {
    let map = super::ledger_map();
    let mut t = token;
    loop {
        if let Some(entry) = map.get(t) {
            return Some(entry.status);
        }
        match t.rfind('-') {
            Some(i) => t = &t[..i],
            None => return None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::theme::Theme;
    use crate::tw::parse;
    use std::collections::BTreeSet;

    #[derive(serde::Deserialize)]
    struct Manifest {
        #[serde(rename = "static")]
        static_utilities: Vec<String>,
        functional: Vec<String>,
    }

    /// The ledger tracks exactly the utility roots Tailwind registers —
    /// nothing missing, nothing invented.
    #[test]
    fn ledger_covers_manifest_exactly() {
        let manifest: Manifest = serde_json::from_str(include_str!("manifest.json")).unwrap();
        let manifest_names: BTreeSet<&str> = manifest
            .static_utilities
            .iter()
            .chain(&manifest.functional)
            .map(|s| s.as_str())
            .collect();
        let ledger_names: BTreeSet<&str> = LEDGER.iter().map(|e| e.name).collect();
        let missing: Vec<_> = manifest_names.difference(&ledger_names).collect();
        let extra: Vec<_> = ledger_names.difference(&manifest_names).collect();
        assert!(
            missing.is_empty() && extra.is_empty(),
            "ledger out of sync with manifest.json — missing: {missing:?}, extra: {extra:?}"
        );
        // No duplicate ledger entries.
        assert_eq!(ledger_names.len(), LEDGER.len(), "duplicate ledger entries");
    }

    /// Every `Supported` sample must parse with nothing unknown or skipped.
    #[test]
    fn supported_samples_parse_cleanly() {
        let theme = Theme::light();
        for entry in LEDGER {
            if entry.status != Status::Supported {
                continue;
            }
            let styles = parse(&theme, entry.sample);
            assert!(
                styles.unknown.is_empty() && styles.skipped.is_empty(),
                "'{}' is ledgered Supported but its sample '{}' did not apply \
                 (unknown: {:?}, skipped: {:?})",
                entry.name,
                entry.sample,
                styles.unknown,
                styles.skipped,
            );
        }
    }

    /// Every `Todo` sample must still be unknown — implementing a utility
    /// forces flipping its ledger status.
    #[test]
    fn todo_samples_are_still_unknown() {
        let theme = Theme::light();
        for entry in LEDGER {
            if entry.status != Status::Todo {
                continue;
            }
            let styles = parse(&theme, entry.sample);
            assert_eq!(
                styles.unknown,
                vec![entry.sample.to_string()],
                "'{}' is ledgered Todo but its sample '{}' no longer parses as \
                 unknown — flip the ledger entry to Supported",
                entry.name,
                entry.sample,
            );
        }
    }

    /// Every `NoEquivalent` sample must be reported as skipped.
    #[test]
    fn no_equivalent_samples_are_skipped() {
        let theme = Theme::light();
        for entry in LEDGER {
            let Status::NoEquivalent(_) = entry.status else {
                continue;
            };
            let styles = parse(&theme, entry.sample);
            assert_eq!(
                styles.skipped,
                vec![entry.sample.to_string()],
                "'{}' is ledgered NoEquivalent but its sample '{}' was not skipped \
                 (unknown: {:?})",
                entry.name,
                entry.sample,
                styles.unknown,
            );
        }
    }

    /// Coverage of the mappable set: Supported / (Supported + Todo).
    /// Ratchet the floor upward as waves land; never lower it.
    #[test]
    fn coverage_floor() {
        let supported = LEDGER
            .iter()
            .filter(|e| e.status == Status::Supported)
            .count();
        let todo = LEDGER.iter().filter(|e| e.status == Status::Todo).count();
        let pct = supported as f32 / (supported + todo) as f32;
        eprintln!(
            "tw coverage: {supported} supported, {todo} todo, {} no-equivalent → {:.1}% of mappable",
            LEDGER.len() - supported - todo,
            pct * 100.
        );
        assert!(
            pct >= 0.90,
            "tw coverage regressed below the 80% floor: {:.1}%",
            pct * 100.
        );
    }
}
