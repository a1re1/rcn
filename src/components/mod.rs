//! rcn components — ports of shadcn base-vega components to gpui.
//!
//! Copy-paste friendly: each module is self-contained apart from the shared
//! [`crate::theme`] tokens (and [`crate::assets`] icons where noted).

pub mod accordion;
pub mod alert;
pub mod alert_dialog;
pub mod aspect_ratio;
pub mod attachment;
pub mod avatar;
pub mod badge;
pub mod breadcrumb;
pub mod bubble;
pub mod button;
pub mod button_group;
pub mod calendar;
pub mod card;
pub mod carousel;
pub mod chart;
pub mod checkbox;
pub mod collapsible;
pub mod combobox;
pub mod command;
pub mod context_menu;
pub mod date_picker;
pub mod dialog;
pub mod drawer;
pub mod dropdown_menu;
pub mod empty;
pub mod field;
pub mod hover_card;
pub mod icon;
pub mod input;
pub mod input_group;
pub mod input_otp;
pub mod item;
pub mod kbd;
pub mod label;
pub mod marker;
pub mod menubar;
pub mod message;
pub mod message_scroller;
pub mod native_select;
pub mod navigation_menu;
pub mod pagination;
pub mod popover;
pub mod progress;
pub mod questionnaire;
pub mod radio_group;
pub mod resizable;
pub mod scroll_area;
pub mod select;
pub mod separator;
pub mod sheet;
pub mod sidebar;
pub mod skeleton;
pub mod slider;
pub mod spinner;
pub mod switch;
pub mod table;
pub mod tabs;
pub mod textarea;
pub mod toast;
pub mod toggle;
pub mod toggle_group;
pub mod tooltip;

pub use accordion::{Accordion, AccordionContent, AccordionItem, AccordionTrigger};
pub use alert::{Alert, AlertDescription, AlertTitle, AlertVariant};
pub use alert_dialog::{
    AlertDialog, AlertDialogDescription, AlertDialogFooter, AlertDialogHeader, AlertDialogTitle,
};
pub use aspect_ratio::AspectRatio;
pub use attachment::{Attachment, AttachmentState};
pub use avatar::{Avatar, AvatarBadge, AvatarGroup, AvatarGroupCount, AvatarSize};
pub use badge::{Badge, BadgeVariant};
pub use breadcrumb::{
    Breadcrumb, BreadcrumbEllipsis, BreadcrumbItem, BreadcrumbLink, BreadcrumbList, BreadcrumbPage,
    BreadcrumbSeparator,
};
pub use bubble::{Bubble, BubbleAlign, BubbleGroup, BubbleReactions, BubbleSide, BubbleVariant};
pub use button::{Button, ButtonSize, ButtonVariant};
pub use button_group::{ButtonGroup, ButtonGroupSeparator, ButtonGroupText};
pub use calendar::{Calendar, CalendarDate};
pub use card::{Card, CardContent, CardDescription, CardFooter, CardHeader, CardSize, CardTitle};
// Standalone action part for manually composed headers (shadcn API parity);
// the storybook goes through CardHeader::action, which wraps it.
#[allow(unused_imports)]
pub use card::CardAction;
pub use carousel::Carousel;
pub use chart::{BarChart, ChartSeries};
pub use checkbox::Checkbox;
pub use collapsible::Collapsible;
pub use combobox::Combobox;
pub use command::{Command, CommandGroup, CommandItem};
pub use context_menu::{ContextMenu, ContextMenuItem};
pub use date_picker::DatePicker;
pub use dialog::{Dialog, DialogDescription, DialogFooter, DialogHeader, DialogTitle};
pub use drawer::{Drawer, DrawerDescription, DrawerFooter, DrawerHeader, DrawerTitle};
pub use dropdown_menu::{DropdownMenu, DropdownMenuItem};
pub use empty::{
    Empty, EmptyContent, EmptyDescription, EmptyHeader, EmptyMedia, EmptyMediaVariant, EmptyTitle,
};
pub use field::{
    Field, FieldContent, FieldDescription, FieldError, FieldGroup, FieldLabel, FieldLegend,
    FieldLegendVariant, FieldOrientation, FieldSeparator, FieldSet, FieldTitle,
};
pub use hover_card::HoverCard;
pub use icon::Icon;
pub use input::Input;
pub use input_group::{InputGroup, InputGroupAddon};
pub use input_otp::InputOtp;
pub use item::{
    Item, ItemActions, ItemContent, ItemDescription, ItemGroup, ItemHeader, ItemMedia,
    ItemMediaVariant, ItemSize, ItemTitle, ItemVariant,
};
// Public API surface matching shadcn `ItemFooter`/`ItemSeparator` (API
// reference only); the current docs examples don't use them.
#[allow(unused_imports)]
pub use item::{ItemFooter, ItemSeparator};
pub use kbd::{Kbd, KbdGroup};
pub use label::Label;
pub use marker::{Marker, MarkerVariant};
pub use menubar::{Menubar, MenubarItem, MenubarMenu};
pub use message::{
    Message, MessageAlign, MessageAvatar, MessageContent, MessageFooter, MessageGroup,
    MessageHeader,
};
pub use message_scroller::MessageScroller;
pub use native_select::NativeSelect;
pub use navigation_menu::{NavigationMenu, NavigationMenuEntry, NavigationMenuLink};
pub use pagination::{
    Pagination, PaginationContent, PaginationEllipsis, PaginationItem, PaginationLink,
    PaginationNext, PaginationPrevious,
};
pub use popover::{Popover, PopoverDescription, PopoverHeader, PopoverTitle};
pub use progress::Progress;
pub use questionnaire::{
    Questionnaire, QuestionnaireActions, QuestionnaireChoice, QuestionnaireChoices,
    QuestionnaireDescription, QuestionnaireProgress, QuestionnaireTitle,
};
pub use radio_group::{RadioGroup, RadioGroupItem};
pub use resizable::{ResizableDirection, ResizableHandle, ResizablePanel, ResizablePanelGroup};
pub use scroll_area::ScrollArea;
pub use select::Select;
pub use separator::Separator;
pub use sheet::{Sheet, SheetDescription, SheetFooter, SheetHeader, SheetSide, SheetTitle};
pub use sidebar::{
    Sidebar, SidebarContent, SidebarFooter, SidebarGroup, SidebarHeader, SidebarMenuButton,
    SidebarProvider, SidebarTrigger,
};
pub use skeleton::Skeleton;
pub use slider::Slider;
pub use spinner::Spinner;
pub use switch::{Switch, SwitchSize};
pub use table::{
    Table, TableBody, TableCaption, TableCell, TableFooter, TableHead, TableHeader, TableRow,
};
pub use tabs::{Tabs, TabsContent, TabsList, TabsTrigger, TabsVariant};
pub use textarea::Textarea;
pub use toast::{Toast, ToastViewport};
pub use toggle::{Toggle, ToggleSize, ToggleVariant};
pub use toggle_group::{ToggleGroup, ToggleGroupItem};
pub use tooltip::Tooltip;
