//! The rcn storybook shell: a three-pane component explorer. The sidebar
//! lists every component; the canvas renders the selected one in isolation;
//! the controls panel binds each of the component's props (variant, size,
//! state flags) to interactive widgets — built from the library's own Button
//! and Switch.
//!
//! The Tokens story is special: its controls are global, modeled on
//! ui.shadcn.com/create — pick a base gray family, a brand color (presets or
//! custom hue/saturation/lightness sliders), and a radius, or shuffle the
//! whole thing — and every other story picks the changes up live through the
//! `Theme` global.

use std::collections::HashMap;
use std::fs;
use std::time::{SystemTime, UNIX_EPOCH};

use serde::Deserialize;

use gpui::{
    AnyElement, App, ClickEvent, Context, DragMoveEvent, ElementId, FontWeight, Hsla, Window, div,
    hsla, prelude::*, px, relative, rgb,
};

use crate::assets::IconLibrary;
use crate::components::{
    Accordion, AccordionContent, AccordionItem, AccordionTrigger, Alert, AlertDescription,
    AlertDialog, AlertDialogDescription, AlertDialogFooter, AlertDialogHeader, AlertDialogTitle,
    AlertTitle, AlertVariant, AspectRatio, Attachment, AttachmentState, Avatar, AvatarGroup,
    AvatarGroupCount, AvatarSize, Badge, BadgeVariant, BarChart, Breadcrumb, BreadcrumbEllipsis,
    BreadcrumbItem, BreadcrumbLink, BreadcrumbList, BreadcrumbPage, BreadcrumbSeparator, Bubble,
    BubbleAlign, BubbleReactions, BubbleSide, BubbleVariant, Button, ButtonGroup,
    ButtonGroupSeparator, ButtonGroupText, ButtonSize, ButtonVariant, Calendar, CalendarDate, Card,
    CardContent, CardDescription, CardFooter, CardHeader, CardSize, CardTitle, Carousel,
    ChartSeries, Checkbox, Collapsible, Combobox, Command, CommandGroup, CommandItem, ContextMenu,
    ContextMenuItem, DatePicker, Dialog, DialogDescription, DialogFooter, DialogHeader,
    DialogTitle, Drawer, DrawerDescription, DrawerFooter, DrawerHeader, DrawerTitle, DropdownMenu,
    DropdownMenuItem, Empty, EmptyContent, EmptyDescription, EmptyHeader, EmptyMedia,
    EmptyMediaVariant, EmptyTitle, Field, FieldContent, FieldDescription, FieldError, FieldGroup,
    FieldLabel, FieldLegend, FieldLegendVariant, FieldOrientation, FieldSeparator, FieldSet,
    FieldTitle, HoverCard, Icon, Input, InputGroup, InputGroupAddon, InputOtp, Item, ItemActions,
    ItemContent, ItemDescription, ItemFooter, ItemGroup, ItemHeader, ItemMedia, ItemMediaVariant,
    ItemSeparator, ItemSize, ItemTitle, ItemVariant, Kbd, KbdGroup, Label, Marker, MarkerVariant,
    Menubar, MenubarItem, MenubarMenu, Message, MessageAlign, MessageAvatar, MessageContent,
    MessageFooter, MessageGroup, MessageHeader, MessageScroller, NativeSelect, NavigationMenu,
    NavigationMenuEntry, NavigationMenuLink, Pagination, PaginationEllipsis, PaginationLink,
    PaginationNext, PaginationPrevious, Popover, PopoverDescription, PopoverHeader, PopoverTitle,
    Progress, Questionnaire, QuestionnaireActions, QuestionnaireChoice, QuestionnaireChoices,
    QuestionnaireDescription, QuestionnaireProgress, QuestionnaireTitle, RadioGroup,
    RadioGroupItem, ResizableDirection, ResizableHandle, ResizablePanel, ResizablePanelGroup,
    ScrollArea, Select, Separator, Sheet, SheetDescription, SheetFooter, SheetHeader, SheetSide,
    SheetTitle, Sidebar, SidebarContent, SidebarFooter, SidebarGroup, SidebarHeader,
    SidebarMenuButton, SidebarProvider, SidebarTrigger, Skeleton, Slider, Spinner, Switch,
    SwitchSize, Table, TableBody, TableCaption, TableCell, TableFooter, TableHead, TableHeader,
    TableRow, Tabs, TabsContent, TabsList, TabsTrigger, TabsVariant, Textarea, Toast,
    ToastViewport, Toggle, ToggleGroup, ToggleGroupItem, ToggleSize, ToggleVariant, Tooltip,
};
use crate::theme::{BaseColor, Theme, alpha, oklch};

// Child module so showcase can call Storybook's private helpers (token_controls, etc.).
#[path = "storybook_showcase.rs"]
mod showcase;

#[derive(Clone, Copy, PartialEq, Eq)]
enum Story {
    Tokens,
    Button,
    Badge,
    Avatar,
    Switch,
    Accordion,
    Popover,
    Separator,
    Skeleton,
    Label,
    Kbd,
    Card,
    Alert,
    Progress,
    Spinner,
    AspectRatio,
    Empty,
    Item,
    Table,
    Breadcrumb,
    Checkbox,
    RadioGroup,
    Toggle,
    ToggleGroup,
    ButtonGroup,
    Collapsible,
    Tabs,
    SliderStory,
    PaginationStory,
    ScrollArea,
    TooltipStory,
    HoverCardStory,
    DialogStory,
    AlertDialogStory,
    SheetStory,
    DrawerStory,
    DropdownMenuStory,
    ContextMenuStory,
    MenubarStory,
    SelectStory,
    NativeSelectStory,
    NavigationMenuStory,
    ToastStory,
    InputStory,
    TextareaStory,
    FieldStory,
    InputGroupStory,
    InputOtpStory,
    CommandStory,
    ComboboxStory,
    CalendarStory,
    DatePickerStory,
    CarouselStory,
    ResizableStory,
    SidebarStory,
    DataTableStory,
    ChartStory,
    MessageStory,
    BubbleStory,
    AttachmentStory,
    MarkerStory,
    MessageScrollerStory,
    QuestionnaireStory,
    // __STORY_VARIANTS__
}

impl Story {
    const ALL: [Story; 63] = [
        Story::Tokens,
        Story::Button,
        Story::Badge,
        Story::Avatar,
        Story::Switch,
        Story::Accordion,
        Story::Popover,
        Story::Separator,
        Story::Skeleton,
        Story::Label,
        Story::Kbd,
        Story::Card,
        Story::Alert,
        Story::Progress,
        Story::Spinner,
        Story::AspectRatio,
        Story::Empty,
        Story::Item,
        Story::Table,
        Story::Breadcrumb,
        Story::Checkbox,
        Story::RadioGroup,
        Story::Toggle,
        Story::ToggleGroup,
        Story::ButtonGroup,
        Story::Collapsible,
        Story::Tabs,
        Story::SliderStory,
        Story::PaginationStory,
        Story::ScrollArea,
        Story::TooltipStory,
        Story::HoverCardStory,
        Story::DialogStory,
        Story::AlertDialogStory,
        Story::SheetStory,
        Story::DrawerStory,
        Story::DropdownMenuStory,
        Story::ContextMenuStory,
        Story::MenubarStory,
        Story::SelectStory,
        Story::NativeSelectStory,
        Story::NavigationMenuStory,
        Story::ToastStory,
        Story::InputStory,
        Story::TextareaStory,
        Story::FieldStory,
        Story::InputGroupStory,
        Story::InputOtpStory,
        Story::CommandStory,
        Story::ComboboxStory,
        Story::CalendarStory,
        Story::DatePickerStory,
        Story::CarouselStory,
        Story::ResizableStory,
        Story::SidebarStory,
        Story::DataTableStory,
        Story::ChartStory,
        Story::MessageStory,
        Story::BubbleStory,
        Story::AttachmentStory,
        Story::MarkerStory,
        Story::MessageScrollerStory,
        Story::QuestionnaireStory,
        // __STORY_ALL__
    ];

    fn label(self) -> &'static str {
        match self {
            Story::Tokens => "Tokens",
            Story::Button => "Button",
            Story::Badge => "Badge",
            Story::Avatar => "Avatar",
            Story::Switch => "Switch",
            Story::Accordion => "Accordion",
            Story::Popover => "Popover",
            Story::Separator => "Separator",
            Story::Skeleton => "Skeleton",
            Story::Label => "Label",
            Story::Kbd => "Kbd",
            Story::Card => "Card",
            Story::Alert => "Alert",
            Story::Progress => "Progress",
            Story::Spinner => "Spinner",
            Story::AspectRatio => "Aspect Ratio",
            Story::Empty => "Empty",
            Story::Item => "Item",
            Story::Table => "Table",
            Story::Breadcrumb => "Breadcrumb",
            Story::Checkbox => "Checkbox",
            Story::RadioGroup => "Radio Group",
            Story::Toggle => "Toggle",
            Story::ToggleGroup => "Toggle Group",
            Story::ButtonGroup => "Button Group",
            Story::Collapsible => "Collapsible",
            Story::Tabs => "Tabs",
            Story::SliderStory => "Slider",
            Story::PaginationStory => "Pagination",
            Story::ScrollArea => "Scroll Area",
            Story::TooltipStory => "Tooltip",
            Story::HoverCardStory => "Hover Card",
            Story::DialogStory => "Dialog",
            Story::AlertDialogStory => "Alert Dialog",
            Story::SheetStory => "Sheet",
            Story::DrawerStory => "Drawer",
            Story::DropdownMenuStory => "Dropdown Menu",
            Story::ContextMenuStory => "Context Menu",
            Story::MenubarStory => "Menubar",
            Story::SelectStory => "Select",
            Story::NativeSelectStory => "Native Select",
            Story::NavigationMenuStory => "Navigation Menu",
            Story::ToastStory => "Toast",
            Story::InputStory => "Input",
            Story::TextareaStory => "Textarea",
            Story::FieldStory => "Field",
            Story::InputGroupStory => "Input Group",
            Story::InputOtpStory => "Input OTP",
            Story::CommandStory => "Command",
            Story::ComboboxStory => "Combobox",
            Story::CalendarStory => "Calendar",
            Story::DatePickerStory => "Date Picker",
            Story::CarouselStory => "Carousel",
            Story::ResizableStory => "Resizable",
            Story::SidebarStory => "Sidebar",
            Story::DataTableStory => "Data Table",
            Story::ChartStory => "Chart",
            Story::MessageStory => "Message",
            Story::BubbleStory => "Bubble",
            Story::AttachmentStory => "Attachment",
            Story::MarkerStory => "Marker",
            Story::MessageScrollerStory => "Message Scroller",
            Story::QuestionnaireStory => "Questionnaire",
            // __STORY_LABELS__
        }
    }

    fn description(self) -> &'static str {
        match self {
            Story::Tokens => {
                "Global design tokens. Tune the base palette, brand color, and radius — or \
                 shuffle — and every component picks the changes up live."
            }
            Story::Button => "Displays a button or a component that looks like a button.",
            Story::Badge => "Displays a badge or a component that looks like a badge.",
            Story::Avatar => "An image element with a fallback for representing the user.",
            Story::Switch => {
                "A control that allows the user to toggle between checked and not checked."
            }
            Story::Accordion => {
                "A vertically stacked set of interactive headings that each reveal a section of content."
            }
            Story::Popover => "Displays rich content in a portal, triggered by a button.",
            Story::Separator => "Visually or semantically separates content.",
            Story::Skeleton => "Use to show a placeholder while content is loading.",
            Story::Label => "Renders an accessible label associated with controls.",
            Story::Kbd => "Used to display textual user input from keyboard.",
            Story::Card => "Displays a card with header, content, and footer.",
            Story::Alert => "Displays a callout for user attention.",
            Story::Progress => "Displays an indicator showing the completion progress of a task.",
            Story::Spinner => "An indicator that can be used to show a loading state.",
            Story::AspectRatio => "Displays content within a desired ratio.",
            Story::Empty => "Use to display an empty state, such as no results or missing data.",
            Story::Item => "A flexible list row with media, content, and actions.",
            Story::Table => "A responsive table component.",
            Story::Breadcrumb => {
                "Displays the path to the current resource using a hierarchy of links."
            }
            Story::Checkbox => {
                "A control that allows the user to toggle between checked and not checked."
            }
            Story::RadioGroup => {
                "A set of checkable buttons where only one can be checked at a time."
            }
            Story::Toggle => "A two-state button that can be either on or off.",
            Story::ToggleGroup => "A set of two-state buttons that can be toggled on or off.",
            Story::ButtonGroup => {
                "A container that groups related buttons together with a consistent style."
            }
            Story::Collapsible => "An interactive component which expands and collapses a panel.",
            Story::Tabs => "A set of layered sections of content displayed one at a time.",
            Story::SliderStory => {
                "An input where the user selects a value from within a given range."
            }
            Story::PaginationStory => "Pagination with page navigation, next and previous links.",
            Story::ScrollArea => {
                "Augments native scroll functionality for custom, cross-browser styling."
            }
            Story::TooltipStory => {
                "A popup that displays information related to an element on hover."
            }
            Story::HoverCardStory => {
                "For sighted users to preview content available behind a link."
            }
            Story::DialogStory => {
                "A window overlaid on the primary window, rendering the content underneath inert."
            }
            Story::AlertDialogStory => {
                "A modal dialog that interrupts the user with important content and expects a response."
            }
            Story::SheetStory => {
                "Extends the Dialog component to display content that complements the main content of the screen."
            }
            Story::DrawerStory => {
                "A drawer component for rendering content from the bottom of the screen."
            }
            Story::DropdownMenuStory => "Displays a menu to the user, triggered by a button.",
            Story::ContextMenuStory => {
                "Displays a menu located at the pointer, triggered by a right click."
            }
            Story::MenubarStory => "A visually persistent menu common in desktop applications.",
            Story::SelectStory => {
                "Displays a list of options for the user to pick from, triggered by a button."
            }
            Story::NativeSelectStory => {
                "A native select element for choosing from a list of options."
            }
            Story::NavigationMenuStory => "A collection of links for navigating websites.",
            Story::ToastStory => "A succinct message that is displayed temporarily.",
            Story::InputStory => "Displays a form input field.",
            Story::TextareaStory => {
                "Displays a form textarea or a component that looks like a textarea."
            }
            Story::FieldStory => {
                "Combine labels, controls, and help text to compose accessible form fields."
            }
            Story::InputGroupStory => {
                "Display additional information or actions to an input or textarea."
            }
            Story::InputOtpStory => {
                "Accessible one-time password component with copy paste functionality."
            }
            Story::CommandStory => "Fast, composable, unstyled command menu.",
            Story::ComboboxStory => {
                "Autocomplete input and command palette with a list of suggestions."
            }
            Story::CalendarStory => {
                "A date field component that allows users to enter and edit date."
            }
            Story::DatePickerStory => "A date picker component with range and presets.",
            Story::CarouselStory => "A carousel with motion and swipe built using Embla.",
            Story::ResizableStory => {
                "Accessible resizable panel groups and layouts with keyboard support."
            }
            Story::SidebarStory => "A composable, themeable and customizable sidebar component.",
            Story::DataTableStory => "Powerful table and datagrids built on the table primitives.",
            Story::ChartStory => "Beautiful charts built with the theme's chart tokens.",
            Story::MessageStory => "A chat message row with avatar, content, and meta rows.",
            Story::BubbleStory => "Chat bubbles with variants and floating reactions.",
            Story::AttachmentStory => {
                "A file chip with media, metadata, upload states, and removal."
            }
            Story::MarkerStory => "Inline conversation markers like date dividers and event notes.",
            Story::MessageScrollerStory => {
                "A scrollable conversation viewport for message threads."
            }
            Story::QuestionnaireStory => "A form-flow step with progress, choices, and actions.",
            // __STORY_DESCRIPTIONS__
        }
    }

    /// The component module backing this story (drives the docs sections);
    /// `None` for pages without a single backing module.
    fn module(self) -> Option<&'static str> {
        Some(match self {
            Story::Tokens => return None,
            Story::Button => "button",
            Story::Badge => "badge",
            Story::Avatar => "avatar",
            Story::Switch => "switch",
            Story::Accordion => "accordion",
            Story::Popover => "popover",
            Story::Separator => "separator",
            Story::Skeleton => "skeleton",
            Story::Label => "label",
            Story::Kbd => "kbd",
            Story::Card => "card",
            Story::Alert => "alert",
            Story::Progress => "progress",
            Story::Spinner => "spinner",
            Story::AspectRatio => "aspect_ratio",
            Story::Empty => "empty",
            Story::Item => "item",
            Story::Table => "table",
            Story::Breadcrumb => "breadcrumb",
            Story::Checkbox => "checkbox",
            Story::RadioGroup => "radio_group",
            Story::Toggle => "toggle",
            Story::ToggleGroup => "toggle_group",
            Story::ButtonGroup => "button_group",
            Story::Collapsible => "collapsible",
            Story::Tabs => "tabs",
            Story::SliderStory => "slider",
            Story::PaginationStory => "pagination",
            Story::ScrollArea => "scroll_area",
            Story::TooltipStory => "tooltip",
            Story::HoverCardStory => "hover_card",
            Story::DialogStory => "dialog",
            Story::AlertDialogStory => "alert_dialog",
            Story::SheetStory => "sheet",
            Story::DrawerStory => "drawer",
            Story::DropdownMenuStory => "dropdown_menu",
            Story::ContextMenuStory => "context_menu",
            Story::MenubarStory => "menubar",
            Story::SelectStory => "select",
            Story::NativeSelectStory => "native_select",
            Story::NavigationMenuStory => "navigation_menu",
            Story::ToastStory => "toast",
            Story::InputStory => "input",
            Story::TextareaStory => "textarea",
            Story::FieldStory => "field",
            Story::InputGroupStory => "input_group",
            Story::InputOtpStory => "input_otp",
            Story::CommandStory => "command",
            Story::ComboboxStory => "combobox",
            Story::CalendarStory => "calendar",
            Story::DatePickerStory => "date_picker",
            Story::CarouselStory => "carousel",
            Story::ResizableStory => "resizable",
            Story::SidebarStory => "sidebar",
            Story::DataTableStory => return None,
            Story::ChartStory => "chart",
            Story::MessageStory => "message",
            Story::BubbleStory => "bubble",
            Story::MessageScrollerStory => "message_scroller",
            Story::AttachmentStory => "attachment",
            Story::QuestionnaireStory => "questionnaire",
            Story::MarkerStory => "marker",
        })
    }
}

const BUTTON_VARIANTS: [(&str, ButtonVariant); 6] = [
    ("default", ButtonVariant::Default),
    ("outline", ButtonVariant::Outline),
    ("secondary", ButtonVariant::Secondary),
    ("ghost", ButtonVariant::Ghost),
    ("destructive", ButtonVariant::Destructive),
    ("link", ButtonVariant::Link),
];

const BUTTON_SIZES: [(&str, ButtonSize); 8] = [
    ("xs", ButtonSize::Xs),
    ("sm", ButtonSize::Sm),
    ("default", ButtonSize::Default),
    ("lg", ButtonSize::Lg),
    ("icon-xs", ButtonSize::IconXs),
    ("icon-sm", ButtonSize::IconSm),
    ("icon", ButtonSize::Icon),
    ("icon-lg", ButtonSize::IconLg),
];

const BADGE_VARIANTS: [(&str, BadgeVariant); 6] = [
    ("default", BadgeVariant::Default),
    ("secondary", BadgeVariant::Secondary),
    ("destructive", BadgeVariant::Destructive),
    ("outline", BadgeVariant::Outline),
    ("ghost", BadgeVariant::Ghost),
    ("link", BadgeVariant::Link),
];

const AVATAR_SIZES: [(&str, AvatarSize); 3] = [
    ("sm", AvatarSize::Sm),
    ("default", AvatarSize::Default),
    ("lg", AvatarSize::Lg),
];

const SWITCH_SIZES: [(&str, SwitchSize); 2] =
    [("sm", SwitchSize::Sm), ("default", SwitchSize::Default)];

const CARD_SIZES: [(&str, CardSize); 2] = [("default", CardSize::Default), ("sm", CardSize::Sm)];

/// Font choices: fonts bundled with macOS, so every pick resolves. `None`
/// is gpui's default UI font.
const FONTS: [(&str, Option<&str>); 8] = [
    ("system", None),
    ("Helvetica Neue", Some("Helvetica Neue")),
    ("Avenir Next", Some("Avenir Next")),
    ("Futura", Some("Futura")),
    ("Gill Sans", Some("Gill Sans")),
    ("Georgia", Some("Georgia")),
    ("Palatino", Some("Palatino")),
    ("Menlo", Some("Menlo")),
];

/// Theme presets — shadcn create's "Theme" picker — as oklch (l, c, h) at
/// the light-mode anchor (tailwind's 600-ish step). `None` is the neutral
/// default (black / near-white).
const THEME_PRESETS: [(&str, Option<(f32, f32, f32)>); 8] = [
    ("default", None),
    ("blue", Some((0.546, 0.245, 262.9))),
    ("green", Some((0.627, 0.194, 149.2))),
    ("orange", Some((0.646, 0.222, 41.1))),
    ("red", Some((0.577, 0.245, 27.3))),
    ("rose", Some((0.586, 0.253, 17.6))),
    ("violet", Some((0.541, 0.281, 293.0))),
    ("yellow", Some((0.795, 0.184, 86.0))),
];

/// The global token adjustments layered over the stock shadcn themes.
struct TokenSettings {
    base: BaseColor,
    /// Body font family; `None` is gpui's default.
    font_sans: Option<&'static str>,
    /// Heading font family; `None` falls back to the body font.
    font_heading: Option<&'static str>,
    icons: IconLibrary,
    /// When false, primary stays the stock neutral (black / near-white).
    custom_primary: bool,
    /// Custom brand color in HSL, each 0..1 (gpui's `Hsla` space).
    hue: f32,
    saturation: f32,
    lightness: f32,
    radius: f32,
}

impl Default for TokenSettings {
    fn default() -> Self {
        Self {
            base: BaseColor::Neutral,
            font_sans: None,
            font_heading: None,
            icons: IconLibrary::default(),
            custom_primary: false,
            hue: 0.6,
            saturation: 0.7,
            lightness: 0.5,
            radius: 10.,
        }
    }
}

impl TokenSettings {
    /// The brand primary for the given mode, if customized. Dark mode lifts
    /// the lightness a step, like shadcn's dark palettes do.
    fn primary(&self, dark: bool) -> Option<Hsla> {
        self.custom_primary.then(|| {
            let l = if dark {
                (self.lightness + 0.08).min(0.85)
            } else {
                self.lightness
            };
            hsla(self.hue, self.saturation, l, 1.)
        })
    }
}

impl TokenSettings {
    /// Back-fill these settings from an imported light theme.
    fn sync_from(&mut self, light: &Theme) {
        let primary = light.primary;
        // A near-black primary is the stock neutral default; anything else is
        // a brand color the sliders should show.
        if primary.s < 0.02 && primary.l < 0.1 {
            self.custom_primary = false;
        } else {
            self.custom_primary = true;
            self.hue = primary.h;
            self.saturation = primary.s;
            self.lightness = primary.l;
        }
        self.radius = (light.radius / px(1.)).clamp(0., 24.);

        // Infer the closest base gray family from the tinted neutrals.
        let distance = |base: BaseColor| -> f32 {
            let candidate = Theme::with_base(base, false);
            [
                (candidate.secondary, light.secondary),
                (candidate.border, light.border),
                (candidate.muted_foreground, light.muted_foreground),
            ]
            .into_iter()
            .map(|(a, b)| {
                let (a, b): (gpui::Rgba, gpui::Rgba) = (a.into(), b.into());
                (a.r - b.r).powi(2) + (a.g - b.g).powi(2) + (a.b - b.b).powi(2)
            })
            .sum()
        };
        self.base = BaseColor::ALL
            .into_iter()
            .min_by(|a, b| distance(*a).total_cmp(&distance(*b)))
            .unwrap_or_default();

        // Fonts only sync when the imported family is in the picker list;
        // otherwise the imported theme keeps carrying them until edited.
        self.font_sans = FONTS
            .iter()
            .filter_map(|(_, family)| *family)
            .find(|family| Some(*family) == light.font_sans.as_deref());
        self.font_heading = FONTS
            .iter()
            .filter_map(|(_, family)| *family)
            .find(|family| Some(*family) == light.font_heading.as_deref());
    }
}

/// Typed payload identifying which slider a drag belongs to.
struct SliderDrag(&'static str);

/// Invisible drag preview: sliders drag a value, not a visual.
struct DragPreview;

impl Render for DragPreview {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        gpui::Empty
    }
}

pub struct Storybook {
    focus_handle: gpui::FocusHandle,
    story: Story,
    dark: bool,
    tokens: TokenSettings,
    /// A (light, dark) pair imported from shadcn theme CSS; overrides the
    /// generated tokens until another token control is touched.
    imported: Option<(Theme, Theme)>,
    /// Feedback from the last import attempt: (message, success).
    import_status: Option<(String, bool)>,
    rng: u64,
    // Button controls
    button_variant: ButtonVariant,
    button_size: ButtonSize,
    button_disabled: bool,
    // Badge controls
    badge_variant: BadgeVariant,
    // Alert controls
    alert_variant: AlertVariant,
    // Avatar controls
    avatar_size: AvatarSize,
    // Switch controls
    switch_checked: bool,
    switch_size: SwitchSize,
    switch_disabled: bool,
    // Progress controls
    progress_value: f32,
    // Item controls
    item_variant: ItemVariant,
    item_size: ItemSize,
    // Table story state
    table_selected: Option<usize>,
    // Checkbox controls
    checkbox_checked: bool,
    // Radio group story state
    radio_selected: usize,
    // Toggle story state
    toggle_pressed: bool,
    toggle_outline_pressed: bool,
    // Toggle group story state
    toggle_group_on: [bool; 3],
    // Collapsible story state
    collapsible_open: bool,
    // Tabs story state
    tabs_active: usize,
    tabs_variant: TabsVariant,
    // Slider story state
    slider_value: f32,
    slider_fine: f32,
    // Pagination story state
    pagination_page: usize,
    // Dialog story state
    dialog_open: bool,
    // Alert dialog story state
    alert_dialog_open: bool,
    // Sheet story state
    sheet_open: bool,
    sheet_side: SheetSide,
    // Drawer story state
    drawer_open: bool,
    // Dropdown menu story state
    dropdown_open: bool,
    dropdown_status_checked: bool,
    // Context menu story state
    context_menu_at: Option<gpui::Point<gpui::Pixels>>,
    // Menubar story state
    menubar_open: Option<usize>,
    // Select story state
    select_value: Option<usize>,
    select_open: bool,
    // Native select story state
    native_select_value: Option<usize>,
    native_select_open: bool,
    // Navigation menu story state
    nav_menu_open: Option<usize>,
    // Toast story state
    toast_visible: bool,
    // Input story state — one entity per shadcn docs example
    input_demo: gpui::Entity<Input>,
    input_disabled: gpui::Entity<Input>,
    input_basic: gpui::Entity<Input>,
    input_field: gpui::Entity<Input>,
    input_fieldgroup_name: gpui::Entity<Input>,
    input_fieldgroup_email: gpui::Entity<Input>,
    input_invalid: gpui::Entity<Input>,
    input_file: gpui::Entity<Input>,
    input_inline: gpui::Entity<Input>,
    input_grid_first: gpui::Entity<Input>,
    input_grid_last: gpui::Entity<Input>,
    input_required: gpui::Entity<Input>,
    input_badge: gpui::Entity<Input>,
    input_ig_url: gpui::Entity<Input>,
    input_bg_search: gpui::Entity<Input>,
    input_form_name: gpui::Entity<Input>,
    input_form_email: gpui::Entity<Input>,
    input_form_phone: gpui::Entity<Input>,
    input_form_address: gpui::Entity<Input>,
    input_form_country: Option<usize>,
    input_form_country_open: bool,
    // Textarea story state
    textarea_input: gpui::Entity<Input>,
    // Field story state
    field_input: gpui::Entity<Input>,
    field_error_input: gpui::Entity<Input>,
    // Field story — preview (Payment Method)
    field_name_input: gpui::Entity<Input>,
    field_card_number: gpui::Entity<Input>,
    field_cvv: gpui::Entity<Input>,
    field_comments: gpui::Entity<Input>,
    field_month: Option<usize>,
    field_month_open: bool,
    field_year: Option<usize>,
    field_year_open: bool,
    field_same_shipping: bool,
    // Field story — Input / Textarea / Select / Slider examples
    field_username: gpui::Entity<Input>,
    field_password: gpui::Entity<Input>,
    field_feedback: gpui::Entity<Input>,
    field_department: Option<usize>,
    field_department_open: bool,
    field_slider: f32,
    // Field story — Fieldset example
    field_street: gpui::Entity<Input>,
    field_city: gpui::Entity<Input>,
    field_zip: gpui::Entity<Input>,
    // Field story — Checkbox example
    field_hard_disks: bool,
    field_external_disks: bool,
    field_cds: bool,
    field_connected_servers: bool,
    field_sync_folders: bool,
    // Field story — Radio / Switch / Choice Card
    field_plan: usize,
    field_switch_2fa: bool,
    field_compute_env: usize,
    // Field story — Field Group example
    field_push_responses: bool,
    field_push_tasks: bool,
    field_email_tasks: bool,
    // Field story — Responsive example
    field_responsive_name: gpui::Entity<Input>,
    field_responsive_width: f32,
    // Input group story state
    input_group_search: gpui::Entity<Input>,
    input_group_url: gpui::Entity<Input>,
    // Kbd story — Input Group example
    kbd_input: gpui::Entity<Input>,
    // Input OTP story state
    input_otp: gpui::Entity<Input>,
    // Command story state
    command_input: gpui::Entity<Input>,
    // Combobox story state
    combobox_search: gpui::Entity<Input>,
    combobox_value: Option<usize>,
    combobox_open: bool,
    // Calendar story state
    calendar_month: (i32, u32),
    calendar_selected: Option<CalendarDate>,
    // Date picker story state
    date_picker_value: Option<CalendarDate>,
    date_picker_month: (i32, u32),
    date_picker_open: bool,
    // Carousel story state
    carousel_index: usize,
    // Sidebar story state
    sidebar_open: bool,
    sidebar_active: usize,
    // Data table story state
    data_table_desc: bool,
    // Bubble story state
    bubble_variant: BubbleVariant,
    // Attachment story state
    attachment_visible: bool,
    // Questionnaire story state
    questionnaire_selected: Option<usize>,
    // Accordion parity controls
    accordion_disable_third: bool,
    accordion_multiple: bool,
    accordion_root_disabled: bool,
    // Controlled-mode example: the open set lives here, not in the component.
    accordion_value: Vec<ElementId>,
    // __STORY_STATE__
    // Popover state
    popover_open: bool,
    // Card controls
    card_size: CardSize,
    card_email_input: gpui::Entity<Input>,
    card_password_input: gpui::Entity<Input>,
    card_spacing: f32,
    card_spacing_email: gpui::Entity<Input>,
    card_spacing_password: gpui::Entity<Input>,
    /// Component label -> verified date, loaded from verification.json at startup.
    verified: HashMap<String, String>,
}

#[derive(Debug, Deserialize)]
struct VerificationEntry {
    name: String,
    verified: Option<String>,
    #[allow(dead_code)]
    by: Option<String>,
}

/// Load verification.json into a name -> date map. Missing or malformed files
/// degrade to an empty map so startup never panics on tracker data.
fn load_verification() -> HashMap<String, String> {
    let path = concat!(env!("CARGO_MANIFEST_DIR"), "/verification.json");
    let Ok(raw) = fs::read_to_string(path) else {
        return HashMap::new();
    };
    let Ok(entries) = serde_json::from_str::<Vec<VerificationEntry>>(&raw) else {
        return HashMap::new();
    };
    entries
        .into_iter()
        .filter_map(|e| e.verified.map(|date| (e.name, date)))
        .collect()
}

impl Storybook {
    pub fn new(cx: &mut Context<Self>) -> Self {
        let focus_handle = cx.focus_handle();
        // input-demo.tsx — API Key password field.
        let input_demo = cx.new(|cx| {
            let mut input = Input::new(cx);
            input.placeholder("sk-...");
            input.set_masked(true);
            input
        });
        let input_basic = cx.new(|cx| {
            let mut input = Input::new(cx);
            input.placeholder("Enter text");
            input
        });
        let input_field = cx.new(|cx| {
            let mut input = Input::new(cx);
            input.placeholder("Enter your username");
            input
        });
        let input_fieldgroup_name = cx.new(|cx| {
            let mut input = Input::new(cx);
            input.placeholder("Jordan Lee");
            input
        });
        let input_fieldgroup_email = cx.new(|cx| {
            let mut input = Input::new(cx);
            input.placeholder("name@example.com");
            input
        });
        let input_invalid = cx.new(|cx| {
            let mut input = Input::new(cx);
            input.placeholder("Error");
            input.set_invalid(true);
            input
        });
        let input_file = cx.new(|cx| {
            let mut input = Input::new(cx);
            input.set_file(true);
            input
        });
        let input_inline = cx.new(|cx| {
            let mut input = Input::new(cx);
            input.placeholder("Search...");
            input
        });
        let input_grid_first = cx.new(|cx| {
            let mut input = Input::new(cx);
            input.placeholder("Jordan");
            input
        });
        let input_grid_last = cx.new(|cx| {
            let mut input = Input::new(cx);
            input.placeholder("Lee");
            input
        });
        let input_required = cx.new(|cx| {
            let mut input = Input::new(cx);
            input.placeholder("This field is required");
            input
        });
        let input_badge = cx.new(|cx| {
            let mut input = Input::new(cx);
            input.placeholder("https://api.example.com/webhook");
            input
        });
        let input_ig_url = cx.new(|cx| {
            let mut input = Input::new(cx);
            input.placeholder("example.com");
            input.set_bare(true);
            input
        });
        let input_bg_search = cx.new(|cx| {
            let mut input = Input::new(cx);
            input.placeholder("Type to search...");
            input
        });
        let input_form_name = cx.new(|cx| {
            let mut input = Input::new(cx);
            input.placeholder("Evil Rabbit");
            input
        });
        let input_form_email = cx.new(|cx| {
            let mut input = Input::new(cx);
            input.placeholder("john@example.com");
            input
        });
        let input_form_phone = cx.new(|cx| {
            let mut input = Input::new(cx);
            input.placeholder("+1 (555) 123-4567");
            input
        });
        let input_form_address = cx.new(|cx| {
            let mut input = Input::new(cx);
            input.placeholder("123 Main St");
            input
        });
        let textarea_input = cx.new(|cx| {
            let mut input = Input::new(cx);
            input.placeholder("Type your message here.");
            input.set_bare(true);
            input
        });
        let field_input = cx.new(|cx| {
            let mut input = Input::new(cx);
            input.placeholder("you@example.com");
            input
        });
        let field_error_input = cx.new(|cx| {
            let mut input = Input::new(cx);
            input.placeholder("evil_rabbit");
            input
        });
        let field_name_input = cx.new(|cx| {
            let mut input = Input::new(cx);
            input.placeholder("Evil Rabbit");
            input
        });
        let field_card_number = cx.new(|cx| {
            let mut input = Input::new(cx);
            input.placeholder("1234 5678 9012 3456");
            input
        });
        let field_cvv = cx.new(|cx| {
            let mut input = Input::new(cx);
            input.placeholder("123");
            input
        });
        let field_comments = cx.new(|cx| {
            let mut input = Input::new(cx);
            input.placeholder("Add any additional comments");
            input.set_bare(true);
            input
        });
        let field_username = cx.new(|cx| {
            let mut input = Input::new(cx);
            input.placeholder("Max Leiter");
            input
        });
        let field_password = cx.new(|cx| {
            let mut input = Input::new(cx);
            input.placeholder("••••••••");
            input
        });
        let field_feedback = cx.new(|cx| {
            let mut input = Input::new(cx);
            input.placeholder("Your feedback helps us improve...");
            input.set_bare(true);
            input
        });
        let field_street = cx.new(|cx| {
            let mut input = Input::new(cx);
            input.placeholder("123 Main St");
            input
        });
        let field_city = cx.new(|cx| {
            let mut input = Input::new(cx);
            input.placeholder("New York");
            input
        });
        let field_zip = cx.new(|cx| {
            let mut input = Input::new(cx);
            input.placeholder("90502");
            input
        });
        let field_responsive_name = cx.new(|cx| {
            let mut input = Input::new(cx);
            input.placeholder("Evil Rabbit");
            input
        });
        let input_group_search = cx.new(|cx| {
            let mut input = Input::new(cx);
            input.placeholder("Search...");
            input.set_bare(true);
            input
        });
        let input_group_url = cx.new(|cx| {
            let mut input = Input::new(cx);
            input.placeholder("example.com");
            input.set_bare(true);
            input
        });
        let kbd_input = cx.new(|cx| {
            let mut input = Input::new(cx);
            input.placeholder("Search...");
            input.set_bare(true);
            input
        });
        let input_otp = cx.new(|cx| {
            let mut input = Input::new(cx);
            input.set_bare(true);
            input
        });
        let command_input = cx.new(|cx| {
            let mut input = Input::new(cx);
            input.placeholder("Type a command or search...");
            input.set_bare(true);
            input
        });
        let combobox_search = cx.new(|cx| {
            let mut input = Input::new(cx);
            input.placeholder("Search framework...");
            input.set_bare(true);
            input
        });
        let card_email_input = cx.new(|cx| {
            let mut input = Input::new(cx);
            input.placeholder("m@example.com");
            input
        });
        let card_password_input = cx.new(|cx| Input::new(cx));
        let card_spacing_email = cx.new(|cx| {
            let mut input = Input::new(cx);
            input.placeholder("m@example.com");
            input
        });
        let card_spacing_password = cx.new(|cx| Input::new(cx));
        // Live-refresh stories that derive UI from input text.
        for input in [
            &command_input,
            &input_otp,
            &input_demo,
            &combobox_search,
            &card_email_input,
            &card_password_input,
            &card_spacing_email,
            &card_spacing_password,
        ] {
            cx.observe(input, |_, _, cx| cx.notify()).detach();
        }
        let input_disabled = cx.new(|cx| {
            let mut input = Input::new(cx);
            input.placeholder("Email");
            input.set_disabled(true);
            input
        });
        let seed = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .map(|d| d.subsec_nanos() as u64 ^ d.as_secs())
            .unwrap_or(0x9e3779b9);
        let verified = load_verification();
        Self {
            story: Story::Tokens,
            dark: false,
            tokens: TokenSettings::default(),
            imported: None,
            import_status: None,
            rng: seed | 1,
            button_variant: ButtonVariant::Default,
            button_size: ButtonSize::Default,
            button_disabled: false,
            badge_variant: BadgeVariant::Default,
            alert_variant: AlertVariant::Default,
            avatar_size: AvatarSize::Default,
            switch_checked: true,
            switch_size: SwitchSize::Default,
            switch_disabled: false,
            progress_value: 60.,
            item_variant: ItemVariant::Outline,
            item_size: ItemSize::Default,
            table_selected: Some(1),
            checkbox_checked: true,
            radio_selected: 1,
            toggle_pressed: true,
            toggle_outline_pressed: false,
            toggle_group_on: [true, false, false],
            collapsible_open: false,
            tabs_active: 0,
            tabs_variant: TabsVariant::Default,
            slider_value: 50.,
            slider_fine: 0.4,
            pagination_page: 2,
            dialog_open: false,
            alert_dialog_open: false,
            sheet_open: false,
            sheet_side: SheetSide::Right,
            drawer_open: false,
            dropdown_open: false,
            dropdown_status_checked: true,
            context_menu_at: None,
            menubar_open: None,
            select_value: None,
            select_open: false,
            native_select_value: Some(0),
            native_select_open: false,
            nav_menu_open: None,
            toast_visible: false,
            input_demo,
            input_disabled,
            input_basic,
            input_field,
            input_fieldgroup_name,
            input_fieldgroup_email,
            input_invalid,
            input_file,
            input_inline,
            input_grid_first,
            input_grid_last,
            input_required,
            input_badge,
            input_ig_url,
            input_bg_search,
            input_form_name,
            input_form_email,
            input_form_phone,
            input_form_address,
            input_form_country: Some(0),
            input_form_country_open: false,
            textarea_input,
            field_input,
            field_error_input,
            field_name_input,
            field_card_number,
            field_cvv,
            field_comments,
            field_month: None,
            field_month_open: false,
            field_year: None,
            field_year_open: false,
            field_same_shipping: true,
            field_username,
            field_password,
            field_feedback,
            field_department: None,
            field_department_open: false,
            field_slider: 200.,
            field_street,
            field_city,
            field_zip,
            field_hard_disks: true,
            field_external_disks: false,
            field_cds: false,
            field_connected_servers: false,
            field_sync_folders: true,
            field_plan: 0,
            field_switch_2fa: false,
            field_compute_env: 0,
            field_push_responses: true,
            field_push_tasks: false,
            field_email_tasks: false,
            field_responsive_name,
            field_responsive_width: 360.,
            input_group_search,
            input_group_url,
            kbd_input,
            input_otp,
            command_input,
            combobox_search,
            combobox_value: None,
            combobox_open: false,
            calendar_month: (2026, 8),
            calendar_selected: Some(CalendarDate::new(2026, 8, 9)),
            date_picker_value: None,
            date_picker_month: (2026, 8),
            date_picker_open: false,
            carousel_index: 0,
            sidebar_open: true,
            sidebar_active: 0,
            data_table_desc: true,
            bubble_variant: BubbleVariant::Muted,
            attachment_visible: true,
            questionnaire_selected: Some(0),
            focus_handle,
            accordion_disable_third: false,
            accordion_multiple: false,
            accordion_root_disabled: false,
            accordion_value: vec![("ex-acc-ctl", 1usize).into()],
            // __STORY_STATE_INIT__
            popover_open: false,
            card_size: CardSize::Default,
            card_email_input,
            card_password_input,
            card_spacing: 16.,
            card_spacing_email,
            card_spacing_password,
            verified,
        }
    }

    /// Rebuild the global `Theme` from the current token settings and mode.
    fn apply_tokens(&self, cx: &mut Context<Self>) {
        let theme = if let Some((light, dark)) = &self.imported {
            if self.dark {
                dark.clone()
            } else {
                light.clone()
            }
        } else {
            let mut theme = Theme::with_base(self.tokens.base, self.dark);
            if let Some(primary) = self.tokens.primary(self.dark) {
                theme.primary = primary;
                theme.primary_foreground = if primary.l > 0.65 {
                    rgb(0x171717).into()
                } else {
                    rgb(0xfafafa).into()
                };
                theme.ring = primary;
            }
            theme.radius = px(self.tokens.radius);
            theme
        };
        // Fonts and icons layer over both generated and imported themes;
        // an imported theme's own fonts win unless the picker overrides them.
        let mut theme = theme;
        if let Some(font) = self.tokens.font_sans {
            theme.font_sans = Some(font.into());
        }
        if let Some(font) = self.tokens.font_heading {
            theme.font_heading = Some(font.into());
        }
        theme.icons = self.tokens.icons;
        cx.set_global(theme);
        cx.notify();
    }

    /// Drop an imported theme (called whenever a generated-token control is
    /// touched, so the controls always describe what's on screen).
    fn clear_import(&mut self) {
        self.imported = None;
        self.import_status = None;
    }

    /// Back-fill the token controls from an imported theme so the panel
    /// reflects it and later adjustments continue from the imported look
    /// instead of reverting to the previous settings.
    fn sync_settings_from(&mut self, light: &Theme) {
        self.tokens.sync_from(light);
    }

    fn import_from_clipboard(&mut self, cx: &mut Context<Self>) {
        let text = cx.read_from_clipboard().and_then(|item| item.text());
        match text.as_deref().and_then(Theme::from_shadcn_css) {
            Some(pair) => {
                self.sync_settings_from(&pair.0);
                self.imported = Some(pair);
                self.import_status = Some(("Theme imported — light + dark applied.".into(), true));
                self.apply_tokens(cx);
            }
            None => {
                self.import_status = Some((
                    "Clipboard doesn't look like shadcn theme CSS (:root { --token: … }).".into(),
                    false,
                ));
                cx.notify();
            }
        }
    }

    fn toggle_theme(&mut self, _: &ClickEvent, _window: &mut Window, cx: &mut Context<Self>) {
        self.dark = !self.dark;
        self.apply_tokens(cx);
    }

    /// xorshift64 — good enough to shuffle a palette, no dependency needed.
    fn rand(&mut self) -> f32 {
        let mut x = self.rng;
        x ^= x << 13;
        x ^= x >> 7;
        x ^= x << 17;
        self.rng = x;
        (x >> 40) as f32 / (1u64 << 24) as f32
    }

    fn shuffle(&mut self, cx: &mut Context<Self>) {
        self.clear_import();
        self.tokens.custom_primary = true;
        self.tokens.hue = self.rand();
        self.tokens.saturation = 0.5 + 0.45 * self.rand();
        self.tokens.lightness = 0.35 + 0.35 * self.rand();
        self.tokens.base = BaseColor::ALL[(self.rand() * 5.) as usize % 5];
        self.tokens.radius = [0., 4., 6., 8., 10., 12., 16., 20.][(self.rand() * 8.) as usize % 8];
        self.apply_tokens(cx);
    }

    // ---- chrome ------------------------------------------------------------

    fn sidebar(&self, cx: &mut Context<Self>) -> impl IntoElement + use<> {
        let theme = Theme::of(cx).clone();
        let mut components: Vec<Story> = Story::ALL
            .into_iter()
            .filter(|story| *story != Story::Tokens)
            .collect();
        components.sort_by_key(|story| story.label());

        Sidebar::new()
            .child(
                SidebarHeader::new().child(
                    div()
                        .px(px(8.))
                        .py(px(6.))
                        .text_size(px(15.))
                        .font_weight(FontWeight::SEMIBOLD)
                        .when_some(theme.heading_font(), |el, font| el.font_family(font))
                        .child("rcn"),
                ),
            )
            .child(
                SidebarContent::new()
                    .child(
                        SidebarGroup::new().label("Theme").child(
                            SidebarMenuButton::new("nav-tokens")
                                .active(self.story == Story::Tokens)
                                .on_click(cx.listener(|this, _, _, cx| {
                                    this.story = Story::Tokens;
                                    cx.notify();
                                }))
                                .child(Story::Tokens.label()),
                        ),
                    )
                    .child(SidebarGroup::new().label("Components").children(
                        components.into_iter().enumerate().map(|(index, story)| {
                            let label = story.label();
                            let verified = self.verified.contains_key(label);
                            SidebarMenuButton::new(("nav-component", index))
                                .active(self.story == story)
                                .on_click(cx.listener(move |this, _, _, cx| {
                                    this.story = story;
                                    cx.notify();
                                }))
                                .child(label)
                                .when(verified, |btn| {
                                    btn.child(
                                        div().ml_auto().child(
                                            Icon::new(theme.icons.check())
                                                .size(px(14.))
                                                .text_color(theme.muted_foreground),
                                        ),
                                    )
                                })
                        }),
                    )),
            )
            .child(
                SidebarFooter::new().child(
                    Button::new("theme-toggle")
                        .variant(ButtonVariant::Outline)
                        .size(ButtonSize::Sm)
                        .on_click(cx.listener(Self::toggle_theme))
                        .child(if theme.dark {
                            "Light mode"
                        } else {
                            "Dark mode"
                        }),
                ),
            )
    }
    fn canvas(&self, cx: &mut Context<Self>) -> impl IntoElement + use<> {
        let theme = Theme::of(cx).clone();
        let preview: AnyElement = match self.story {
            Story::Tokens => self.tokens_preview(cx).into_any_element(),
            Story::Button => self.button_preview(cx).into_any_element(),
            Story::Badge => self.badge_preview().into_any_element(),
            Story::Avatar => self.avatar_preview().into_any_element(),
            Story::Switch => self.switch_preview(cx).into_any_element(),
            Story::Accordion => self.accordion_preview(cx).into_any_element(),
            Story::Popover => self.popover_preview(cx).into_any_element(),
            Story::Separator => Self::separator_preview(cx).into_any_element(),
            Story::Skeleton => Self::skeleton_preview().into_any_element(),
            Story::Label => Self::label_preview().into_any_element(),
            Story::Kbd => Self::kbd_preview().into_any_element(),
            Story::Card => self.card_preview(cx).into_any_element(),
            Story::Alert => self.alert_preview().into_any_element(),
            Story::Progress => self.progress_preview().into_any_element(),
            Story::Spinner => Self::spinner_preview(cx).into_any_element(),
            Story::AspectRatio => Self::aspect_ratio_preview(cx).into_any_element(),
            Story::Empty => Self::empty_preview(cx).into_any_element(),
            Story::Item => self.item_preview(cx).into_any_element(),
            Story::Table => self.table_preview(cx).into_any_element(),
            Story::Breadcrumb => self.breadcrumb_preview(cx).into_any_element(),
            Story::Checkbox => self.checkbox_preview(cx).into_any_element(),
            Story::RadioGroup => self.radio_group_preview(cx).into_any_element(),
            Story::Toggle => self.toggle_preview(cx).into_any_element(),
            Story::ToggleGroup => self.toggle_group_preview(cx).into_any_element(),
            Story::ButtonGroup => Self::button_group_preview(cx).into_any_element(),
            Story::Collapsible => self.collapsible_preview(cx).into_any_element(),
            Story::Tabs => self.tabs_preview(cx).into_any_element(),
            Story::SliderStory => self.slider_preview(cx).into_any_element(),
            Story::PaginationStory => self.pagination_preview(cx).into_any_element(),
            Story::ScrollArea => Self::scroll_area_preview(cx).into_any_element(),
            Story::TooltipStory => Self::tooltip_preview().into_any_element(),
            Story::HoverCardStory => Self::hover_card_preview().into_any_element(),
            Story::DialogStory => self.dialog_preview(cx).into_any_element(),
            Story::AlertDialogStory => self.alert_dialog_preview(cx).into_any_element(),
            Story::SheetStory => self.sheet_preview(cx).into_any_element(),
            Story::DrawerStory => self.drawer_preview(cx).into_any_element(),
            Story::DropdownMenuStory => self.dropdown_menu_preview(cx).into_any_element(),
            Story::ContextMenuStory => self.context_menu_preview(cx).into_any_element(),
            Story::MenubarStory => self.menubar_preview(cx).into_any_element(),
            Story::SelectStory => self.select_preview(cx).into_any_element(),
            Story::NativeSelectStory => self.native_select_preview(cx).into_any_element(),
            Story::NavigationMenuStory => self.navigation_menu_preview(cx).into_any_element(),
            Story::ToastStory => self.toast_preview(cx).into_any_element(),
            Story::InputStory => self.input_preview(cx).into_any_element(),
            Story::TextareaStory => self.textarea_preview(cx).into_any_element(),
            Story::FieldStory => self.field_preview(cx).into_any_element(),
            Story::InputGroupStory => self.input_group_preview(cx).into_any_element(),
            Story::InputOtpStory => self.input_otp_preview(cx).into_any_element(),
            Story::CommandStory => self.command_preview(cx).into_any_element(),
            Story::ComboboxStory => self.combobox_preview(cx).into_any_element(),
            Story::CalendarStory => self.calendar_preview(cx).into_any_element(),
            Story::DatePickerStory => self.date_picker_preview(cx).into_any_element(),
            Story::CarouselStory => self.carousel_preview(cx).into_any_element(),
            Story::ResizableStory => self.resizable_preview(cx).into_any_element(),
            Story::SidebarStory => self.sidebar_preview(cx).into_any_element(),
            Story::DataTableStory => self.data_table_preview(cx).into_any_element(),
            Story::ChartStory => self.chart_preview(cx).into_any_element(),
            Story::MessageStory => self.message_preview(cx).into_any_element(),
            Story::BubbleStory => self.bubble_preview().into_any_element(),
            Story::AttachmentStory => self.attachment_preview(cx).into_any_element(),
            Story::MarkerStory => self.marker_preview(cx).into_any_element(),
            Story::MessageScrollerStory => self.message_scroller_preview(cx).into_any_element(),
            Story::QuestionnaireStory => self.questionnaire_preview(cx).into_any_element(),
            // __STORY_CANVAS__
        };
        let docs = self
            .story
            .module()
            .and_then(crate::storybook_docs::docs_for);
        let extra_examples = self.story_examples(cx);

        div()
            .id("canvas")
            .flex()
            .flex_col()
            .flex_1()
            .min_w(px(0.))
            .h_full()
            .overflow_y_scroll()
            .overflow_x_hidden()
            .child(
                div()
                    .flex()
                    .flex_col()
                    .gap(px(24.))
                    .px(px(28.))
                    .py(px(24.))
                    // Title + description
                    .child(
                        div()
                            .flex()
                            .flex_col()
                            .gap(px(4.))
                            .child(
                                div()
                                    .text_size(px(20.))
                                    .line_height(px(28.))
                                    .font_weight(FontWeight::SEMIBOLD)
                                    .when_some(theme.heading_font(), |el, font| {
                                        el.font_family(font)
                                    })
                                    .child(self.story.label()),
                            )
                            .child(
                                div()
                                    .text_size(px(14.))
                                    .line_height(px(20.))
                                    .text_color(theme.muted_foreground)
                                    .child(self.story.description()),
                            )
                            .when_some(
                                self.verified.get(self.story.label()).cloned(),
                                |el, date| {
                                    el.child(
                                        div()
                                            .flex()
                                            .flex_row()
                                            .items_center()
                                            .gap(px(6.))
                                            .text_size(px(13.))
                                            .text_color(theme.muted_foreground)
                                            .child(
                                                Icon::new(theme.icons.check())
                                                    .size(px(14.))
                                                    .text_color(theme.muted_foreground),
                                            )
                                            .child(format!("Verified {date}")),
                                    )
                                },
                            ),
                    )
                    // Primary example, in a frame that grows with content
                    .child(Self::example_frame(&theme, None, None, preview))
                    // About blurb (the shadcn docs "About" section)
                    .when_some(Self::story_about(self.story), |el, about| {
                        el.child(Self::docs_heading(&theme, "About"))
                            .child(Self::docs_paragraph(&theme, about))
                    })
                    // Extra example flavors
                    .children(extra_examples.into_iter().map(|(title, example)| {
                        Self::example_frame(
                            &theme,
                            Some(title),
                            Self::example_description(self.story, title),
                            example,
                        )
                    }))
                    // Installation
                    .when_some(docs, |el, docs| {
                        el.child(Self::docs_heading(&theme, "Installation"))
                            .child(Self::code_block(
                                &theme,
                                "install-code",
                                &format!(
                                    "# rcn is copy-paste, shadcn style: vendor the component\n# (plus src/theme.rs and src/motion.rs) into your gpui app.\ncp {} your-app/src/components/",
                                    docs.source_path
                                ),
                            ))
                            .child(Self::docs_heading(&theme, "Usage"))
                            .child(Self::code_block(&theme, "usage-code", docs.usage))
                            .child(Self::docs_heading(&theme, "API Reference"))
                            .child(Self::api_table(&theme, docs.api))
                    }),
            )
    }

    /// A titled example frame that grows with its content (no clipping).
    fn example_frame(
        theme: &Theme,
        title: Option<&'static str>,
        description: Option<&'static str>,
        example: AnyElement,
    ) -> impl IntoElement + use<> {
        div()
            .flex()
            .flex_col()
            .gap(px(8.))
            .when_some(title, |el, title| {
                el.child(Self::docs_heading(theme, title))
            })
            .when_some(description, |el, description| {
                el.child(Self::docs_paragraph(theme, description))
            })
            .child(
                div()
                    .flex()
                    .w_full()
                    .min_h(px(220.))
                    .items_center()
                    .justify_center()
                    .rounded(theme.radius_lg())
                    .border_1()
                    .border_color(theme.border)
                    .p(px(32.))
                    .child(example),
            )
    }

    fn docs_heading(theme: &Theme, title: &'static str) -> impl IntoElement + use<> {
        div()
            .pt(px(8.))
            .text_size(px(16.))
            .line_height(px(24.))
            .font_weight(FontWeight::SEMIBOLD)
            .when_some(theme.heading_font(), |el, font| el.font_family(font))
            .child(title)
    }

    /// Prose under a docs heading (the shadcn section description).
    fn docs_paragraph(theme: &Theme, text: &'static str) -> impl IntoElement + use<> {
        div()
            .text_size(px(14.))
            .line_height(px(20.))
            .text_color(theme.muted_foreground)
            .child(text)
    }

    /// A monospace code block (line-per-row so formatting is preserved).
    fn code_block(theme: &Theme, id: &'static str, code: &str) -> impl IntoElement + use<> {
        div()
            .id(id)
            .w_full()
            .rounded(theme.radius_lg())
            .border_1()
            .border_color(theme.border)
            .bg(if theme.dark {
                theme.card
            } else {
                theme.secondary
            })
            .p(px(16.))
            .overflow_x_scroll()
            .child(
                div()
                    .flex()
                    .flex_col()
                    .font_family("Menlo")
                    .text_size(px(12.))
                    .line_height(px(18.))
                    .text_color(theme.foreground)
                    .children(code.lines().map(|line| {
                        div().whitespace_nowrap().child(if line.is_empty() {
                            " ".to_string()
                        } else {
                            line.to_string()
                        })
                    })),
            )
    }

    /// The generated API reference: builder signatures grouped by type.
    fn api_table(
        theme: &Theme,
        api: &'static [crate::storybook_docs::ApiEntry],
    ) -> impl IntoElement + use<> {
        let mut rows: Vec<AnyElement> = Vec::new();
        let mut last_type = "";
        for entry in api {
            if entry.type_name != last_type {
                last_type = entry.type_name;
                rows.push(
                    div()
                        .pt(px(12.))
                        .text_size(px(14.))
                        .font_weight(FontWeight::MEDIUM)
                        .child(entry.type_name)
                        .into_any_element(),
                );
            }
            rows.push(
                div()
                    .flex()
                    .flex_col()
                    .gap(px(2.))
                    .py(px(6.))
                    .border_b_1()
                    .border_color(theme.border)
                    .child(
                        div()
                            .font_family("Menlo")
                            .text_size(px(12.))
                            .line_height(px(17.))
                            .text_color(theme.foreground)
                            .child(entry.signature),
                    )
                    .when(!entry.doc.is_empty(), |el| {
                        el.child(
                            div()
                                .text_size(px(13.))
                                .line_height(px(18.))
                                .text_color(theme.muted_foreground)
                                .child(entry.doc),
                        )
                    })
                    .into_any_element(),
            );
        }
        div().flex().flex_col().children(rows)
    }

    /// Additional named example flavors per story (shadcn docs style);
    /// stories default to just their primary example.
    fn story_examples(&self, cx: &mut Context<Self>) -> Vec<(&'static str, AnyElement)> {
        let theme = Theme::of(cx).clone();
        match self.story {
            Story::Button => vec![
                (
                    "Sizes",
                    div()
                        .flex()
                        .flex_row()
                        .flex_wrap()
                        .items_center()
                        .gap(px(8.))
                        .child(
                            Button::new("ex-btn-xs")
                                .variant(ButtonVariant::Outline)
                                .size(ButtonSize::Xs)
                                .child("Extra Small"),
                        )
                        .child(
                            Button::new("ex-btn-sm")
                                .variant(ButtonVariant::Outline)
                                .size(ButtonSize::Sm)
                                .child("Small"),
                        )
                        .child(
                            Button::new("ex-btn-md")
                                .variant(ButtonVariant::Outline)
                                .child("Default"),
                        )
                        .child(
                            Button::new("ex-btn-lg")
                                .variant(ButtonVariant::Outline)
                                .size(ButtonSize::Lg)
                                .child("Large"),
                        )
                        .child(
                            Button::new("ex-btn-icon")
                                .variant(ButtonVariant::Outline)
                                .size(ButtonSize::Icon)
                                .child(Icon::new(theme.icons.chevron_right())),
                        )
                        .into_any_element(),
                ),
                (
                    "Default",
                    Button::new("ex-btn-default")
                        .child("Button")
                        .into_any_element(),
                ),
                (
                    "Outline",
                    Button::new("ex-btn-outline")
                        .variant(ButtonVariant::Outline)
                        .child("Outline")
                        .into_any_element(),
                ),
                (
                    "Secondary",
                    Button::new("ex-btn-secondary")
                        .variant(ButtonVariant::Secondary)
                        .child("Secondary")
                        .into_any_element(),
                ),
                (
                    "Ghost",
                    Button::new("ex-btn-ghost")
                        .variant(ButtonVariant::Ghost)
                        .child("Ghost")
                        .into_any_element(),
                ),
                (
                    "Destructive",
                    Button::new("ex-btn-destructive")
                        .variant(ButtonVariant::Destructive)
                        .child("Destructive")
                        .into_any_element(),
                ),
                (
                    "Link",
                    Button::new("ex-btn-link")
                        .variant(ButtonVariant::Link)
                        .child("Link")
                        .into_any_element(),
                ),
                (
                    "Icon",
                    div()
                        .flex()
                        .flex_row()
                        .flex_wrap()
                        .items_center()
                        .gap(px(8.))
                        .child(
                            Button::new("ex-btn-icon-default")
                                .size(ButtonSize::Icon)
                                .child(Icon::new(theme.icons.chevron_right())),
                        )
                        .child(
                            Button::new("ex-btn-icon-sm")
                                .variant(ButtonVariant::Outline)
                                .size(ButtonSize::IconSm)
                                .child(Icon::new(theme.icons.chevron_right())),
                        )
                        .child(
                            Button::new("ex-btn-icon-lg")
                                .variant(ButtonVariant::Outline)
                                .size(ButtonSize::IconLg)
                                .child(Icon::new(theme.icons.chevron_right())),
                        )
                        .into_any_element(),
                ),
                (
                    "With Icon",
                    Button::new("ex-btn-with-icon")
                        .icon_inline_start()
                        .child(Icon::new(theme.icons.check()))
                        .child("Confirm")
                        .into_any_element(),
                ),
                (
                    "Rounded",
                    div()
                        .flex()
                        .flex_row()
                        .flex_wrap()
                        .items_center()
                        .gap(px(8.))
                        .child(Button::new("ex-btn-rounded").rounded_full().child("Button"))
                        .child(
                            Button::new("ex-btn-rounded-outline")
                                .variant(ButtonVariant::Outline)
                                .rounded_full()
                                .child("Outline"),
                        )
                        .child(
                            Button::new("ex-btn-rounded-icon")
                                .size(ButtonSize::Icon)
                                .rounded_full()
                                .child(Icon::new(theme.icons.chevron_right())),
                        )
                        .into_any_element(),
                ),
                (
                    "Spinner",
                    Button::new("ex-btn-spinner")
                        .disabled(true)
                        .icon_inline_start()
                        .child(Spinner::new())
                        .child("Loading")
                        .into_any_element(),
                ),
                (
                    "Button Group",
                    ButtonGroup::new()
                        .item(
                            Button::new("ex-btn-group-archive")
                                .variant(ButtonVariant::Outline)
                                .child("Archive"),
                        )
                        .item(
                            Button::new("ex-btn-group-report")
                                .variant(ButtonVariant::Outline)
                                .child("Report"),
                        )
                        .item(
                            Button::new("ex-btn-group-snooze")
                                .variant(ButtonVariant::Outline)
                                .child("Snooze"),
                        )
                        .into_any_element(),
                ),
                (
                    "As Link",
                    Button::new("ex-btn-as-link")
                        .variant(ButtonVariant::Link)
                        .child("Login")
                        .on_click(|_, _, cx| {
                            cx.open_url("https://ui.shadcn.com/docs/components/base/button")
                        })
                        .into_any_element(),
                ),
                (
                    "Disabled",
                    Button::new("ex-btn-disabled")
                        .disabled(true)
                        .child("Disabled")
                        .into_any_element(),
                ),
            ],
            Story::Accordion => vec![
                (
                    "Multiple",
                    div()
                        .w(px(384.))
                        .child(
                            Accordion::new("ex-acc-multiple")
                                .multiple(true)
                                .default_value(["ex-acc-multi-1", "ex-acc-multi-2"])
                                .child(
                                    AccordionItem::new("ex-acc-multi-1")
                                        .trigger("First item")
                                        .content("Open by default alongside the second item."),
                                )
                                .child(
                                    AccordionItem::new("ex-acc-multi-2")
                                        .trigger("Second item")
                                        .content("Multiple mode keeps several panels open."),
                                )
                                .child(
                                    AccordionItem::new("ex-acc-multi-3")
                                        .trigger("Third item")
                                        .content("Toggle any combination of panels."),
                                ),
                        )
                        .into_any_element(),
                ),
                (
                    "Disabled item",
                    div()
                        .w(px(384.))
                        .child(
                            Accordion::new("ex-acc-disabled")
                                .default_value(["ex-acc-dis-1"])
                                .child(
                                    AccordionItem::new("ex-acc-dis-1")
                                        .trigger("Available item")
                                        .content("This item can be toggled."),
                                )
                                .child(
                                    AccordionItem::new("ex-acc-dis-2")
                                        .trigger("Disabled item")
                                        .content("Unreachable.")
                                        .disabled(true),
                                ),
                        )
                        .into_any_element(),
                ),
                (
                    "Borders",
                    div()
                        .w(px(384.))
                        .child(
                            Accordion::new("ex-acc-borders")
                                .bordered(true)
                                .default_value(["ex-acc-border-1"])
                                .child(
                                    AccordionItem::new("ex-acc-border-1")
                                        .trigger("Bordered shell")
                                        .content(
                                            "Outer border, rounded corners, and horizontal padding.",
                                        ),
                                )
                                .child(
                                    AccordionItem::new("ex-acc-border-2")
                                        .trigger("Between-item rules")
                                        .content("Items still draw a divider between rows."),
                                ),
                        )
                        .into_any_element(),
                ),
                (
                    "Controlled",
                    div()
                        .w(px(384.))
                        .child(
                            Accordion::new("ex-acc-controlled")
                                .value(self.accordion_value.iter().cloned())
                                .on_value_change(cx.listener(
                                    |this, value: &[ElementId], _, cx| {
                                        this.accordion_value = value.to_vec();
                                        cx.notify();
                                    },
                                ))
                                .children((1..=2usize).map(|n| {
                                    AccordionItem::new(("ex-acc-ctl", n))
                                        .trigger(format!("Controlled item {n}"))
                                        .content(
                                            "The storybook owns this open set via value + \
                                             on_value_change.",
                                        )
                                })),
                        )
                        .into_any_element(),
                ),
                (
                    "In a card",
                    div()
                        .w(px(384.))
                        .child(
                            Card::new().child(
                                CardContent::new().child(
                                    Accordion::new("ex-acc-card")
                                        .default_value(["ex-acc-card-1"])
                                        .child(
                                            AccordionItem::new("ex-acc-card-1")
                                                .trigger("Inside a card")
                                                .content(
                                                    "The card resizes smoothly as this panel opens and closes.",
                                                ),
                                        )
                                        .child(
                                            AccordionItem::new("ex-acc-card-2")
                                                .trigger("Another section")
                                                .content(
                                                    "Height animation keeps the surrounding card in flow.",
                                                ),
                                        ),
                                ),
                            ),
                        )
                        .into_any_element(),
                ),
            ],
            Story::Badge => vec![(
                "Variants",
                div()
                    .flex()
                    .flex_row()
                    .flex_wrap()
                    .items_center()
                    .gap(px(8.))
                    .child(Badge::new().variant(BadgeVariant::Default).child("Default"))
                    .child(
                        Badge::new()
                            .variant(BadgeVariant::Secondary)
                            .child("Secondary"),
                    )
                    .child(
                        Badge::new()
                            .variant(BadgeVariant::Destructive)
                            .child("Destructive"),
                    )
                    .child(Badge::new().variant(BadgeVariant::Outline).child("Outline"))
                    .child(Badge::new().variant(BadgeVariant::Ghost).child("Ghost"))
                    .child(Badge::new().variant(BadgeVariant::Link).child("Link"))
                    .into_any_element(),
            )],
            Story::Switch => vec![(
                "Sizes",
                div()
                    .flex()
                    .flex_row()
                    .flex_wrap()
                    .items_center()
                    .gap(px(8.))
                    .child(
                        Switch::new("ex-switch-sm")
                            .size(SwitchSize::Sm)
                            .checked(true),
                    )
                    .child(Switch::new("ex-switch-default").checked(true))
                    .child(Switch::new("ex-switch-disabled").disabled(true))
                    .into_any_element(),
            )],
            Story::Checkbox => vec![(
                "States",
                div()
                    .flex()
                    .flex_col()
                    .gap(px(8.))
                    .child(
                        div()
                            .flex()
                            .flex_row()
                            .items_center()
                            .gap(px(8.))
                            .child(Checkbox::new("ex-cb-unchecked").checked(false))
                            .child(Label::new().child("Unchecked")),
                    )
                    .child(
                        div()
                            .flex()
                            .flex_row()
                            .items_center()
                            .gap(px(8.))
                            .child(Checkbox::new("ex-cb-checked").checked(true))
                            .child(Label::new().child("Checked")),
                    )
                    .child(
                        div()
                            .flex()
                            .flex_row()
                            .items_center()
                            .gap(px(8.))
                            .child(
                                Checkbox::new("ex-cb-disabled")
                                    .checked(false)
                                    .disabled(true),
                            )
                            .child(Label::new().disabled(true).child("Disabled")),
                    )
                    .child(
                        div()
                            .flex()
                            .flex_row()
                            .items_center()
                            .gap(px(8.))
                            .child(
                                Checkbox::new("ex-cb-disabled-checked")
                                    .checked(true)
                                    .disabled(true),
                            )
                            .child(Label::new().disabled(true).child("Disabled checked")),
                    )
                    .into_any_element(),
            )],
            Story::Toggle => vec![(
                "Variants and sizes",
                div()
                    .flex()
                    .flex_row()
                    .flex_wrap()
                    .items_center()
                    .gap(px(8.))
                    .child(
                        Toggle::new("ex-toggle-default")
                            .variant(ToggleVariant::Default)
                            .pressed(true)
                            .child("Default"),
                    )
                    .child(
                        Toggle::new("ex-toggle-outline")
                            .variant(ToggleVariant::Outline)
                            .pressed(true)
                            .child("Outline"),
                    )
                    .child(
                        Toggle::new("ex-toggle-sm")
                            .size(ToggleSize::Sm)
                            .pressed(true)
                            .child("Small"),
                    )
                    .child(
                        Toggle::new("ex-toggle-md")
                            .size(ToggleSize::Default)
                            .pressed(true)
                            .child("Default"),
                    )
                    .child(
                        Toggle::new("ex-toggle-lg")
                            .size(ToggleSize::Lg)
                            .pressed(true)
                            .child("Large"),
                    )
                    .into_any_element(),
            )],
            Story::Alert => vec![(
                "Destructive",
                Alert::new()
                    .variant(AlertVariant::Destructive)
                    .icon(crate::assets::ICON_CIRCLE_ALERT)
                    .child(AlertTitle::new().child("Error"))
                    .child(
                        AlertDescription::new()
                            .child("Your session has expired. Please log in again."),
                    )
                    .into_any_element(),
            )],
            Story::Card => {
                let spacing = px(self.card_spacing);
                let feature_row = |text: &'static str| {
                    div()
                        .flex()
                        .flex_row()
                        .gap(px(8.))
                        .child(
                            Icon::new(theme.icons.chevron_right())
                                .size(px(16.))
                                .text_color(theme.muted_foreground)
                                .mt(px(2.)),
                        )
                        .child(div().child(text))
                };
                vec![
                    (
                        "Small size",
                        div()
                            .w(px(320.))
                            .child(
                                Card::new()
                                    .size(CardSize::Sm)
                                    .child(
                                        CardHeader::new()
                                            .size(CardSize::Sm)
                                            .child(
                                                CardTitle::new()
                                                    .size(CardSize::Sm)
                                                    .child("Scheduled reports"),
                                            )
                                            .child(
                                                CardDescription::new().child(
                                                    "Weekly snapshots. No more manual exports.",
                                                ),
                                            ),
                                    )
                                    .child(
                                        CardContent::new().size(CardSize::Sm).child(
                                            div()
                                                .flex()
                                                .flex_col()
                                                .gap(px(8.))
                                                .py(px(8.))
                                                .child(feature_row(
                                                    "Choose a schedule (daily, or weekly).",
                                                ))
                                                .child(feature_row(
                                                    "Send to channels or specific teammates.",
                                                ))
                                                .child(feature_row(
                                                    "Include charts, tables, and key metrics.",
                                                )),
                                        ),
                                    )
                                    .child(
                                        CardFooter::new().size(CardSize::Sm).child(
                                            div()
                                                .w_full()
                                                .flex()
                                                .flex_col()
                                                .gap(px(8.))
                                                .child(
                                                    div().w_full().child(
                                                        Button::new("card-sm-setup")
                                                            .size(ButtonSize::Sm)
                                                            .child("Set up scheduled reports"),
                                                    ),
                                                )
                                                .child(
                                                    div().w_full().child(
                                                        Button::new("card-sm-new")
                                                            .variant(ButtonVariant::Outline)
                                                            .size(ButtonSize::Sm)
                                                            .child("See what's new"),
                                                    ),
                                                ),
                                        ),
                                    ),
                            )
                            .into_any_element(),
                    ),
                    (
                        "Spacing",
                        div()
                            .flex()
                            .flex_col()
                            .gap(px(16.))
                            .child(
                                div().flex().justify_center().child(
                                    ToggleGroup::new()
                                        .variant(ToggleVariant::Outline)
                                        .size(ToggleSize::Sm)
                                        .item(
                                            ToggleGroupItem::new("card-sp-16")
                                                .pressed(self.card_spacing == 16.)
                                                .on_change(cx.listener(
                                                    |this, pressed: &bool, _, cx| {
                                                        if *pressed {
                                                            this.card_spacing = 16.;
                                                            cx.notify();
                                                        }
                                                    },
                                                ))
                                                .child("16px"),
                                        )
                                        .item(
                                            ToggleGroupItem::new("card-sp-20")
                                                .pressed(self.card_spacing == 20.)
                                                .on_change(cx.listener(
                                                    |this, pressed: &bool, _, cx| {
                                                        if *pressed {
                                                            this.card_spacing = 20.;
                                                            cx.notify();
                                                        }
                                                    },
                                                ))
                                                .child("20px"),
                                        )
                                        .item(
                                            ToggleGroupItem::new("card-sp-24")
                                                .pressed(self.card_spacing == 24.)
                                                .on_change(cx.listener(
                                                    |this, pressed: &bool, _, cx| {
                                                        if *pressed {
                                                            this.card_spacing = 24.;
                                                            cx.notify();
                                                        }
                                                    },
                                                ))
                                                .child("24px"),
                                        )
                                        .item(
                                            ToggleGroupItem::new("card-sp-32")
                                                .pressed(self.card_spacing == 32.)
                                                .on_change(cx.listener(
                                                    |this, pressed: &bool, _, cx| {
                                                        if *pressed {
                                                            this.card_spacing = 32.;
                                                            cx.notify();
                                                        }
                                                    },
                                                ))
                                                .child("32px"),
                                        ),
                                ),
                            )
                            .child(
                                div().w(px(384.)).child(
                                    Card::new()
                                        .spacing(spacing)
                                        .child(
                                            CardHeader::new()
                                                .spacing(spacing)
                                                .action(
                                                    Button::new("card-sp-sign-up")
                                                        .variant(ButtonVariant::Link)
                                                        .child("Sign Up"),
                                                )
                                                .child(
                                                    CardTitle::new()
                                                        .child("Login to your account"),
                                                )
                                                .child(
                                                    CardDescription::new().child(
                                                        "Enter your email below to login to your account",
                                                    ),
                                                ),
                                        )
                                        .child(
                                            CardContent::new().spacing(spacing).child(
                                                div()
                                                    .flex()
                                                    .flex_col()
                                                    .gap(px(24.))
                                                    .child(
                                                        div()
                                                            .flex()
                                                            .flex_col()
                                                            .gap(px(8.))
                                                            .child(Label::new().child("Email"))
                                                            .child(
                                                                self.card_spacing_email.clone(),
                                                            ),
                                                    )
                                                    .child(
                                                        div()
                                                            .flex()
                                                            .flex_col()
                                                            .gap(px(8.))
                                                            .child(
                                                                div()
                                                                    .flex()
                                                                    .flex_row()
                                                                    .items_center()
                                                                    .child(
                                                                        Label::new()
                                                                            .child("Password"),
                                                                    )
                                                                    .child(
                                                                        div()
                                                                            .id("card-sp-forgot")
                                                                            .ml_auto()
                                                                            .text_size(px(14.))
                                                                            .line_height(px(20.))
                                                                            .cursor_pointer()
                                                                            .hover(|s| {
                                                                                s.underline()
                                                                            })
                                                                            .child(
                                                                                "Forgot your password?",
                                                                            ),
                                                                    ),
                                                            )
                                                            .child(
                                                                self.card_spacing_password.clone(),
                                                            ),
                                                    ),
                                            ),
                                        )
                                        .child(
                                            CardFooter::new().spacing(spacing).child(
                                                div()
                                                    .w_full()
                                                    .flex()
                                                    .flex_col()
                                                    .gap(px(8.))
                                                    .child(
                                                        div().w_full().child(
                                                            Button::new("card-sp-login")
                                                                .child("Login"),
                                                        ),
                                                    )
                                                    .child(
                                                        div().w_full().child(
                                                            Button::new("card-sp-login-google")
                                                                .variant(ButtonVariant::Outline)
                                                                .child("Login with Google"),
                                                        ),
                                                    ),
                                            ),
                                        ),
                                ),
                            )
                            .into_any_element(),
                    ),
                    (
                        "Edge to edge",
                        div()
                            .w(px(384.))
                            .child(
                                Card::new()
                                    .child(
                                        CardHeader::new()
                                            .child(CardTitle::new().child("Terms of Service"))
                                            .child(
                                                CardDescription::new().child(
                                                    "Review the terms before accepting the agreement.",
                                                ),
                                            ),
                                    )
                                    .child(
                                        CardContent::new().flush_bottom().child(
                                            div()
                                                .mx(px(-16.))
                                                .border_t_1()
                                                .border_color(theme.border)
                                                .bg(alpha(theme.muted, 0.5))
                                                .child(
                                                    // max-h-48 px/py live on the scroller itself.
                                                    ScrollArea::new("card-edge-scroll")
                                                        .h(px(192.))
                                                        .child(
                                                            div()
                                                                .flex()
                                                                .flex_col()
                                                                .gap(px(16.))
                                                                .px(px(16.))
                                                                .py(px(16.))
                                                                .text_size(px(14.))
                                                                .line_height(px(23.))
                                                                .child(
                                                                    "These terms govern your use of the workspace, including access to shared documents, project files, and collaboration tools.",
                                                                )
                                                                .child(
                                                                    "You are responsible for the content you upload and for ensuring that your team has the appropriate permissions to view or edit it.",
                                                                )
                                                                .child(
                                                                    "We may update features or limits as the service evolves. When those changes materially affect your workflow, we will notify your workspace administrators.",
                                                                )
                                                                .child(
                                                                    "By continuing, you agree to keep your account credentials secure and to follow your organization's acceptable use policies.",
                                                                ),
                                                        ),
                                                ),
                                        ),
                                    )
                                    .child(
                                        CardFooter::new().child(
                                            div()
                                                .w_full()
                                                .flex()
                                                .flex_row()
                                                .justify_end()
                                                .gap(px(8.))
                                                .child(
                                                    Button::new("card-edge-decline")
                                                        .variant(ButtonVariant::Outline)
                                                        .child("Decline"),
                                                )
                                                .child(
                                                    Button::new("card-edge-accept")
                                                        .child("Accept"),
                                                ),
                                        ),
                                    ),
                            )
                            .into_any_element(),
                    ),
                    (
                        "Image",
                        div()
                            .w(px(384.))
                            .child(
                                Card::new()
                                    .flush_top()
                                    .child(
                                        div()
                                            .relative()
                                            .w_full()
                                            .h(px(216.))
                                            // Grayscale gradient standing in for the docs'
                                            // avatar.vercel.sh cover (rendered grayscale +
                                            // brightness-60 there).
                                            .bg(gpui::linear_gradient(
                                                135.,
                                                gpui::linear_color_stop(
                                                    gpui::hsla(0., 0., 0.55, 1.),
                                                    0.,
                                                ),
                                                gpui::linear_color_stop(
                                                    gpui::hsla(0., 0., 0.3, 1.),
                                                    1.,
                                                ),
                                            ))
                                            .child(
                                                // bg-black/35 overlay from the docs example.
                                                div()
                                                    .absolute()
                                                    .inset_0()
                                                    .bg(gpui::hsla(0., 0., 0., 0.35)),
                                            ),
                                    )
                                    .child(
                                        CardHeader::new()
                                            .action(
                                                Badge::new()
                                                    .variant(BadgeVariant::Secondary)
                                                    .child("Featured"),
                                            )
                                            .child(
                                                CardTitle::new()
                                                    .child("Design systems meetup"),
                                            )
                                            .child(
                                                CardDescription::new().child(
                                                    "A practical talk on component APIs, accessibility, and shipping faster.",
                                                ),
                                            ),
                                    )
                                    .child(
                                        CardFooter::new().child(
                                            div().w_full().child(
                                                Button::new("card-img-view")
                                                    .child("View Event"),
                                            ),
                                        ),
                                    ),
                            )
                            .into_any_element(),
                    ),
                ]
            }
            Story::Tabs => vec![(
                "Line variant",
                Tabs::new()
                    .child(
                        TabsList::new()
                            .variant(TabsVariant::Line)
                            .trigger(
                                TabsTrigger::new("ex-tabs-line-1")
                                    .active(true)
                                    .child("Account"),
                            )
                            .trigger(TabsTrigger::new("ex-tabs-line-2").child("Password")),
                    )
                    .into_any_element(),
            )],
            Story::Avatar => vec![(
                "Sizes",
                div()
                    .flex()
                    .flex_row()
                    .items_center()
                    .gap(px(12.))
                    .child(Avatar::new("SM").size(AvatarSize::Sm))
                    .child(Avatar::new("DF").size(AvatarSize::Default))
                    .child(Avatar::new("LG").size(AvatarSize::Lg))
                    .into_any_element(),
            )],
            Story::Kbd => vec![
                (
                    "Group",
                    div()
                        .flex()
                        .flex_row()
                        .items_center()
                        .gap(px(4.))
                        .text_size(px(14.))
                        .line_height(px(20.))
                        .text_color(theme.muted_foreground)
                        .child("Use")
                        .child(
                            KbdGroup::new()
                                .child(Kbd::new().child("Ctrl + B"))
                                .child(Kbd::new().child("Ctrl + K")),
                        )
                        .child("to open the command palette")
                        .into_any_element(),
                ),
                (
                    "Button",
                    Button::new("kbd-ex-accept")
                        .variant(ButtonVariant::Outline)
                        .icon_inline_end()
                        .child("Accept")
                        .child(
                            div()
                                .relative()
                                .left(px(2.))
                                .child(Kbd::new().child("⏎")),
                        )
                        .into_any_element(),
                ),
                (
                    "Tooltip",
                    ButtonGroup::new()
                        .item(
                            Button::new("kbd-ex-save")
                                .variant(ButtonVariant::Outline)
                                .child("Save")
                                .tooltip_rich(|_, _| {
                                    div()
                                        .flex()
                                        .flex_row()
                                        .items_center()
                                        .gap(px(4.))
                                        .child("Save Changes")
                                        .child(Kbd::new().in_tooltip().child("S"))
                                        .into_any_element()
                                }),
                        )
                        .item(
                            Button::new("kbd-ex-print")
                                .variant(ButtonVariant::Outline)
                                .child("Print")
                                .tooltip_rich(|_, _| {
                                    div()
                                        .flex()
                                        .flex_row()
                                        .items_center()
                                        .gap(px(4.))
                                        .child("Print Document")
                                        .child(
                                            KbdGroup::new()
                                                .child(Kbd::new().in_tooltip().child("Ctrl"))
                                                .child(Kbd::new().in_tooltip().child("P")),
                                        )
                                        .into_any_element()
                                }),
                        )
                        .into_any_element(),
                ),
                (
                    "Input Group",
                    div()
                        .w(px(320.))
                        .child(
                            InputGroup::new(self.kbd_input.clone())
                                .leading(InputGroupAddon::new().child(
                                    gpui::svg()
                                        .path(crate::assets::ICON_SEARCH)
                                        .size(px(16.))
                                        .text_color(theme.muted_foreground),
                                ))
                                .trailing(
                                    InputGroupAddon::new()
                                        .child(Kbd::new().child("⌘"))
                                        .child(Kbd::new().child("K")),
                                ),
                        )
                        .into_any_element(),
                ),
                (
                    "RTL (mirrored)",
                    // gpui has no direction context — mirror the demo composition manually
                    div()
                        .flex()
                        .flex_col()
                        .items_center()
                        .gap(px(16.))
                        .child(
                            KbdGroup::new()
                                .child(Kbd::new().child("⌃"))
                                .child(Kbd::new().child("⌥"))
                                .child(Kbd::new().child("⇧"))
                                .child(Kbd::new().child("⌘")),
                        )
                        .child(
                            KbdGroup::new()
                                .child(Kbd::new().child("B"))
                                .child("+")
                                .child(Kbd::new().child("Ctrl")),
                        )
                        .into_any_element(),
                ),
            ],
            Story::Spinner => vec![(
                "Sizes",
                div()
                    .flex()
                    .flex_row()
                    .items_center()
                    .gap(px(16.))
                    .child(Spinner::new().size(px(16.)))
                    .child(Spinner::new().size(px(24.)))
                    .child(Spinner::new().size(px(32.)))
                    .into_any_element(),
            )],
            Story::Progress => vec![(
                "Values",
                div()
                    .flex()
                    .flex_col()
                    .gap(px(12.))
                    .w(px(320.))
                    .child(Progress::new(25.).show_value())
                    .child(Progress::new(60.).show_value())
                    .child(Progress::new(90.).show_value())
                    .into_any_element(),
            )],
            Story::SliderStory => vec![(
                "Disabled",
                Slider::new("ex-slider-disabled")
                    .value(40.)
                    .disabled(true)
                    .into_any_element(),
            )],
            Story::InputStory => vec![
                ("Basic", self.input_example_basic(cx).into_any_element()),
                ("Field", self.input_example_field(cx).into_any_element()),
                (
                    "Field Group",
                    self.input_example_fieldgroup(cx).into_any_element(),
                ),
                (
                    "Disabled",
                    self.input_example_disabled(cx).into_any_element(),
                ),
                ("Invalid", self.input_example_invalid(cx).into_any_element()),
                ("File", self.input_example_file(cx).into_any_element()),
                ("Inline", self.input_example_inline(cx).into_any_element()),
                ("Grid", self.input_example_grid(cx).into_any_element()),
                (
                    "Required",
                    self.input_example_required(cx).into_any_element(),
                ),
                ("Badge", self.input_example_badge(cx).into_any_element()),
                (
                    "Input Group",
                    self.input_example_input_group(cx).into_any_element(),
                ),
                (
                    "Button Group",
                    self.input_example_button_group(cx).into_any_element(),
                ),
                ("Form", self.input_example_form(cx).into_any_element()),
            ],
            Story::FieldStory => vec![
                (
                    "Input",
                    self.field_example_input(cx).into_any_element(),
                ),
                (
                    "Textarea",
                    self.field_example_textarea(cx).into_any_element(),
                ),
                (
                    "Select",
                    self.field_example_select(cx).into_any_element(),
                ),
                (
                    "Slider",
                    self.field_example_slider(cx).into_any_element(),
                ),
                (
                    "Fieldset",
                    self.field_example_fieldset(cx).into_any_element(),
                ),
                (
                    "Checkbox",
                    self.field_example_checkbox(cx).into_any_element(),
                ),
                (
                    "Radio",
                    self.field_example_radio(cx).into_any_element(),
                ),
                (
                    "Switch",
                    self.field_example_switch(cx).into_any_element(),
                ),
                (
                    "Choice Card",
                    self.field_example_choice_card(cx).into_any_element(),
                ),
                (
                    "Field Group",
                    self.field_example_field_group(cx).into_any_element(),
                ),
                (
                    "Responsive",
                    self.field_example_responsive(cx).into_any_element(),
                ),
                (
                    "Validation",
                    self.field_example_validation(cx).into_any_element(),
                ),
            ],
            Story::ResizableStory => vec![
                (
                    "Vertical",
                    Self::resizable_frame(
                        &theme,
                        ResizablePanelGroup::new("resizable-vertical")
                            .direction(ResizableDirection::Vertical)
                            .panel(
                                ResizablePanel::new()
                                    .default_size(0.25)
                                    .child(Self::resizable_label(&theme, "Header")),
                            )
                            .handle(ResizableHandle::new())
                            .panel(
                                ResizablePanel::new()
                                    .default_size(0.75)
                                    .child(Self::resizable_label(&theme, "Content")),
                            )
                            .into_any_element(),
                    ),
                ),
                (
                    "Handle",
                    Self::resizable_frame(
                        &theme,
                        ResizablePanelGroup::new("resizable-handle")
                            .panel(
                                ResizablePanel::new()
                                    .default_size(0.25)
                                    .child(Self::resizable_label(&theme, "Sidebar")),
                            )
                            .handle(ResizableHandle::new().with_handle(true))
                            .panel(
                                ResizablePanel::new()
                                    .default_size(0.75)
                                    .child(Self::resizable_label(&theme, "Content")),
                            )
                            .into_any_element(),
                    ),
                ),
                (
                    "Collapsible",
                    Self::resizable_frame(
                        &theme,
                        ResizablePanelGroup::new("resizable-collapsible")
                            .panel(
                                ResizablePanel::new()
                                    .default_size(0.3)
                                    .min_size(0.2)
                                    .collapsible(true)
                                    .collapsed_size(0.05)
                                    .child(Self::resizable_label(&theme, "Sidebar")),
                            )
                            .handle(ResizableHandle::new().with_handle(true))
                            .panel(
                                ResizablePanel::new()
                                    .default_size(0.7)
                                    .child(Self::resizable_label(&theme, "Content")),
                            )
                            .into_any_element(),
                    ),
                ),
            ],
            _ => Vec::new(),
        }
    }

    /// The docs "About" blurb shown under the primary example.
    fn story_about(story: Story) -> Option<&'static str> {
        match story {
            Story::ResizableStory => Some(
                "Resizable is a gpui port of shadcn/ui's Resizable, which is built on \
                 react-resizable-panels: panel groups with draggable handles, nested groups, \
                 min/max/collapsible panel sizes, and keyboard resize. Layout state is managed \
                 internally, so groups are resizable with no wiring; pass .on_layout_change(..) \
                 to observe sizes.",
            ),
            _ => None,
        }
    }

    /// Prose under an extra example's heading, mirroring the shadcn docs
    /// section descriptions.
    fn example_description(story: Story, title: &'static str) -> Option<&'static str> {
        match (story, title) {
            (Story::InputStory, "Field") => Some(
                "Use Field, FieldLabel, and FieldDescription to create an input with a \
                 label and description.",
            ),
            (Story::InputStory, "Field Group") => {
                Some("Use FieldGroup to show multiple Field blocks and to build forms.")
            }
            (Story::InputStory, "Disabled") => {
                Some("Use .set_disabled(true) to disable the input.")
            }
            (Story::InputStory, "Invalid") => {
                Some("Use .set_invalid(true) to mark the input as invalid.")
            }
            (Story::InputStory, "File") => Some("Use .set_file(true) to create a file input."),
            (Story::InputStory, "Inline") => Some(
                "Use Field with FieldOrientation::Horizontal to create an inline input. \
                 Pair with Button to create a search input with a button.",
            ),
            (Story::InputStory, "Grid") => {
                Some("Use a grid layout to place multiple inputs side by side.")
            }
            (Story::InputStory, "Required") => {
                Some("Mark required inputs with a destructive asterisk in the label.")
            }
            (Story::InputStory, "Badge") => {
                Some("Use Badge in the label to highlight a recommended field.")
            }
            (Story::InputStory, "Input Group") => Some(
                "To add icons, text, or buttons inside an input, use the InputGroup \
                 component.",
            ),
            (Story::InputStory, "Button Group") => {
                Some("To add buttons to an input, pair it with adjacent Buttons.")
            }
            (Story::InputStory, "Form") => {
                Some("A full form example with multiple inputs, a select, and a button.")
            }
            (Story::ResizableStory, "Vertical") => {
                Some("Use .direction(ResizableDirection::Vertical) for vertical resizing.")
            }
            (Story::ResizableStory, "Handle") => Some(
                "Use .with_handle(true) on ResizableHandle to show a visible handle. \
                 Handles are focusable: arrow keys resize in 10% steps, Home/End jump to the \
                 size limits, and Enter toggles collapse.",
            ),
            (Story::ResizableStory, "Collapsible") => Some(
                "Set .collapsible(true) on a ResizablePanel to let a drag past half its \
                 .min_size(..) snap the panel closed to its .collapsed_size(..). Drag the \
                 sidebar below its minimum to collapse it, or press Enter on the focused \
                 handle.",
            ),
            _ => None,
        }
    }

    fn controls_panel(&self, cx: &mut Context<Self>) -> impl IntoElement + use<> {
        let theme = Theme::of(cx).clone();
        let rows: Vec<AnyElement> = match self.story {
            Story::Tokens => self.token_controls(cx),
            Story::Button => vec![
                Self::control_row(
                    "variant",
                    Self::choices(
                        "button-variant",
                        &BUTTON_VARIANTS,
                        self.button_variant,
                        cx,
                        |this, v, cx| {
                            this.button_variant = v;
                            cx.notify();
                        },
                    ),
                    &theme,
                ),
                Self::control_row(
                    "size",
                    Self::choices(
                        "button-size",
                        &BUTTON_SIZES,
                        self.button_size,
                        cx,
                        |this, v, cx| {
                            this.button_size = v;
                            cx.notify();
                        },
                    ),
                    &theme,
                ),
                Self::control_row(
                    "disabled",
                    Switch::new("button-disabled")
                        .checked(self.button_disabled)
                        .size(SwitchSize::Sm)
                        .on_change(cx.listener(|this, checked: &bool, _, cx| {
                            this.button_disabled = *checked;
                            cx.notify();
                        }))
                        .into_any_element(),
                    &theme,
                ),
            ],
            Story::Badge => vec![Self::control_row(
                "variant",
                Self::choices(
                    "badge-variant",
                    &BADGE_VARIANTS,
                    self.badge_variant,
                    cx,
                    |this, v, cx| {
                        this.badge_variant = v;
                        cx.notify();
                    },
                ),
                &theme,
            )],
            Story::Avatar => vec![Self::control_row(
                "size",
                Self::choices(
                    "avatar-size",
                    &AVATAR_SIZES,
                    self.avatar_size,
                    cx,
                    |this, v, cx| {
                        this.avatar_size = v;
                        cx.notify();
                    },
                ),
                &theme,
            )],
            Story::Switch => vec![
                Self::control_row(
                    "checked",
                    Switch::new("ctl-switch-checked")
                        .checked(self.switch_checked)
                        .size(SwitchSize::Sm)
                        .on_change(cx.listener(|this, checked: &bool, _, cx| {
                            this.switch_checked = *checked;
                            cx.notify();
                        }))
                        .into_any_element(),
                    &theme,
                ),
                Self::control_row(
                    "size",
                    Self::choices(
                        "switch-size",
                        &SWITCH_SIZES,
                        self.switch_size,
                        cx,
                        |this, v, cx| {
                            this.switch_size = v;
                            cx.notify();
                        },
                    ),
                    &theme,
                ),
                Self::control_row(
                    "disabled",
                    Switch::new("ctl-switch-disabled")
                        .checked(self.switch_disabled)
                        .size(SwitchSize::Sm)
                        .on_change(cx.listener(|this, checked: &bool, _, cx| {
                            this.switch_disabled = *checked;
                            cx.notify();
                        }))
                        .into_any_element(),
                    &theme,
                ),
            ],
            Story::Accordion => vec![
                Self::control_row(
                    "multiple",
                    Switch::new("ctl-accordion-multiple")
                        .checked(self.accordion_multiple)
                        .size(SwitchSize::Sm)
                        .on_change(cx.listener(|this, on: &bool, _, cx| {
                            this.accordion_multiple = *on;
                            cx.notify();
                        }))
                        .into_any_element(),
                    &theme,
                ),
                Self::control_row(
                    "disable all",
                    Switch::new("ctl-accordion-root-disabled")
                        .checked(self.accordion_root_disabled)
                        .size(SwitchSize::Sm)
                        .on_change(cx.listener(|this, on: &bool, _, cx| {
                            this.accordion_root_disabled = *on;
                            cx.notify();
                        }))
                        .into_any_element(),
                    &theme,
                ),
                Self::control_row(
                    "disable third item",
                    Switch::new("ctl-accordion-disabled")
                        .checked(self.accordion_disable_third)
                        .size(SwitchSize::Sm)
                        .on_change(cx.listener(|this, on: &bool, _, cx| {
                            this.accordion_disable_third = *on;
                            cx.notify();
                        }))
                        .into_any_element(),
                    &theme,
                ),
            ],
            Story::Separator => Vec::new(),
            Story::Progress => vec![Self::control_row(
                format!("value \u{00b7} {:.0}%", self.progress_value),
                Self::slider(
                    "progress-value",
                    self.progress_value / 100.,
                    cx,
                    |this, f| {
                        this.progress_value = (f * 100.).round();
                    },
                ),
                &theme,
            )],
            Story::Spinner => Vec::new(),
            Story::AspectRatio => Vec::new(),
            Story::Empty => Vec::new(),
            Story::Item => vec![
                Self::control_row(
                    "variant",
                    Self::choices(
                        "item-variant",
                        &[
                            ("default", ItemVariant::Default),
                            ("outline", ItemVariant::Outline),
                            ("muted", ItemVariant::Muted),
                        ],
                        self.item_variant,
                        cx,
                        |this, v, cx| {
                            this.item_variant = v;
                            cx.notify();
                        },
                    ),
                    &theme,
                ),
                Self::control_row(
                    "size",
                    Self::choices(
                        "item-size",
                        &[
                            ("default", ItemSize::Default),
                            ("sm", ItemSize::Sm),
                            ("xs", ItemSize::Xs),
                        ],
                        self.item_size,
                        cx,
                        |this, v, cx| {
                            this.item_size = v;
                            cx.notify();
                        },
                    ),
                    &theme,
                ),
            ],
            Story::Table => Vec::new(),
            Story::Breadcrumb => Vec::new(),
            Story::Checkbox => vec![Self::control_row(
                "checked",
                Switch::new("ctl-checkbox-checked")
                    .checked(self.checkbox_checked)
                    .size(SwitchSize::Sm)
                    .on_change(cx.listener(|this, checked: &bool, _, cx| {
                        this.checkbox_checked = *checked;
                        cx.notify();
                    }))
                    .into_any_element(),
                &theme,
            )],
            Story::RadioGroup => Vec::new(),
            Story::Toggle => Vec::new(),
            Story::ToggleGroup => Vec::new(),
            Story::ButtonGroup => Vec::new(),
            Story::Collapsible => vec![Self::control_row(
                "open",
                Switch::new("ctl-collapsible-open")
                    .checked(self.collapsible_open)
                    .size(SwitchSize::Sm)
                    .on_change(cx.listener(|this, open: &bool, _, cx| {
                        this.collapsible_open = *open;
                        cx.notify();
                    }))
                    .into_any_element(),
                &theme,
            )],
            Story::Tabs => vec![Self::control_row(
                "variant",
                Self::choices(
                    "tabs-variant",
                    &[
                        ("default", TabsVariant::Default),
                        ("line", TabsVariant::Line),
                    ],
                    self.tabs_variant,
                    cx,
                    |this, v, cx| {
                        this.tabs_variant = v;
                        cx.notify();
                    },
                ),
                &theme,
            )],
            Story::SliderStory => Vec::new(),
            Story::PaginationStory => Vec::new(),
            Story::ScrollArea => Vec::new(),
            Story::TooltipStory => Vec::new(),
            Story::HoverCardStory => Vec::new(),
            Story::DialogStory => vec![Self::control_row(
                "open",
                Switch::new("ctl-dialog-open")
                    .checked(self.dialog_open)
                    .size(SwitchSize::Sm)
                    .on_change(cx.listener(|this, open: &bool, _, cx| {
                        this.dialog_open = *open;
                        cx.notify();
                    }))
                    .into_any_element(),
                &theme,
            )],
            Story::AlertDialogStory => Vec::new(),
            Story::SheetStory => vec![Self::control_row(
                "side",
                Self::choices(
                    "sheet-side",
                    &[
                        ("top", SheetSide::Top),
                        ("right", SheetSide::Right),
                        ("bottom", SheetSide::Bottom),
                        ("left", SheetSide::Left),
                    ],
                    self.sheet_side,
                    cx,
                    |this, v, cx| {
                        this.sheet_side = v;
                        cx.notify();
                    },
                ),
                &theme,
            )],
            Story::DrawerStory => Vec::new(),
            Story::DropdownMenuStory => Vec::new(),
            Story::ContextMenuStory => Vec::new(),
            Story::MenubarStory => Vec::new(),
            Story::SelectStory => Vec::new(),
            Story::NativeSelectStory => Vec::new(),
            Story::NavigationMenuStory => Vec::new(),
            Story::ToastStory => Vec::new(),
            Story::InputStory => Vec::new(),
            Story::TextareaStory => Vec::new(),
            Story::FieldStory => Vec::new(),
            Story::InputGroupStory => Vec::new(),
            Story::InputOtpStory => Vec::new(),
            Story::CommandStory => Vec::new(),
            Story::ComboboxStory => Vec::new(),
            Story::CalendarStory => Vec::new(),
            Story::DatePickerStory => Vec::new(),
            Story::CarouselStory => Vec::new(),
            Story::ResizableStory => Vec::new(),
            Story::SidebarStory => Vec::new(),
            Story::DataTableStory => Vec::new(),
            Story::ChartStory => Vec::new(),
            Story::MessageStory => Vec::new(),
            Story::BubbleStory => vec![Self::control_row(
                "variant",
                Self::choices(
                    "bubble-variant",
                    &[
                        ("default", BubbleVariant::Default),
                        ("secondary", BubbleVariant::Secondary),
                        ("muted", BubbleVariant::Muted),
                        ("tinted", BubbleVariant::Tinted),
                        ("outline", BubbleVariant::Outline),
                        ("ghost", BubbleVariant::Ghost),
                    ],
                    self.bubble_variant,
                    cx,
                    |this, v, cx| {
                        this.bubble_variant = v;
                        cx.notify();
                    },
                ),
                &theme,
            )],
            Story::AttachmentStory => Vec::new(),
            Story::MarkerStory => Vec::new(),
            Story::MessageScrollerStory => Vec::new(),
            Story::QuestionnaireStory => Vec::new(),
            // __STORY_CONTROLS__
            Story::Alert => vec![Self::control_row(
                "variant",
                Self::choices(
                    "alert-variant",
                    &[
                        ("default", AlertVariant::Default),
                        ("destructive", AlertVariant::Destructive),
                    ],
                    self.alert_variant,
                    cx,
                    |this, v, cx| {
                        this.alert_variant = v;
                        cx.notify();
                    },
                ),
                &theme,
            )],
            Story::Skeleton => Vec::new(),
            Story::Label => Vec::new(),
            Story::Kbd => Vec::new(),
            Story::Card => vec![Self::control_row(
                "size",
                Self::choices(
                    "card-size",
                    &CARD_SIZES,
                    self.card_size,
                    cx,
                    |this, v, cx| {
                        this.card_size = v;
                        cx.notify();
                    },
                ),
                &theme,
            )],
            Story::Popover => vec![Self::control_row(
                "open",
                Switch::new("ctl-popover-open")
                    .checked(self.popover_open)
                    .size(SwitchSize::Sm)
                    .on_change(cx.listener(|this, open: &bool, _, cx| {
                        this.popover_open = *open;
                        cx.notify();
                    }))
                    .into_any_element(),
                &theme,
            )],
        };

        div()
            .flex()
            .flex_col()
            .w(px(280.))
            .flex_shrink_0()
            .h_full()
            .border_l_1()
            .border_color(theme.border)
            .child(
                div()
                    .px(px(16.))
                    .py(px(14.))
                    .border_b_1()
                    .border_color(theme.border)
                    .text_size(px(13.))
                    .font_weight(FontWeight::MEDIUM)
                    .child("Controls"),
            )
            .child(
                div()
                    .id("controls")
                    .flex()
                    .flex_col()
                    .flex_1()
                    .overflow_y_scroll()
                    .child(
                        div()
                            .flex()
                            .flex_col()
                            .gap(px(16.))
                            .p(px(16.))
                            .children(rows),
                    ),
            )
    }

    // ---- control widgets ---------------------------------------------------

    /// One labeled control row: prop name above the widget.
    fn control_row(label: impl Into<String>, control: AnyElement, theme: &Theme) -> AnyElement {
        div()
            .flex()
            .flex_col()
            .gap(px(6.))
            .child(
                div()
                    .text_size(px(12.))
                    .line_height(px(16.))
                    .font_weight(FontWeight::MEDIUM)
                    .text_color(theme.muted_foreground)
                    .child(label.into()),
            )
            .child(control)
            .into_any_element()
    }

    /// A segmented value picker: one xs button per option, the current value
    /// rendered filled.
    fn choices<T: Copy + PartialEq + 'static>(
        id: &'static str,
        options: &[(&'static str, T)],
        current: T,
        cx: &mut Context<Self>,
        set: impl Fn(&mut Self, T, &mut Context<Self>) + Copy + 'static,
    ) -> AnyElement {
        div()
            .flex()
            .flex_row()
            .flex_wrap()
            .gap(px(4.))
            .children(options.iter().enumerate().map(|(index, (label, value))| {
                let value = *value;
                let selected = value == current;
                Button::new((id, index))
                    .variant(if selected {
                        ButtonVariant::Default
                    } else {
                        ButtonVariant::Outline
                    })
                    .size(ButtonSize::Xs)
                    .on_click(cx.listener(move |this, _, _, cx| set(this, value, cx)))
                    .child(*label)
            }))
            .into_any_element()
    }

    /// A horizontal slider over 0..1. Dragging anywhere (even past the track)
    /// keeps updating; `set` receives the new fraction.
    fn slider(
        id: &'static str,
        fraction: f32,
        cx: &mut Context<Self>,
        set: impl Fn(&mut Self, f32) + Copy + 'static,
    ) -> AnyElement {
        let theme = Theme::of(cx).clone();
        let fraction = fraction.clamp(0., 1.);
        div()
            .id(id)
            .h(px(16.))
            .w_full()
            .flex()
            .items_center()
            .on_drag(SliderDrag(id), |_, _, _, cx| cx.new(|_| DragPreview))
            .on_drag_move(
                cx.listener(move |this, event: &DragMoveEvent<SliderDrag>, _, cx| {
                    if event.drag(cx).0 == id {
                        let f = ((event.event.position.x - event.bounds.origin.x)
                            / event.bounds.size.width)
                            .clamp(0., 1.);
                        set(this, f);
                        this.apply_tokens(cx);
                    }
                }),
            )
            .child(
                div()
                    .relative()
                    .w_full()
                    .h(px(4.))
                    .rounded_full()
                    .bg(theme.input)
                    .child(
                        div()
                            .h_full()
                            .w(relative(fraction))
                            .rounded_full()
                            .bg(theme.primary),
                    )
                    .child(Self::slider_thumb(fraction, &theme)),
            )
            .into_any_element()
    }

    /// The hue slider: same interaction as [`Self::slider`], but the track is
    /// a rainbow.
    fn hue_slider(
        id: &'static str,
        fraction: f32,
        cx: &mut Context<Self>,
        set: impl Fn(&mut Self, f32) + Copy + 'static,
    ) -> AnyElement {
        let theme = Theme::of(cx).clone();
        let fraction = fraction.clamp(0., 1.);
        const SLICES: usize = 16;
        div()
            .id(id)
            .h(px(16.))
            .w_full()
            .flex()
            .items_center()
            .on_drag(SliderDrag(id), |_, _, _, cx| cx.new(|_| DragPreview))
            .on_drag_move(
                cx.listener(move |this, event: &DragMoveEvent<SliderDrag>, _, cx| {
                    if event.drag(cx).0 == id {
                        let f = ((event.event.position.x - event.bounds.origin.x)
                            / event.bounds.size.width)
                            .clamp(0., 1.);
                        set(this, f);
                        this.apply_tokens(cx);
                    }
                }),
            )
            .child(
                div()
                    .relative()
                    .w_full()
                    .h(px(4.))
                    .flex()
                    .flex_row()
                    .children((0..SLICES).map(|i| {
                        div()
                            .flex_1()
                            .h_full()
                            .bg(hsla(i as f32 / SLICES as f32, 0.8, 0.55, 1.))
                            .when(i == 0, |el| el.rounded_l_full())
                            .when(i == SLICES - 1, |el| el.rounded_r_full())
                    }))
                    .child(Self::slider_thumb(fraction, &theme)),
            )
            .into_any_element()
    }

    fn slider_thumb(fraction: f32, theme: &Theme) -> AnyElement {
        div()
            .absolute()
            .top(px(-4.))
            .left(relative(fraction))
            .ml(px(-6.))
            .size(px(12.))
            .rounded_full()
            .bg(theme.background)
            .border_1()
            .border_color(theme.ring)
            .shadow_xs()
            .into_any_element()
    }

    // ---- token (global) controls -------------------------------------------

    fn token_controls(&self, cx: &mut Context<Self>) -> Vec<AnyElement> {
        let theme = Theme::of(cx).clone();

        let base_row = Self::choices(
            "base-color",
            &BaseColor::ALL.map(|b| (b.label(), b)),
            self.tokens.base,
            cx,
            |this, v, cx| {
                this.clear_import();
                this.tokens.base = v;
                this.apply_tokens(cx);
            },
        );

        let current_primary = self.tokens.primary(false);
        let preset_row = div()
            .flex()
            .flex_row()
            .flex_wrap()
            .gap(px(6.))
            .children(
                THEME_PRESETS
                    .iter()
                    .enumerate()
                    .map(|(index, (_name, preset))| {
                        let swatch: Hsla = match preset {
                            Some((l, c, h)) => oklch(*l, *c, *h),
                            None => {
                                if theme.dark {
                                    rgb(0xe5e5e5).into()
                                } else {
                                    rgb(0x000000).into()
                                }
                            }
                        };
                        let selected = match (preset, current_primary) {
                            (None, None) => true,
                            (Some((l, c, h)), Some(current)) => {
                                let p = oklch(*l, *c, *h);
                                (p.h - current.h).abs() < 0.01 && (p.l - current.l).abs() < 0.02
                            }
                            _ => false,
                        };
                        let value = *preset;
                        div()
                            .id(("theme-preset", index))
                            .size(px(22.))
                            .rounded_full()
                            .bg(swatch)
                            .border_2()
                            .border_color(if selected { theme.ring } else { theme.border })
                            .on_click(cx.listener(move |this, _, _, cx| {
                                this.clear_import();
                                match value {
                                    Some((l, c, h)) => {
                                        let p = oklch(l, c, h);
                                        this.tokens.custom_primary = true;
                                        this.tokens.hue = p.h;
                                        this.tokens.saturation = p.s;
                                        this.tokens.lightness = p.l;
                                    }
                                    None => this.tokens.custom_primary = false,
                                }
                                this.apply_tokens(cx);
                            }))
                    }),
            )
            .into_any_element();

        let hue = self.tokens.hue;
        let saturation = self.tokens.saturation;
        let lightness = self.tokens.lightness;
        let radius = self.tokens.radius;

        let import_row = div()
            .flex()
            .flex_col()
            .gap(px(6.))
            .child(
                Button::new("import-theme")
                    .variant(ButtonVariant::Outline)
                    .size(ButtonSize::Sm)
                    .on_click(cx.listener(|this, _, _, cx| this.import_from_clipboard(cx)))
                    .child("Paste from clipboard"),
            )
            .when_some(self.import_status.clone(), |el, (message, success)| {
                el.child(
                    div()
                        .text_size(px(11.))
                        .line_height(px(15.))
                        .text_color(if success {
                            theme.muted_foreground
                        } else {
                            theme.destructive
                        })
                        .child(message),
                )
            })
            .into_any_element();

        vec![
            Self::control_row("import theme css", import_row, &theme),
            Self::control_row("base color", base_row, &theme),
            Self::control_row("theme", preset_row, &theme),
            Self::control_row(
                format!("hue · {:.0}°", hue * 360.),
                Self::hue_slider("hue-slider", hue, cx, |this, f| {
                    this.clear_import();
                    this.tokens.custom_primary = true;
                    this.tokens.hue = f;
                }),
                &theme,
            ),
            Self::control_row(
                format!("saturation · {:.0}%", saturation * 100.),
                Self::slider("saturation-slider", saturation, cx, |this, f| {
                    this.clear_import();
                    this.tokens.custom_primary = true;
                    this.tokens.saturation = f;
                }),
                &theme,
            ),
            Self::control_row(
                format!("lightness · {:.0}%", lightness * 100.),
                Self::slider("lightness-slider", lightness, cx, |this, f| {
                    this.clear_import();
                    this.tokens.custom_primary = true;
                    this.tokens.lightness = f;
                }),
                &theme,
            ),
            Self::control_row(
                format!("radius · {:.0}px", radius),
                Self::slider("radius-slider", radius / 24., cx, |this, f| {
                    this.clear_import();
                    this.tokens.radius = (f * 24.).round();
                }),
                &theme,
            ),
            Self::control_row(
                "font",
                Self::choices(
                    "font-sans",
                    &FONTS,
                    self.tokens.font_sans,
                    cx,
                    |this, v, cx| {
                        this.tokens.font_sans = v;
                        this.apply_tokens(cx);
                    },
                ),
                &theme,
            ),
            Self::control_row(
                "heading font",
                Self::choices(
                    "font-heading",
                    &FONTS,
                    self.tokens.font_heading,
                    cx,
                    |this, v, cx| {
                        this.tokens.font_heading = v;
                        this.apply_tokens(cx);
                    },
                ),
                &theme,
            ),
            Self::control_row(
                "icon library",
                Self::choices(
                    "icon-library",
                    &IconLibrary::ALL.map(|lib| (lib.label(), lib)),
                    self.tokens.icons,
                    cx,
                    |this, v, cx| {
                        this.tokens.icons = v;
                        this.apply_tokens(cx);
                    },
                ),
                &theme,
            ),
            div()
                .flex()
                .flex_row()
                .gap(px(8.))
                .pt(px(4.))
                .child(
                    Button::new("shuffle-tokens")
                        .variant(ButtonVariant::Default)
                        .size(ButtonSize::Sm)
                        .on_click(cx.listener(|this, _, _, cx| this.shuffle(cx)))
                        .child("Shuffle"),
                )
                .child(
                    Button::new("reset-tokens")
                        .variant(ButtonVariant::Outline)
                        .size(ButtonSize::Sm)
                        .on_click(cx.listener(|this, _, _, cx| {
                            this.clear_import();
                            this.tokens = TokenSettings::default();
                            this.apply_tokens(cx);
                        }))
                        .child("Reset"),
                )
                .into_any_element(),
        ]
    }

    // ---- stories -----------------------------------------------------------

    #[allow(dead_code)]
    fn tokens_preview(&self, cx: &mut Context<Self>) -> impl IntoElement + use<> {
        let theme = Theme::of(cx).clone();
        let swatches = [
            ("background", theme.background),
            ("foreground", theme.foreground),
            ("card", theme.card),
            ("card-foreground", theme.card_foreground),
            ("popover", theme.popover),
            ("popover-foreground", theme.popover_foreground),
            ("primary", theme.primary),
            ("primary-foreground", theme.primary_foreground),
            ("secondary", theme.secondary),
            ("secondary-foreground", theme.secondary_foreground),
            ("muted", theme.muted),
            ("muted-foreground", theme.muted_foreground),
            ("accent", theme.accent),
            ("accent-foreground", theme.accent_foreground),
            ("destructive", theme.destructive),
            ("destructive-foreground", theme.destructive_foreground),
            ("border", theme.border),
            ("input", theme.input),
            ("ring", theme.ring),
        ];
        let radii = [
            ("sm", theme.radius_sm()),
            ("md", theme.radius_md()),
            ("lg", theme.radius_lg()),
            ("xl", theme.radius_xl()),
        ];
        div()
            .flex()
            .flex_col()
            .gap(px(24.))
            .max_w(px(480.))
            .child(div().flex().flex_row().flex_wrap().gap(px(8.)).children(
                swatches.into_iter().map(|(name, color)| {
                    div()
                        .flex()
                        .flex_col()
                        .items_center()
                        .gap(px(4.))
                        .w(px(72.))
                        .child(
                            div()
                                .size(px(32.))
                                .rounded(theme.radius_sm())
                                .border_1()
                                .border_color(theme.border)
                                .bg(color),
                        )
                        .child(
                            div()
                                .text_size(px(10.))
                                .line_height(px(14.))
                                .text_color(theme.muted_foreground)
                                .child(name),
                        )
                }),
            ))
            .child(div().flex().flex_row().items_end().gap(px(12.)).children(
                radii.into_iter().map(|(name, radius)| {
                    div()
                        .flex()
                        .flex_col()
                        .items_center()
                        .gap(px(4.))
                        .child(
                            div()
                                .size(px(40.))
                                .rounded(radius)
                                .border_1()
                                .border_color(theme.ring)
                                .bg(theme.muted),
                        )
                        .child(
                            div()
                                .text_size(px(11.))
                                .text_color(theme.muted_foreground)
                                .child(name),
                        )
                }),
            ))
            // Live sample so slider feedback is instant without switching
            // stories.
            .child(
                div()
                    .flex()
                    .flex_row()
                    .flex_wrap()
                    .items_center()
                    .gap(px(12.))
                    .pt(px(8.))
                    .border_t_1()
                    .border_color(theme.border)
                    .child(Button::new("tokens-button").child("Button"))
                    .child(
                        Button::new("tokens-outline")
                            .variant(ButtonVariant::Outline)
                            .child("Outline"),
                    )
                    .child(Badge::new().child("Badge"))
                    .child(
                        Switch::new("tokens-switch")
                            .checked(self.switch_checked)
                            .on_change(cx.listener(|this, checked: &bool, _, cx| {
                                this.switch_checked = *checked;
                                cx.notify();
                            })),
                    )
                    .child(Avatar::new("CN")),
            )
    }

    fn button_preview(&self, cx: &App) -> impl IntoElement + use<> {
        let theme = Theme::of(cx);
        let icon_only = matches!(
            self.button_size,
            ButtonSize::Icon | ButtonSize::IconXs | ButtonSize::IconSm | ButtonSize::IconLg
        );
        let button = Button::new("preview-button")
            .variant(self.button_variant)
            .size(self.button_size)
            .disabled(self.button_disabled);
        if icon_only {
            button.child(Icon::new(theme.icons.chevron_right()))
        } else {
            button.child("Button")
        }
    }

    fn badge_preview(&self) -> impl IntoElement + use<> {
        Badge::new().variant(self.badge_variant).child("Badge")
    }

    fn avatar_preview(&self) -> impl IntoElement + use<> {
        div()
            .flex()
            .flex_col()
            .items_center()
            .gap(px(24.))
            .child(Avatar::new("CN").size(self.avatar_size))
            .child(
                AvatarGroup::new()
                    .size(self.avatar_size)
                    .child(Avatar::new("CN").size(self.avatar_size))
                    .child(Avatar::new("ER").size(self.avatar_size))
                    .child(Avatar::new("LR").size(self.avatar_size))
                    .child(AvatarGroupCount::new(3).size(self.avatar_size)),
            )
    }

    fn switch_preview(&self, cx: &mut Context<Self>) -> impl IntoElement + use<> {
        Switch::new("preview-switch")
            .checked(self.switch_checked)
            .size(self.switch_size)
            .disabled(self.switch_disabled)
            .on_change(cx.listener(|this, checked: &bool, _, cx| {
                this.switch_checked = *checked;
                cx.notify();
            }))
    }

    fn accordion_preview(&self, _cx: &mut Context<Self>) -> impl IntoElement + use<> {
        div().w(px(384.)).child(
            Accordion::new("accordion-demo")
                .default_value(["item-1"])
                .multiple(self.accordion_multiple)
                .disabled(self.accordion_root_disabled)
                .child(
                    AccordionItem::new("item-1")
                        .child(AccordionTrigger::new().child("Product Information"))
                        .child(AccordionContent::new().child(
                            "Our flagship product combines cutting-edge technology with sleek \
                             design. Built with premium materials, it offers unparalleled \
                             performance and reliability.",
                        )),
                )
                .child(
                    AccordionItem::new("item-2")
                        .child(AccordionTrigger::new().child("Shipping Details"))
                        .child(AccordionContent::new().child(
                            "We offer worldwide shipping through trusted courier partners. \
                             Standard delivery takes 3-5 business days, while express shipping \
                             ensures delivery within 1-2 business days.",
                        )),
                )
                .child(
                    AccordionItem::new("item-3")
                        .child(AccordionTrigger::new().child("Return Policy"))
                        .child(AccordionContent::new().child(
                            "We stand behind our products with a comprehensive 30-day return \
                             policy. If you're not completely satisfied, simply return the item \
                             in its original condition.",
                        ))
                        .disabled(self.accordion_disable_third),
                ),
        )
    }

    fn separator_preview(cx: &App) -> impl IntoElement + use<> {
        let theme = Theme::of(cx).clone();
        div()
            .flex()
            .flex_col()
            .gap(px(16.))
            .w(px(288.))
            .child(
                div()
                    .flex()
                    .flex_col()
                    .gap(px(4.))
                    .child(
                        div()
                            .text_size(px(14.))
                            .line_height(px(20.))
                            .font_weight(FontWeight::MEDIUM)
                            .child("rcn"),
                    )
                    .child(
                        div()
                            .text_size(px(14.))
                            .line_height(px(20.))
                            .text_color(theme.muted_foreground)
                            .child("A copy-paste component library for gpui."),
                    ),
            )
            .child(Separator::new())
            .child(
                div()
                    .flex()
                    .flex_row()
                    .items_center()
                    .gap(px(16.))
                    .h(px(20.))
                    .text_size(px(14.))
                    .child("Blog")
                    .child(Separator::vertical())
                    .child("Docs")
                    .child(Separator::vertical())
                    .child("Source"),
            )
    }

    fn skeleton_preview() -> impl IntoElement + use<> {
        // Mirrors the shadcn docs example: avatar row + card-shaped block.
        div()
            .flex()
            .flex_col()
            .gap(px(24.))
            .child(
                div()
                    .flex()
                    .flex_row()
                    .items_center()
                    .gap(px(16.))
                    .child(Skeleton::new().w(px(48.)).h(px(48.)).rounded_full())
                    .child(
                        div()
                            .flex()
                            .flex_col()
                            .gap(px(8.))
                            .child(Skeleton::new().w(px(200.)).h(px(16.)))
                            .child(Skeleton::new().w(px(160.)).h(px(16.))),
                    ),
            )
            .child(
                div()
                    .flex()
                    .flex_col()
                    .gap(px(8.))
                    .child(Skeleton::new().w(px(200.)).h(px(100.)))
                    .child(Skeleton::new().w(px(200.)).h(px(16.)))
                    .child(Skeleton::new().w(px(160.)).h(px(16.))),
            )
    }

    fn label_preview() -> impl IntoElement + use<> {
        div()
            .flex()
            .flex_col()
            .gap(px(12.))
            .child(
                div()
                    .flex()
                    .flex_row()
                    .items_center()
                    .gap(px(8.))
                    .child(Switch::new("label-switch").checked(true))
                    .child(Label::new().child("Airplane Mode")),
            )
            .child(Label::new().disabled(true).child("Disabled label"))
    }

    fn kbd_preview() -> impl IntoElement + use<> {
        // kbd-demo: ⌘⇧⌥⌃ group + Ctrl + B group
        div()
            .flex()
            .flex_col()
            .items_center()
            .gap(px(16.))
            .child(
                KbdGroup::new()
                    .child(Kbd::new().child("⌘"))
                    .child(Kbd::new().child("⇧"))
                    .child(Kbd::new().child("⌥"))
                    .child(Kbd::new().child("⌃")),
            )
            .child(
                KbdGroup::new()
                    .child(Kbd::new().child("Ctrl"))
                    .child("+")
                    .child(Kbd::new().child("B")),
            )
    }

    fn alert_preview(&self) -> impl IntoElement + use<> {
        div()
            .flex()
            .flex_col()
            .gap(px(12.))
            .w(px(384.))
            .child(
                Alert::new()
                    .variant(self.alert_variant)
                    .icon(crate::assets::ICON_CIRCLE_CHECK)
                    .child(AlertTitle::new().child("Success! Your changes have been saved"))
                    .child(
                        AlertDescription::new()
                            .child("This is an alert with icon, title and description."),
                    ),
            )
            .child(
                Alert::new()
                    .variant(self.alert_variant)
                    .icon(crate::assets::ICON_CIRCLE_ALERT)
                    .child(AlertTitle::new().child("This one has an icon and a title only.")),
            )
    }

    fn progress_preview(&self) -> impl IntoElement + use<> {
        div().w(px(288.)).child(
            Progress::new(self.progress_value)
                .label("Uploading\u{2026}")
                .show_value(),
        )
    }

    fn spinner_preview(cx: &App) -> impl IntoElement + use<> {
        let theme = Theme::of(cx).clone();
        div()
            .flex()
            .flex_row()
            .items_center()
            .gap(px(16.))
            .child(Spinner::new())
            .child(Spinner::new().size(px(24.)))
            .child(Spinner::new().size(px(32.)).color(theme.muted_foreground))
            .child(
                Button::new("spinner-button")
                    .disabled(true)
                    .child(Spinner::new().size(px(16.)).color(theme.primary_foreground))
                    .child("Please wait"),
            )
    }

    fn aspect_ratio_preview(cx: &App) -> impl IntoElement + use<> {
        let theme = Theme::of(cx).clone();
        div().w(px(384.)).child(
            AspectRatio::new(16. / 9.).child(
                div()
                    .size_full()
                    .rounded(theme.radius_lg())
                    .bg(theme.muted)
                    .flex()
                    .items_center()
                    .justify_center()
                    .text_size(px(13.))
                    .text_color(theme.muted_foreground)
                    .child("16 : 9"),
            ),
        )
    }

    fn empty_preview(cx: &App) -> impl IntoElement + use<> {
        let theme = Theme::of(cx).clone();
        div().w(px(420.)).child(
            Empty::new()
                .child(
                    EmptyHeader::new()
                        .child(
                            EmptyMedia::new().variant(EmptyMediaVariant::Icon).child(
                                gpui::svg()
                                    .path(theme.icons.chevron_right())
                                    .size(px(24.))
                                    .text_color(theme.foreground),
                            ),
                        )
                        .child(EmptyTitle::new().child("No Projects Yet"))
                        .child(EmptyDescription::new().child(
                            "You haven't created any projects yet. Get started by creating your first project.",
                        )),
                )
                .child(
                    EmptyContent::new().child(
                        div()
                            .flex()
                            .flex_row()
                            .gap(px(8.))
                            .child(Button::new("empty-create").child("Create Project"))
                            .child(
                                Button::new("empty-import")
                                    .variant(ButtonVariant::Outline)
                                    .child("Import Project"),
                            ),
                    ),
                ),
        )
    }

    fn item_preview(&self, cx: &App) -> impl IntoElement + use<> {
        let theme = Theme::of(cx).clone();
        div().w(px(420.)).child(
            ItemGroup::new()
                .child(
                    Item::new()
                        .variant(self.item_variant)
                        .size(self.item_size)
                        .child(
                            ItemMedia::new().variant(ItemMediaVariant::Icon).child(
                                gpui::svg()
                                    .path(theme.icons.chevron_right())
                                    .size(px(16.))
                                    .text_color(theme.foreground),
                            ),
                        )
                        .child(
                            ItemContent::new()
                                .child(ItemTitle::new().child("Basic Item"))
                                .child(
                                    ItemDescription::new()
                                        .child("A simple item with title and description."),
                                ),
                        )
                        .child(
                            ItemActions::new().child(
                                Button::new("item-action")
                                    .variant(ButtonVariant::Outline)
                                    .size(ButtonSize::Sm)
                                    .child("Action"),
                            ),
                        ),
                )
                .child(ItemSeparator::new())
                .child(
                    Item::new()
                        .variant(self.item_variant)
                        .child(ItemMedia::new().child(Avatar::new("CN")))
                        .child(
                            ItemContent::new()
                                .child(ItemTitle::new().child("Evil Rabbit"))
                                .child(ItemDescription::new().child("Last seen 5 months ago")),
                        )
                        .child(
                            ItemActions::new().child(
                                Button::new("item-add")
                                    .variant(ButtonVariant::Outline)
                                    .size(ButtonSize::IconSm)
                                    .child(
                                        gpui::svg()
                                            .path(theme.icons.chevron_right())
                                            .size(px(16.))
                                            .text_color(theme.foreground),
                                    ),
                            ),
                        ),
                )
                .child(
                    Item::new()
                        .variant(self.item_variant)
                        .size(self.item_size)
                        .child(
                            ItemHeader::new()
                                .child(ItemTitle::new().child("Deployment"))
                                .child(Badge::new().variant(BadgeVariant::Secondary).child("Live")),
                        )
                        .child(ItemContent::new().child(
                            ItemDescription::new().child("Deployed 2 hours ago by evil rabbit."),
                        ))
                        .child(
                            ItemFooter::new()
                                .child(
                                    div()
                                        .text_size(px(12.))
                                        .text_color(theme.muted_foreground)
                                        .child("main / a1b2c3d"),
                                )
                                .child(
                                    Button::new("item-rollback")
                                        .variant(ButtonVariant::Ghost)
                                        .size(ButtonSize::Xs)
                                        .child("Rollback"),
                                ),
                        ),
                ),
        )
    }

    fn table_preview(&self, cx: &mut Context<Self>) -> impl IntoElement + use<> {
        let invoices = [
            ("INV001", "Paid", "Credit Card", "$250.00"),
            ("INV002", "Pending", "PayPal", "$150.00"),
            ("INV003", "Unpaid", "Bank Transfer", "$350.00"),
            ("INV004", "Paid", "Credit Card", "$450.00"),
        ];
        let count = invoices.len();
        div()
            .w(px(480.))
            .child(
                Table::new()
                    .child(
                        TableHeader::new().child(
                            TableRow::new()
                                .child(TableHead::new().w(px(100.)).child("Invoice"))
                                .child(TableHead::new().child("Status"))
                                .child(TableHead::new().child("Method"))
                                .child(TableHead::new().w(px(100.)).child("Amount")),
                        ),
                    )
                    .child(
                        TableBody::new().children(invoices.into_iter().enumerate().map(
                            |(index, (invoice, status, method, amount))| {
                                TableRow::new()
                                    .id(("table-row", index))
                                    .selected(self.table_selected == Some(index))
                                    .last(index + 1 == count)
                                    .child(TableCell::new().w(px(100.)).child(invoice))
                                    .child(TableCell::new().child(status))
                                    .child(TableCell::new().child(method))
                                    .child(TableCell::new().w(px(100.)).child(amount))
                                    .into_any_element()
                            },
                        )),
                    )
                    .child(
                        TableFooter::new().child(
                            TableRow::new()
                                .last(true)
                                .child(TableCell::new().child("Total"))
                                .child(TableCell::new().w(px(100.)).child("$1,200.00")),
                        ),
                    )
                    .child(TableCaption::new().child("A list of your recent invoices.")),
            )
            .id("table-click-catcher")
            .on_click(cx.listener(|this, _, _, cx| {
                this.table_selected = match this.table_selected {
                    Some(i) => Some((i + 1) % 4),
                    None => Some(0),
                };
                cx.notify();
            }))
    }

    fn breadcrumb_preview(&self, cx: &mut Context<Self>) -> impl IntoElement + use<> {
        Breadcrumb::new().child(
            BreadcrumbList::new()
                .child(
                    BreadcrumbItem::new().child(
                        BreadcrumbLink::new("bc-home")
                            .on_click(cx.listener(|this, _, _, cx| {
                                this.story = Story::Tokens;
                                cx.notify();
                            }))
                            .child("Home"),
                    ),
                )
                .child(BreadcrumbSeparator::new())
                .child(BreadcrumbItem::new().child(BreadcrumbEllipsis::new()))
                .child(BreadcrumbSeparator::new())
                .child(
                    BreadcrumbItem::new()
                        .child(BreadcrumbLink::new("bc-components").child("Components")),
                )
                .child(BreadcrumbSeparator::new())
                .child(BreadcrumbItem::new().child(BreadcrumbPage::new().child("Breadcrumb"))),
        )
    }

    fn checkbox_preview(&self, cx: &mut Context<Self>) -> impl IntoElement + use<> {
        div()
            .flex()
            .flex_col()
            .gap(px(12.))
            .child(
                div()
                    .flex()
                    .flex_row()
                    .items_center()
                    .gap(px(8.))
                    .child(
                        Checkbox::new("checkbox-terms")
                            .checked(self.checkbox_checked)
                            .on_change(cx.listener(|this, checked: &bool, _, cx| {
                                this.checkbox_checked = *checked;
                                cx.notify();
                            })),
                    )
                    .child(Label::new().child("Accept terms and conditions")),
            )
            .child(
                div()
                    .flex()
                    .flex_row()
                    .items_center()
                    .gap(px(8.))
                    .child(Checkbox::new("checkbox-disabled").disabled(true))
                    .child(Label::new().disabled(true).child("Disabled")),
            )
            .child(
                div()
                    .flex()
                    .flex_row()
                    .items_center()
                    .gap(px(8.))
                    .child(
                        Checkbox::new("checkbox-disabled-checked")
                            .checked(true)
                            .disabled(true),
                    )
                    .child(Label::new().disabled(true).child("Disabled checked")),
            )
    }
    fn radio_group_preview(&self, cx: &mut Context<Self>) -> impl IntoElement + use<> {
        let options = ["Default", "Comfortable", "Compact"];
        RadioGroup::new()
            .child(
                div()
                    .flex()
                    .flex_row()
                    .items_center()
                    .gap(px(8.))
                    .child(RadioGroupItem::new("radio-disabled").disabled(true))
                    .child(Label::new().disabled(true).child("Disabled option")),
            )
            .children(options.into_iter().enumerate().map(|(index, label)| {
                div()
                    .flex()
                    .flex_row()
                    .items_center()
                    .gap(px(8.))
                    .child(
                        RadioGroupItem::new(("radio-item", index))
                            .checked(self.radio_selected == index)
                            .on_select(cx.listener(move |this, _, _, cx| {
                                this.radio_selected = index;
                                cx.notify();
                            })),
                    )
                    .child(Label::new().child(label))
            }))
    }
    fn toggle_preview(&self, cx: &mut Context<Self>) -> impl IntoElement + use<> {
        let theme = Theme::of(cx).clone();
        div()
            .flex()
            .flex_row()
            .items_center()
            .gap(px(8.))
            .child(
                Toggle::new("toggle-italic")
                    .pressed(self.toggle_pressed)
                    .on_change(cx.listener(|this, pressed: &bool, _, cx| {
                        this.toggle_pressed = *pressed;
                        cx.notify();
                    }))
                    .child("Italic"),
            )
            .child(
                Toggle::new("toggle-outline")
                    .variant(ToggleVariant::Outline)
                    .pressed(self.toggle_outline_pressed)
                    .on_change(cx.listener(|this, pressed: &bool, _, cx| {
                        this.toggle_outline_pressed = *pressed;
                        cx.notify();
                    }))
                    .child("Outline"),
            )
            .child(
                Toggle::new("toggle-icon")
                    .size(ToggleSize::Sm)
                    .pressed(self.toggle_pressed)
                    .on_change(cx.listener(|this, pressed: &bool, _, cx| {
                        this.toggle_pressed = *pressed;
                        cx.notify();
                    }))
                    .child(
                        gpui::svg()
                            .path(theme.icons.chevron_down())
                            .size(px(16.))
                            .text_color(theme.foreground),
                    ),
            )
            .child(
                Toggle::new("toggle-disabled")
                    .size(ToggleSize::Lg)
                    .disabled(true)
                    .child("Disabled"),
            )
    }
    fn toggle_group_preview(&self, cx: &mut Context<Self>) -> impl IntoElement + use<> {
        let labels = ["Bold", "Italic", "Underline"];
        div()
            .flex()
            .flex_col()
            .gap(px(16.))
            .child(
                ToggleGroup::new()
                    .variant(ToggleVariant::Outline)
                    .size(ToggleSize::Sm)
                    .item(ToggleGroupItem::new("tg-sm-a").pressed(true).child("Sm"))
                    .item(
                        ToggleGroupItem::new("tg-sm-b")
                            .disabled(true)
                            .child("Disabled"),
                    ),
            )
            .children(
                [ToggleVariant::Default, ToggleVariant::Outline].map(|variant| {
                    let mut group = ToggleGroup::new().variant(variant);
                    for (index, label) in labels.into_iter().enumerate() {
                        let on = self.toggle_group_on[index];
                        group = group.item(
                            ToggleGroupItem::new((
                                if variant == ToggleVariant::Outline {
                                    "tg-outline"
                                } else {
                                    "tg-default"
                                },
                                index,
                            ))
                            .pressed(on)
                            .on_change(cx.listener(move |this, pressed: &bool, _, cx| {
                                this.toggle_group_on[index] = *pressed;
                                cx.notify();
                            }))
                            .child(label),
                        );
                    }
                    group
                }),
            )
    }
    fn button_group_preview(cx: &App) -> impl IntoElement + use<> {
        let theme = Theme::of(cx).clone();
        div()
            .flex()
            .flex_col()
            .items_start()
            .gap(px(16.))
            .child(
                ButtonGroup::new()
                    .item(
                        Button::new("bg-archive")
                            .variant(ButtonVariant::Outline)
                            .child("Archive"),
                    )
                    .item(
                        Button::new("bg-report")
                            .variant(ButtonVariant::Outline)
                            .child("Report"),
                    )
                    .item(
                        Button::new("bg-snooze")
                            .variant(ButtonVariant::Outline)
                            .child("Snooze"),
                    ),
            )
            .child(
                div()
                    .flex()
                    .flex_row()
                    .items_center()
                    .gap(px(8.))
                    .child(
                        ButtonGroup::new()
                            .item(
                                Button::new("bg-follow")
                                    .variant(ButtonVariant::Outline)
                                    .child("Follow"),
                            )
                            .item(
                                Button::new("bg-follow-more")
                                    .variant(ButtonVariant::Outline)
                                    .size(ButtonSize::Icon)
                                    .child(
                                        gpui::svg()
                                            .path(theme.icons.chevron_down())
                                            .size(px(16.))
                                            .text_color(theme.foreground),
                                    ),
                            ),
                    )
                    .child(ButtonGroupSeparator::new())
                    .child(ButtonGroupText::new().child("12 followers")),
            )
    }
    fn collapsible_preview(&self, cx: &mut Context<Self>) -> impl IntoElement + use<> {
        let theme = Theme::of(cx).clone();
        let repo_row = |text: &'static str| {
            div()
                .rounded(theme.radius_md())
                .border_1()
                .border_color(theme.border)
                .px(px(16.))
                .py(px(8.))
                .text_size(px(14.))
                .line_height(px(20.))
                .child(text)
        };
        div().w(px(350.)).child(
            Collapsible::new("collapsible-repos")
                .open(self.collapsible_open)
                .on_toggle(cx.listener(|this, _, _, cx| {
                    this.collapsible_open = !this.collapsible_open;
                    cx.notify();
                }))
                .trigger(
                    div()
                        .flex()
                        .flex_row()
                        .items_center()
                        .justify_between()
                        .gap(px(16.))
                        .child(
                            div()
                                .text_size(px(14.))
                                .font_weight(FontWeight::MEDIUM)
                                .child("@peduarte starred 3 repositories"),
                        )
                        .child(
                            gpui::svg()
                                .path(if self.collapsible_open {
                                    theme.icons.chevron_up()
                                } else {
                                    theme.icons.chevron_down()
                                })
                                .size(px(16.))
                                .text_color(theme.muted_foreground),
                        ),
                )
                .content(
                    div()
                        .flex()
                        .flex_col()
                        .gap(px(8.))
                        .child(repo_row("@radix-ui/primitives"))
                        .child(repo_row("@radix-ui/colors"))
                        .child(repo_row("@stitches/react")),
                ),
        )
    }
    fn tabs_preview(&self, cx: &mut Context<Self>) -> impl IntoElement + use<> {
        let theme = Theme::of(cx).clone();
        let panel = |title: &'static str, body: &'static str| {
            Card::new()
                .size(CardSize::Sm)
                .child(
                    CardHeader::new()
                        .size(CardSize::Sm)
                        .child(CardTitle::new().child(title))
                        .child(CardDescription::new().child(body)),
                )
                .child(
                    CardContent::new().size(CardSize::Sm).child(
                        div()
                            .h(px(48.))
                            .w_full()
                            .rounded(theme.radius_md())
                            .bg(theme.muted),
                    ),
                )
        };
        div().w(px(400.)).child(
            Tabs::new()
                .child(
                    TabsList::new()
                        .variant(self.tabs_variant)
                        .trigger(
                            TabsTrigger::new("tab-account")
                                .active(self.tabs_active == 0)
                                .on_select(cx.listener(|this, _, _, cx| {
                                    this.tabs_active = 0;
                                    cx.notify();
                                }))
                                .child("Account"),
                        )
                        .trigger(
                            TabsTrigger::new("tab-billing")
                                .disabled(true)
                                .child("Billing"),
                        )
                        .trigger(
                            TabsTrigger::new("tab-password")
                                .active(self.tabs_active == 1)
                                .on_select(cx.listener(|this, _, _, cx| {
                                    this.tabs_active = 1;
                                    cx.notify();
                                }))
                                .child("Password"),
                        ),
                )
                .child(TabsContent::new().child(if self.tabs_active == 0 {
                    panel(
                        "Account",
                        "Make changes to your account here. Click save when you're done.",
                    )
                } else {
                    panel(
                        "Password",
                        "Change your password here. After saving, you'll be logged out.",
                    )
                })),
        )
    }
    fn slider_preview(&self, cx: &mut Context<Self>) -> impl IntoElement + use<> {
        div()
            .flex()
            .flex_col()
            .gap(px(24.))
            .w(px(288.))
            .child(
                Slider::new("slider-demo")
                    .value(self.slider_value)
                    .on_change(cx.listener(|this, value: &f32, _, cx| {
                        this.slider_value = *value;
                        cx.notify();
                    })),
            )
            .child(
                div()
                    .text_size(px(13.))
                    .text_color(Theme::of(cx).muted_foreground)
                    .child(format!("value: {:.0}", self.slider_value)),
            )
            .child(
                Slider::new("slider-fine")
                    .min(0.)
                    .max(1.)
                    .step(0.01)
                    .value(self.slider_fine)
                    .on_change(cx.listener(|this, value: &f32, _, cx| {
                        this.slider_fine = *value;
                        cx.notify();
                    })),
            )
            .child(Slider::new("slider-disabled").value(30.).disabled(true))
    }
    fn pagination_preview(&self, cx: &mut Context<Self>) -> impl IntoElement + use<> {
        Pagination::new()
            .child(
                PaginationPrevious::new("page-prev").on_click(cx.listener(|this, _, _, cx| {
                    this.pagination_page = this.pagination_page.saturating_sub(1).max(1);
                    cx.notify();
                })),
            )
            .children((1..=3).map(|page| {
                PaginationLink::new(("page-link", page), page.to_string())
                    .active(self.pagination_page == page)
                    .on_click(cx.listener(move |this, _, _, cx| {
                        this.pagination_page = page;
                        cx.notify();
                    }))
            }))
            .child(PaginationEllipsis::new())
            .child(
                PaginationNext::new("page-next").on_click(cx.listener(|this, _, _, cx| {
                    this.pagination_page = (this.pagination_page + 1).min(3);
                    cx.notify();
                })),
            )
    }
    fn scroll_area_preview(cx: &App) -> impl IntoElement + use<> {
        let theme = Theme::of(cx).clone();
        div()
            .rounded(theme.radius_md())
            .border_1()
            .border_color(theme.border)
            .child(
                ScrollArea::new("scroll-area-tags")
                    .h(px(200.))
                    .w(px(192.))
                    .child(
                        div()
                            .flex()
                            .flex_col()
                            .p(px(16.))
                            .child(
                                div()
                                    .text_size(px(14.))
                                    .font_weight(FontWeight::MEDIUM)
                                    .pb(px(8.))
                                    .child("Tags"),
                            )
                            .children((1..=20).flat_map(|version| {
                                [
                                    div()
                                        .py(px(6.))
                                        .text_size(px(13.))
                                        .child(format!("v1.2.0-beta.{version}"))
                                        .into_any_element(),
                                    Separator::new().into_any_element(),
                                ]
                            })),
                    ),
            )
    }
    fn tooltip_preview() -> impl IntoElement + use<> {
        div()
            .flex()
            .flex_row()
            .items_center()
            .gap(px(12.))
            .child(
                Tooltip::new("tooltip-demo", "Add to library").child(
                    Button::new("tooltip-trigger")
                        .variant(ButtonVariant::Outline)
                        .child("Hover me"),
                ),
            )
            .child(
                Tooltip::rich("tooltip-rich-demo", |_, _| {
                    div()
                        .flex()
                        .flex_row()
                        .items_center()
                        .gap(px(4.))
                        .child("Save Changes")
                        .child(Kbd::new().in_tooltip().child("S"))
                        .into_any_element()
                })
                .child(
                    Button::new("tooltip-rich-trigger")
                        .variant(ButtonVariant::Outline)
                        .child("Rich tooltip"),
                ),
            )
    }
    fn hover_card_preview() -> impl IntoElement + use<> {
        div().child(
            HoverCard::new("hover-card-demo")
                .content(|cx| {
                    let theme = Theme::of(cx).clone();
                    div()
                        .flex()
                        .flex_row()
                        .gap(px(12.))
                        .child(Avatar::new("VC"))
                        .child(
                            div()
                                .flex()
                                .flex_col()
                                .gap(px(4.))
                                .child(
                                    div()
                                        .text_size(px(14.))
                                        .font_weight(FontWeight::SEMIBOLD)
                                        .child("@nextjs"),
                                )
                                .child(div().text_size(px(14.)).child(
                                    "The React Framework - created and maintained by @vercel.",
                                ))
                                .child(
                                    div()
                                        .text_size(px(12.))
                                        .text_color(theme.muted_foreground)
                                        .child("Joined December 2021"),
                                ),
                        )
                        .into_any_element()
                })
                .child(
                    Button::new("hover-card-trigger")
                        .variant(ButtonVariant::Link)
                        .child("@nextjs"),
                ),
        )
    }
    fn dialog_preview(&self, cx: &mut Context<Self>) -> impl IntoElement + use<> {
        let theme = Theme::of(cx).clone();
        div()
            .child(
                Button::new("dialog-trigger")
                    .variant(ButtonVariant::Outline)
                    .on_click(cx.listener(|this, _, _, cx| {
                        this.dialog_open = true;
                        cx.notify();
                    }))
                    .child("Edit Profile"),
            )
            .child(
                Dialog::new("dialog-demo")
                    .open(self.dialog_open)
                    .on_open_change(cx.listener(|this, open: &bool, _, cx| {
                        this.dialog_open = *open;
                        cx.notify();
                    }))
                    .child(
                        DialogHeader::new()
                            .child(DialogTitle::new().child("Edit profile"))
                            .child(DialogDescription::new().child(
                                "Make changes to your profile here. Click save when you're done.",
                            )),
                    )
                    .child(
                        div()
                            .h(px(80.))
                            .w_full()
                            .rounded(theme.radius_md())
                            .bg(theme.muted),
                    )
                    .child(
                        DialogFooter::new()
                            .child(
                                Button::new("dialog-cancel")
                                    .variant(ButtonVariant::Outline)
                                    .on_click(cx.listener(|this, _, _, cx| {
                                        this.dialog_open = false;
                                        cx.notify();
                                    }))
                                    .child("Cancel"),
                            )
                            .child(
                                Button::new("dialog-save")
                                    .on_click(cx.listener(|this, _, _, cx| {
                                        this.dialog_open = false;
                                        cx.notify();
                                    }))
                                    .child("Save changes"),
                            ),
                    ),
            )
    }
    fn alert_dialog_preview(&self, cx: &mut Context<Self>) -> impl IntoElement + use<> {
        div()
            .child(
                Button::new("alert-dialog-trigger")
                    .variant(ButtonVariant::Outline)
                    .on_click(cx.listener(|this, _, _, cx| {
                        this.alert_dialog_open = true;
                        cx.notify();
                    }))
                    .child("Show Dialog"),
            )
            .child(
                AlertDialog::new("alert-dialog-demo")
                    .open(self.alert_dialog_open)
                    .child(
                        AlertDialogHeader::new()
                            .child(AlertDialogTitle::new().child("Are you absolutely sure?"))
                            .child(AlertDialogDescription::new().child(
                                "This action cannot be undone. This will permanently delete your \
                                 account and remove your data from our servers.",
                            )),
                    )
                    .child(
                        AlertDialogFooter::new()
                            .child(
                                Button::new("alert-dialog-cancel")
                                    .variant(ButtonVariant::Outline)
                                    .on_click(cx.listener(|this, _, _, cx| {
                                        this.alert_dialog_open = false;
                                        cx.notify();
                                    }))
                                    .child("Cancel"),
                            )
                            .child(
                                Button::new("alert-dialog-action")
                                    .on_click(cx.listener(|this, _, _, cx| {
                                        this.alert_dialog_open = false;
                                        cx.notify();
                                    }))
                                    .child("Continue"),
                            ),
                    ),
            )
    }
    fn sheet_preview(&self, cx: &mut Context<Self>) -> impl IntoElement + use<> {
        div()
            .child(
                Button::new("sheet-trigger")
                    .variant(ButtonVariant::Outline)
                    .on_click(cx.listener(|this, _, _, cx| {
                        this.sheet_open = true;
                        cx.notify();
                    }))
                    .child("Open Sheet"),
            )
            .child(
                Sheet::new("sheet-demo")
                    .open(self.sheet_open)
                    .side(self.sheet_side)
                    .on_open_change(cx.listener(|this, open: &bool, _, cx| {
                        this.sheet_open = *open;
                        cx.notify();
                    }))
                    .child(
                        SheetHeader::new()
                            .child(SheetTitle::new().child("Edit profile"))
                            .child(SheetDescription::new().child(
                                "Make changes to your profile here. Click save when you're done.",
                            )),
                    )
                    .child(
                        SheetFooter::new().child(
                            Button::new("sheet-save")
                                .on_click(cx.listener(|this, _, _, cx| {
                                    this.sheet_open = false;
                                    cx.notify();
                                }))
                                .child("Save changes"),
                        ),
                    ),
            )
    }
    fn drawer_preview(&self, cx: &mut Context<Self>) -> impl IntoElement + use<> {
        div()
            .child(
                Button::new("drawer-trigger")
                    .variant(ButtonVariant::Outline)
                    .on_click(cx.listener(|this, _, _, cx| {
                        this.drawer_open = true;
                        cx.notify();
                    }))
                    .child("Open Drawer"),
            )
            .child(
                Drawer::new("drawer-demo")
                    .open(self.drawer_open)
                    .on_open_change(cx.listener(|this, open: &bool, _, cx| {
                        this.drawer_open = *open;
                        cx.notify();
                    }))
                    .child(
                        DrawerHeader::new()
                            .child(DrawerTitle::new().child("Move Goal"))
                            .child(DrawerDescription::new().child("Set your daily activity goal.")),
                    )
                    .child(
                        DrawerFooter::new()
                            .child(
                                Button::new("drawer-cancel")
                                    .variant(ButtonVariant::Outline)
                                    .on_click(cx.listener(|this, _, _, cx| {
                                        this.drawer_open = false;
                                        cx.notify();
                                    }))
                                    .child("Cancel"),
                            )
                            .child(
                                Button::new("drawer-submit")
                                    .on_click(cx.listener(|this, _, _, cx| {
                                        this.drawer_open = false;
                                        cx.notify();
                                    }))
                                    .child("Submit"),
                            ),
                    ),
            )
    }
    fn dropdown_menu_preview(&self, cx: &mut Context<Self>) -> impl IntoElement + use<> {
        DropdownMenu::new("dropdown-demo")
            .open(self.dropdown_open)
            .on_open_change(cx.listener(|this, open: &bool, _, cx| {
                this.dropdown_open = *open;
                cx.notify();
            }))
            .trigger(
                Button::new("dropdown-trigger")
                    .variant(ButtonVariant::Outline)
                    .child("Open Menu"),
            )
            .label("My Account")
            .separator()
            .item(
                DropdownMenuItem::new("dd-profile")
                    .shortcut("\u{2318}P")
                    .child("Profile"),
            )
            .item(
                DropdownMenuItem::new("dd-billing")
                    .shortcut("\u{2318}B")
                    .child("Billing"),
            )
            .item(
                DropdownMenuItem::new("dd-status")
                    .checked(self.dropdown_status_checked)
                    .on_select(cx.listener(|this, _, _, cx| {
                        this.dropdown_status_checked = !this.dropdown_status_checked;
                        cx.notify();
                    }))
                    .child("Show Status Bar"),
            )
            .item(
                DropdownMenuItem::new("dd-disabled")
                    .disabled(true)
                    .child("API (disabled)"),
            )
            .separator()
            .item(
                DropdownMenuItem::new("dd-logout")
                    .destructive(true)
                    .shortcut("\u{21e7}\u{2318}Q")
                    .child("Log out"),
            )
    }
    fn context_menu_preview(&self, cx: &mut Context<Self>) -> impl IntoElement + use<> {
        let theme = Theme::of(cx).clone();
        ContextMenu::new("context-menu-demo")
            .open_at(self.context_menu_at)
            .on_request_open(
                cx.listener(|this, position: &gpui::Point<gpui::Pixels>, _, cx| {
                    this.context_menu_at = Some(*position);
                    cx.notify();
                }),
            )
            .on_open_change(cx.listener(|this, _open: &bool, _, cx| {
                this.context_menu_at = None;
                cx.notify();
            }))
            .trigger(
                div()
                    .flex()
                    .h(px(150.))
                    .w(px(300.))
                    .items_center()
                    .justify_center()
                    .rounded(theme.radius_md())
                    .border_1()
                    .border_color(theme.border)
                    .text_size(px(14.))
                    .child("Right click here"),
            )
            .label("Navigation")
            .item(
                ContextMenuItem::new("cm-back")
                    .shortcut("\u{2318}[")
                    .child("Back"),
            )
            .item(
                ContextMenuItem::new("cm-forward")
                    .disabled(true)
                    .shortcut("\u{2318}]")
                    .child("Forward"),
            )
            .item(
                ContextMenuItem::new("cm-reload")
                    .shortcut("\u{2318}R")
                    .child("Reload"),
            )
            .separator()
            .item(
                ContextMenuItem::new("cm-bookmarks")
                    .checked(true)
                    .child("Show Bookmarks"),
            )
            .item(
                ContextMenuItem::new("cm-fullurls")
                    .checked(false)
                    .child("Show Full URLs"),
            )
    }
    fn menubar_preview(&self, cx: &mut Context<Self>) -> impl IntoElement + use<> {
        Menubar::new("menubar-demo")
            .open(self.menubar_open)
            .on_open_change(cx.listener(|this, open: &Option<usize>, _, cx| {
                this.menubar_open = *open;
                cx.notify();
            }))
            .menu(
                MenubarMenu::new("File")
                    .item(
                        MenubarItem::new("mb-new-tab")
                            .shortcut("\u{2318}T")
                            .child("New Tab"),
                    )
                    .item(
                        MenubarItem::new("mb-new-window")
                            .shortcut("\u{2318}N")
                            .child("New Window"),
                    )
                    .separator()
                    .item(
                        MenubarItem::new("mb-print")
                            .shortcut("\u{2318}P")
                            .child("Print..."),
                    ),
            )
            .menu(
                MenubarMenu::new("Edit")
                    .item(
                        MenubarItem::new("mb-undo")
                            .shortcut("\u{2318}Z")
                            .child("Undo"),
                    )
                    .item(
                        MenubarItem::new("mb-redo")
                            .shortcut("\u{21e7}\u{2318}Z")
                            .child("Redo"),
                    ),
            )
            .menu(
                MenubarMenu::new("View")
                    .item(
                        MenubarItem::new("mb-reload")
                            .checked(true)
                            .child("Always Show Bookmarks"),
                    )
                    .item(MenubarItem::new("mb-fullscreen").child("Toggle Fullscreen")),
            )
    }
    fn select_preview(&self, cx: &mut Context<Self>) -> impl IntoElement + use<> {
        Select::new("select-fruit")
            .placeholder("Select a fruit")
            .option("Apple")
            .options(["Banana", "Blueberry", "Grapes", "Pineapple"])
            .value(self.select_value)
            .open(self.select_open)
            .on_change(cx.listener(|this, value: &usize, _, cx| {
                this.select_value = Some(*value);
                cx.notify();
            }))
            .on_open_change(cx.listener(|this, open: &bool, _, cx| {
                this.select_open = *open;
                cx.notify();
            }))
    }
    fn native_select_preview(&self, cx: &mut Context<Self>) -> impl IntoElement + use<> {
        NativeSelect::new("native-select-status")
            .disabled(false)
            .placeholder("Select status")
            .options(["Todo", "In Progress", "Done", "Cancelled"])
            .value(self.native_select_value)
            .open(self.native_select_open)
            .on_change(cx.listener(|this, value: &usize, _, cx| {
                this.native_select_value = Some(*value);
                cx.notify();
            }))
            .on_open_change(cx.listener(|this, open: &bool, _, cx| {
                this.native_select_open = *open;
                cx.notify();
            }))
    }
    fn navigation_menu_preview(&self, cx: &mut Context<Self>) -> impl IntoElement + use<> {
        NavigationMenu::new("nav-menu-demo")
            .open(self.nav_menu_open)
            .on_open_change(cx.listener(|this, open: &Option<usize>, _, cx| {
                this.nav_menu_open = *open;
                cx.notify();
            }))
            .entry(
                NavigationMenuEntry::new("Getting started").content(
                    div()
                        .flex()
                        .flex_col()
                        .w(px(288.))
                        .gap(px(2.))
                        .child(
                            NavigationMenuLink::new("nav-intro", "Introduction")
                                .on_click(cx.listener(|this, _, _, cx| {
                                    this.nav_menu_open = None;
                                    cx.notify();
                                }))
                                .description("Copy-paste components for gpui apps."),
                        )
                        .child(
                            NavigationMenuLink::new("nav-install", "Installation")
                                .description("How to vendor components into your project."),
                        ),
                ),
            )
            .entry(
                NavigationMenuEntry::new("Components").content(
                    div()
                        .flex()
                        .flex_col()
                        .w(px(288.))
                        .gap(px(2.))
                        .child(
                            NavigationMenuLink::new("nav-button", "Button")
                                .description("Displays a button."),
                        )
                        .child(
                            NavigationMenuLink::new("nav-badge", "Badge")
                                .description("Displays a badge."),
                        ),
                ),
            )
            .entry(NavigationMenuEntry::new("Docs"))
    }
    fn toast_preview(&self, cx: &mut Context<Self>) -> impl IntoElement + use<> {
        div()
            .child(
                Button::new("toast-trigger")
                    .variant(ButtonVariant::Outline)
                    .on_click(cx.listener(|this, _, _, cx| {
                        this.toast_visible = true;
                        cx.notify();
                    }))
                    .child("Show Toast"),
            )
            .when(self.toast_visible, |el| {
                el.child(
                    ToastViewport::new().child(
                        Toast::new("toast-demo", "Event has been created")
                            .description("Sunday, December 03, 2023 at 9:00 AM")
                            .action(
                                Button::new("toast-undo")
                                    .variant(ButtonVariant::Outline)
                                    .size(ButtonSize::Xs)
                                    .on_click(cx.listener(|this, _, _, cx| {
                                        this.toast_visible = false;
                                        cx.notify();
                                    }))
                                    .child("Undo"),
                            )
                            .on_close(cx.listener(|this, _: &ClickEvent, _, cx| {
                                this.toast_visible = false;
                                cx.notify();
                            })),
                    ),
                )
            })
    }
    /// Port of input-demo.tsx — API Key password field.
    fn input_preview(&self, _cx: &mut Context<Self>) -> impl IntoElement + use<> {
        div().w(px(320.)).child(
            Field::new()
                .child(FieldLabel::new().child("API Key"))
                .child(self.input_demo.clone())
                .child(
                    FieldDescription::new().child("Your API key is encrypted and stored securely."),
                ),
        )
    }

    /// Port of input-basic.tsx.
    fn input_example_basic(&self, _cx: &mut Context<Self>) -> impl IntoElement + use<> {
        div().w(px(320.)).child(self.input_basic.clone())
    }

    /// Port of input-field.tsx.
    fn input_example_field(&self, _cx: &mut Context<Self>) -> impl IntoElement + use<> {
        div().w(px(320.)).child(
            Field::new()
                .child(FieldLabel::new().child("Username"))
                .child(self.input_field.clone())
                .child(FieldDescription::new().child("Choose a unique username for your account.")),
        )
    }

    /// Port of input-fieldgroup.tsx.
    fn input_example_fieldgroup(&self, _cx: &mut Context<Self>) -> impl IntoElement + use<> {
        div().w(px(320.)).child(
            FieldGroup::new()
                .child(
                    Field::new()
                        .child(FieldLabel::new().child("Name"))
                        .child(self.input_fieldgroup_name.clone()),
                )
                .child(
                    Field::new()
                        .child(FieldLabel::new().child("Email"))
                        .child(self.input_fieldgroup_email.clone())
                        .child(
                            FieldDescription::new().child("We'll send updates to this address."),
                        ),
                )
                .child(
                    Field::new()
                        .orientation(FieldOrientation::Horizontal)
                        .child(
                            Button::new("input-fg-reset")
                                .variant(ButtonVariant::Outline)
                                .child("Reset"),
                        )
                        .child(Button::new("input-fg-submit").child("Submit")),
                ),
        )
    }

    /// Port of input-disabled.tsx.
    fn input_example_disabled(&self, _cx: &mut Context<Self>) -> impl IntoElement + use<> {
        div().w(px(320.)).child(
            Field::new()
                .child(FieldLabel::new().disabled(true).child("Email"))
                .child(self.input_disabled.clone())
                .child(FieldDescription::new().child("This field is currently disabled.")),
        )
    }

    /// Port of input-invalid.tsx.
    fn input_example_invalid(&self, _cx: &mut Context<Self>) -> impl IntoElement + use<> {
        div().w(px(320.)).child(
            Field::new()
                .invalid(true)
                .child(FieldLabel::new().child("Invalid Input"))
                .child(self.input_invalid.clone())
                .child(FieldDescription::new().child("This field contains validation errors.")),
        )
    }

    /// Port of input-file.tsx.
    fn input_example_file(&self, _cx: &mut Context<Self>) -> impl IntoElement + use<> {
        div().w(px(320.)).child(
            Field::new()
                .child(FieldLabel::new().child("Picture"))
                .child(self.input_file.clone())
                .child(FieldDescription::new().child("Select a picture to upload.")),
        )
    }

    /// Port of input-inline.tsx.
    fn input_example_inline(&self, _cx: &mut Context<Self>) -> impl IntoElement + use<> {
        div().w(px(320.)).child(
            Field::new()
                .orientation(FieldOrientation::Horizontal)
                .child(div().flex_1().child(self.input_inline.clone()))
                .child(
                    Button::new("input-inline-search")
                        .size(ButtonSize::Sm)
                        .child("Search"),
                ),
        )
    }

    /// Port of input-grid.tsx — FieldGroup as a two-column grid (gap-7).
    fn input_example_grid(&self, _cx: &mut Context<Self>) -> impl IntoElement + use<> {
        div().w(px(384.)).child(
            div()
                .flex()
                .flex_row()
                .gap(px(28.))
                .child(
                    div().flex_1().child(
                        Field::new()
                            .child(FieldLabel::new().child("First Name"))
                            .child(self.input_grid_first.clone()),
                    ),
                )
                .child(
                    div().flex_1().child(
                        Field::new()
                            .child(FieldLabel::new().child("Last Name"))
                            .child(self.input_grid_last.clone()),
                    ),
                ),
        )
    }

    /// Port of input-required.tsx.
    fn input_example_required(&self, cx: &mut Context<Self>) -> impl IntoElement + use<> {
        let theme = Theme::of(cx).clone();
        div().w(px(320.)).child(
            Field::new()
                .child(
                    FieldLabel::new()
                        .child("Required Field")
                        .child(div().text_color(theme.destructive).child("*")),
                )
                .child(self.input_required.clone())
                .child(FieldDescription::new().child("This field must be filled out.")),
        )
    }

    /// Port of input-badge.tsx.
    fn input_example_badge(&self, _cx: &mut Context<Self>) -> impl IntoElement + use<> {
        div().w(px(320.)).child(
            Field::new()
                .child(
                    FieldLabel::new()
                        .child("Webhook URL")
                        .child(Badge::new().variant(BadgeVariant::Secondary).child("Beta")),
                )
                .child(self.input_badge.clone()),
        )
    }

    /// Port of input-input-group.tsx.
    fn input_example_input_group(&self, cx: &mut Context<Self>) -> impl IntoElement + use<> {
        let theme = Theme::of(cx).clone();
        div().w(px(320.)).child(
            Field::new()
                .child(FieldLabel::new().child("Website URL"))
                .child(
                    InputGroup::new(self.input_ig_url.clone())
                        .leading(InputGroupAddon::new().child("https://"))
                        .trailing(
                            InputGroupAddon::new().child(
                                gpui::svg()
                                    .path(crate::assets::ICON_INFO)
                                    .size(px(16.))
                                    .text_color(theme.muted_foreground),
                            ),
                        ),
                ),
        )
    }

    /// Port of input-button-group.tsx. ButtonGroup only accepts Buttons, so
    /// the joined input+button row is approximated with an adjacent flex row
    /// (shared-border grouping is a button-group audit TODO).
    fn input_example_button_group(&self, _cx: &mut Context<Self>) -> impl IntoElement + use<> {
        div().w(px(320.)).child(
            Field::new().child(FieldLabel::new().child("Search")).child(
                div()
                    .flex()
                    .flex_row()
                    .w_full()
                    .child(div().flex_1().child(self.input_bg_search.clone()))
                    .child(
                        div().ml(px(-1.)).child(
                            Button::new("input-bg-search")
                                .variant(ButtonVariant::Outline)
                                .size(ButtonSize::Sm)
                                .child("Search"),
                        ),
                    ),
            ),
        )
    }

    /// Port of input-form.tsx — full form with inputs, a select, and buttons.
    fn input_example_form(&self, cx: &mut Context<Self>) -> impl IntoElement + use<> {
        div().w(px(384.)).child(
            FieldGroup::new()
                .child(
                    Field::new()
                        .child(FieldLabel::new().child("Name"))
                        .child(self.input_form_name.clone()),
                )
                .child(
                    Field::new()
                        .child(FieldLabel::new().child("Email"))
                        .child(self.input_form_email.clone())
                        .child(
                            FieldDescription::new()
                                .child("We'll never share your email with anyone."),
                        ),
                )
                .child(
                    div()
                        .flex()
                        .flex_row()
                        .gap(px(16.))
                        .w_full()
                        .child(
                            div().flex_1().child(
                                Field::new()
                                    .child(FieldLabel::new().child("Phone"))
                                    .child(self.input_form_phone.clone()),
                            ),
                        )
                        .child(
                            div().flex_1().child(
                                Field::new()
                                    .child(FieldLabel::new().child("Country"))
                                    .child(
                                        Select::new("input-form-country")
                                            .options(["United States", "United Kingdom", "Canada"])
                                            .value(self.input_form_country)
                                            .open(self.input_form_country_open)
                                            .on_change(cx.listener(|this, value: &usize, _, cx| {
                                                this.input_form_country = Some(*value);
                                                cx.notify();
                                            }))
                                            .on_open_change(cx.listener(
                                                |this, open: &bool, _, cx| {
                                                    this.input_form_country_open = *open;
                                                    cx.notify();
                                                },
                                            )),
                                    ),
                            ),
                        ),
                )
                .child(
                    Field::new()
                        .child(FieldLabel::new().child("Address"))
                        .child(self.input_form_address.clone()),
                )
                .child(
                    Field::new()
                        .orientation(FieldOrientation::Horizontal)
                        .child(
                            Button::new("input-form-cancel")
                                .variant(ButtonVariant::Outline)
                                .child("Cancel"),
                        )
                        .child(Button::new("input-form-submit").child("Submit")),
                ),
        )
    }
    fn textarea_preview(&self, _cx: &mut Context<Self>) -> impl IntoElement + use<> {
        div()
            .w(px(288.))
            .child(Textarea::new(self.textarea_input.clone()).rows(4))
    }
    fn field_preview(&self, cx: &mut Context<Self>) -> impl IntoElement + use<> {
        // Port of field-demo.tsx — Payment Method checkout form.
        div().w(px(448.)).child(
            FieldGroup::new()
                .child(
                    FieldSet::new()
                        .legend(FieldLegend::new().child("Payment Method"))
                        .description(
                            FieldDescription::new()
                                .child("All transactions are secure and encrypted"),
                        )
                        .child(
                            FieldGroup::new()
                                .child(
                                    Field::new()
                                        .child(FieldLabel::new().child("Name on Card"))
                                        .child(self.field_name_input.clone()),
                                )
                                .child(
                                    Field::new()
                                        .child(FieldLabel::new().child("Card Number"))
                                        .child(self.field_card_number.clone())
                                        .child(
                                            FieldDescription::new()
                                                .child("Enter your 16-digit card number"),
                                        ),
                                )
                                .child(
                                    div()
                                        .flex()
                                        .flex_row()
                                        .gap(px(16.))
                                        .w_full()
                                        .child(
                                            div().flex_1().child(
                                                Field::new()
                                                    .child(FieldLabel::new().child("Month"))
                                                    .child(
                                                        Select::new("field-month")
                                                            .placeholder("MM")
                                                            .options([
                                                                "01", "02", "03", "04", "05", "06",
                                                                "07", "08", "09", "10", "11", "12",
                                                            ])
                                                            .value(self.field_month)
                                                            .open(self.field_month_open)
                                                            .on_change(cx.listener(
                                                                |this, value: &usize, _, cx| {
                                                                    this.field_month = Some(*value);
                                                                    cx.notify();
                                                                },
                                                            ))
                                                            .on_open_change(cx.listener(
                                                                |this, open: &bool, _, cx| {
                                                                    this.field_month_open = *open;
                                                                    cx.notify();
                                                                },
                                                            )),
                                                    ),
                                            ),
                                        )
                                        .child(
                                            div().flex_1().child(
                                                Field::new()
                                                    .child(FieldLabel::new().child("Year"))
                                                    .child(
                                                        Select::new("field-year")
                                                            .placeholder("YYYY")
                                                            .options([
                                                                "2024", "2025", "2026", "2027",
                                                                "2028", "2029",
                                                            ])
                                                            .value(self.field_year)
                                                            .open(self.field_year_open)
                                                            .on_change(cx.listener(
                                                                |this, value: &usize, _, cx| {
                                                                    this.field_year = Some(*value);
                                                                    cx.notify();
                                                                },
                                                            ))
                                                            .on_open_change(cx.listener(
                                                                |this, open: &bool, _, cx| {
                                                                    this.field_year_open = *open;
                                                                    cx.notify();
                                                                },
                                                            )),
                                                    ),
                                            ),
                                        )
                                        .child(
                                            div().flex_1().child(
                                                Field::new()
                                                    .child(FieldLabel::new().child("CVV"))
                                                    .child(self.field_cvv.clone()),
                                            ),
                                        ),
                                ),
                        ),
                )
                .child(FieldSeparator::new())
                .child(
                    FieldSet::new()
                        .legend(FieldLegend::new().child("Billing Address"))
                        .description(
                            FieldDescription::new()
                                .child("The billing address associated with your payment method"),
                        )
                        .child(
                            FieldGroup::new().child(
                                Field::new()
                                    .orientation(FieldOrientation::Horizontal)
                                    .child(
                                        Checkbox::new("field-same-shipping")
                                            .checked(self.field_same_shipping)
                                            .on_change(cx.listener(
                                                |this, checked: &bool, _, cx| {
                                                    this.field_same_shipping = *checked;
                                                    cx.notify();
                                                },
                                            )),
                                    )
                                    .child(
                                        FieldLabel::new()
                                            .font_normal()
                                            .child("Same as shipping address"),
                                    ),
                            ),
                        ),
                )
                .child(
                    FieldSet::new().child(
                        FieldGroup::new().child(
                            Field::new()
                                .child(FieldLabel::new().child("Comments"))
                                .child(Textarea::new(self.field_comments.clone()).rows(3)),
                        ),
                    ),
                )
                .child(
                    Field::new()
                        .orientation(FieldOrientation::Horizontal)
                        .child(
                            Button::new("field-submit")
                                .variant(ButtonVariant::Default)
                                .child("Submit"),
                        )
                        .child(
                            Button::new("field-cancel")
                                .variant(ButtonVariant::Outline)
                                .child("Cancel"),
                        ),
                ),
        )
    }

    /// Port of field-input.tsx — Username + Password fields.
    fn field_example_input(&self, _cx: &mut Context<Self>) -> impl IntoElement + use<> {
        div().w(px(320.)).child(
            FieldSet::new().child(
                FieldGroup::new()
                    .child(
                        Field::new()
                            .child(FieldLabel::new().child("Username"))
                            .child(self.field_username.clone())
                            .child(
                                FieldDescription::new()
                                    .child("Choose a unique username for your account."),
                            ),
                    )
                    .child(
                        Field::new()
                            .child(FieldLabel::new().child("Password"))
                            .child(
                                FieldDescription::new()
                                    .child("Must be at least 8 characters long."),
                            )
                            .child(self.field_password.clone()),
                    ),
            ),
        )
    }

    /// Port of field-textarea.tsx — Feedback textarea.
    fn field_example_textarea(&self, _cx: &mut Context<Self>) -> impl IntoElement + use<> {
        div().w(px(320.)).child(
            FieldSet::new().child(
                FieldGroup::new().child(
                    Field::new()
                        .child(FieldLabel::new().child("Feedback"))
                        .child(Textarea::new(self.field_feedback.clone()).rows(4))
                        .child(
                            FieldDescription::new().child("Share your thoughts about our service."),
                        ),
                ),
            ),
        )
    }

    /// Port of field-select.tsx — Department select.
    fn field_example_select(&self, cx: &mut Context<Self>) -> impl IntoElement + use<> {
        div().w(px(320.)).child(
            Field::new()
                .child(FieldLabel::new().child("Department"))
                .child(
                    Select::new("field-department")
                        .placeholder("Choose department")
                        .options([
                            "Engineering",
                            "Design",
                            "Marketing",
                            "Sales",
                            "Customer Support",
                            "Human Resources",
                            "Finance",
                            "Operations",
                        ])
                        .value(self.field_department)
                        .open(self.field_department_open)
                        .on_change(cx.listener(|this, value: &usize, _, cx| {
                            this.field_department = Some(*value);
                            cx.notify();
                        }))
                        .on_open_change(cx.listener(|this, open: &bool, _, cx| {
                            this.field_department_open = *open;
                            cx.notify();
                        })),
                )
                .child(FieldDescription::new().child("Select your department or area of work.")),
        )
    }

    /// Port of field-slider.tsx — Price Range (single-thumb adapted).
    fn field_example_slider(&self, cx: &mut Context<Self>) -> impl IntoElement + use<> {
        div().w(px(320.)).child(
            Field::new()
                .child(FieldTitle::new().child("Price Range"))
                .child(FieldDescription::new().child(format!(
                    "Set your budget range (${:.0}).",
                    self.field_slider
                )))
                .child(
                    Slider::new("field-slider")
                        .min(0.)
                        .max(1000.)
                        .step(10.)
                        .value(self.field_slider)
                        .on_change(cx.listener(|this, value: &f32, _, cx| {
                            this.field_slider = *value;
                            cx.notify();
                        })),
                ),
        )
    }

    /// Port of field-fieldset.tsx — Address Information.
    fn field_example_fieldset(&self, _cx: &mut Context<Self>) -> impl IntoElement + use<> {
        div().w(px(384.)).child(
            FieldSet::new()
                .legend(FieldLegend::new().child("Address Information"))
                .description(
                    FieldDescription::new().child("We need your address to deliver your order."),
                )
                .child(
                    FieldGroup::new()
                        .child(
                            Field::new()
                                .child(FieldLabel::new().child("Street Address"))
                                .child(self.field_street.clone()),
                        )
                        .child(
                            div()
                                .flex()
                                .flex_row()
                                .gap(px(16.))
                                .child(
                                    div().flex_1().child(
                                        Field::new()
                                            .child(FieldLabel::new().child("City"))
                                            .child(self.field_city.clone()),
                                    ),
                                )
                                .child(
                                    div().flex_1().child(
                                        Field::new()
                                            .child(FieldLabel::new().child("Postal Code"))
                                            .child(self.field_zip.clone()),
                                    ),
                                ),
                        ),
                ),
        )
    }

    /// Port of field-checkbox.tsx — desktop item checkboxes + sync toggle.
    fn field_example_checkbox(&self, cx: &mut Context<Self>) -> impl IntoElement + use<> {
        div().w(px(320.)).child(
            FieldGroup::new()
                .child(
                    FieldSet::new()
                        .legend(
                            FieldLegend::new()
                                .variant(FieldLegendVariant::Label)
                                .child("Show these items on the desktop"),
                        )
                        .description(
                            FieldDescription::new()
                                .child("Select the items you want to show on the desktop."),
                        )
                        .child(
                            FieldGroup::new().gap(px(12.))
                                .child(
                                    Field::new()
                                        .orientation(FieldOrientation::Horizontal)
                                        .child(
                                            Checkbox::new("field-hard-disks")
                                                .checked(self.field_hard_disks)
                                                .on_change(cx.listener(
                                                    |this, checked: &bool, _, cx| {
                                                        this.field_hard_disks = *checked;
                                                        cx.notify();
                                                    },
                                                )),
                                        )
                                        .child(
                                            FieldLabel::new()
                                                .font_normal()
                                                .child("Hard disks"),
                                        ),
                                )
                                .child(
                                    Field::new()
                                        .orientation(FieldOrientation::Horizontal)
                                        .child(
                                            Checkbox::new("field-external-disks")
                                                .checked(self.field_external_disks)
                                                .on_change(cx.listener(
                                                    |this, checked: &bool, _, cx| {
                                                        this.field_external_disks = *checked;
                                                        cx.notify();
                                                    },
                                                )),
                                        )
                                        .child(
                                            FieldLabel::new()
                                                .font_normal()
                                                .child("External disks"),
                                        ),
                                )
                                .child(
                                    Field::new()
                                        .orientation(FieldOrientation::Horizontal)
                                        .child(
                                            Checkbox::new("field-cds")
                                                .checked(self.field_cds)
                                                .on_change(cx.listener(
                                                    |this, checked: &bool, _, cx| {
                                                        this.field_cds = *checked;
                                                        cx.notify();
                                                    },
                                                )),
                                        )
                                        .child(
                                            FieldLabel::new()
                                                .font_normal()
                                                .child("CDs, DVDs, and iPods"),
                                        ),
                                )
                                .child(
                                    Field::new()
                                        .orientation(FieldOrientation::Horizontal)
                                        .child(
                                            Checkbox::new("field-connected-servers")
                                                .checked(self.field_connected_servers)
                                                .on_change(cx.listener(
                                                    |this, checked: &bool, _, cx| {
                                                        this.field_connected_servers = *checked;
                                                        cx.notify();
                                                    },
                                                )),
                                        )
                                        .child(
                                            FieldLabel::new()
                                                .font_normal()
                                                .child("Connected servers"),
                                        ),
                                ),
                        ),
                )
                .child(FieldSeparator::new())
                .child(
                    Field::new()
                        .orientation(FieldOrientation::Horizontal)
                        .child(
                            Checkbox::new("field-sync-folders")
                                .checked(self.field_sync_folders)
                                .on_change(cx.listener(|this, checked: &bool, _, cx| {
                                    this.field_sync_folders = *checked;
                                    cx.notify();
                                })),
                        )
                        .content(
                            FieldContent::new()
                                .child(
                                    FieldLabel::new()
                                        .child("Sync Desktop & Documents folders"),
                                )
                                .child(
                                    FieldDescription::new().child(
                                        "Your Desktop & Documents folders are being synced with iCloud Drive. You can access them from other devices.",
                                    ),
                                ),
                        ),
                ),
        )
    }

    /// Port of field-radio.tsx — Subscription Plan radio group.
    fn field_example_radio(&self, cx: &mut Context<Self>) -> impl IntoElement + use<> {
        let plans = [
            ("plan-monthly", "Monthly ($9.99/month)"),
            ("plan-yearly", "Yearly ($99.99/year)"),
            ("plan-lifetime", "Lifetime ($299.99)"),
        ];
        div().w(px(320.)).child(
            FieldSet::new()
                .legend(
                    FieldLegend::new()
                        .variant(FieldLegendVariant::Label)
                        .child("Subscription Plan"),
                )
                .description(
                    FieldDescription::new()
                        .child("Yearly and lifetime plans offer significant savings."),
                )
                .gap(px(12.))
                .child(
                    RadioGroup::new().children(plans.into_iter().enumerate().map(
                        |(index, (id, label))| {
                            Field::new()
                                .orientation(FieldOrientation::Horizontal)
                                .child(
                                    RadioGroupItem::new(id)
                                        .checked(self.field_plan == index)
                                        .on_select(cx.listener(move |this, _, _, cx| {
                                            this.field_plan = index;
                                            cx.notify();
                                        })),
                                )
                                .child(FieldLabel::new().font_normal().child(label))
                        },
                    )),
                ),
        )
    }

    /// Port of field-switch.tsx — MFA switch (w-fit via flex-row wrap).
    fn field_example_switch(&self, cx: &mut Context<Self>) -> impl IntoElement + use<> {
        div().flex().flex_row().child(
            Field::new()
                .orientation(FieldOrientation::Horizontal)
                .child(FieldLabel::new().child("Multi-factor authentication"))
                .child(
                    Switch::new("field-2fa")
                        .checked(self.field_switch_2fa)
                        .on_change(cx.listener(|this, checked: &bool, _, cx| {
                            this.field_switch_2fa = *checked;
                            cx.notify();
                        })),
                ),
        )
    }

    /// Port of field-choice-card.tsx — Compute Environment choice cards.
    fn field_example_choice_card(&self, cx: &mut Context<Self>) -> impl IntoElement + use<> {
        let envs = [
            (
                "kubernetes-r2h",
                "Kubernetes",
                "Run GPU workloads on a K8s cluster.",
            ),
            (
                "vm-z4k",
                "Virtual Machine",
                "Access a cluster to run GPU workloads.",
            ),
        ];
        div().w(px(320.)).child(
            FieldGroup::new().child(
                FieldSet::new()
                    .legend(
                        FieldLegend::new()
                            .variant(FieldLegendVariant::Label)
                            .child("Compute Environment"),
                    )
                    .description(
                        FieldDescription::new()
                            .child("Select the compute environment for your cluster."),
                    )
                    .gap(px(12.))
                    .child(RadioGroup::new().children(envs.into_iter().enumerate().map(
                        |(index, (id, title, description))| {
                            FieldLabel::new()
                                .choice_card(self.field_compute_env == index)
                                .child(
                                    Field::new()
                                        .orientation(FieldOrientation::Horizontal)
                                        .content(
                                            FieldContent::new()
                                                .child(FieldTitle::new().child(title))
                                                .child(FieldDescription::new().child(description)),
                                        )
                                        .child(
                                            RadioGroupItem::new(id)
                                                .checked(self.field_compute_env == index)
                                                .on_select(cx.listener(move |this, _, _, cx| {
                                                    this.field_compute_env = index;
                                                    cx.notify();
                                                })),
                                        ),
                                )
                        },
                    ))),
            ),
        )
    }

    /// Port of field-group.tsx — Responses + Tasks notification groups.
    fn field_example_field_group(&self, cx: &mut Context<Self>) -> impl IntoElement + use<> {
        div().w(px(320.)).child(
            FieldGroup::new()
                .child(
                    FieldSet::new()
                        // has-[>[data-slot=checkbox-group]]:gap-3
                        .gap(px(12.))
                        .child(FieldLabel::new().child("Responses"))
                        .child(
                            FieldDescription::new().child(
                                "Get notified when ChatGPT responds to requests that take time, like research or image generation.",
                            ),
                        )
                        .child(
                            FieldGroup::new().gap(px(12.)).child(
                                Field::new()
                                    .orientation(FieldOrientation::Horizontal)
                                    .child(
                                        Checkbox::new("field-push-responses")
                                            .checked(self.field_push_responses)
                                            .disabled(true),
                                    )
                                    .child(
                                        FieldLabel::new()
                                            .font_normal()
                                            .child("Push notifications"),
                                    ),
                            ),
                        ),
                )
                .child(FieldSeparator::new())
                .child(
                    FieldSet::new()
                        .gap(px(12.))
                        .child(FieldLabel::new().child("Tasks"))
                        .child(
                            FieldDescription::new().child(
                                "Get notified when tasks you've created have updates.",
                            ),
                        )
                        .child(
                            FieldGroup::new()
                                .gap(px(12.))
                                .child(
                                    Field::new()
                                        .orientation(FieldOrientation::Horizontal)
                                        .child(
                                            Checkbox::new("field-push-tasks")
                                                .checked(self.field_push_tasks)
                                                .on_change(cx.listener(
                                                    |this, checked: &bool, _, cx| {
                                                        this.field_push_tasks = *checked;
                                                        cx.notify();
                                                    },
                                                )),
                                        )
                                        .child(
                                            FieldLabel::new()
                                                .font_normal()
                                                .child("Push notifications"),
                                        ),
                                )
                                .child(
                                    Field::new()
                                        .orientation(FieldOrientation::Horizontal)
                                        .child(
                                            Checkbox::new("field-email-tasks")
                                                .checked(self.field_email_tasks)
                                                .on_change(cx.listener(
                                                    |this, checked: &bool, _, cx| {
                                                        this.field_email_tasks = *checked;
                                                        cx.notify();
                                                    },
                                                )),
                                        )
                                        .child(
                                            FieldLabel::new()
                                                .font_normal()
                                                .child("Email notifications"),
                                        ),
                                ),
                        ),
                ),
        )
    }

    /// Port of field-responsive.tsx — Profile form with width toggle (360/560)
    /// so Responsive orientation actually switches.
    fn field_example_responsive(&self, cx: &mut Context<Self>) -> impl IntoElement + use<> {
        let width = self.field_responsive_width;
        div()
            .flex()
            .flex_col()
            .gap(px(16.))
            .child(
                div().flex().justify_center().child(
                    ToggleGroup::new()
                        .variant(ToggleVariant::Outline)
                        .size(ToggleSize::Sm)
                        .item(
                            ToggleGroupItem::new("field-resp-360")
                                .pressed(width == 360.)
                                .on_change(cx.listener(|this, pressed: &bool, _, cx| {
                                    if *pressed {
                                        this.field_responsive_width = 360.;
                                        cx.notify();
                                    }
                                }))
                                .child("360px"),
                        )
                        .item(
                            ToggleGroupItem::new("field-resp-560")
                                .pressed(width == 560.)
                                .on_change(cx.listener(|this, pressed: &bool, _, cx| {
                                    if *pressed {
                                        this.field_responsive_width = 560.;
                                        cx.notify();
                                    }
                                }))
                                .child("560px"),
                        ),
                ),
            )
            .child(
                div().w(px(width)).child(
                    FieldSet::new()
                        .legend(FieldLegend::new().child("Profile"))
                        .description(
                            FieldDescription::new().child("Fill in your profile information."),
                        )
                        .child(
                            FieldGroup::new()
                                .child(
                                    Field::new()
                                        .id("field-responsive-name")
                                        .orientation(FieldOrientation::Responsive)
                                        .content(
                                            FieldContent::new()
                                                .child(FieldLabel::new().child("Name"))
                                                .child(FieldDescription::new().child(
                                                    "Provide your full name for identification",
                                                )),
                                        )
                                        .child(self.field_responsive_name.clone()),
                                )
                                .child(
                                    Field::new()
                                        .id("field-responsive-actions")
                                        .orientation(FieldOrientation::Responsive)
                                        .child(
                                            Button::new("field-resp-submit")
                                                .variant(ButtonVariant::Default)
                                                .child("Submit"),
                                        )
                                        .child(
                                            Button::new("field-resp-cancel")
                                                .variant(ButtonVariant::Outline)
                                                .child("Cancel"),
                                        ),
                                ),
                        ),
                ),
            )
    }

    /// Port of the docs "Validation and Errors" section — `data-invalid`
    /// fields with a single FieldError and an errors-array bullet list.
    fn field_example_validation(&self, _cx: &mut Context<Self>) -> impl IntoElement + use<> {
        div().w(px(320.)).child(
            FieldGroup::new()
                .child(
                    Field::new()
                        .invalid(true)
                        .child(FieldLabel::new().child("Email"))
                        .child(self.field_input.clone())
                        .child(FieldError::new().child("Enter a valid email address.")),
                )
                .child(
                    Field::new()
                        .invalid(true)
                        .child(FieldLabel::new().child("Username"))
                        .child(self.field_error_input.clone())
                        .child(FieldError::new().errors([
                            "Username must be at least 3 characters.",
                            "This username is already taken.",
                        ])),
                ),
        )
    }

    fn input_group_preview(&self, cx: &mut Context<Self>) -> impl IntoElement + use<> {
        let theme = Theme::of(cx).clone();
        div()
            .flex()
            .flex_col()
            .gap(px(16.))
            .w(px(288.))
            .child(
                InputGroup::new(self.input_group_search.clone())
                    .leading(
                        InputGroupAddon::new().child(
                            gpui::svg()
                                .path(theme.icons.chevron_right())
                                .size(px(16.))
                                .text_color(theme.muted_foreground),
                        ),
                    )
                    .trailing(InputGroupAddon::new().child(Kbd::new().child("\u{2318}K"))),
            )
            .child(
                InputGroup::new(self.input_group_url.clone())
                    .leading(InputGroupAddon::new().child("https://"))
                    .trailing(
                        InputGroupAddon::new().child(
                            Button::new("ig-copy")
                                .variant(ButtonVariant::Ghost)
                                .size(ButtonSize::Xs)
                                .child("Copy"),
                        ),
                    ),
            )
    }
    fn input_otp_preview(&self, cx: &mut Context<Self>) -> impl IntoElement + use<> {
        let theme = Theme::of(cx).clone();
        div()
            .flex()
            .flex_col()
            .gap(px(12.))
            .child(InputOtp::new(self.input_otp.clone(), 6).group(3))
            .child(
                div()
                    .text_size(px(12.))
                    .text_color(theme.muted_foreground)
                    .child("Click the slots, then type the one-time password."),
            )
    }
    fn command_preview(&self, cx: &mut Context<Self>) -> impl IntoElement + use<> {
        div().w(px(360.)).child(
            Command::new(self.command_input.clone())
                .empty_message("No results found.")
                .group(
                    CommandGroup::new("Suggestions")
                        .item(CommandItem::new("cmd-calendar", "Calendar"))
                        .item(CommandItem::new("cmd-emoji", "Search Emoji"))
                        .item(
                            CommandItem::new("cmd-calc", "Calculator").on_select(cx.listener(
                                |this, _, window, cx| {
                                    this.command_input.update(cx, |input, cx| {
                                        input.set_text("", window, cx);
                                    });
                                    cx.notify();
                                },
                            )),
                        ),
                )
                .group(
                    CommandGroup::new("Settings")
                        .item(CommandItem::new("cmd-profile", "Profile").shortcut("\u{2318}P"))
                        .item(CommandItem::new("cmd-billing", "Billing").shortcut("\u{2318}B"))
                        .item(CommandItem::new("cmd-settings", "Settings").shortcut("\u{2318}S")),
                ),
        )
    }
    fn combobox_preview(&self, cx: &mut Context<Self>) -> impl IntoElement + use<> {
        Combobox::new("combobox-framework", self.combobox_search.clone())
            .placeholder("Select framework...")
            .empty_message("No framework found.")
            .options(["Next.js", "SvelteKit", "Nuxt.js", "Remix", "Astro"])
            .value(self.combobox_value)
            .open(self.combobox_open)
            .on_change(cx.listener(|this, value: &usize, _, cx| {
                this.combobox_value = Some(*value);
                cx.notify();
            }))
            .on_open_change(cx.listener(|this, open: &bool, _, cx| {
                this.combobox_open = *open;
                cx.notify();
            }))
    }
    fn calendar_preview(&self, cx: &mut Context<Self>) -> impl IntoElement + use<> {
        Calendar::new(self.calendar_month.0, self.calendar_month.1)
            .selected(self.calendar_selected)
            .on_month_change(cx.listener(|this, month: &(i32, u32), _, cx| {
                this.calendar_month = *month;
                cx.notify();
            }))
            .on_select(cx.listener(|this, date: &CalendarDate, _, cx| {
                this.calendar_selected = Some(*date);
                cx.notify();
            }))
    }
    fn date_picker_preview(&self, cx: &mut Context<Self>) -> impl IntoElement + use<> {
        DatePicker::new("date-picker-demo", self.date_picker_month)
            .placeholder("Pick a date")
            .value(self.date_picker_value)
            .open(self.date_picker_open)
            .on_select(cx.listener(|this, date: &CalendarDate, _, cx| {
                this.date_picker_value = Some(*date);
                cx.notify();
            }))
            .on_month_change(cx.listener(|this, month: &(i32, u32), _, cx| {
                this.date_picker_month = *month;
                cx.notify();
            }))
            .on_open_change(cx.listener(|this, open: &bool, _, cx| {
                this.date_picker_open = *open;
                cx.notify();
            }))
    }
    fn carousel_preview(&self, cx: &mut Context<Self>) -> impl IntoElement + use<> {
        let theme = Theme::of(cx).clone();
        Carousel::new("carousel-demo")
            .index(self.carousel_index)
            .on_index_change(cx.listener(|this, index: &usize, _, cx| {
                this.carousel_index = *index;
                cx.notify();
            }))
            .children((1..=5).map(|number| {
                Card::new().size(CardSize::Sm).child(
                    CardContent::new().size(CardSize::Sm).child(
                        div()
                            .flex()
                            .size(px(160.))
                            .items_center()
                            .justify_center()
                            .text_size(px(36.))
                            .font_weight(FontWeight::SEMIBOLD)
                            .text_color(theme.foreground)
                            .child(number.to_string()),
                    ),
                )
            }))
    }
    /// Panel content for the resizable demos (shadcn's centered semibold label).
    fn resizable_label(theme: &Theme, label: &'static str) -> AnyElement {
        div()
            .flex()
            .size_full()
            .items_center()
            .justify_center()
            .p(px(24.))
            .text_size(px(14.))
            .font_weight(FontWeight::SEMIBOLD)
            .text_color(theme.foreground)
            .child(label)
            .into_any_element()
    }

    /// Bordered demo frame around a panel group (shadcn's "rounded-lg border").
    fn resizable_frame(theme: &Theme, group: AnyElement) -> AnyElement {
        div()
            .w(px(384.))
            .h(px(200.))
            .rounded(theme.radius_lg())
            .border_1()
            .border_color(theme.border)
            .overflow_hidden()
            .child(group)
            .into_any_element()
    }

    fn resizable_preview(&self, cx: &mut Context<Self>) -> impl IntoElement + use<> {
        let theme = Theme::of(cx).clone();
        let panel = |label| Self::resizable_label(&theme, label);
        Self::resizable_frame(
            &theme,
            ResizablePanelGroup::new("resizable-demo")
                .panel(ResizablePanel::new().default_size(0.5).child(panel("One")))
                .handle(ResizableHandle::new().with_handle(true))
                .panel(
                    ResizablePanel::new().default_size(0.5).child(
                        ResizablePanelGroup::new("resizable-demo-nested")
                            .direction(ResizableDirection::Vertical)
                            .panel(ResizablePanel::new().default_size(0.25).child(panel("Two")))
                            .handle(ResizableHandle::new().with_handle(true))
                            .panel(
                                ResizablePanel::new()
                                    .default_size(0.75)
                                    .child(panel("Three")),
                            ),
                    ),
                )
                .into_any_element(),
        )
    }
    fn sidebar_preview(&self, cx: &mut Context<Self>) -> impl IntoElement + use<> {
        let theme = Theme::of(cx).clone();
        let items = ["Home", "Inbox", "Calendar", "Search", "Settings"];
        div()
            .w(px(480.))
            .h(px(320.))
            .rounded(theme.radius_lg())
            .border_1()
            .border_color(theme.border)
            .overflow_hidden()
            .child(
                SidebarProvider::new()
                    .open(self.sidebar_open)
                    .sidebar(
                        Sidebar::new()
                            .child(
                                SidebarHeader::new().child(
                                    div()
                                        .px(px(8.))
                                        .text_size(px(14.))
                                        .font_weight(FontWeight::SEMIBOLD)
                                        .child("Acme Inc"),
                                ),
                            )
                            .child(SidebarContent::new().child(
                                SidebarGroup::new().label("Application").children(
                                    items.into_iter().enumerate().map(|(index, label)| {
                                        SidebarMenuButton::new(("sidebar-item", index))
                                            .active(self.sidebar_active == index)
                                            .on_click(cx.listener(move |this, _, _, cx| {
                                                this.sidebar_active = index;
                                                cx.notify();
                                            }))
                                            .child(label)
                                    }),
                                ),
                            ))
                            .child(
                                SidebarFooter::new().child(
                                    div()
                                        .px(px(8.))
                                        .text_size(px(12.))
                                        .text_color(theme.muted_foreground)
                                        .child("evil rabbit"),
                                ),
                            ),
                    )
                    .inset(
                        div()
                            .flex()
                            .flex_col()
                            .gap(px(8.))
                            .p(px(12.))
                            .child(SidebarTrigger::new("sidebar-trigger").on_click(cx.listener(
                                |this, _, _, cx| {
                                    this.sidebar_open = !this.sidebar_open;
                                    cx.notify();
                                },
                            )))
                            .child(
                                div()
                                    .text_size(px(14.))
                                    .text_color(theme.muted_foreground)
                                    .child(format!("Active: {}", items[self.sidebar_active])),
                            ),
                    ),
            )
    }
    fn data_table_preview(&self, cx: &mut Context<Self>) -> impl IntoElement + use<> {
        let theme = Theme::of(cx).clone();
        let mut payments: Vec<(&str, &str, u32)> = vec![
            ("ken99@example.com", "Success", 316),
            ("abe45@example.com", "Success", 242),
            ("monserrat44@example.com", "Processing", 837),
            ("silas22@example.com", "Failed", 874),
            ("carmella@example.com", "Pending", 721),
        ];
        payments.sort_by(|a, b| {
            if self.data_table_desc {
                b.2.cmp(&a.2)
            } else {
                a.2.cmp(&b.2)
            }
        });
        let count = payments.len();
        div().w(px(480.)).child(
            Table::new()
                .child(
                    TableHeader::new().child(
                        TableRow::new()
                            .child(TableHead::new().child("Email"))
                            .child(TableHead::new().w(px(110.)).child("Status"))
                            .child(
                                TableHead::new().w(px(110.)).child(
                                    div()
                                        .id("data-table-sort")
                                        .flex()
                                        .flex_row()
                                        .items_center()
                                        .gap(px(4.))
                                        .hover(|s| s.text_color(theme.foreground))
                                        .on_click(cx.listener(|this, _, _, cx| {
                                            this.data_table_desc = !this.data_table_desc;
                                            cx.notify();
                                        }))
                                        .child("Amount")
                                        .child(
                                            gpui::svg()
                                                .path(if self.data_table_desc {
                                                    theme.icons.chevron_down()
                                                } else {
                                                    theme.icons.chevron_up()
                                                })
                                                .size(px(12.))
                                                .text_color(theme.muted_foreground),
                                        ),
                                ),
                            ),
                    ),
                )
                .child(
                    TableBody::new().children(payments.into_iter().enumerate().map(
                        |(index, (email, status, amount))| {
                            TableRow::new()
                                .id(("data-table-row", index))
                                .last(index + 1 == count)
                                .child(TableCell::new().child(email))
                                .child(
                                    TableCell::new().w(px(110.)).child(
                                        Badge::new()
                                            .variant(match status {
                                                "Success" => BadgeVariant::Secondary,
                                                "Failed" => BadgeVariant::Destructive,
                                                _ => BadgeVariant::Outline,
                                            })
                                            .child(status),
                                    ),
                                )
                                .child(TableCell::new().w(px(110.)).child(format!("${amount}.00")))
                                .into_any_element()
                        },
                    )),
                )
                .child(TableCaption::new().child("Click the Amount header to flip the sort.")),
        )
    }
    fn chart_preview(&self, _cx: &mut Context<Self>) -> impl IntoElement + use<> {
        div().w(px(420.)).child(
            BarChart::new(["Jan", "Feb", "Mar", "Apr", "May", "Jun"])
                .height(180.)
                .series(ChartSeries::new(
                    "Desktop",
                    [186., 305., 237., 73., 209., 214.],
                ))
                .series(ChartSeries::new(
                    "Mobile",
                    [80., 200., 120., 190., 130., 140.],
                )),
        )
    }
    fn message_preview(&self, cx: &mut Context<Self>) -> impl IntoElement + use<> {
        let theme = Theme::of(cx).clone();
        let plain_bubble = |text: &'static str, end: bool| {
            div()
                .w_auto()
                .max_w(px(280.))
                .rounded(theme.radius_xl())
                .px(px(12.))
                .py(px(8.))
                .map(|el| {
                    if end {
                        el.bg(theme.primary).text_color(theme.primary_foreground)
                    } else {
                        el.bg(theme.muted).text_color(theme.foreground)
                    }
                })
                .child(text)
        };
        div().w(px(384.)).child(
            MessageGroup::new()
                .child(
                    Message::new()
                        .child(MessageAvatar::new().child(Avatar::new("LR")))
                        .child(
                            MessageContent::new()
                                .child(MessageHeader::new().child("Lord Rabbit \u{00b7} 10:42"))
                                .child(plain_bubble(
                                    "Hey! How's the gpui port coming along?",
                                    false,
                                )),
                        ),
                )
                .child(
                    Message::new().align(MessageAlign::End).child(
                        MessageContent::new()
                            .align(MessageAlign::End)
                            .child(plain_bubble(
                                "Almost done \u{2014} shipping the last components now.",
                                true,
                            ))
                            .child(MessageFooter::new().child("Read 10:45")),
                    ),
                ),
        )
    }
    fn bubble_preview(&self) -> impl IntoElement + use<> {
        div().w(px(384.)).child(
            MessageGroup::new()
                .child(
                    Message::new().child(
                        MessageContent::new().child(
                            Bubble::new()
                                .variant(self.bubble_variant)
                                .content("This bubble follows the variant control.")
                                .child(
                                    BubbleReactions::new()
                                        .side(BubbleSide::Bottom)
                                        .align(BubbleAlign::End)
                                        .child("\u{2764}\u{fe0f} 2"),
                                ),
                        ),
                    ),
                )
                .child(
                    Message::new().align(MessageAlign::End).child(
                        MessageContent::new().align(MessageAlign::End).child(
                            Bubble::new()
                                .variant(BubbleVariant::Default)
                                .content("And this one is the sender side.")
                                .child(
                                    BubbleReactions::new()
                                        .side(BubbleSide::Top)
                                        .align(BubbleAlign::Start)
                                        .child("\u{1f44d}"),
                                ),
                        ),
                    ),
                ),
        )
    }
    fn attachment_preview(&self, cx: &mut Context<Self>) -> impl IntoElement + use<> {
        div()
            .flex()
            .flex_col()
            .items_start()
            .gap(px(12.))
            .when(self.attachment_visible, |el| {
                el.child(
                    Attachment::new("attachment-report", "quarterly-report.pdf")
                        .description("1.2 MB \u{00b7} PDF")
                        .on_remove(cx.listener(|this, _, _, cx| {
                            this.attachment_visible = false;
                            cx.notify();
                        })),
                )
            })
            .child(
                Attachment::new("attachment-uploading", "screenshot.png")
                    .description("Uploading\u{2026}")
                    .state(AttachmentState::Uploading)
                    .media(Spinner::new().size(px(16.))),
            )
            .child(
                Attachment::new("attachment-error", "huge-video.mov")
                    .description("File exceeds the 25 MB limit")
                    .state(AttachmentState::Error),
            )
            .when(!self.attachment_visible, |el| {
                el.child(
                    Button::new("attachment-restore")
                        .variant(ButtonVariant::Outline)
                        .size(ButtonSize::Sm)
                        .on_click(cx.listener(|this, _, _, cx| {
                            this.attachment_visible = true;
                            cx.notify();
                        }))
                        .child("Restore attachment"),
                )
            })
    }
    fn marker_preview(&self, _cx: &mut Context<Self>) -> impl IntoElement + use<> {
        div()
            .flex()
            .flex_col()
            .gap(px(16.))
            .w(px(384.))
            .child(Marker::new().child("Yesterday"))
            .child(
                Marker::new()
                    .variant(MarkerVariant::Separator)
                    .child("Today"),
            )
            .child(
                Marker::new()
                    .variant(MarkerVariant::Border)
                    .child("Alex joined the conversation"),
            )
    }
    fn message_scroller_preview(&self, cx: &mut Context<Self>) -> impl IntoElement + use<> {
        let theme = Theme::of(cx).clone();
        div()
            .w(px(384.))
            .rounded(theme.radius_lg())
            .border_1()
            .border_color(theme.border)
            .p(px(12.))
            .child(
                MessageScroller::new("message-scroller-demo")
                    .h(px(240.))
                    .children((1..=8).map(|number| {
                        let end = number % 2 == 0;
                        Message::new()
                            .align(if end {
                                MessageAlign::End
                            } else {
                                MessageAlign::Start
                            })
                            .child(
                                MessageContent::new()
                                    .align(if end {
                                        MessageAlign::End
                                    } else {
                                        MessageAlign::Start
                                    })
                                    .child(
                                        Bubble::new()
                                            .variant(if end {
                                                BubbleVariant::Default
                                            } else {
                                                BubbleVariant::Muted
                                            })
                                            .content(format!("Message number {number}")),
                                    ),
                            )
                    })),
            )
    }
    fn questionnaire_preview(&self, cx: &mut Context<Self>) -> impl IntoElement + use<> {
        let options = [
            ("Daily", "I use it every day"),
            ("Weekly", "A few times a week"),
            ("Rarely", "Once in a while"),
        ];
        div().w(px(384.)).child(
            Questionnaire::new()
                .child(QuestionnaireProgress::new(2, 5))
                .child(QuestionnaireTitle::new().child("How often do you use rcn?"))
                .child(
                    QuestionnaireDescription::new()
                        .child("This helps us prioritize what to build next."),
                )
                .child(
                    QuestionnaireChoices::new().children(options.into_iter().enumerate().map(
                        |(index, (label, description))| {
                            QuestionnaireChoice::new(("questionnaire-choice", index), label)
                                .description(description)
                                .checked(self.questionnaire_selected == Some(index))
                                .on_select(cx.listener(move |this, _, _, cx| {
                                    this.questionnaire_selected = Some(index);
                                    cx.notify();
                                }))
                        },
                    )),
                )
                .child(
                    QuestionnaireActions::new()
                        .previous(
                            Button::new("questionnaire-previous")
                                .variant(ButtonVariant::Outline)
                                .child("Previous"),
                        )
                        .action(
                            Button::new("questionnaire-skip")
                                .variant(ButtonVariant::Ghost)
                                .child("Skip"),
                        )
                        .action(Button::new("questionnaire-next").child("Next")),
                ),
        )
    }

    // __STORY_PREVIEWS__

    fn card_preview(&self, _cx: &App) -> impl IntoElement + use<> {
        let size = self.card_size;
        // Card is RenderOnce+ParentElement (not Styled); width goes on a wrapper.
        div().w(px(384.)).child(
            Card::new()
                .size(size)
                .child(
                    CardHeader::new()
                        .size(size)
                        .action(
                            Button::new("card-sign-up")
                                .variant(ButtonVariant::Link)
                                .child("Sign Up"),
                        )
                        .child(CardTitle::new().size(size).child("Login to your account"))
                        .child(
                            CardDescription::new()
                                .child("Enter your email below to login to your account"),
                        ),
                )
                .child(
                    CardContent::new().size(size).child(
                        div()
                            .flex()
                            .flex_col()
                            .gap(px(24.))
                            .child(
                                div()
                                    .flex()
                                    .flex_col()
                                    .gap(px(8.))
                                    .child(Label::new().child("Email"))
                                    .child(self.card_email_input.clone()),
                            )
                            .child(
                                div()
                                    .flex()
                                    .flex_col()
                                    .gap(px(8.))
                                    .child(
                                        div()
                                            .flex()
                                            .flex_row()
                                            .items_center()
                                            .child(Label::new().child("Password"))
                                            .child(
                                                div()
                                                    .id("card-forgot-password")
                                                    .ml_auto()
                                                    .text_size(px(14.))
                                                    .line_height(px(20.))
                                                    .cursor_pointer()
                                                    .hover(|s| s.underline())
                                                    .child("Forgot your password?"),
                                            ),
                                    )
                                    .child(self.card_password_input.clone()),
                            ),
                    ),
                )
                .child(
                    CardFooter::new().size(size).child(
                        div()
                            .w_full()
                            .flex()
                            .flex_col()
                            .gap(px(8.))
                            .child(
                                div().w_full().child(
                                    Button::new("card-login")
                                        .variant(ButtonVariant::Default)
                                        .child("Login"),
                                ),
                            )
                            .child(
                                div().w_full().child(
                                    Button::new("card-login-google")
                                        .variant(ButtonVariant::Outline)
                                        .child("Login with Google"),
                                ),
                            ),
                    ),
                ),
        )
    }

    fn popover_preview(&self, cx: &mut Context<Self>) -> impl IntoElement + use<> {
        Popover::new("preview-popover")
            .trigger(
                Button::new("popover-trigger")
                    .variant(ButtonVariant::Outline)
                    .child("Open popover"),
            )
            .open(self.popover_open)
            .on_open_change(cx.listener(|this, open: &bool, _, cx| {
                this.popover_open = *open;
                cx.notify();
            }))
            .content(
                PopoverHeader::new()
                    .child(PopoverTitle::new().child("Dimensions"))
                    .child(PopoverDescription::new().child("Set the dimensions for the layer.")),
            )
    }
}

impl gpui::Focusable for Storybook {
    fn focus_handle(&self, _cx: &App) -> gpui::FocusHandle {
        self.focus_handle.clone()
    }
}

impl Render for Storybook {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let theme = Theme::of(cx).clone();
        div()
            .track_focus(&self.focus_handle)
            .on_key_down(cx.listener(|_, event: &gpui::KeyDownEvent, window, cx| {
                // Keyboard traversal across the component tab stops
                // (shadcn inherits this from the browser; gpui needs it
                // wired once at the root).
                if event.keystroke.key == "tab" {
                    if event.keystroke.modifiers.shift {
                        window.focus_prev(cx);
                    } else {
                        window.focus_next(cx);
                    }
                    cx.stop_propagation();
                }
            }))
            .on_key_down(cx.listener(|this, event: &gpui::KeyDownEvent, _, cx| {
                // Escape dismisses whichever overlay is open (shadcn overlays
                // close on Escape; component-level focus traps are a TODO,
                // so the demo handles it at the root).
                if event.keystroke.key == "escape" {
                    this.dialog_open = false;
                    this.alert_dialog_open = false;
                    this.sheet_open = false;
                    this.drawer_open = false;
                    this.popover_open = false;
                    this.dropdown_open = false;
                    this.select_open = false;
                    this.native_select_open = false;
                    this.combobox_open = false;
                    this.date_picker_open = false;
                    this.menubar_open = None;
                    this.nav_menu_open = None;
                    this.context_menu_at = None;
                    this.toast_visible = false;
                    cx.notify();
                }
            }))
            .size_full()
            .bg(theme.background)
            .text_color(theme.foreground)
            .when_some(theme.font_sans.clone(), |el, font| el.font_family(font))
            .child(SidebarProvider::new().sidebar(self.sidebar(cx)).inset(
                if self.story == Story::Tokens {
                    self.showcase(cx)
                } else {
                    div()
                        .flex()
                        .flex_row()
                        .size_full()
                        .min_w(px(0.))
                        .child(self.canvas(cx))
                        .child(self.controls_panel(cx))
                        .into_any_element()
                },
            ))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Importing a theme must back-fill the controls: primary onto the
    /// sliders, radius onto its slider, and the nearest base gray family.
    #[test]
    fn import_syncs_controls() {
        let css = r#"
:root {
  --background: oklch(1 0 0);
  --foreground: oklch(0.147 0.004 49.3);
  --primary: oklch(0.553 0.195 38.402);
  --secondary: oklch(0.967 0.001 286.375);
  --muted-foreground: oklch(0.547 0.021 43.1);
  --border: oklch(0.922 0.005 34.3);
  --radius: 0;
}"#;
        let (light, _) = Theme::from_shadcn_css(css).expect("should parse");
        let mut tokens = TokenSettings::default();
        tokens.sync_from(&light);
        assert!(tokens.custom_primary);
        assert!(
            tokens.hue < 0.12,
            "red-orange brand should land on a warm hue, got {}",
            tokens.hue
        );
        assert!(tokens.saturation > 0.3);
        assert_eq!(tokens.radius, 0.);
        // Warm-tinted neutrals sit closest to the stone family.
        assert_eq!(tokens.base, BaseColor::Stone);
    }

    /// A stock-neutral theme (black primary) must not flip the sliders into
    /// custom-brand mode.
    #[test]
    fn neutral_import_keeps_default_primary() {
        let mut tokens = TokenSettings::default();
        tokens.custom_primary = true;
        tokens.sync_from(&Theme::light());
        assert!(!tokens.custom_primary);
        assert_eq!(tokens.base, BaseColor::Neutral);
    }
}
