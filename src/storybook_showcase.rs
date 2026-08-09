//! Tokens story live theming playground — shadcn/create-style multi-card canvas
//! with a floating Customize panel over a scrollable component grid.
//!
//! Child module of `storybook` so it can reach private `Storybook` helpers
//! (`token_controls`, existing input entities, calendar state, etc.).

use gpui::{AnyElement, App, Context, FontWeight, SharedString, div, prelude::*, px};

use crate::components::*;
use crate::theme::Theme;

use super::Storybook;

impl Storybook {
    /// Full-canvas Tokens playground: scrollable multi-column demo grid with a
    /// floating left "Customize" panel that reuses `token_controls`.
    pub(super) fn showcase(&self, cx: &mut Context<Self>) -> AnyElement {
        let theme = Theme::of(cx).clone();

        // Distribute cards round-robin across 3 columns for balanced heights.
        let cards: Vec<AnyElement> = self.showcase_cards(cx);
        let mut cols: [Vec<AnyElement>; 3] = [Vec::new(), Vec::new(), Vec::new()];
        for (i, card) in cards.into_iter().enumerate() {
            cols[i % 3].push(card);
        }

        div()
            .relative()
            .size_full()
            .child(
                // Full-size vertically scrollable canvas (muted preview bg).
                div()
                    .id("canvas")
                    .size_full()
                    .overflow_y_scroll()
                    .overflow_x_hidden()
                    .bg(theme.muted)
                    .child(
                        div()
                            // Left pad clears the floating Customize panel (~300 + gutters).
                            .pl(px(340.))
                            .pr(px(24.))
                            .pt(px(24.))
                            .pb(px(24.))
                            .flex()
                            .flex_row()
                            .items_start()
                            .gap(px(16.))
                            .w_full()
                            .children(cols.into_iter().map(|col_cards| {
                                div()
                                    .flex()
                                    .flex_col()
                                    .flex_1()
                                    .min_w(px(0.))
                                    .gap(px(16.))
                                    .children(col_cards)
                                    .into_any_element()
                            })),
                    ),
            )
            .child(self.showcase_token_panel(cx))
            .into_any_element()
    }

    /// Floating left Customize card — reuses `token_controls` unchanged.
    fn showcase_token_panel(&self, cx: &mut Context<Self>) -> impl IntoElement + use<> {
        let theme = Theme::of(cx).clone();
        let rows = self.token_controls(cx);

        div()
            .absolute()
            .left(px(16.))
            .top(px(16.))
            .bottom(px(16.))
            .w(px(300.))
            .flex()
            .flex_col()
            .bg(theme.card)
            .border_1()
            .border_color(theme.border)
            .rounded(theme.radius_xl())
            .shadow_md()
            .overflow_hidden()
            .child(
                div()
                    .id("token-panel")
                    .flex()
                    .flex_col()
                    .flex_1()
                    .min_h(px(0.))
                    .overflow_y_scroll()
                    .overflow_x_hidden()
                    .p(px(16.))
                    .gap(px(12.))
                    .child(
                        div()
                            .text_size(px(16.))
                            .line_height(px(24.))
                            .font_weight(FontWeight::SEMIBOLD)
                            .when_some(theme.heading_font(), |el, font| el.font_family(font))
                            .child("Customize"),
                    )
                    .children(rows),
            )
    }

    /// All ~37 demo cards for the Tokens playground grid.
    fn showcase_cards(&self, cx: &mut Context<Self>) -> Vec<AnyElement> {
        vec![
            self.card_contribution_history(cx),
            self.card_distribute_track(cx),
            self.card_scan_to_connect(cx),
            self.card_q2_dividend(cx),
            self.card_dollar_cost_averaging(cx),
            self.card_syncing_accounts(cx),
            self.card_payout_threshold(cx),
            self.card_claimable_balance(cx),
            self.card_preferences(cx),
            self.card_savings_progress(cx),
            self.card_kitchen_island(cx),
            self.card_savings_targets(cx),
            self.card_buy_investment(cx),
            self.card_recent_transactions(cx),
            self.card_overview_sidebar(cx),
            self.card_account_sidebar(cx),
            self.card_faq(cx),
            self.card_payments_breadcrumb(cx),
            self.card_front_door(cx),
            self.card_search_holdings(cx),
            self.card_account_access(cx),
            self.card_card_balance(cx),
            self.card_payment_due(cx),
            self.card_yearly_activity(cx),
            self.card_transfer_funds(cx),
            self.card_upload_artwork(cx),
            self.card_loading_skeleton(cx),
            self.card_receiving_method(cx),
            self.card_power_usage(cx),
            self.card_set_up_payouts(cx),
            self.card_upcoming_payments(cx),
            self.card_living_room(cx),
            self.card_stock_performance(cx),
            self.card_empty_catalog(cx),
            self.card_set_milestone(cx),
            self.card_social_links(cx),
            self.card_notifications(cx),
        ]
    }

    // ── Shared helpers ──────────────────────────────────────────────────

    fn showcase_heading(&self, cx: &App, text: impl Into<SharedString>) -> AnyElement {
        let theme = Theme::of(cx);
        div()
            .text_size(px(16.))
            .line_height(px(24.))
            .font_weight(FontWeight::SEMIBOLD)
            .when_some(theme.heading_font(), |el, font| el.font_family(font))
            .child(text.into())
            .into_any_element()
    }

    fn showcase_muted(&self, cx: &App, text: impl Into<SharedString>) -> AnyElement {
        let theme = Theme::of(cx);
        div()
            .text_size(px(13.))
            .line_height(px(18.))
            .text_color(theme.muted_foreground)
            .child(text.into())
            .into_any_element()
    }

    fn showcase_select_trigger(&self, cx: &App, label: impl Into<SharedString>) -> AnyElement {
        let theme = Theme::of(cx);
        div()
            .flex()
            .items_center()
            .justify_between()
            .w_full()
            .h(px(36.))
            .px(px(12.))
            .rounded(theme.radius_md())
            .border_1()
            .border_color(theme.input)
            .bg(theme.background)
            .child(
                div()
                    .text_size(px(14.))
                    .text_color(theme.foreground)
                    .child(label.into()),
            )
            .child(
                div()
                    .text_size(px(12.))
                    .text_color(theme.muted_foreground)
                    .child("▾"),
            )
            .into_any_element()
    }

    fn showcase_stat_tile(
        &self,
        cx: &App,
        overline: impl Into<SharedString>,
        body: impl Into<SharedString>,
    ) -> AnyElement {
        let theme = Theme::of(cx);
        div()
            .flex()
            .flex_col()
            .gap(px(4.))
            .flex_1()
            .p(px(12.))
            .rounded(theme.radius_md())
            .bg(theme.muted)
            .child(
                div()
                    .text_size(px(11.))
                    .font_weight(FontWeight::MEDIUM)
                    .text_color(theme.muted_foreground)
                    .child(overline.into()),
            )
            .child(
                div()
                    .text_size(px(13.))
                    .line_height(px(18.))
                    .text_color(theme.foreground)
                    .child(body.into()),
            )
            .into_any_element()
    }

    fn showcase_sparkline(&self, cx: &App, heights: &[f32]) -> AnyElement {
        let theme = Theme::of(cx);
        div()
            .flex()
            .items_end()
            .gap(px(2.))
            .h(px(28.))
            .children(heights.iter().copied().map(|h| {
                div()
                    .w(px(4.))
                    .h(px(h.max(4.)))
                    .rounded(theme.radius_sm())
                    .bg(theme.primary)
                    .into_any_element()
            }))
            .into_any_element()
    }

    // ── Cards 1–10 ──────────────────────────────────────────────────────

    fn card_contribution_history(&self, cx: &mut Context<Self>) -> AnyElement {
        Card::new()
            .child(
                CardHeader::new()
                    .child(CardTitle::new().child("Contribution History"))
                    .child(
                        CardDescription::new()
                            .child("Monthly contributions over the last two quarters."),
                    ),
            )
            .child(
                CardContent::new().child(
                    div()
                        .flex()
                        .flex_col()
                        .gap(px(16.))
                        .child(
                            BarChart::new(["Jan", "Feb", "Mar", "Apr", "May", "Jun"])
                                .height(140.)
                                .series(ChartSeries::new(
                                    "Contributions",
                                    [820., 940., 780., 1100., 980., 1200.],
                                )),
                        )
                        .child(
                            div()
                                .flex()
                                .gap(px(8.))
                                .child(self.showcase_stat_tile(
                                    cx,
                                    "UPCOMING",
                                    "May 25, 2024 / $1,000 scheduled",
                                ))
                                .child(self.showcase_stat_tile(
                                    cx,
                                    "AUTO-SAVE PLAN",
                                    "Accelerated / Recurring weekly",
                                )),
                        )
                        .child(
                            div().w_full().child(
                                Button::new("sc-contrib-report")
                                    .variant(ButtonVariant::Default)
                                    .child("View Full Report"),
                            ),
                        ),
                ),
            )
            .into_any_element()
    }

    fn card_distribute_track(&self, cx: &mut Context<Self>) -> AnyElement {
        Card::new()
            .child(
                CardContent::new().child(
                    Empty::new()
                        .child(
                            EmptyHeader::new()
                                .child(
                                    EmptyMedia::new().variant(EmptyMediaVariant::Icon).child(
                                        div()
                                            .flex()
                                            .items_center()
                                            .justify_center()
                                            .size(px(40.))
                                            .rounded_full()
                                            .bg(Theme::of(cx).muted)
                                            .child("♪"),
                                    ),
                                )
                                .child(EmptyTitle::new().child("Distribute Track"))
                                .child(EmptyDescription::new().child(
                                    "Package your release for streaming platforms and stores.",
                                )),
                        )
                        .child(
                            EmptyContent::new().child(
                                Button::new("sc-create-release")
                                    .variant(ButtonVariant::Default)
                                    .child("Create Release"),
                            ),
                        ),
                ),
            )
            .into_any_element()
    }

    fn card_scan_to_connect(&self, cx: &mut Context<Self>) -> AnyElement {
        let theme = Theme::of(cx).clone();
        // Simple 8×8 checkerboard QR placeholder from theme colors.
        let cells: Vec<AnyElement> = (0..64)
            .map(|i| {
                let row = i / 8;
                let col = i % 8;
                let on = (row + col) % 2 == 0;
                div()
                    .size(px(10.))
                    .bg(if on {
                        theme.foreground
                    } else {
                        theme.background
                    })
                    .into_any_element()
            })
            .collect();

        Card::new()
            .child(
                CardContent::new().child(
                    div()
                        .flex()
                        .flex_col()
                        .items_center()
                        .gap(px(12.))
                        .py(px(8.))
                        .child(
                            div()
                                .flex()
                                .flex_col()
                                .gap(px(0.))
                                .p(px(8.))
                                .bg(theme.background)
                                .border_1()
                                .border_color(theme.border)
                                .rounded(theme.radius_md())
                                .child(div().flex().flex_wrap().w(px(80.)).children(cells)),
                        )
                        .child(self.showcase_heading(cx, "Scan to connect"))
                        .child(self.showcase_muted(
                            cx,
                            "Open the mobile app and scan this code to link your account.",
                        ))
                        .child(
                            Button::new("sc-scan-got-it")
                                .variant(ButtonVariant::Outline)
                                .child("Got it"),
                        ),
                ),
            )
            .into_any_element()
    }

    fn card_q2_dividend(&self, cx: &mut Context<Self>) -> AnyElement {
        let holdings = [
            (
                "Vanguard S&P 500",
                "42.1 shares",
                [8., 14., 10., 18., 16.],
                "$186.40",
            ),
            (
                "Schwab US Dividend",
                "18.0 shares",
                [6., 10., 12., 9., 14.],
                "$94.12",
            ),
            (
                "iShares Core REIT",
                "65.4 shares",
                [12., 8., 11., 15., 10.],
                "$72.05",
            ),
            (
                "Total Bond Market",
                "120.0 shares",
                [4., 5., 6., 5., 7.],
                "$41.88",
            ),
        ];

        Card::new()
            .child(
                CardHeader::new()
                    .child(CardTitle::new().child("Q2 Dividend Income"))
                    .child(CardDescription::new().child("Estimated payouts by holding.")),
            )
            .child(
                CardContent::new().child(ItemGroup::new().children(holdings.into_iter().map(
                    |(name, shares, spark, amount)| {
                        Item::new()
                            .child(
                                ItemContent::new()
                                    .child(ItemTitle::new().child(name))
                                    .child(ItemDescription::new().child(shares)),
                            )
                            .child(self.showcase_sparkline(cx, &spark))
                            .child(
                                div()
                                    .text_size(px(14.))
                                    .font_weight(FontWeight::MEDIUM)
                                    .child(amount),
                            )
                    },
                ))),
            )
            .into_any_element()
    }

    fn card_dollar_cost_averaging(&self, _cx: &mut Context<Self>) -> AnyElement {
        Card::new()
            .child(
                CardHeader::new()
                    .child(CardTitle::new().child("Dollar-Cost Averaging"))
                    .child(CardDescription::new().child(
                        "Invest a fixed amount on a schedule to smooth out market volatility over time. \
                         Your recurring buys continue regardless of price swings.",
                    )),
            )
            .into_any_element()
    }

    fn card_syncing_accounts(&self, _cx: &mut Context<Self>) -> AnyElement {
        Card::new()
            .child(
                CardContent::new().child(
                    Empty::new()
                        .child(
                            EmptyHeader::new()
                                .child(
                                    EmptyMedia::new()
                                        .variant(EmptyMediaVariant::Icon)
                                        .child(Spinner::new()),
                                )
                                .child(EmptyTitle::new().child("Syncing your accounts"))
                                .child(
                                    EmptyDescription::new().child(
                                        "Pulling the latest balances and transactions. This usually takes under a minute.",
                                    ),
                                ),
                        )
                        .child(
                            EmptyContent::new().child(
                                Button::new("sc-sync-cancel")
                                    .variant(ButtonVariant::Outline)
                                    .child("Cancel"),
                            ),
                        ),
                ),
            )
            .into_any_element()
    }

    fn card_payout_threshold(&self, cx: &mut Context<Self>) -> AnyElement {
        let theme = Theme::of(cx).clone();
        Card::new()
            .child(
                CardHeader::new()
                    .child(CardTitle::new().child("Payout Threshold"))
                    .child(
                        CardDescription::new()
                            .child("Set when earnings are automatically transferred."),
                    ),
            )
            .child(
                CardContent::new().child(
                    div()
                        .flex()
                        .flex_col()
                        .gap(px(16.))
                        .child(
                            Field::new()
                                .child(Label::new().child("Preferred Currency"))
                                .child(self.showcase_select_trigger(cx, "USD — US Dollar")),
                        )
                        .child(
                            Field::new()
                                .child(Label::new().child("Minimum Payout Amount"))
                                .child(
                                    div()
                                        .text_size(px(28.))
                                        .line_height(px(34.))
                                        .font_weight(FontWeight::SEMIBOLD)
                                        .when_some(theme.heading_font(), |el, font| {
                                            el.font_family(font)
                                        })
                                        .child("$2,500.00"),
                                )
                                .child(Slider::new("sc-payout-slider").value(0.55))
                                .child(
                                    div()
                                        .flex()
                                        .justify_between()
                                        .child(self.showcase_muted(cx, "Min $100"))
                                        .child(self.showcase_muted(cx, "Max $10,000")),
                                ),
                        )
                        .child(
                            Field::new().child(Label::new().child("Notes")).child(
                                // Static stand-in for Textarea (presentational).
                                div()
                                    .w_full()
                                    .min_h(px(72.))
                                    .p(px(12.))
                                    .rounded(theme.radius_md())
                                    .border_1()
                                    .border_color(theme.input)
                                    .bg(theme.background)
                                    .text_size(px(14.))
                                    .text_color(theme.muted_foreground)
                                    .child("Optional notes for your finance team…"),
                            ),
                        )
                        .child(
                            div().w_full().child(
                                Button::new("sc-save-threshold")
                                    .variant(ButtonVariant::Default)
                                    .child("Save Threshold"),
                            ),
                        ),
                ),
            )
            .into_any_element()
    }

    fn card_claimable_balance(&self, cx: &mut Context<Self>) -> AnyElement {
        let theme = Theme::of(cx).clone();
        Card::new()
            .child(
                CardContent::new().child(
                    div()
                        .flex()
                        .flex_col()
                        .gap(px(12.))
                        .child(
                            div()
                                .flex()
                                .items_center()
                                .justify_between()
                                .child(
                                    div()
                                        .text_size(px(32.))
                                        .line_height(px(38.))
                                        .font_weight(FontWeight::SEMIBOLD)
                                        .when_some(theme.heading_font(), |el, font| {
                                            el.font_family(font)
                                        })
                                        .child("$0.00"),
                                )
                                .child(
                                    Badge::new()
                                        .variant(BadgeVariant::Secondary)
                                        .child("Pending Setup"),
                                ),
                        )
                        .child(self.showcase_heading(cx, "Claimable Balance"))
                        .child(
                            div()
                                .flex()
                                .flex_col()
                                .gap(px(8.))
                                .p(px(12.))
                                .rounded(theme.radius_md())
                                .bg(theme.muted)
                                .child(self.kv_row(cx, "Net Royalties", "$0.00"))
                                .child(self.kv_row(cx, "Processing Fee", "-$0.00"))
                                .child(Separator::new())
                                .child(self.kv_row(cx, "Total Ready to Claim", "$0.00 USD")),
                        )
                        .child(self.showcase_muted(
                            cx,
                            "Complete payout setup to start claiming royalties from your catalog.",
                        )),
                ),
            )
            .into_any_element()
    }

    fn kv_row(
        &self,
        cx: &App,
        label: impl Into<SharedString>,
        value: impl Into<SharedString>,
    ) -> AnyElement {
        let theme = Theme::of(cx);
        div()
            .flex()
            .justify_between()
            .items_center()
            .child(
                div()
                    .text_size(px(13.))
                    .text_color(theme.muted_foreground)
                    .child(label.into()),
            )
            .child(
                div()
                    .text_size(px(13.))
                    .font_weight(FontWeight::MEDIUM)
                    .text_color(theme.foreground)
                    .child(value.into()),
            )
            .into_any_element()
    }

    fn card_preferences(&self, cx: &mut Context<Self>) -> AnyElement {
        Card::new()
            .child(
                CardHeader::new()
                    .child(CardTitle::new().child("Preferences"))
                    .child(CardDescription::new().child("Control visibility and alerts.")),
            )
            .child(
                CardContent::new().child(
                    div()
                        .flex()
                        .flex_col()
                        .gap(px(16.))
                        .child(
                            Field::new()
                                .child(Label::new().child("Default Currency"))
                                .child(self.showcase_select_trigger(cx, "USD — US Dollar")),
                        )
                        .child(Separator::new())
                        .child(self.switch_row(
                            cx,
                            "sc-pref-public",
                            "Public Statistics",
                            "Allow others to see anonymized portfolio stats.",
                            true,
                        ))
                        .child(self.switch_row(
                            cx,
                            "sc-pref-email",
                            "Email Notifications",
                            "Weekly digest of account activity.",
                            false,
                        )),
                ),
            )
            .child(
                CardFooter::new().child(
                    div()
                        .flex()
                        .w_full()
                        .justify_end()
                        .gap(px(8.))
                        .child(
                            Button::new("sc-pref-reset")
                                .variant(ButtonVariant::Outline)
                                .child("Reset"),
                        )
                        .child(
                            Button::new("sc-pref-save")
                                .variant(ButtonVariant::Default)
                                .child("Save Preferences"),
                        ),
                ),
            )
            .into_any_element()
    }

    fn switch_row(
        &self,
        cx: &App,
        id: &'static str,
        title: impl Into<SharedString>,
        desc: impl Into<SharedString>,
        checked: bool,
    ) -> AnyElement {
        let theme = Theme::of(cx);
        div()
            .flex()
            .items_start()
            .justify_between()
            .gap(px(12.))
            .child(
                div()
                    .flex()
                    .flex_col()
                    .gap(px(2.))
                    .flex_1()
                    .min_w(px(0.))
                    .child(
                        div()
                            .text_size(px(14.))
                            .font_weight(FontWeight::MEDIUM)
                            .text_color(theme.foreground)
                            .child(title.into()),
                    )
                    .child(
                        div()
                            .text_size(px(12.))
                            .text_color(theme.muted_foreground)
                            .child(desc.into()),
                    ),
            )
            .child(Switch::new(id).checked(checked).size(SwitchSize::Sm))
            .into_any_element()
    }

    fn card_savings_progress(&self, cx: &mut Context<Self>) -> AnyElement {
        let theme = Theme::of(cx).clone();
        Card::new()
            .child(
                CardContent::new().child(
                    div()
                        .flex()
                        .flex_col()
                        .gap(px(12.))
                        .child(
                            div()
                                .text_size(px(32.))
                                .line_height(px(38.))
                                .font_weight(FontWeight::SEMIBOLD)
                                .when_some(theme.heading_font(), |el, font| el.font_family(font))
                                .child("$24,000"),
                        )
                        .child(self.showcase_muted(cx, "80% of $30,000 goal"))
                        .child(Progress::new(80.0))
                        .child(Separator::new())
                        .child(self.kv_row(cx, "Projected Finish", "Oct 2025"))
                        .child(self.kv_row(cx, "Monthly Average", "$1,200"))
                        .child(self.kv_row(cx, "Top Contributor", "Payroll Direct")),
                ),
            )
            .into_any_element()
    }

    // ── Cards 11–20 ─────────────────────────────────────────────────────

    fn card_kitchen_island(&self, cx: &mut Context<Self>) -> AnyElement {
        Card::new()
            .child(
                CardHeader::new()
                    .child(CardTitle::new().child("Kitchen Island"))
                    .child(CardDescription::new().child("Hue Color Ambient"))
                    .child(
                        CardAction::new().child(
                            Switch::new("sc-kitchen-power")
                                .checked(true)
                                .size(SwitchSize::Sm),
                        ),
                    ),
            )
            .child(
                CardContent::new().child(
                    div()
                        .flex()
                        .flex_col()
                        .gap(px(14.))
                        .child(
                            ToggleGroup::new()
                                .variant(ToggleVariant::Outline)
                                .size(ToggleSize::Sm)
                                .item(
                                    ToggleGroupItem::new("sc-km-cook")
                                        .pressed(true)
                                        .child("Cooking"),
                                )
                                .item(
                                    ToggleGroupItem::new("sc-km-dine")
                                        .pressed(false)
                                        .child("Dining"),
                                )
                                .item(
                                    ToggleGroupItem::new("sc-km-night")
                                        .pressed(false)
                                        .child("Nightlight"),
                                )
                                .item(
                                    ToggleGroupItem::new("sc-km-focus")
                                        .pressed(false)
                                        .child("Focus"),
                                ),
                        )
                        .child(self.labeled_slider(cx, "sc-kit-bright", "Brightness", 0.72))
                        .child(self.labeled_slider(cx, "sc-kit-temp", "Color Temp", 0.45))
                        .child(self.labeled_slider(cx, "sc-kit-vol", "Volume", 0.3))
                        .child(self.labeled_slider(cx, "sc-kit-fade", "Fade", 0.6)),
                ),
            )
            .into_any_element()
    }

    fn labeled_slider(
        &self,
        cx: &App,
        id: &'static str,
        label: impl Into<SharedString>,
        value: f32,
    ) -> AnyElement {
        div()
            .flex()
            .flex_col()
            .gap(px(6.))
            .child(
                div()
                    .text_size(px(13.))
                    .font_weight(FontWeight::MEDIUM)
                    .text_color(Theme::of(cx).foreground)
                    .child(label.into()),
            )
            .child(Slider::new(id).value(value))
            .into_any_element()
    }

    fn card_savings_targets(&self, cx: &mut Context<Self>) -> AnyElement {
        let theme = Theme::of(cx).clone();
        Card::new()
            .child(
                CardHeader::new()
                    .child(CardTitle::new().child("Savings Targets"))
                    .child(CardDescription::new().child("Milestones across life goals."))
                    .child(
                        CardAction::new().child(
                            Button::new("sc-new-goal")
                                .variant(ButtonVariant::Outline)
                                .size(ButtonSize::Sm)
                                .child("New Goal"),
                        ),
                    ),
            )
            .child(
                CardContent::new().child(
                    div()
                        .flex()
                        .flex_col()
                        .gap(px(12.))
                        .child(self.milestone_tile(cx, "RETIREMENT", "$180,000", 0.62, "62% achieved", "Target $290,000"))
                        .child(self.milestone_tile(cx, "REAL ESTATE", "$42,500", 0.35, "35% achieved", "Target $120,000"))
                        .child(
                            div()
                                .p(px(10.))
                                .rounded(theme.radius_md())
                                .bg(theme.muted)
                                .child(self.showcase_muted(
                                    cx,
                                    "You're on track for retirement. Real estate is pacing 2 months behind plan.",
                                )),
                        ),
                ),
            )
            .into_any_element()
    }

    fn milestone_tile(
        &self,
        cx: &App,
        overline: impl Into<SharedString>,
        amount: impl Into<SharedString>,
        progress: f32,
        pct: impl Into<SharedString>,
        target: impl Into<SharedString>,
    ) -> AnyElement {
        let theme = Theme::of(cx);
        div()
            .flex()
            .flex_col()
            .gap(px(8.))
            .p(px(12.))
            .rounded(theme.radius_md())
            .bg(theme.muted)
            .child(
                div()
                    .text_size(px(11.))
                    .font_weight(FontWeight::MEDIUM)
                    .text_color(theme.muted_foreground)
                    .child(overline.into()),
            )
            .child(
                div()
                    .text_size(px(22.))
                    .font_weight(FontWeight::SEMIBOLD)
                    .when_some(theme.heading_font(), |el, font| el.font_family(font))
                    .child(amount.into()),
            )
            .child(Progress::new(progress * 100.))
            .child(
                div()
                    .flex()
                    .justify_between()
                    .child(
                        div()
                            .text_size(px(12.))
                            .text_color(theme.foreground)
                            .child(pct.into()),
                    )
                    .child(
                        div()
                            .text_size(px(12.))
                            .text_color(theme.muted_foreground)
                            .child(target.into()),
                    ),
            )
            .into_any_element()
    }

    fn card_buy_investment(&self, cx: &mut Context<Self>) -> AnyElement {
        let theme = Theme::of(cx).clone();
        Card::new()
            .child(
                CardHeader::new()
                    .child(CardTitle::new().child("Buy Investment"))
                    .child(CardDescription::new().child("Place a market or limit order.")),
            )
            .child(
                CardContent::new().child(
                    div()
                        .flex()
                        .flex_col()
                        .gap(px(14.))
                        .child(
                            Field::new()
                                .child(Label::new().child("Amount to Invest"))
                                .child(
                                    // Static InputGroup stand-in with $ addon.
                                    div()
                                        .flex()
                                        .items_center()
                                        .w_full()
                                        .h(px(36.))
                                        .rounded(theme.radius_md())
                                        .border_1()
                                        .border_color(theme.input)
                                        .bg(theme.background)
                                        .child(
                                            div()
                                                .px(px(12.))
                                                .text_size(px(14.))
                                                .text_color(theme.muted_foreground)
                                                .child("$"),
                                        )
                                        .child(
                                            div()
                                                .flex_1()
                                                .text_size(px(14.))
                                                .text_color(theme.foreground)
                                                .child("1,000.00"),
                                        ),
                                ),
                        )
                        .child(
                            Field::new()
                                .child(Label::new().child("Order Type"))
                                .child(self.showcase_select_trigger(cx, "Market order")),
                        )
                        .child(self.showcase_muted(
                            cx,
                            "Orders placed after market close execute at next open.",
                        ))
                        .child(self.kv_row(cx, "Estimated Shares", "2.41"))
                        .child(self.kv_row(cx, "Buying Power", "$12,480.00"))
                        .child(
                            div().w_full().child(
                                Button::new("sc-review-order")
                                    .variant(ButtonVariant::Default)
                                    .child("Review Order"),
                            ),
                        )
                        .child(self.showcase_muted(
                            cx,
                            "Securities offered through RCN Brokerage LLC, member FINRA/SIPC.",
                        )),
                ),
            )
            .into_any_element()
    }

    fn card_recent_transactions(&self, cx: &mut Context<Self>) -> AnyElement {
        let theme = Theme::of(cx).clone();
        let rows = [
            (
                "JD",
                "Payroll Deposit",
                "Income",
                "May 1",
                "+$3,200.00",
                true,
            ),
            (
                "AC",
                "Apartment Rent",
                "Housing",
                "May 1",
                "-$1,850.00",
                false,
            ),
            (
                "VF",
                "VTI Purchase",
                "Investing",
                "Apr 28",
                "-$500.00",
                false,
            ),
            ("UB", "Utilities", "Bills", "Apr 27", "-$124.60", false),
            ("CS", "Coffee Shop", "Food", "Apr 26", "-$6.45", false),
        ];

        Card::new()
            .child(
                CardHeader::new()
                    .child(CardTitle::new().child("Recent Transactions"))
                    .child(CardDescription::new().child("Last five account movements."))
                    .child(
                        CardAction::new().child(
                            Button::new("sc-tx-view-all")
                                .variant(ButtonVariant::Ghost)
                                .size(ButtonSize::Sm)
                                .child("View All"),
                        ),
                    ),
            )
            .child(
                CardContent::new().child(ItemGroup::new().children(rows.into_iter().map(
                    |(initials, name, cat, date, amount, positive)| {
                        Item::new()
                            .child(
                                ItemMedia::new().variant(ItemMediaVariant::Icon).child(
                                    div()
                                        .flex()
                                        .items_center()
                                        .justify_center()
                                        .size(px(36.))
                                        .rounded(theme.radius_md())
                                        .bg(theme.muted)
                                        .text_size(px(12.))
                                        .font_weight(FontWeight::SEMIBOLD)
                                        .child(initials),
                                ),
                            )
                            .child(
                                ItemContent::new()
                                    .child(ItemTitle::new().child(name))
                                    .child(ItemDescription::new().child(format!("{cat} · {date}"))),
                            )
                            .child(
                                div()
                                    .text_size(px(13.))
                                    .font_weight(FontWeight::MEDIUM)
                                    .text_color(if positive {
                                        theme.primary
                                    } else {
                                        theme.foreground
                                    })
                                    .child(amount),
                            )
                    },
                ))),
            )
            .into_any_element()
    }

    fn card_overview_sidebar(&self, cx: &mut Context<Self>) -> AnyElement {
        self.nav_card(
            cx,
            "Overview",
            &[
                (
                    "Overview",
                    true,
                    &[
                        ("Dashboard", true),
                        ("Transactions", false),
                        ("Investments", false),
                        ("Accounts", false),
                        ("Spending", false),
                    ],
                ),
                ("Planning", false, &[("Budgets", false), ("Goals", false)]),
            ],
        )
    }

    fn card_account_sidebar(&self, cx: &mut Context<Self>) -> AnyElement {
        self.nav_card(
            cx,
            "Account",
            &[
                (
                    "Account",
                    true,
                    &[
                        ("Profile", true),
                        ("Billing", false),
                        ("Notifications", false),
                        ("Security", false),
                        ("Appearance", false),
                    ],
                ),
                (
                    "Support",
                    false,
                    &[
                        ("Help Center", false),
                        ("Contact Us", false),
                        ("Status", false),
                    ],
                ),
            ],
        )
    }

    fn nav_card(
        &self,
        cx: &App,
        title: &'static str,
        groups: &[(&'static str, bool, &[(&'static str, bool)])],
    ) -> AnyElement {
        let theme = Theme::of(cx);
        Card::new()
            .child(
                CardHeader::new()
                    .child(CardTitle::new().child(title))
                    .child(CardDescription::new().child("Sidebar-style navigation.")),
            )
            .child(
                CardContent::new().child(div().flex().flex_col().gap(px(12.)).children(
                    groups.iter().enumerate().map(|(gi, (label, _, items))| {
                        div()
                            .flex()
                            .flex_col()
                            .gap(px(4.))
                            .child(
                                div()
                                    .px(px(8.))
                                    .text_size(px(11.))
                                    .font_weight(FontWeight::MEDIUM)
                                    .text_color(theme.muted_foreground)
                                    .child(*label),
                            )
                            .children(items.iter().enumerate().map(|(ii, (name, active))| {
                                let id = format!("sc-nav-{title}-{gi}-{ii}");
                                SidebarMenuButton::new(id)
                                    .active(*active)
                                    .child(*name)
                                    .into_any_element()
                            }))
                            .into_any_element()
                    }),
                )),
            )
            .into_any_element()
    }

    fn card_faq(&self, _cx: &mut Context<Self>) -> AnyElement {
        Card::new()
            .child(
                CardHeader::new()
                    .child(CardTitle::new().child("FAQ"))
                    .child(CardDescription::new().child("Common questions about your account.")),
            )
            .child(
                CardContent::new().child(
                    div()
                        .flex()
                        .flex_col()
                        .gap(px(12.))
                        .child(
                            Tabs::new()
                                .child(
                                    TabsList::new()
                                        .trigger(
                                            TabsTrigger::new("general")
                                                .active(true)
                                                .child("General"),
                                        )
                                        .trigger(
                                            TabsTrigger::new("billing").child("Billing"),
                                        )
                                        .trigger(
                                            TabsTrigger::new("goals").child("Goals"),
                                        ),
                                )
                                .child(
                                    TabsContent::new().child(
                                        Accordion::new("sc-faq-acc")
                                            .default_value(["faq-1"])
                                            .child(
                                                AccordionItem::new("faq-1")
                                                    .child(
                                                        AccordionTrigger::new()
                                                            .child("How do recurring investments work?"),
                                                    )
                                                    .child(
                                                        AccordionContent::new().child(
                                                            "We invest your chosen amount on the schedule you set, buying fractional shares when needed.",
                                                        ),
                                                    ),
                                            )
                                            .child(
                                                AccordionItem::new("faq-2")
                                                    .child(
                                                        AccordionTrigger::new()
                                                            .child("When do dividends settle?"),
                                                    )
                                                    .child(
                                                        AccordionContent::new().child(
                                                            "Most dividends post within 2–3 business days of the pay date.",
                                                        ),
                                                    ),
                                            )
                                            .child(
                                                AccordionItem::new("faq-3")
                                                    .child(
                                                        AccordionTrigger::new()
                                                            .child("Can I pause auto-invest?"),
                                                    )
                                                    .child(
                                                        AccordionContent::new().child(
                                                            "Yes — pause anytime from Preferences without closing the plan.",
                                                        ),
                                                    ),
                                            ),
                                    ),
                                ),
                        ),
                ),
            )
            .into_any_element()
    }

    fn card_payments_breadcrumb(&self, _cx: &mut Context<Self>) -> AnyElement {
        Card::new()
            .child(
                CardContent::new().child(
                    div()
                        .flex()
                        .flex_col()
                        .gap(px(14.))
                        .child(
                            Breadcrumb::new().child(
                                BreadcrumbList::new()
                                    .child(
                                        BreadcrumbItem::new()
                                            .child(BreadcrumbLink::new("sc-bc-home").child("Home")),
                                    )
                                    .child(BreadcrumbSeparator::new())
                                    .child(BreadcrumbItem::new().child(BreadcrumbEllipsis::new()))
                                    .child(BreadcrumbSeparator::new())
                                    .child(
                                        BreadcrumbItem::new()
                                            .child(BreadcrumbPage::new().child("Payments")),
                                    ),
                            ),
                        )
                        .child(
                            ItemGroup::new().children([
                                Item::new().child(
                                    ItemContent::new()
                                        .child(ItemTitle::new().child("Change transfer limit"))
                                        .child(
                                            ItemDescription::new()
                                                .child("Adjust daily and monthly outbound caps."),
                                        ),
                                ),
                                Item::new().child(
                                    ItemContent::new()
                                        .child(ItemTitle::new().child("Scheduled transfers"))
                                        .child(
                                            ItemDescription::new()
                                                .child("Review upcoming automated moves."),
                                        ),
                                ),
                                Item::new().child(
                                    ItemContent::new()
                                        .child(ItemTitle::new().child("Direct Debits"))
                                        .child(
                                            ItemDescription::new()
                                                .child("Manage authorized billers and merchants."),
                                        ),
                                ),
                                Item::new().child(
                                    ItemContent::new()
                                        .child(ItemTitle::new().child("Recurring card payments"))
                                        .child(
                                            ItemDescription::new()
                                                .child("Subscriptions charged to your debit card."),
                                        ),
                                ),
                            ]),
                        ),
                ),
            )
            .into_any_element()
    }

    fn card_front_door(&self, cx: &mut Context<Self>) -> AnyElement {
        let theme = Theme::of(cx).clone();
        Card::new()
            .child(
                CardHeader::new()
                    .child(CardTitle::new().child("Front Door"))
                    .child(CardDescription::new().child("Smart Lock Pro"))
                    .child(
                        CardAction::new().child(
                            Badge::new()
                                .variant(BadgeVariant::Secondary)
                                .child("Locked"),
                        ),
                    ),
            )
            .child(
                CardContent::new().child(
                    // AspectRatio-ish muted placeholder block.
                    div()
                        .w_full()
                        .h(px(140.))
                        .rounded(theme.radius_md())
                        .bg(theme.muted)
                        .border_1()
                        .border_color(theme.border)
                        .flex()
                        .items_center()
                        .justify_center()
                        .child(self.showcase_muted(cx, "Camera preview")),
                ),
            )
            .into_any_element()
    }

    fn card_search_holdings(&self, cx: &mut Context<Self>) -> AnyElement {
        let theme = Theme::of(cx).clone();
        let holdings = [
            (
                "VOO",
                "Vanguard S&P 500",
                "128 SHARES · Apr 12",
                "ETF",
                "$512.40",
            ),
            (
                "VIG",
                "Dividend Appreciation",
                "64 SHARES · Mar 3",
                "ETF",
                "$178.22",
            ),
            (
                "AAPL",
                "Apple Inc.",
                "20 SHARES · Jan 18",
                "Stock",
                "$189.05",
            ),
            ("O", "Realty Income", "90 SHARES · Feb 9", "REIT", "$54.10"),
        ];

        Card::new()
            .child(
                CardHeader::new()
                    .child(CardTitle::new().child("Search holdings"))
                    .child(CardDescription::new().child("Filter your portfolio by asset class.")),
            )
            .child(
                CardContent::new().child(
                    div()
                        .flex()
                        .flex_col()
                        .gap(px(12.))
                        .child(
                            // Static search field stand-in.
                            div()
                                .flex()
                                .items_center()
                                .w_full()
                                .h(px(36.))
                                .px(px(12.))
                                .rounded(theme.radius_md())
                                .border_1()
                                .border_color(theme.input)
                                .bg(theme.background)
                                .child(
                                    div()
                                        .text_size(px(14.))
                                        .text_color(theme.muted_foreground)
                                        .child("Search ticker or name…"),
                                ),
                        )
                        .child(
                            ToggleGroup::new()
                                .variant(ToggleVariant::Outline)
                                .size(ToggleSize::Sm)
                                .item(
                                    ToggleGroupItem::new("sc-hf-stocks")
                                        .pressed(true)
                                        .child("Stocks"),
                                )
                                .item(
                                    ToggleGroupItem::new("sc-hf-etfs")
                                        .pressed(false)
                                        .child("ETFs"),
                                )
                                .item(
                                    ToggleGroupItem::new("sc-hf-reits")
                                        .pressed(false)
                                        .child("REITs"),
                                ),
                        )
                        .child(ItemGroup::new().children(holdings.into_iter().map(
                            |(ticker, name, meta, kind, price)| {
                                Item::new()
                                    .child(
                                        ItemMedia::new().variant(ItemMediaVariant::Icon).child(
                                            div()
                                                .flex()
                                                .items_center()
                                                .justify_center()
                                                .size(px(36.))
                                                .rounded(theme.radius_md())
                                                .bg(theme.muted)
                                                .text_size(px(11.))
                                                .font_weight(FontWeight::SEMIBOLD)
                                                .child(ticker),
                                        ),
                                    )
                                    .child(
                                        ItemContent::new()
                                            .child(ItemTitle::new().child(name))
                                            .child(ItemDescription::new().child(meta)),
                                    )
                                    .child(Badge::new().variant(BadgeVariant::Outline).child(kind))
                                    .child(
                                        div()
                                            .text_size(px(13.))
                                            .font_weight(FontWeight::MEDIUM)
                                            .child(price),
                                    )
                            },
                        ))),
                ),
            )
            .into_any_element()
    }

    // ── Cards 21–30 ─────────────────────────────────────────────────────

    fn card_account_access(&self, cx: &mut Context<Self>) -> AnyElement {
        let theme = Theme::of(cx).clone();
        Card::new()
            .child(
                CardHeader::new()
                    .child(CardTitle::new().child("Account Access"))
                    .child(CardDescription::new().child("Update sign-in credentials.")),
            )
            .child(
                CardContent::new().child(
                    div()
                        .flex()
                        .flex_col()
                        .gap(px(14.))
                        .child(
                            Field::new()
                                .child(Label::new().child("Email"))
                                .child(self.static_input(cx, "alex@example.com")),
                        )
                        .child(
                            Field::new()
                                .child(Label::new().child("Password"))
                                .child(self.static_input(cx, "••••••••••••")),
                        )
                        .child(
                            div().w_full().child(
                                Button::new("sc-update-security")
                                    .variant(ButtonVariant::Default)
                                    .child("Update Security"),
                            ),
                        )
                        .child(
                            Item::new()
                                .child(
                                    ItemMedia::new().variant(ItemMediaVariant::Icon).child(
                                        div()
                                            .flex()
                                            .items_center()
                                            .justify_center()
                                            .size(px(36.))
                                            .rounded_full()
                                            .bg(theme.muted)
                                            .text_size(px(12.))
                                            .font_weight(FontWeight::SEMIBOLD)
                                            .child("AR"),
                                    ),
                                )
                                .child(
                                    ItemContent::new()
                                        .child(ItemTitle::new().child("Last changed 14 days ago"))
                                        .child(
                                            ItemDescription::new()
                                                .child("Password strength: strong"),
                                        ),
                                ),
                        ),
                ),
            )
            .into_any_element()
    }

    fn static_input(&self, cx: &App, value: impl Into<SharedString>) -> AnyElement {
        let theme = Theme::of(cx);
        div()
            .flex()
            .items_center()
            .w_full()
            .h(px(36.))
            .px(px(12.))
            .rounded(theme.radius_md())
            .border_1()
            .border_color(theme.input)
            .bg(theme.background)
            .text_size(px(14.))
            .text_color(theme.foreground)
            .child(value.into())
            .into_any_element()
    }

    fn card_card_balance(&self, cx: &mut Context<Self>) -> AnyElement {
        let theme = Theme::of(cx).clone();
        Card::new()
            .child(
                CardContent::new().child(
                    div()
                        .flex()
                        .flex_col()
                        .gap(px(6.))
                        .child(
                            div()
                                .text_size(px(32.))
                                .line_height(px(38.))
                                .font_weight(FontWeight::SEMIBOLD)
                                .when_some(theme.heading_font(), |el, font| el.font_family(font))
                                .child("US$12.94"),
                        )
                        .child(self.showcase_heading(cx, "Card Balance"))
                        .child(self.showcase_muted(cx, "Available to spend · refreshes daily")),
                ),
            )
            .into_any_element()
    }

    fn card_payment_due(&self, cx: &mut Context<Self>) -> AnyElement {
        let theme = Theme::of(cx).clone();
        Card::new()
            .child(
                CardContent::new().child(
                    div()
                        .flex()
                        .flex_col()
                        .gap(px(10.))
                        .child(
                            div()
                                .text_size(px(32.))
                                .line_height(px(38.))
                                .font_weight(FontWeight::SEMIBOLD)
                                .when_some(theme.heading_font(), |el, font| el.font_family(font))
                                .child("1 Apr"),
                        )
                        .child(self.showcase_heading(cx, "Payment Due"))
                        .child(
                            Button::new("sc-pay-early")
                                .variant(ButtonVariant::Outline)
                                .child("Pay Early"),
                        ),
                ),
            )
            .into_any_element()
    }

    fn card_yearly_activity(&self, _cx: &mut Context<Self>) -> AnyElement {
        Card::new()
            .child(
                CardHeader::new()
                    .child(CardTitle::new().child("Yearly Activity"))
                    .child(CardDescription::new().child("Contributions across the calendar year."))
                    .child(
                        CardAction::new()
                            .child(Badge::new().variant(BadgeVariant::Secondary).child("+12%")),
                    ),
            )
            .child(
                CardContent::new().child(
                    BarChart::new(["J", "F", "M", "A", "M", "J", "J", "A"])
                        .height(120.)
                        .series(ChartSeries::new(
                            "Activity",
                            [40., 65., 52., 80., 74., 90., 68., 85.],
                        )),
                ),
            )
            .into_any_element()
    }

    fn card_transfer_funds(&self, cx: &mut Context<Self>) -> AnyElement {
        let theme = Theme::of(cx).clone();
        Card::new()
            .child(
                CardHeader::new()
                    .child(CardTitle::new().child("Transfer Funds"))
                    .child(CardDescription::new().child("Move money between accounts.")),
            )
            .child(
                CardContent::new().child(
                    div()
                        .flex()
                        .flex_col()
                        .gap(px(12.))
                        .child(Field::new().child(Label::new().child("From")).child(
                            self.showcase_select_trigger(cx, "Checking ···4821 — $8,420.12"),
                        ))
                        .child(Field::new().child(Label::new().child("To")).child(
                            self.showcase_select_trigger(cx, "Brokerage ···9033 — $24,110.00"),
                        ))
                        .child(
                            Field::new().child(Label::new().child("Amount")).child(
                                div()
                                    .flex()
                                    .items_center()
                                    .w_full()
                                    .h(px(36.))
                                    .rounded(theme.radius_md())
                                    .border_1()
                                    .border_color(theme.input)
                                    .bg(theme.background)
                                    .child(
                                        div()
                                            .px(px(12.))
                                            .text_color(theme.muted_foreground)
                                            .child("$"),
                                    )
                                    .child(div().flex_1().text_size(px(14.)).child("500.00")),
                            ),
                        )
                        .child(Separator::new())
                        .child(self.kv_row(cx, "Transfer fee", "$0.00"))
                        .child(
                            div().w_full().child(
                                Button::new("sc-confirm-transfer")
                                    .variant(ButtonVariant::Default)
                                    .child("Confirm Transfer"),
                            ),
                        ),
                ),
            )
            .into_any_element()
    }

    fn card_upload_artwork(&self, cx: &mut Context<Self>) -> AnyElement {
        let theme = Theme::of(cx).clone();
        Card::new()
            .child(
                CardContent::new().child(
                    div()
                        .flex()
                        .flex_col()
                        .gap(px(12.))
                        .child(Label::new().child("Cover Artwork"))
                        .child(
                            div()
                                .flex()
                                .flex_col()
                                .items_center()
                                .justify_center()
                                .gap(px(8.))
                                .w_full()
                                .h(px(140.))
                                .rounded(theme.radius_md())
                                .border_1()
                                .border_color(theme.border)
                                .bg(theme.muted)
                                .child(
                                    div()
                                        .text_size(px(28.))
                                        .text_color(theme.muted_foreground)
                                        .child("+"),
                                )
                                .child(self.showcase_muted(
                                    cx,
                                    "Minimum 3000 × 3000px / JPEG or PNG only",
                                )),
                        )
                        .child(
                            Button::new("sc-browse-files")
                                .variant(ButtonVariant::Outline)
                                .child("Browse Files"),
                        ),
                ),
            )
            .into_any_element()
    }

    fn card_loading_skeleton(&self, _cx: &mut Context<Self>) -> AnyElement {
        Card::new()
            .child(
                CardContent::new().child(
                    div()
                        .flex()
                        .flex_col()
                        .gap(px(12.))
                        .child(
                            div()
                                .flex()
                                .items_center()
                                .gap(px(12.))
                                .child(Skeleton::new().w(px(40.)).h(px(40.)).rounded_full())
                                .child(
                                    div()
                                        .flex()
                                        .flex_col()
                                        .gap(px(6.))
                                        .flex_1()
                                        .child(Skeleton::new().w(px(160.)).h(px(14.)))
                                        .child(Skeleton::new().w(px(120.)).h(px(12.))),
                                ),
                        )
                        .child(Skeleton::new().w(px(280.)).h(px(12.)))
                        .child(Skeleton::new().w(px(240.)).h(px(12.)))
                        .child(Skeleton::new().w(px(200.)).h(px(12.)))
                        .child(Skeleton::new().w(px(280.)).h(px(96.))),
                ),
            )
            .into_any_element()
    }

    fn card_receiving_method(&self, cx: &mut Context<Self>) -> AnyElement {
        Card::new()
            .child(
                CardHeader::new()
                    .child(CardTitle::new().child("Receiving Method"))
                    .child(CardDescription::new().child("Where payouts should be sent.")),
            )
            .child(
                CardContent::new().child(
                    div()
                        .flex()
                        .flex_col()
                        .gap(px(14.))
                        .child(
                            Field::new()
                                .child(Label::new().child("Account Holder"))
                                .child(self.static_input(cx, "Alex Rivera")),
                        )
                        .child(
                            FieldSet::new()
                                .child(FieldLegend::new().child("Payout Method"))
                                .child(
                                    RadioGroup::new()
                                        .child(
                                            div()
                                                .flex()
                                                .flex_col()
                                                .gap(px(4.))
                                                .child(
                                                    div()
                                                        .flex()
                                                        .flex_row()
                                                        .items_center()
                                                        .gap(px(8.))
                                                        .child(
                                                            RadioGroupItem::new("sc-bank")
                                                                .checked(true),
                                                        )
                                                        .child(Label::new().child("Bank Transfer")),
                                                )
                                                .child(
                                                    div().pl(px(28.)).child(
                                                        FieldDescription::new()
                                                            .child("1–2 business days · no fee"),
                                                    ),
                                                ),
                                        )
                                        .child(
                                            div()
                                                .flex()
                                                .flex_col()
                                                .gap(px(4.))
                                                .child(
                                                    div()
                                                        .flex()
                                                        .flex_row()
                                                        .items_center()
                                                        .gap(px(8.))
                                                        .child(
                                                            RadioGroupItem::new("sc-paypal")
                                                                .checked(false),
                                                        )
                                                        .child(Label::new().child("PayPal")),
                                                )
                                                .child(
                                                    div().pl(px(28.)).child(
                                                        FieldDescription::new()
                                                            .child("Instant · 1.5% fee"),
                                                    ),
                                                ),
                                        )
                                        .child(
                                            div()
                                                .flex()
                                                .flex_col()
                                                .gap(px(4.))
                                                .child(
                                                    div()
                                                        .flex()
                                                        .flex_row()
                                                        .items_center()
                                                        .gap(px(8.))
                                                        .child(
                                                            RadioGroupItem::new("sc-crypto")
                                                                .checked(false),
                                                        )
                                                        .child(Label::new().child("Crypto")),
                                                )
                                                .child(
                                                    div().pl(px(28.)).child(
                                                        FieldDescription::new()
                                                            .child("Network fees vary"),
                                                    ),
                                                ),
                                        ),
                                ),
                        )
                        .child(
                            div().w_full().child(
                                Button::new("sc-save-payout")
                                    .variant(ButtonVariant::Default)
                                    .child("Save Payout Settings"),
                            ),
                        ),
                ),
            )
            .into_any_element()
    }

    fn card_power_usage(&self, cx: &mut Context<Self>) -> AnyElement {
        Card::new()
            .child(
                CardHeader::new()
                    .child(CardTitle::new().child("Power Usage"))
                    .child(CardDescription::new().child("Whole Home")),
            )
            .child(
                CardContent::new().child(
                    div()
                        .flex()
                        .flex_col()
                        .gap(px(12.))
                        .child(
                            BarChart::new(["Mon", "Tue", "Wed", "Thu", "Fri", "Sat", "Sun"])
                                .height(120.)
                                .series(ChartSeries::new(
                                    "kWh",
                                    [22., 28., 25., 30., 27., 34., 31.],
                                )),
                        )
                        .child(Separator::new())
                        .child(
                            div()
                                .flex()
                                .flex_col()
                                .gap(px(6.))
                                .child(
                                    div()
                                        .flex()
                                        .justify_between()
                                        .child(
                                            div()
                                                .text_size(px(13.))
                                                .font_weight(FontWeight::MEDIUM)
                                                .child("Monthly budget"),
                                        )
                                        .child(self.showcase_muted(cx, "72%")),
                                )
                                .child(Progress::new(72.0)),
                        ),
                ),
            )
            .into_any_element()
    }

    fn card_set_up_payouts(&self, cx: &mut Context<Self>) -> AnyElement {
        Card::new()
            .child(
                CardContent::new().child(
                    Empty::new()
                        .child(
                            EmptyHeader::new()
                                .child(
                                    EmptyMedia::new().variant(EmptyMediaVariant::Icon).child(
                                        div()
                                            .flex()
                                            .items_center()
                                            .justify_center()
                                            .size(px(40.))
                                            .rounded_full()
                                            .bg(Theme::of(cx).muted)
                                            .child("$"),
                                    ),
                                )
                                .child(EmptyTitle::new().child("No payout method"))
                                .child(EmptyDescription::new().child(
                                    "Add a bank account or wallet to start receiving royalties.",
                                )),
                        )
                        .child(
                            EmptyContent::new().child(
                                Button::new("sc-setup-payouts")
                                    .variant(ButtonVariant::Default)
                                    .child("Set Up Payouts"),
                            ),
                        ),
                ),
            )
            .into_any_element()
    }

    // ── Cards 31–37 ─────────────────────────────────────────────────────

    fn card_upcoming_payments(&self, cx: &mut Context<Self>) -> AnyElement {
        // Reuse storybook calendar state for a live-but-static calendar.
        let (year, month) = self.calendar_month;
        Card::new()
            .child(
                CardHeader::new()
                    .child(CardTitle::new().child("Upcoming Payments"))
                    .child(CardDescription::new().child("Bills and transfers on the calendar.")),
            )
            .child(
                CardContent::new().child(
                    div()
                        .flex()
                        .flex_col()
                        .gap(px(12.))
                        .child(
                            Calendar::new(year, month)
                                .selected(self.calendar_selected)
                                .on_month_change(cx.listener(|this, (y, m): &(i32, u32), _, cx| {
                                    this.calendar_month = (*y, *m);
                                    cx.notify();
                                }))
                                .on_select(cx.listener(|this, date: &CalendarDate, _, cx| {
                                    this.calendar_selected = Some(*date);
                                    cx.notify();
                                })),
                        )
                        .child(Separator::new())
                        .child(
                            ItemGroup::new().children([
                                Item::new()
                                    .child(
                                        ItemContent::new()
                                            .child(ItemTitle::new().child("Mortgage"))
                                            .child(ItemDescription::new().child("Due May 1")),
                                    )
                                    .child(
                                        Badge::new().variant(BadgeVariant::Outline).child("$2,140"),
                                    ),
                                Item::new()
                                    .child(
                                        ItemContent::new()
                                            .child(ItemTitle::new().child("Auto loan"))
                                            .child(ItemDescription::new().child("Due May 5")),
                                    )
                                    .child(
                                        Badge::new().variant(BadgeVariant::Secondary).child("$386"),
                                    ),
                            ]),
                        ),
                ),
            )
            .into_any_element()
    }

    fn card_living_room(&self, cx: &mut Context<Self>) -> AnyElement {
        Card::new()
            .child(
                CardHeader::new()
                    .child(CardTitle::new().child("Living Room"))
                    .child(CardDescription::new().child("Roller Shades")),
            )
            .child(
                CardContent::new().child(
                    div()
                        .flex()
                        .flex_col()
                        .gap(px(14.))
                        .child(self.labeled_slider(cx, "sc-shade-pos", "Position", 0.4))
                        .child(
                            ToggleGroup::new()
                                .variant(ToggleVariant::Outline)
                                .size(ToggleSize::Sm)
                                .item(
                                    ToggleGroupItem::new("sc-sh-open")
                                        .pressed(false)
                                        .child("Open"),
                                )
                                .item(
                                    ToggleGroupItem::new("sc-sh-half")
                                        .pressed(true)
                                        .child("Half"),
                                )
                                .item(
                                    ToggleGroupItem::new("sc-sh-closed")
                                        .pressed(false)
                                        .child("Closed"),
                                ),
                        ),
                ),
            )
            .into_any_element()
    }

    fn card_stock_performance(&self, cx: &mut Context<Self>) -> AnyElement {
        let theme = Theme::of(cx).clone();
        Card::new()
            .child(
                CardHeader::new()
                    .child(CardTitle::new().child("Stock Performance"))
                    .child(CardDescription::new().child("6-month price history")),
            )
            .child(
                CardContent::new().child(
                    div()
                        .flex()
                        .flex_col()
                        .gap(px(12.))
                        .child(
                            div()
                                .flex()
                                .items_center()
                                .w_full()
                                .h(px(36.))
                                .px(px(12.))
                                .rounded(theme.radius_md())
                                .border_1()
                                .border_color(theme.input)
                                .bg(theme.background)
                                .child(
                                    div()
                                        .text_size(px(14.))
                                        .font_weight(FontWeight::MEDIUM)
                                        .child("VOO"),
                                )
                                .child(
                                    div()
                                        .ml_auto()
                                        .text_size(px(12.))
                                        .text_color(theme.muted_foreground)
                                        .child("S&P 500 ETF"),
                                ),
                        )
                        .child(Separator::new())
                        .child(
                            BarChart::new(["Nov", "Dec", "Jan", "Feb", "Mar", "Apr"])
                                .height(130.)
                                .series(ChartSeries::new(
                                    "Close",
                                    [420., 435., 428., 450., 462., 480.],
                                )),
                        ),
                ),
            )
            .into_any_element()
    }

    fn card_empty_catalog(&self, cx: &mut Context<Self>) -> AnyElement {
        Card::new()
            .child(
                CardContent::new().child(
                    Empty::new()
                        .child(
                            EmptyHeader::new()
                                .child(
                                    EmptyMedia::new().variant(EmptyMediaVariant::Icon).child(
                                        div()
                                            .flex()
                                            .items_center()
                                            .justify_center()
                                            .size(px(40.))
                                            .rounded_full()
                                            .bg(Theme::of(cx).muted)
                                            .child("∅"),
                                    ),
                                )
                                .child(EmptyTitle::new().child("No investments yet"))
                                .child(
                                    EmptyDescription::new()
                                        .child("Browse the catalog to build your first portfolio."),
                                ),
                        )
                        .child(
                            EmptyContent::new().child(
                                Button::new("sc-view-catalog")
                                    .variant(ButtonVariant::Outline)
                                    .child("View Catalog"),
                            ),
                        ),
                ),
            )
            .into_any_element()
    }

    fn card_set_milestone(&self, cx: &mut Context<Self>) -> AnyElement {
        let theme = Theme::of(cx).clone();
        Card::new()
            .child(
                CardHeader::new()
                    .child(CardTitle::new().child("Set a new milestone"))
                    .child(CardDescription::new().child("Define a savings target and date.")),
            )
            .child(
                CardContent::new().child(
                    div()
                        .flex()
                        .flex_col()
                        .gap(px(12.))
                        .child(
                            Field::new()
                                .child(Label::new().child("Goal Name"))
                                .child(self.static_input(cx, "Emergency Fund")),
                        )
                        .child(
                            Field::new()
                                .child(Label::new().child("Target Amount"))
                                .child(
                                    div()
                                        .flex()
                                        .items_center()
                                        .w_full()
                                        .h(px(36.))
                                        .rounded(theme.radius_md())
                                        .border_1()
                                        .border_color(theme.input)
                                        .bg(theme.background)
                                        .child(
                                            div()
                                                .px(px(12.))
                                                .text_color(theme.muted_foreground)
                                                .child("$"),
                                        )
                                        .child(
                                            div().flex_1().text_size(px(14.)).child("10,000.00"),
                                        ),
                                ),
                        )
                        .child(
                            Field::new()
                                .child(Label::new().child("Target Date"))
                                .child(self.static_input(cx, "2026-12-31")),
                        ),
                ),
            )
            .child(
                CardFooter::new().child(
                    div()
                        .flex()
                        .w_full()
                        .justify_end()
                        .gap(px(8.))
                        .child(
                            Button::new("sc-milestone-cancel")
                                .variant(ButtonVariant::Outline)
                                .child("Cancel"),
                        )
                        .child(
                            Button::new("sc-milestone-create")
                                .variant(ButtonVariant::Default)
                                .child("Create Goal"),
                        ),
                ),
            )
            .into_any_element()
    }

    fn card_social_links(&self, cx: &mut Context<Self>) -> AnyElement {
        let theme = Theme::of(cx).clone();
        let rows = [
            ("twitter.com/", "rcn_finance"),
            ("github.com/", "rcn-hq"),
            ("linkedin.com/in/", "alex-rivera"),
        ];
        Card::new()
            .child(
                CardHeader::new()
                    .child(CardTitle::new().child("Social Links"))
                    .child(CardDescription::new().child("Public profiles shown on your page.")),
            )
            .child(
                CardContent::new().child(div().flex().flex_col().gap(px(10.)).children(
                    rows.into_iter().map(|(prefix, value)| {
                        div()
                            .flex()
                            .items_center()
                            .w_full()
                            .h(px(36.))
                            .rounded(theme.radius_md())
                            .border_1()
                            .border_color(theme.input)
                            .bg(theme.background)
                            .child(
                                div()
                                    .px(px(10.))
                                    .text_size(px(13.))
                                    .text_color(theme.muted_foreground)
                                    .child(prefix),
                            )
                            .child(
                                div()
                                    .flex_1()
                                    .text_size(px(14.))
                                    .text_color(theme.foreground)
                                    .child(value),
                            )
                            .into_any_element()
                    }),
                )),
            )
            .child(
                CardFooter::new().child(
                    div()
                        .flex()
                        .w_full()
                        .justify_end()
                        .gap(px(8.))
                        .child(
                            Button::new("sc-social-discard")
                                .variant(ButtonVariant::Outline)
                                .child("Discard"),
                        )
                        .child(
                            Button::new("sc-social-save")
                                .variant(ButtonVariant::Default)
                                .child("Save Changes"),
                        ),
                ),
            )
            .into_any_element()
    }

    fn card_notifications(&self, cx: &mut Context<Self>) -> AnyElement {
        let items = [
            (
                "sc-n-payout",
                "Payout confirmations",
                "Email when a payout is sent or fails.",
                true,
            ),
            (
                "sc-n-weekly",
                "Weekly summary",
                "Monday digest of balances and activity.",
                true,
            ),
            (
                "sc-n-security",
                "Security alerts",
                "Sign-ins from new devices and password changes.",
                true,
            ),
            (
                "sc-n-product",
                "Product updates",
                "New features and occasional tips.",
                false,
            ),
        ];
        Card::new()
            .child(
                CardHeader::new()
                    .child(CardTitle::new().child("Notifications"))
                    .child(CardDescription::new().child("Choose what we email you about.")),
            )
            .child(
                CardContent::new().child(
                    div()
                        .flex()
                        .flex_col()
                        .gap(px(14.))
                        .children(items.into_iter().map(|(id, title, desc, checked)| {
                            div()
                                .flex()
                                .items_start()
                                .gap(px(10.))
                                .child(Checkbox::new(id).checked(checked))
                                .child(
                                    div()
                                        .flex()
                                        .flex_col()
                                        .gap(px(2.))
                                        .flex_1()
                                        .min_w(px(0.))
                                        .child(
                                            div()
                                                .text_size(px(14.))
                                                .font_weight(FontWeight::MEDIUM)
                                                .child(title),
                                        )
                                        .child(
                                            div()
                                                .text_size(px(12.))
                                                .text_color(Theme::of(cx).muted_foreground)
                                                .child(desc),
                                        ),
                                )
                                .into_any_element()
                        }))
                        .child(
                            div().w_full().child(
                                Button::new("sc-notif-save")
                                    .variant(ButtonVariant::Default)
                                    .child("Save Preferences"),
                            ),
                        ),
                ),
            )
            .into_any_element()
    }
}
