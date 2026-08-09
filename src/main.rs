//! rcn storybook: renders every component in the library so you can browse
//! and interact with them. Sections mirror the shadcn docs examples for each
//! ported component; the header button flips the light/dark theme to prove
//! the token system.

mod assets;
mod components;
mod theme;

use gpui::{
    App, AppContext, Application, Bounds, ClickEvent, Context, FontWeight, QuitMode, Window,
    WindowBounds, WindowOptions, div, prelude::*, px, size,
};

use assets::{Assets, ICON_CHEVRON_RIGHT};
use components::{
    Accordion, AccordionItem, Avatar, AvatarGroup, AvatarGroupCount, AvatarSize, Badge,
    BadgeVariant, Button, ButtonSize, ButtonVariant, Popover, PopoverDescription, PopoverHeader,
    PopoverTitle, Switch, SwitchSize,
};
use theme::Theme;

struct Storybook {
    airplane_mode: bool,
    bluetooth: bool,
    accordion_open: Option<usize>,
    popover_open: bool,
}

impl Storybook {
    fn new() -> Self {
        Self {
            airplane_mode: false,
            bluetooth: true,
            accordion_open: Some(0),
            popover_open: false,
        }
    }

    fn toggle_theme(&mut self, _: &ClickEvent, _window: &mut Window, cx: &mut Context<Self>) {
        let next = if Theme::of(cx).dark {
            Theme::light()
        } else {
            Theme::dark()
        };
        cx.set_global(next);
        cx.notify();
    }

    fn section<B: IntoElement>(
        title: &'static str,
        body: B,
        cx: &App,
    ) -> impl IntoElement + use<B> {
        let theme = Theme::of(cx);
        div()
            .flex()
            .flex_col()
            .gap(px(12.))
            .child(
                div()
                    .text_size(px(12.))
                    .line_height(px(16.))
                    .font_weight(FontWeight::MEDIUM)
                    .text_color(theme.muted_foreground)
                    .child(title),
            )
            .child(body)
    }

    fn tokens_section(cx: &App) -> impl IntoElement + use<> {
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
            .gap(px(12.))
            .child(div().flex().flex_row().flex_wrap().gap(px(4.)).children(
                swatches.into_iter().map(|(_name, color)| {
                    div()
                        .size(px(24.))
                        .rounded(theme.radius_sm())
                        .border_1()
                        .border_color(theme.border)
                        .bg(color)
                }),
            ))
            .child(
                div()
                    .flex()
                    .flex_row()
                    .items_end()
                    .gap(px(8.))
                    .children(radii.into_iter().map(|(name, radius)| {
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
                    })),
            )
    }

    fn buttons_section(cx: &App) -> impl IntoElement + use<> {
        let variants = div()
            .flex()
            .flex_row()
            .flex_wrap()
            .items_center()
            .gap(px(8.))
            .child(Button::new("btn-default").child("Button"))
            .child(
                Button::new("btn-outline")
                    .variant(ButtonVariant::Outline)
                    .child("Outline"),
            )
            .child(
                Button::new("btn-secondary")
                    .variant(ButtonVariant::Secondary)
                    .child("Secondary"),
            )
            .child(
                Button::new("btn-ghost")
                    .variant(ButtonVariant::Ghost)
                    .child("Ghost"),
            )
            .child(
                Button::new("btn-destructive")
                    .variant(ButtonVariant::Destructive)
                    .child("Destructive"),
            )
            .child(
                Button::new("btn-link")
                    .variant(ButtonVariant::Link)
                    .child("Link"),
            )
            .child(Button::new("btn-disabled").disabled(true).child("Disabled"));

        let theme = Theme::of(cx);
        let icon = |color| {
            gpui::svg()
                .path(ICON_CHEVRON_RIGHT)
                .size(px(16.))
                .text_color(color)
        };
        let sizes = div()
            .flex()
            .flex_row()
            .flex_wrap()
            .items_center()
            .gap(px(8.))
            .child(
                Button::new("btn-xs")
                    .variant(ButtonVariant::Outline)
                    .size(ButtonSize::Xs)
                    .child("Extra Small"),
            )
            .child(
                Button::new("btn-sm")
                    .variant(ButtonVariant::Outline)
                    .size(ButtonSize::Sm)
                    .child("Small"),
            )
            .child(
                Button::new("btn-md")
                    .variant(ButtonVariant::Outline)
                    .child("Default"),
            )
            .child(
                Button::new("btn-lg")
                    .variant(ButtonVariant::Outline)
                    .size(ButtonSize::Lg)
                    .child("Large"),
            )
            .child(
                Button::new("btn-icon-xs")
                    .variant(ButtonVariant::Outline)
                    .size(ButtonSize::IconXs)
                    .child(icon(theme.foreground)),
            )
            .child(
                Button::new("btn-icon-sm")
                    .variant(ButtonVariant::Outline)
                    .size(ButtonSize::IconSm)
                    .child(icon(theme.foreground)),
            )
            .child(
                Button::new("btn-icon")
                    .variant(ButtonVariant::Outline)
                    .size(ButtonSize::Icon)
                    .child(icon(theme.foreground)),
            )
            .child(
                Button::new("btn-icon-lg")
                    .variant(ButtonVariant::Outline)
                    .size(ButtonSize::IconLg)
                    .child(icon(theme.foreground)),
            );

        div()
            .flex()
            .flex_col()
            .gap(px(12.))
            .child(variants)
            .child(sizes)
    }

    fn badges_section() -> impl IntoElement {
        div()
            .flex()
            .flex_row()
            .flex_wrap()
            .items_center()
            .gap(px(8.))
            .child(Badge::new().child("Badge"))
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
    }

    fn avatars_section() -> impl IntoElement {
        div()
            .flex()
            .flex_row()
            .items_center()
            .gap(px(16.))
            .child(Avatar::new("CN").size(AvatarSize::Sm))
            .child(Avatar::new("CN"))
            .child(Avatar::new("ER").size(AvatarSize::Lg))
            .child(
                AvatarGroup::new()
                    .child(Avatar::new("CN"))
                    .child(Avatar::new("ER"))
                    .child(Avatar::new("LR"))
                    .child(AvatarGroupCount::new(3)),
            )
            .child(
                AvatarGroup::new()
                    .child(Avatar::new("CN").size(AvatarSize::Sm))
                    .child(Avatar::new("ER").size(AvatarSize::Sm))
                    .child(AvatarGroupCount::new(2).size(AvatarSize::Sm)),
            )
    }

    fn switches_section(&self, cx: &mut Context<Self>) -> impl IntoElement + use<> {
        let theme = Theme::of(cx);
        let label = |text: &'static str, color| {
            div()
                .text_size(px(14.))
                .line_height(px(20.))
                .font_weight(FontWeight::MEDIUM)
                .text_color(color)
                .child(text)
        };
        div()
            .flex()
            .flex_row()
            .flex_wrap()
            .items_center()
            .gap(px(24.))
            .child(
                div()
                    .flex()
                    .flex_row()
                    .items_center()
                    .gap(px(8.))
                    .child(
                        Switch::new("switch-airplane")
                            .checked(self.airplane_mode)
                            .on_change(cx.listener(|this, checked: &bool, _window, cx| {
                                this.airplane_mode = *checked;
                                cx.notify();
                            })),
                    )
                    .child(label("Airplane Mode", theme.foreground)),
            )
            .child(
                div()
                    .flex()
                    .flex_row()
                    .items_center()
                    .gap(px(8.))
                    .child(
                        Switch::new("switch-bluetooth")
                            .size(SwitchSize::Sm)
                            .checked(self.bluetooth)
                            .on_change(cx.listener(|this, checked: &bool, _window, cx| {
                                this.bluetooth = *checked;
                                cx.notify();
                            })),
                    )
                    .child(label("Bluetooth (sm)", theme.foreground)),
            )
            .child(
                div()
                    .flex()
                    .flex_row()
                    .items_center()
                    .gap(px(8.))
                    .child(Switch::new("switch-disabled").checked(true).disabled(true))
                    .child(label("Disabled", theme.muted_foreground)),
            )
    }

    fn accordion_section(&self, cx: &mut Context<Self>) -> impl IntoElement + use<> {
        // Copy from the shadcn accordion docs example.
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
        Accordion::new().children(items.into_iter().enumerate().map(|(index, (title, body))| {
            AccordionItem::new(("accordion-item", index))
                .trigger(title)
                .content(body)
                .open(self.accordion_open == Some(index))
                .last(index + 1 == count)
                .on_toggle(cx.listener(move |this, _: &ClickEvent, _window, cx| {
                    this.accordion_open = if this.accordion_open == Some(index) {
                        None
                    } else {
                        Some(index)
                    };
                    cx.notify();
                }))
        }))
    }

    fn popover_section(&self, cx: &mut Context<Self>) -> impl IntoElement + use<> {
        Popover::new("popover-dimensions")
            .trigger(
                Button::new("popover-trigger")
                    .variant(ButtonVariant::Outline)
                    .child("Open popover"),
            )
            .open(self.popover_open)
            .on_open_change(cx.listener(|this, open: &bool, _window, cx| {
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

        let header = div()
            .flex()
            .flex_row()
            .items_center()
            .justify_between()
            .child(
                div()
                    .flex()
                    .flex_col()
                    .gap(px(2.))
                    .child(
                        div()
                            .text_size(px(20.))
                            .line_height(px(28.))
                            .font_weight(FontWeight::SEMIBOLD)
                            .child("rcn"),
                    )
                    .child(
                        div()
                            .text_size(px(13.))
                            .text_color(theme.muted_foreground)
                            .child("shadcn components, ported to gpui"),
                    ),
            )
            .child(
                Button::new("theme-toggle")
                    .variant(ButtonVariant::Outline)
                    .size(ButtonSize::Sm)
                    .on_click(cx.listener(Self::toggle_theme))
                    .child(if theme.dark { "Light" } else { "Dark" }),
            );

        div()
            .id("storybook")
            .size_full()
            .overflow_y_scroll()
            .bg(theme.background)
            .text_color(theme.foreground)
            .child(
                div()
                    .flex()
                    .flex_col()
                    .gap(px(32.))
                    .p(px(32.))
                    .max_w(px(640.))
                    .child(header)
                    .child(Self::section("Tokens", Self::tokens_section(cx), cx))
                    .child(Self::section("Buttons", Self::buttons_section(cx), cx))
                    .child(Self::section("Badges", Self::badges_section(), cx))
                    .child(Self::section("Avatars", Self::avatars_section(), cx))
                    .child(Self::section("Switch", self.switches_section(cx), cx))
                    .child(Self::section("Accordion", self.accordion_section(cx), cx))
                    .child(Self::section("Popover", self.popover_section(cx), cx)),
            )
    }
}

fn main() {
    // At this gpui rev the platform lives in the gpui_platform crate; zed's own
    // main builds it the same way (current_platform → Application::with_platform).
    let platform = gpui_platform::current_platform(false);
    // macOS's default keeps the process alive after the last window closes
    // (document-app convention); a single-window tool should just quit.
    let app = Application::with_platform(platform)
        .with_assets(Assets)
        .with_quit_mode(QuitMode::LastWindowClosed);
    app.run(|cx: &mut App| {
        cx.set_global(Theme::light());
        let bounds = Bounds::centered(None, size(px(800.0), px(720.0)), cx);
        cx.open_window(
            WindowOptions {
                window_bounds: Some(WindowBounds::Windowed(bounds)),
                titlebar: Some(gpui::TitlebarOptions {
                    title: Some("rcn".into()),
                    ..Default::default()
                }),
                ..Default::default()
            },
            |_window, cx| cx.new(|_cx| Storybook::new()),
        )
        .expect("failed to open window");
        cx.activate(true);
    });
}
