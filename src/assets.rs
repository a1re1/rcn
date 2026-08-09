//! Embedded asset source: serves the handful of lucide icons the components
//! need (accordion chevrons, showcase examples) to gpui's `svg()` element.

use std::borrow::Cow;

use gpui::{AssetSource, Result, SharedString};

pub const ICON_CHEVRON_DOWN: &str = "icons/chevron-down.svg";
pub const ICON_CHEVRON_UP: &str = "icons/chevron-up.svg";
pub const ICON_CHEVRON_RIGHT: &str = "icons/chevron-right.svg";

const CHEVRON_DOWN: &[u8] = br##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><path d="m6 9 6 6 6-6"/></svg>"##;
const CHEVRON_UP: &[u8] = br##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><path d="m18 15-6-6-6 6"/></svg>"##;
const CHEVRON_RIGHT: &[u8] = br##"<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><path d="m9 18 6-6-6-6"/></svg>"##;

pub struct Assets;

impl AssetSource for Assets {
    fn load(&self, path: &str) -> Result<Option<Cow<'static, [u8]>>> {
        Ok(match path {
            ICON_CHEVRON_DOWN => Some(Cow::Borrowed(CHEVRON_DOWN)),
            ICON_CHEVRON_UP => Some(Cow::Borrowed(CHEVRON_UP)),
            ICON_CHEVRON_RIGHT => Some(Cow::Borrowed(CHEVRON_RIGHT)),
            _ => None,
        })
    }

    fn list(&self, path: &str) -> Result<Vec<SharedString>> {
        Ok([ICON_CHEVRON_DOWN, ICON_CHEVRON_UP, ICON_CHEVRON_RIGHT]
            .iter()
            .filter(|p| p.starts_with(path))
            .map(|p| SharedString::from(*p))
            .collect())
    }
}
