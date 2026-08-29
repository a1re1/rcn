//! <https://tailwindcss.com/docs/font-variant-numeric>

use crate::tw_docs::demo::{el, labeled};

use crate::tw_docs::{Node, TwExample, TwPage, TwSection};

/// <https://tailwindcss.com/docs/font-variant-numeric>
pub static FONT_VARIANT_NUMERIC: TwPage = TwPage {
    slug: "font-variant-numeric",
    title: "Font-variant-numeric",
    section: TwSection::Typography,
    description: "Utilities for controlling the variant of numbers.",
    reference: &[
        ("normal-nums", "font-variant-numeric: normal;"),
        ("ordinal", "font-variant-numeric: ordinal;"),
        ("slashed-zero", "font-variant-numeric: slashed-zero;"),
        ("lining-nums", "font-variant-numeric: lining-nums;"),
        ("oldstyle-nums", "font-variant-numeric: oldstyle-nums;"),
        (
            "proportional-nums",
            "font-variant-numeric: proportional-nums;",
        ),
        ("tabular-nums", "font-variant-numeric: tabular-nums;"),
        (
            "diagonal-fractions",
            "font-variant-numeric: diagonal-fractions;",
        ),
        (
            "stacked-fractions",
            "font-variant-numeric: stacked-fractions;",
        ),
    ],
    examples: &[
        TwExample {
            title: "Using ordinal glyphs",
            prose: &[
                "Use the ordinal utility to enable special glyphs for the ordinal markers in fonts that support them:",
                "1st",
            ],
            snippet: r#"<p class="ordinal ...">1st</p>"#,
            demo: el(
                "flex w-full flex-col gap-3",
                &[el("ordinal text-2xl", &[Node::Prose("1st")])],
            ),
        },
        TwExample {
            title: "Using slashed zeroes",
            prose: &[
                "Use the slashed-zero utility to force a zero with a slash in fonts that support them:",
                "0",
            ],
            snippet: r#"<p class="slashed-zero ...">0</p>"#,
            demo: el(
                "flex w-full flex-col gap-3",
                &[el("slashed-zero text-2xl", &[Node::Prose("0")])],
            ),
        },
        TwExample {
            title: "Using lining figures",
            prose: &[
                "Use the lining-nums utility to use numeric glyphs that are aligned by their baseline in fonts that support them:",
                "1234567890",
            ],
            snippet: r#"<p class="lining-nums ...">1234567890</p>"#,
            demo: el(
                "flex w-full flex-col gap-3",
                &[el("lining-nums text-2xl", &[Node::Prose("1234567890")])],
            ),
        },
        TwExample {
            title: "Using oldstyle figures",
            prose: &[
                "Use the oldstyle-nums utility to use numeric glyphs where some numbers have descenders in fonts that support them:",
                "1234567890",
            ],
            snippet: r#"<p class="oldstyle-nums ...">1234567890</p>"#,
            demo: el(
                "flex w-full flex-col gap-3",
                &[el("oldstyle-nums text-2xl", &[Node::Prose("1234567890")])],
            ),
        },
        TwExample {
            title: "Using proportional figures",
            prose: &[
                "Use the proportional-nums utility to use numeric glyphs that have proportional widths in fonts that support them:",
                "12121",
                "90909",
            ],
            snippet: r#"<p class="proportional-nums ...">12121</p>
<p class="proportional-nums ...">90909</p>"#,
            demo: el(
                "flex w-full flex-col gap-3",
                &[
                    el("proportional-nums text-2xl", &[Node::Prose("12121")]),
                    el("proportional-nums text-2xl", &[Node::Prose("90909")]),
                ],
            ),
        },
        TwExample {
            title: "Using tabular figures",
            prose: &[
                "Use the tabular-nums utility to use numeric glyphs that have uniform/tabular widths in fonts that support them:",
                "12121",
                "90909",
            ],
            snippet: r#"<p class="tabular-nums ...">12121</p>
<p class="tabular-nums ...">90909</p>"#,
            demo: el(
                "flex w-full flex-col gap-3",
                &[
                    el("tabular-nums text-2xl", &[Node::Prose("12121")]),
                    el("tabular-nums text-2xl", &[Node::Prose("90909")]),
                ],
            ),
        },
        TwExample {
            title: "Using diagonal fractions",
            prose: &[
                "Use the diagonal-fractions utility to replace numbers separated by a slash with common diagonal fractions in fonts that support them:",
                "1/2 3/4 5/6",
            ],
            snippet: r#"<p class="diagonal-fractions ...">1/2 3/4 5/6</p>"#,
            demo: el(
                "flex w-full flex-col gap-3",
                &[el(
                    "diagonal-fractions text-2xl",
                    &[Node::Prose("1/2 3/4 5/6")],
                )],
            ),
        },
        TwExample {
            title: "Using stacked fractions",
            prose: &[
                "Use the stacked-fractions utility to replace numbers separated by a slash with common stacked fractions in fonts that support them:",
                "1/2 3/4 5/6",
            ],
            snippet: r#"<p class="stacked-fractions ...">1/2 3/4 5/6</p>"#,
            demo: el(
                "flex w-full flex-col gap-3",
                &[el(
                    "stacked-fractions text-2xl",
                    &[Node::Prose("1/2 3/4 5/6")],
                )],
            ),
        },
        TwExample {
            title: "Stacking multiple utilities",
            prose: &[
                "The font-variant-numeric utilities are composable so you can enable multiple variants by combining them:",
            ],
            snippet: r#"<dl class="...">
<dt class="...">Subtotal</dt>
<dd class="text-right slashed-zero tabular-nums ...">$100.00</dd>
<dt class="...">Tax</dt>
<dd class="text-right slashed-zero tabular-nums ...">$14.50</dd>
<dt class="...">Total</dt>
<dd class="text-right slashed-zero tabular-nums ...">$114.50</dd>
</dl>"#,
            demo: el(
                "grid w-64 grid-cols-2 gap-2",
                &[
                    Node::Prose("Subtotal"),
                    el(
                        "text-right slashed-zero tabular-nums",
                        &[Node::Prose("$100.00")],
                    ),
                    Node::Prose("Tax"),
                    el(
                        "text-right slashed-zero tabular-nums",
                        &[Node::Prose("$14.50")],
                    ),
                    el("font-semibold", &[Node::Prose("Total")]),
                    el(
                        "text-right slashed-zero tabular-nums font-semibold",
                        &[Node::Prose("$114.50")],
                    ),
                ],
            ),
        },
        TwExample {
            title: "Resetting numeric font variants",
            prose: &["Use the normal-nums property to reset numeric font variants:"],
            snippet: r#"<p class="slashed-zero tabular-nums md:normal-nums ...">
<!-- ... -->
</p>"#,
            demo: el(
                "flex w-full flex-col gap-3",
                &[
                    el(
                        "slashed-zero tabular-nums text-2xl",
                        &[Node::Prose("0123456789")],
                    ),
                    labeled(
                        "text-xs text-slate-500",
                        "md:normal-nums has no equivalent — the variants stay on",
                    ),
                ],
            ),
        },
        TwExample {
            title: "Responsive design",
            prose: &[
                "Prefix a font-variant-numeric utility with a breakpoint variant like md: to only apply the utility at medium screen sizes and above:",
                "Learn more about using variants in the variants documentation.",
            ],
            snippet: r#"<p class="proportional-nums md:tabular-nums ...">  Lorem ipsum dolor sit amet...</p>"#,
            demo: el(
                "flex w-full flex-col gap-3",
                &[el(
                    "proportional-nums md:tabular-nums text-2xl",
                    &[Node::Prose("1234567890")],
                )],
            ),
        },
    ],
};
