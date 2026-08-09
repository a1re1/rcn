//! rcn components — ports of shadcn base-vega components to gpui.
//!
//! Copy-paste friendly: each module is self-contained apart from the shared
//! [`crate::theme`] tokens (and [`crate::assets`] icons where noted).

pub mod accordion;
pub mod alert;
pub mod alert_dialog;
pub mod aspect_ratio;
pub mod avatar;
pub mod badge;
pub mod breadcrumb;
pub mod button;
pub mod button_group;
pub mod card;
pub mod checkbox;
pub mod collapsible;
pub mod dialog;
pub mod empty;
pub mod hover_card;
pub mod item;
pub mod kbd;
pub mod label;
pub mod pagination;
pub mod popover;
pub mod progress;
pub mod radio_group;
pub mod scroll_area;
pub mod separator;
pub mod skeleton;
pub mod slider;
pub mod spinner;
pub mod switch;
pub mod table;
pub mod tabs;
pub mod toggle;
pub mod toggle_group;
pub mod tooltip;

pub use accordion::{Accordion, AccordionItem};
pub use alert::{Alert, AlertDescription, AlertTitle, AlertVariant};
pub use alert_dialog::{
    AlertDialog, AlertDialogDescription, AlertDialogFooter, AlertDialogHeader, AlertDialogTitle,
};
pub use aspect_ratio::AspectRatio;
pub use avatar::{Avatar, AvatarGroup, AvatarGroupCount, AvatarSize};
pub use badge::{Badge, BadgeVariant};
pub use breadcrumb::{
    Breadcrumb, BreadcrumbEllipsis, BreadcrumbItem, BreadcrumbLink, BreadcrumbList, BreadcrumbPage,
    BreadcrumbSeparator,
};
pub use button::{Button, ButtonSize, ButtonVariant};
pub use button_group::{ButtonGroup, ButtonGroupSeparator, ButtonGroupText};
pub use card::{
    Card, CardAction, CardContent, CardDescription, CardFooter, CardHeader, CardSize, CardTitle,
};
pub use checkbox::Checkbox;
pub use collapsible::Collapsible;
pub use dialog::{Dialog, DialogDescription, DialogFooter, DialogHeader, DialogTitle};
pub use empty::{
    Empty, EmptyContent, EmptyDescription, EmptyHeader, EmptyMedia, EmptyMediaVariant, EmptyTitle,
};
pub use hover_card::HoverCard;
pub use item::{
    Item, ItemActions, ItemContent, ItemDescription, ItemFooter, ItemGroup, ItemHeader, ItemMedia,
    ItemMediaVariant, ItemSeparator, ItemSize, ItemTitle, ItemVariant,
};
pub use kbd::{Kbd, KbdGroup};
pub use label::Label;
pub use pagination::{
    Pagination, PaginationEllipsis, PaginationLink, PaginationNext, PaginationPrevious,
};
pub use popover::{Popover, PopoverDescription, PopoverHeader, PopoverTitle};
pub use progress::Progress;
pub use radio_group::{RadioGroup, RadioGroupItem};
pub use scroll_area::ScrollArea;
pub use separator::Separator;
pub use skeleton::Skeleton;
pub use slider::Slider;
pub use spinner::Spinner;
pub use switch::{Switch, SwitchSize};
pub use table::{
    Table, TableBody, TableCaption, TableCell, TableFooter, TableHead, TableHeader, TableRow,
};
pub use tabs::{Tabs, TabsContent, TabsList, TabsTrigger, TabsVariant};
pub use toggle::{Toggle, ToggleSize, ToggleVariant};
pub use toggle_group::{ToggleGroup, ToggleGroupItem};
pub use tooltip::Tooltip;
