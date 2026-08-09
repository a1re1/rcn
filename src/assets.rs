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
];

pub struct Assets;

impl AssetSource for Assets {
    fn load(&self, path: &str) -> Result<Option<Cow<'static, [u8]>>> {
        Ok(ICONS
            .iter()
            .find(|(name, _)| *name == path)
            .map(|(_, bytes)| Cow::Borrowed(*bytes)))
    }

    fn list(&self, path: &str) -> Result<Vec<SharedString>> {
        Ok(ICONS
            .iter()
            .filter(|(name, _)| name.starts_with(path))
            .map(|(name, _)| SharedString::from(*name))
            .collect())
    }
}
