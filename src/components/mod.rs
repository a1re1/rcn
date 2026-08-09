//! rcn components — ports of shadcn base-vega components to gpui.
//!
//! Copy-paste friendly: each module is self-contained apart from the shared
//! [`crate::theme`] tokens (and [`crate::assets`] icons where noted).

pub mod accordion;
pub mod avatar;
pub mod badge;
pub mod button;
pub mod card;
pub mod kbd;
pub mod label;
pub mod popover;
pub mod separator;
pub mod skeleton;
pub mod switch;

#[allow(unused_imports)] // public API; storybook uses Separator::vertical()
use self::SeparatorOrientation as _;
pub use accordion::{Accordion, AccordionItem};
pub use avatar::{Avatar, AvatarGroup, AvatarGroupCount, AvatarSize};
pub use badge::{Badge, BadgeVariant};
pub use button::{Button, ButtonSize, ButtonVariant};
pub use card::{
    Card, CardAction, CardContent, CardDescription, CardFooter, CardHeader, CardSize, CardTitle,
};
pub use kbd::{Kbd, KbdGroup};
pub use label::Label;
pub use popover::{Popover, PopoverDescription, PopoverHeader, PopoverTitle};
pub use separator::{Separator, SeparatorOrientation}; // re-exported for callers
pub use skeleton::Skeleton;
pub use switch::{Switch, SwitchSize};
