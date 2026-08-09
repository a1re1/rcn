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

use std::time::{SystemTime, UNIX_EPOCH};

use gpui::{
    AnyElement, App, ClickEvent, Context, DragMoveEvent, FontWeight, Hsla, Window, div, hsla,
    prelude::*, px, relative, rgb,
};

use crate::assets::IconLibrary;
use crate::components::{
    Accordion, AccordionItem, Alert, AlertDescription, AlertDialog, AlertDialogDescription,
    AlertDialogFooter, AlertDialogHeader, AlertDialogTitle, AlertTitle, AlertVariant, AspectRatio,
    Attachment, AttachmentState, Avatar, AvatarGroup, AvatarGroupCount, AvatarSize, Badge,
    BadgeVariant, BarChart, Breadcrumb, BreadcrumbEllipsis, BreadcrumbItem, BreadcrumbLink,
    BreadcrumbList, BreadcrumbPage, BreadcrumbSeparator, Bubble, BubbleAlign, BubbleReactions,
    BubbleSide, BubbleVariant, Button, ButtonGroup, ButtonGroupSeparator, ButtonGroupText,
    ButtonSize, ButtonVariant, Calendar, CalendarDate, Card, CardAction, CardContent,
    CardDescription, CardFooter, CardHeader, CardSize, CardTitle, Carousel, ChartSeries, Checkbox,
    Collapsible, Combobox, Command, CommandGroup, CommandItem, ContextMenu, ContextMenuItem,
    DatePicker, Dialog, DialogDescription, DialogFooter, DialogHeader, DialogTitle, Drawer,
    DrawerDescription, DrawerFooter, DrawerHeader, DrawerTitle, DropdownMenu, DropdownMenuItem,
    Empty, EmptyContent, EmptyDescription, EmptyHeader, EmptyMedia, EmptyMediaVariant, EmptyTitle,
    Field, FieldDescription, FieldError, FieldGroup, FieldLegend, FieldSet, HoverCard, Input,
    InputGroup, InputGroupAddon, InputOtp, Item, ItemActions, ItemContent, ItemDescription,
    ItemFooter, ItemGroup, ItemHeader, ItemMedia, ItemMediaVariant, ItemSeparator, ItemSize,
    ItemTitle, ItemVariant, Kbd, KbdGroup, Label, Menubar, MenubarItem, MenubarMenu, Message,
    MessageAlign, MessageAvatar, MessageContent, MessageFooter, MessageGroup, MessageHeader,
    NativeSelect, NavigationMenu, NavigationMenuEntry, NavigationMenuLink, Pagination,
    PaginationEllipsis, PaginationLink, PaginationNext, PaginationPrevious, Popover,
    PopoverDescription, PopoverHeader, PopoverTitle, Progress, RadioGroup, RadioGroupItem,
    ResizableDirection, ResizablePanelGroup, ScrollArea, Select, Separator, Sheet,
    SheetDescription, SheetFooter, SheetHeader, SheetSide, SheetTitle, Sidebar, SidebarContent,
    SidebarFooter, SidebarGroup, SidebarHeader, SidebarMenuButton, SidebarProvider, SidebarTrigger,
    Skeleton, Slider, Spinner, Switch, SwitchSize, Table, TableBody, TableCaption, TableCell,
    TableFooter, TableHead, TableHeader, TableRow, Tabs, TabsContent, TabsList, TabsTrigger,
    TabsVariant, Textarea, Toast, ToastViewport, Toggle, ToggleGroup, ToggleGroupItem, ToggleSize,
    ToggleVariant, Tooltip,
};
use crate::theme::{BaseColor, Theme, oklch};

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
    // __STORY_VARIANTS__
}

impl Story {
    const ALL: [Story; 60] = [
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
            } // __STORY_DESCRIPTIONS__
        }
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
    // Input story state
    input_demo: gpui::Entity<Input>,
    input_disabled: gpui::Entity<Input>,
    // Textarea story state
    textarea_input: gpui::Entity<Input>,
    // Field story state
    field_input: gpui::Entity<Input>,
    field_error_input: gpui::Entity<Input>,
    // Input group story state
    input_group_search: gpui::Entity<Input>,
    input_group_url: gpui::Entity<Input>,
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
    // Resizable story state
    resizable_fraction: f32,
    // Sidebar story state
    sidebar_open: bool,
    sidebar_active: usize,
    // Data table story state
    data_table_desc: bool,
    // Bubble story state
    bubble_variant: BubbleVariant,
    // Attachment story state
    attachment_visible: bool,
    // __STORY_STATE__
    // Accordion / Popover state
    accordion_open: Option<usize>,
    popover_open: bool,
    // Card controls
    card_size: CardSize,
}

impl Storybook {
    pub fn new(cx: &mut Context<Self>) -> Self {
        let input_demo = cx.new(|cx| {
            let mut input = Input::new(cx);
            input.placeholder("Email");
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
        // Live-refresh stories that derive UI from input text.
        for input in [&command_input, &input_otp, &input_demo, &combobox_search] {
            cx.observe(input, |_, _, cx| cx.notify()).detach();
        }
        let input_disabled = cx.new(|cx| {
            let mut input = Input::new(cx);
            input.placeholder("Disabled");
            input.set_disabled(true);
            input
        });
        let seed = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .map(|d| d.subsec_nanos() as u64 ^ d.as_secs())
            .unwrap_or(0x9e3779b9);
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
            textarea_input,
            field_input,
            field_error_input,
            input_group_search,
            input_group_url,
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
            resizable_fraction: 0.5,
            sidebar_open: true,
            sidebar_active: 0,
            data_table_desc: true,
            bubble_variant: BubbleVariant::Muted,
            attachment_visible: true,
            // __STORY_STATE_INIT__
            accordion_open: Some(0),
            popover_open: false,
            card_size: CardSize::Default,
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
        div()
            .flex()
            .flex_col()
            .w(px(200.))
            .flex_shrink_0()
            .h_full()
            .border_r_1()
            .border_color(theme.border)
            .bg(if theme.dark {
                theme.card
            } else {
                theme.secondary
            })
            .child(
                div()
                    .px(px(16.))
                    .py(px(14.))
                    .text_size(px(15.))
                    .font_weight(FontWeight::SEMIBOLD)
                    .when_some(theme.heading_font(), |el, font| el.font_family(font))
                    .child("rcn"),
            )
            .child(
                div()
                    .flex()
                    .flex_col()
                    .flex_1()
                    .gap(px(2.))
                    .px(px(8.))
                    .children(Story::ALL.into_iter().enumerate().map(|(index, story)| {
                        let selected = self.story == story;
                        div()
                            .id(("nav", index))
                            .px(px(8.))
                            .py(px(5.))
                            .rounded(theme.radius_sm())
                            .text_size(px(13.))
                            .line_height(px(18.))
                            .map(|el| {
                                if selected {
                                    el.bg(theme.primary)
                                        .text_color(theme.primary_foreground)
                                        .font_weight(FontWeight::MEDIUM)
                                } else {
                                    el.text_color(theme.foreground).hover(|s| s.bg(theme.muted))
                                }
                            })
                            .on_click(cx.listener(move |this, _, _, cx| {
                                this.story = story;
                                cx.notify();
                            }))
                            .child(story.label())
                    })),
            )
            .child(
                div().p(px(12.)).child(
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
            // __STORY_CANVAS__
        };
        div()
            .id("canvas")
            .flex()
            .flex_col()
            .flex_1()
            // Shrinkable below its content's min-width, so a narrow window
            // squeezes the canvas instead of pushing the controls panel out.
            .min_w(px(0.))
            .h_full()
            .overflow_y_scroll()
            .overflow_x_hidden()
            .child(
                div()
                    .flex()
                    .flex_col()
                    .gap(px(4.))
                    .px(px(28.))
                    .pt(px(24.))
                    .child(
                        div()
                            .text_size(px(18.))
                            .line_height(px(24.))
                            .font_weight(FontWeight::SEMIBOLD)
                            .when_some(theme.heading_font(), |el, font| el.font_family(font))
                            .child(self.story.label()),
                    )
                    .child(
                        div()
                            .text_size(px(13.))
                            .line_height(px(18.))
                            .text_color(theme.muted_foreground)
                            .child(self.story.description()),
                    ),
            )
            .child(
                div().flex().flex_1().p(px(28.)).child(
                    div()
                        .flex()
                        .flex_1()
                        .min_h(px(280.))
                        .items_center()
                        .justify_center()
                        .rounded(theme.radius_lg())
                        .border_1()
                        .border_color(theme.border)
                        .p(px(32.))
                        .child(preview),
                ),
            )
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
            Story::Accordion => vec![Self::control_row(
                "open item",
                Self::choices(
                    "accordion-open",
                    &[
                        ("none", None),
                        ("first", Some(0)),
                        ("second", Some(1)),
                        ("third", Some(2)),
                    ],
                    self.accordion_open,
                    cx,
                    |this, v, cx| {
                        this.accordion_open = v;
                        cx.notify();
                    },
                ),
                &theme,
            )],
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
            button.child(
                gpui::svg()
                    .path(theme.icons.chevron_right())
                    .size(px(16.))
                    .text_color(theme.foreground),
            )
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

    fn accordion_preview(&self, cx: &mut Context<Self>) -> impl IntoElement + use<> {
        let items = [
            (
                "Product Information",
                "Our flagship product combines cutting-edge technology with sleek design. \
                 Built with premium materials, it offers unparalleled performance and \
                 reliability.",
            ),
            (
                "Shipping Details",
                "We offer worldwide shipping through trusted courier partners. Standard \
                 delivery takes 3-5 business days, while express shipping ensures delivery \
                 within 1-2 business days.",
            ),
            (
                "Return Policy",
                "We stand behind our products with a comprehensive 30-day return policy. If \
                 you're not completely satisfied, simply return the item in its original \
                 condition.",
            ),
        ];
        let count = items.len();
        div()
            .w(px(384.))
            .child(Accordion::new().children(items.into_iter().enumerate().map(
                |(index, (title, body))| {
                    AccordionItem::new(("accordion-item", index))
                        .trigger(title)
                        .content(body)
                        .open(self.accordion_open == Some(index))
                        .last(index + 1 == count)
                        .on_toggle(cx.listener(move |this, _: &ClickEvent, _, cx| {
                            this.accordion_open = if this.accordion_open == Some(index) {
                                None
                            } else {
                                Some(index)
                            };
                            cx.notify();
                        }))
                },
            )))
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
        div()
            .flex()
            .flex_col()
            .gap(px(12.))
            .child(
                KbdGroup::new()
                    .child(Kbd::new().child("⌘"))
                    .child(Kbd::new().child("K")),
            )
            .child(
                div()
                    .flex()
                    .flex_row()
                    .items_center()
                    .gap(px(4.))
                    .child(Kbd::new().child("Ctrl"))
                    .child(Kbd::new().child("⇧"))
                    .child(Kbd::new().child("Alt")),
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
        div().child(
            Tooltip::new("tooltip-demo", "Add to library").child(
                Button::new("tooltip-trigger")
                    .variant(ButtonVariant::Outline)
                    .child("Hover me"),
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
    fn input_preview(&self, cx: &mut Context<Self>) -> impl IntoElement + use<> {
        let theme = Theme::of(cx).clone();
        let value = self.input_demo.read(cx).text().to_string();
        div()
            .flex()
            .flex_col()
            .gap(px(16.))
            .w(px(288.))
            .child(self.input_demo.clone())
            .child(self.input_disabled.clone())
            .child(
                Button::new("input-clear")
                    .variant(ButtonVariant::Outline)
                    .size(ButtonSize::Sm)
                    .on_click(cx.listener(|this, _, _, cx| {
                        this.input_demo
                            .update(cx, |input, cx| input.set_text("", cx));
                        cx.notify();
                    }))
                    .child("Clear"),
            )
            .child(
                div()
                    .text_size(px(12.))
                    .text_color(theme.muted_foreground)
                    .child(if value.is_empty() {
                        "value: (empty)".to_string()
                    } else {
                        format!("value: {value}")
                    }),
            )
    }
    fn textarea_preview(&self, _cx: &mut Context<Self>) -> impl IntoElement + use<> {
        div()
            .w(px(288.))
            .child(Textarea::new(self.textarea_input.clone()).rows(4))
    }
    fn field_preview(&self, _cx: &mut Context<Self>) -> impl IntoElement + use<> {
        div().w(px(320.)).child(
            FieldSet::new()
                .child(FieldLegend::new().child("Profile"))
                .child(
                    FieldGroup::new()
                        .child(
                            Field::new()
                                .child(Label::new().child("Email"))
                                .child(self.field_input.clone())
                                .child(
                                    FieldDescription::new()
                                        .child("We'll use this to send you receipts."),
                                ),
                        )
                        .child(
                            Field::new()
                                .child(Label::new().child("Username"))
                                .child(self.field_error_input.clone())
                                .child(FieldError::new().child("This username is taken.")),
                        ),
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
                                |this, _, _, cx| {
                                    this.command_input.update(cx, |input, cx| {
                                        input.set_text("", cx);
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
    fn resizable_preview(&self, cx: &mut Context<Self>) -> impl IntoElement + use<> {
        let theme = Theme::of(cx).clone();
        let panel = |label: String| {
            div()
                .flex()
                .size_full()
                .items_center()
                .justify_center()
                .text_size(px(14.))
                .font_weight(FontWeight::MEDIUM)
                .text_color(theme.foreground)
                .child(label)
        };
        div()
            .flex()
            .flex_col()
            .gap(px(16.))
            .child(
                div().w(px(420.)).h(px(160.)).child(
                    ResizablePanelGroup::new("resizable-vertical")
                        .direction(ResizableDirection::Vertical)
                        .fraction(0.4)
                        .first(panel("Header".into()))
                        .second(panel("Content".into())),
                ),
            )
            .child(
                div().w(px(420.)).h(px(200.)).child(
                    ResizablePanelGroup::new("resizable-demo")
                        .fraction(self.resizable_fraction)
                        .on_fraction_change(cx.listener(|this, fraction: &f32, _, cx| {
                            this.resizable_fraction = *fraction;
                            cx.notify();
                        }))
                        .first(panel(format!("{:.0}%", self.resizable_fraction * 100.)))
                        .second(panel(format!(
                            "{:.0}%",
                            (1. - self.resizable_fraction) * 100.
                        ))),
                ),
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

    // __STORY_PREVIEWS__

    fn card_preview(&self, cx: &App) -> impl IntoElement + use<> {
        let theme = Theme::of(cx);
        let size = self.card_size;
        // Card is RenderOnce+ParentElement (not Styled); width goes on a wrapper.
        div().w(px(350.)).child(
            Card::new()
                .size(size)
                .child(
                    CardHeader::new()
                        .size(size)
                        .child(
                            div()
                                .flex()
                                .flex_row()
                                .items_start()
                                .child(CardTitle::new().child("Login to your account"))
                                .child(
                                    CardAction::new().child(
                                        Button::new("card-sign-up")
                                            .variant(ButtonVariant::Outline)
                                            .size(ButtonSize::Sm)
                                            .child("Sign Up"),
                                    ),
                                ),
                        )
                        .child(
                            CardDescription::new()
                                .child("Enter your details below to login to your account"),
                        ),
                )
                .child(
                    CardContent::new().size(size).child(
                        div()
                            .h(px(64.))
                            .w_full()
                            .rounded(theme.radius_md())
                            .bg(theme.muted),
                    ),
                )
                .child(
                    CardFooter::new().size(size).child(
                        div().w_full().child(
                            Button::new("card-login")
                                .variant(ButtonVariant::Default)
                                .child("Login"),
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

impl Render for Storybook {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let theme = Theme::of(cx).clone();
        div()
            .flex()
            .flex_row()
            .size_full()
            .bg(theme.background)
            .text_color(theme.foreground)
            .when_some(theme.font_sans.clone(), |el, font| el.font_family(font))
            .child(self.sidebar(cx))
            .child(self.canvas(cx))
            .child(self.controls_panel(cx))
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
