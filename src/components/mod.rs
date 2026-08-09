//! rcn components — ports of shadcn base-vega components to gpui.
//!
//! Copy-paste friendly: each module is self-contained apart from the shared
//! [`crate::theme`] tokens (and [`crate::assets`] icons where noted).

pub mod accordion;
pub mod alert;
pub mod aspect_ratio;
pub mod avatar;
pub mod badge;
pub mod breadcrumb;
pub mod button;
pub mod card;
pub mod checkbox;
pub mod empty;
pub mod item;
pub mod kbd;
pub mod label;
pub mod popover;
pub mod progress;
pub mod separator;
pub mod skeleton;
pub mod spinner;
pub mod switch;
pub mod table;

pub use accordion::{Accordion, AccordionItem};
pub use alert::{Alert, AlertDescription, AlertTitle, AlertVariant};
pub use aspect_ratio::AspectRatio;
pub use avatar::{Avatar, AvatarGroup, AvatarGroupCount, AvatarSize};
pub use badge::{Badge, BadgeVariant};
pub use breadcrumb::{
    Breadcrumb, BreadcrumbEllipsis, BreadcrumbItem, BreadcrumbLink, BreadcrumbList, BreadcrumbPage,
    BreadcrumbSeparator,
};
pub use button::{Button, ButtonSize, ButtonVariant};
pub use card::{
    Card, CardAction, CardContent, CardDescription, CardFooter, CardHeader, CardSize, CardTitle,
};
pub use checkbox::Checkbox;
pub use empty::{
    Empty, EmptyContent, EmptyDescription, EmptyHeader, EmptyMedia, EmptyMediaVariant, EmptyTitle,
};
pub use item::{
    Item, ItemActions, ItemContent, ItemDescription, ItemFooter, ItemGroup, ItemHeader, ItemMedia,
    ItemMediaVariant, ItemSeparator, ItemSize, ItemTitle, ItemVariant,
};
pub use kbd::{Kbd, KbdGroup};
pub use label::Label;
pub use popover::{Popover, PopoverDescription, PopoverHeader, PopoverTitle};
pub use progress::Progress;
pub use separator::Separator;
pub use skeleton::Skeleton;
pub use spinner::Spinner;
pub use switch::{Switch, SwitchSize};
pub use table::{
    Table, TableBody, TableCaption, TableCell, TableFooter, TableHead, TableHeader, TableRow,
};
