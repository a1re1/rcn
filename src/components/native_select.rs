//! NativeSelect — port of shadcn base-vega `ui/native-select.tsx`.
//!
//! Upstream this styles the platform `<select>` element. gpui has no
//! OS-native dropdown to defer to, so `NativeSelect` is the [`Select`]
//! picker under the native-select name — same input-styled trigger and
//! option list, one implementation to maintain.

pub use crate::components::select::Select as NativeSelect;
