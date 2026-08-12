//! Embedded asset source serving the chevron icons the components need, in
//! each supported icon library's own drawing style (mirroring shadcn's icon
//! library choice). All four sets are permissively licensed (Lucide ISC,
//! Tabler MIT, Phosphor MIT, Remix Apache-2.0).

use std::borrow::Cow;

use gpui::{AssetSource, Result, SharedString};

/// The icon set components draw from — shadcn create's "Icon Library" pick.
#[derive(Clone, Copy, PartialEq, Eq, Debug, Default)]
pub enum IconLibrary {
    #[default]
    Lucide,
    Tabler,
    Phosphor,
    Remix,
}

impl IconLibrary {
    pub const ALL: [IconLibrary; 4] = [
        IconLibrary::Lucide,
        IconLibrary::Tabler,
        IconLibrary::Phosphor,
        IconLibrary::Remix,
    ];

    pub fn label(self) -> &'static str {
        match self {
            IconLibrary::Lucide => "lucide",
            IconLibrary::Tabler => "tabler",
            IconLibrary::Phosphor => "phosphor",
            IconLibrary::Remix => "remix",
        }
    }

    fn dir(self) -> &'static str {
        self.label()
    }

    pub fn chevron_down(self) -> String {
        format!("icons/{}/chevron-down.svg", self.dir())
    }

    pub fn chevron_up(self) -> String {
        format!("icons/{}/chevron-up.svg", self.dir())
    }

    pub fn chevron_right(self) -> String {
        format!("icons/{}/chevron-right.svg", self.dir())
    }

    pub fn chevron_left(self) -> String {
        format!("icons/{}/chevron-left.svg", self.dir())
    }

    pub fn x(self) -> String {
        format!("icons/{}/x.svg", self.dir())
    }

    pub fn check(self) -> String {
        format!("icons/{}/check.svg", self.dir())
    }
}

/// A stroked 24px icon in the Lucide/Tabler style.
macro_rules! stroked {
    ($path:literal) => {
        concat!(
            r#"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><path d=""#,
            $path,
            r#""/></svg>"#
        )
        .as_bytes()
    };
}

/// A filled icon (Phosphor's 256 grid, Remix's 24 grid).
macro_rules! filled {
    ($viewbox:literal, $path:literal) => {
        concat!(
            r#"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 "#,
            $viewbox,
            r#"" fill="currentColor"><path d=""#,
            $path,
            r#""/></svg>"#
        )
        .as_bytes()
    };
}

const ICONS: &[(&str, &[u8])] = &[
    ("icons/lucide/chevron-down.svg", stroked!("m6 9 6 6 6-6")),
    ("icons/lucide/chevron-up.svg", stroked!("m18 15-6-6-6 6")),
    ("icons/lucide/chevron-right.svg", stroked!("m9 18 6-6-6-6")),
    ("icons/tabler/chevron-down.svg", stroked!("M6 9l6 6l6 -6")),
    ("icons/tabler/chevron-up.svg", stroked!("M6 15l6 -6l6 6")),
    ("icons/tabler/chevron-right.svg", stroked!("M9 6l6 6l-6 6")),
    (
        "icons/phosphor/chevron-down.svg",
        filled!(
            "256 256",
            "M213.66,101.66l-80,80a8,8,0,0,1-11.32,0l-80-80A8,8,0,0,1,53.66,90.34L128,164.69l74.34-74.35a8,8,0,0,1,11.32,11.32Z"
        ),
    ),
    (
        "icons/phosphor/chevron-up.svg",
        filled!(
            "256 256",
            "M213.66,165.66a8,8,0,0,1-11.32,0L128,91.31,53.66,165.66a8,8,0,0,1-11.32-11.32l80-80a8,8,0,0,1,11.32,0l80,80A8,8,0,0,1,213.66,165.66Z"
        ),
    ),
    (
        "icons/phosphor/chevron-right.svg",
        filled!(
            "256 256",
            "M181.66,133.66l-80,80a8,8,0,0,1-11.32-11.32L164.69,128,90.34,53.66a8,8,0,0,1,11.32-11.32l80,80A8,8,0,0,1,181.66,133.66Z"
        ),
    ),
    (
        "icons/remix/chevron-down.svg",
        filled!(
            "24 24",
            "M12 13.1717L16.9497 8.22192L18.364 9.63614L12 16.0001L5.63604 9.63614L7.05025 8.22192L12 13.1717Z"
        ),
    ),
    (
        "icons/remix/chevron-up.svg",
        filled!(
            "24 24",
            "M12 10.8284L7.05025 15.7782L5.63604 14.364L12 8L18.364 14.364L16.9497 15.7782L12 10.8284Z"
        ),
    ),
    (
        "icons/remix/chevron-right.svg",
        filled!(
            "24 24",
            "M13.1717 12L8.22192 7.05025L9.63614 5.63604L16.0001 12L9.63614 18.364L8.22192 16.9497L13.1717 12Z"
        ),
    ),
    ("icons/lucide/chevron-left.svg", stroked!("m15 18-6-6 6-6")),
    ("icons/tabler/chevron-left.svg", stroked!("M15 6l-6 6l6 6")),
    (
        "icons/phosphor/chevron-left.svg",
        filled!(
            "256 256",
            "M165.66,202.34a8,8,0,0,1-11.32,11.32l-80-80a8,8,0,0,1,0-11.32l80-80a8,8,0,0,1,11.32,11.32L91.31,128Z"
        ),
    ),
    (
        "icons/remix/chevron-left.svg",
        filled!(
            "24 24",
            "M10.8284 12L15.7782 16.9497L14.364 18.364L8 12L14.364 5.63604L15.7782 7.05025L10.8284 12Z"
        ),
    ),
    ("icons/lucide/x.svg", stroked!("M18 6 6 18M6 6l12 12")),
    ("icons/tabler/x.svg", stroked!("M18 6l-12 12M6 6l12 12")),
    (
        "icons/phosphor/x.svg",
        filled!(
            "256 256",
            "M205.66,194.34a8,8,0,0,1-11.32,11.32L128,139.31,61.66,205.66a8,8,0,0,1-11.32-11.32L116.69,128,50.34,61.66A8,8,0,0,1,61.66,50.34L128,116.69l66.34-66.35a8,8,0,0,1,11.32,11.32L139.31,128Z"
        ),
    ),
    (
        "icons/remix/x.svg",
        filled!(
            "24 24",
            "M12 10.5858L16.2426 6.34315L17.6569 7.75736L13.4142 12L17.6569 16.2426L16.2426 17.6569L12 13.4142L7.75736 17.6569L6.34315 16.2426L10.5858 12L6.34315 7.75736L7.75736 6.34315L12 10.5858Z"
        ),
    ),
    ("icons/lucide/check.svg", stroked!("M20 6 9 17l-5-5")),
    ("icons/tabler/check.svg", stroked!("M5 12l5 5l10 -10")),
    (
        "icons/phosphor/check.svg",
        filled!(
            "256 256",
            "M229.66,77.66l-128,128a8,8,0,0,1-11.32,0l-56-56a8,8,0,0,1,11.32-11.32L96,188.69,218.34,66.34a8,8,0,0,1,11.32,11.32Z"
        ),
    ),
    (
        "icons/remix/check.svg",
        filled!(
            "24 24",
            "M10 15.1716L19.1924 5.97919L20.6066 7.3934L10 18L3.63604 11.636L5.05025 10.2218L10 15.1716Z"
        ),
    ),
    // Status icons (lucide drawings shared across libraries for now —
    // TODO(rcn): per-library variants).
    (
        "icons/circle-alert.svg",
        br##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><circle cx="12" cy="12" r="10"/><line x1="12" x2="12" y1="8" y2="12"/><line x1="12" x2="12.01" y1="16" y2="16"/></svg>"##,
    ),
    (
        "icons/loader.svg",
        br##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><path d="M21 12a9 9 0 1 1-6.219-8.56"/></svg>"##,
    ),
    (
        "icons/ellipsis.svg",
        br##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><circle cx="12" cy="12" r="1"/><circle cx="19" cy="12" r="1"/><circle cx="5" cy="12" r="1"/></svg>"##,
    ),
    (
        "icons/circle-check.svg",
        br##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><circle cx="12" cy="12" r="10"/><path d="m9 12 2 2 4-4"/></svg>"##,
    ),
    (
        "icons/search.svg",
        br##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><circle cx="11" cy="11" r="8"/><path d="m21 21-4.3-4.3"/></svg>"##,
    ),
    // Lucide `save` — used by the tooltip keyboard-shortcut story.
    (
        "icons/save.svg",
        br##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><path d="M15.2 3a2 2 0 0 1 1.4.6l3.8 3.8a2 2 0 0 1 .6 1.4V19a2 2 0 0 1-2 2H5a2 2 0 0 1-2-2V5a2 2 0 0 1 2-2z"/><path d="M17 21v-7a1 1 0 0 0-1-1H8a1 1 0 0 0-1 1v7"/><path d="M7 3v4a1 1 0 0 0 1 1h7"/></svg>"##,
    ),
    // Pre-rotated 10px diamond (shadcn's `size-2.5 rotate-45 rounded-[2px]`
    // tooltip arrow). gpui divs cannot rotate, so the fill is baked in.
    // Slight path rounding approximates the 2px corner radius.
    (
        "icons/tooltip-arrow.svg",
        br##"<svg xmlns="http://www.w3.org/2000/svg" width="10" height="10" viewBox="0 0 10 10" fill="currentColor"><path d="M4.15.85c.47-.47 1.23-.47 1.7 0l3.3 3.3c.47.47.47 1.23 0 1.7l-3.3 3.3c-.47.47-1.23.47-1.7 0l-3.3-3.3c-.47-.47-.47-1.23 0-1.7l3.3-3.3z"/></svg>"##,
    ),
    (
        "icons/info.svg",
        br##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><circle cx="12" cy="12" r="10"/><path d="M12 16v-4"/><path d="M12 8h.01"/></svg>"##,
    ),
    (
        "icons/inbox.svg",
        br##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><polyline points="22 12 16 12 14 15 10 15 8 12 2 12"/><path d="M5.45 5.11 2 12v6a2 2 0 0 0 2 2h16a2 2 0 0 0 2-2v-6l-3.45-6.89A2 2 0 0 0 16.76 4H7.24a2 2 0 0 0-1.79 1.11z"/></svg>"##,
    ),
    (
        "icons/shield-alert.svg",
        br##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><path d="M20 13c0 5-3.5 7.5-7.66 8.95a1 1 0 0 1-.67-.01C7.5 20.5 4 18 4 13V6a1 1 0 0 1 1-1c2 0 4.5-1.2 6.24-2.72a1.17 1.17 0 0 1 1.52 0C14.51 3.81 17 5 19 5a1 1 0 0 1 1 1z"/><path d="M12 8v4"/><path d="M12 16h.01"/></svg>"##,
    ),
    (
        "icons/badge-check.svg",
        br##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><path d="M3.85 8.62a4 4 0 0 1 4.78-4.77 4 4 0 0 1 6.74 0 4 4 0 0 1 4.78 4.78 4 4 0 0 1 0 6.74 4 4 0 0 1-4.77 4.78 4 4 0 0 1-6.75 0 4 4 0 0 1-4.78-4.77 4 4 0 0 1 0-6.76Z"/><path d="m9 12 2 2 4-4"/></svg>"##,
    ),
    ("icons/plus.svg", stroked!("M5 12h14M12 5v14")),
    (
        "icons/external-link.svg",
        br##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><path d="M15 3h6v6"/><path d="M10 14 21 3"/><path d="M18 13v6a2 2 0 0 1-2 2H5a2 2 0 0 1-2-2V5a2 2 0 0 1 2-2h6"/></svg>"##,
    ),
    (
        "icons/bookmark.svg",
        br##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><path d="m19 21-7-4-7 4V5a2 2 0 0 1 2-2h10a2 2 0 0 1 2 2v16z"/></svg>"##,
    ),
    (
        "icons/bookmark-filled.svg",
        br##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="currentColor" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><path d="m19 21-7-4-7 4V5a2 2 0 0 1 2-2h10a2 2 0 0 1 2 2v16z"/></svg>"##,
    ),
    (
        "icons/italic.svg",
        br##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><line x1="19" x2="10" y1="4" y2="4"/><line x1="14" x2="5" y1="20" y2="20"/><line x1="15" x2="9" y1="4" y2="20"/></svg>"##,
    ),
    (
        "icons/bold.svg",
        br##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><path d="M6 12h9a4 4 0 0 1 0 8H7a1 1 0 0 1-1-1V5a1 1 0 0 1 1-1h7a4 4 0 0 1 0 8"/></svg>"##,
    ),
    (
        // The native textarea corner resizer: two diagonal grip lines.
        "icons/resize-grip.svg",
        br##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round"><path d="M21 15 15 21"/><path d="M21 9 9 21"/></svg>"##,
    ),
    (
        "icons/arrow-up-right.svg",
        br##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><path d="M7 7h10v10"/><path d="M7 17 17 7"/></svg>"##,
    ),
];

/// Embedded photos for the ScrollArea horizontal docs demo (the shadcn demo's
/// three Unsplash artworks, fetched at `w=300&q=80` like the docs page).
static PHOTOS: &[(&str, &[u8])] = &[
    (
        "photos/ornella-binni.jpg",
        include_bytes!("../assets/photos/ornella-binni.jpg"),
    ),
    (
        "photos/tom-byrom.jpg",
        include_bytes!("../assets/photos/tom-byrom.jpg"),
    ),
    (
        "photos/vladimir-malyavko.jpg",
        include_bytes!("../assets/photos/vladimir-malyavko.jpg"),
    ),
];

pub const PHOTO_ORNELLA_BINNI: &str = "photos/ornella-binni.jpg";
pub const PHOTO_TOM_BYROM: &str = "photos/tom-byrom.jpg";
pub const PHOTO_VLADIMIR_MALYAVKO: &str = "photos/vladimir-malyavko.jpg";

pub const ICON_CIRCLE_ALERT: &str = "icons/circle-alert.svg";
pub const ICON_LOADER: &str = "icons/loader.svg";
pub const ICON_ELLIPSIS: &str = "icons/ellipsis.svg";
pub const ICON_CIRCLE_CHECK: &str = "icons/circle-check.svg";
pub const ICON_SEARCH: &str = "icons/search.svg";
pub const ICON_SAVE: &str = "icons/save.svg";
pub const ICON_TOOLTIP_ARROW: &str = "icons/tooltip-arrow.svg";
pub const ICON_INFO: &str = "icons/info.svg";
pub const ICON_INBOX: &str = "icons/inbox.svg";
pub const ICON_SHIELD_ALERT: &str = "icons/shield-alert.svg";
pub const ICON_BADGE_CHECK: &str = "icons/badge-check.svg";
pub const ICON_PLUS: &str = "icons/plus.svg";
pub const ICON_EXTERNAL_LINK: &str = "icons/external-link.svg";
pub const ICON_BOOKMARK: &str = "icons/bookmark.svg";
pub const ICON_BOOKMARK_FILLED: &str = "icons/bookmark-filled.svg";
pub const ICON_ITALIC: &str = "icons/italic.svg";
pub const ICON_BOLD: &str = "icons/bold.svg";
pub const ICON_ARROW_UP_RIGHT: &str = "icons/arrow-up-right.svg";
pub const ICON_RESIZE_GRIP: &str = "icons/resize-grip.svg";

/// Embedded photos for the storybook docs examples (the same images the
/// shadcn docs pages use, checked into `assets/images/`, normalized to
/// baseline PNG).
const IMAGES: &[(&str, &[u8])] = &[
    (
        "images/avatar-shadcn.png",
        include_bytes!("../assets/images/avatar-shadcn.png"),
    ),
    (
        "images/avatar-maxleiter.png",
        include_bytes!("../assets/images/avatar-maxleiter.png"),
    ),
    (
        "images/avatar-evilrabbit.png",
        include_bytes!("../assets/images/avatar-evilrabbit.png"),
    ),
    (
        "images/avatar-pranathip.png",
        include_bytes!("../assets/images/avatar-pranathip.png"),
    ),
    (
        "images/tile-midnight.png",
        include_bytes!("../assets/images/tile-midnight.png"),
    ),
    (
        "images/tile-coffee.png",
        include_bytes!("../assets/images/tile-coffee.png"),
    ),
    (
        "images/tile-digital.png",
        include_bytes!("../assets/images/tile-digital.png"),
    ),
    (
        "images/header-v0-sm.png",
        include_bytes!("../assets/images/header-v0-sm.png"),
    ),
    (
        "images/header-v0-lg.png",
        include_bytes!("../assets/images/header-v0-lg.png"),
    ),
    (
        "images/header-v0-mini.png",
        include_bytes!("../assets/images/header-v0-mini.png"),
    ),
];

pub const IMAGE_AVATAR_SHADCN: &str = "images/avatar-shadcn.png";
pub const IMAGE_AVATAR_MAXLEITER: &str = "images/avatar-maxleiter.png";
pub const IMAGE_AVATAR_EVILRABBIT: &str = "images/avatar-evilrabbit.png";
pub const IMAGE_AVATAR_PRANATHIP: &str = "images/avatar-pranathip.png";
pub const IMAGE_TILE_MIDNIGHT: &str = "images/tile-midnight.png";
pub const IMAGE_TILE_COFFEE: &str = "images/tile-coffee.png";
pub const IMAGE_TILE_DIGITAL: &str = "images/tile-digital.png";
pub const IMAGE_HEADER_V0_SM: &str = "images/header-v0-sm.png";
pub const IMAGE_HEADER_V0_LG: &str = "images/header-v0-lg.png";
pub const IMAGE_HEADER_V0_MINI: &str = "images/header-v0-mini.png";

pub struct Assets;

impl AssetSource for Assets {
    fn load(&self, path: &str) -> Result<Option<Cow<'static, [u8]>>> {
        Ok(ICONS
            .iter()
            .chain(IMAGES.iter())
            .chain(PHOTOS.iter())
            .find(|(name, _)| *name == path)
            .map(|(_, bytes)| Cow::Borrowed(*bytes)))
    }

    fn list(&self, path: &str) -> Result<Vec<SharedString>> {
        Ok(ICONS
            .iter()
            .chain(IMAGES.iter())
            .chain(PHOTOS.iter())
            .filter(|(name, _)| name.starts_with(path))
            .map(|(name, _)| SharedString::from(*name))
            .collect())
    }
}
