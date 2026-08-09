//! rcn components — ports of shadcn base-vega components to gpui.
//!
//! Copy-paste friendly: each module is self-contained apart from the shared
//! [`crate::theme`] tokens (and [`crate::assets`] icons where noted).

pub mod accordion;
pub mod avatar;
pub mod badge;
pub mod button;
pub mod popover;
pub mod separator;
pub mod switch;

pub use accordion::{Accordion, AccordionItem};
pub use avatar::{Avatar, AvatarGroup, AvatarGroupCount, AvatarSize};
pub use badge::{Badge, BadgeVariant};
pub use button::{Button, ButtonSize, ButtonVariant};
pub use popover::{Popover, PopoverDescription, PopoverHeader, PopoverTitle};
pub use separator::{Separator, SeparatorOrientation};
pub use switch::{Switch, SwitchSize};
