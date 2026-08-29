//! Tailwind docs · Spacing: `padding`, `margin`.
//!
//! Logical-property demos show the LTR mapping only (`ps`/`pe` → left/right),
//! which is what the parser implements; `pbs`/`pbe`/`space-x-reverse` are
//! ledgered as having no gpui equivalent and are left to the snippet.

use super::demo::{el, labeled};
use super::{TwExample, TwPage, TwSection};

/// <https://tailwindcss.com/docs/padding>
pub static PADDING: TwPage = TwPage {
    slug: "padding",
    title: "Padding",
    section: TwSection::Spacing,
    description: "Utilities for controlling an element's padding.",
    reference: &[
        ("p-<number>", "padding: calc(var(--spacing) * <number>);"),
        ("p-px", "padding: 1px;"),
        ("p-(<custom-property>)", "padding: var(<custom-property>);"),
        ("p-[<value>]", "padding: <value>;"),
        (
            "px-<number>",
            "padding-inline: calc(var(--spacing) * <number>);",
        ),
        ("px-px", "padding-inline: 1px;"),
        (
            "px-(<custom-property>)",
            "padding-inline: var(<custom-property>);",
        ),
        ("px-[<value>]", "padding-inline: <value>;"),
        (
            "py-<number>",
            "padding-block: calc(var(--spacing) * <number>);",
        ),
        ("py-px", "padding-block: 1px;"),
        (
            "py-(<custom-property>)",
            "padding-block: var(<custom-property>);",
        ),
        ("py-[<value>]", "padding-block: <value>;"),
        (
            "ps-<number>",
            "padding-inline-start: calc(var(--spacing) * <number>);",
        ),
        ("ps-px", "padding-inline-start: 1px;"),
        (
            "ps-(<custom-property>)",
            "padding-inline-start: var(<custom-property>);",
        ),
        ("ps-[<value>]", "padding-inline-start: <value>;"),
        (
            "pe-<number>",
            "padding-inline-end: calc(var(--spacing) * <number>);",
        ),
        ("pe-px", "padding-inline-end: 1px;"),
        (
            "pe-(<custom-property>)",
            "padding-inline-end: var(<custom-property>);",
        ),
        ("pe-[<value>]", "padding-inline-end: <value>;"),
        (
            "pbs-<number>",
            "padding-block-start: calc(var(--spacing) * <number>);",
        ),
        ("pbs-px", "padding-block-start: 1px;"),
        (
            "pbs-(<custom-property>)",
            "padding-block-start: var(<custom-property>);",
        ),
        ("pbs-[<value>]", "padding-block-start: <value>;"),
        (
            "pbe-<number>",
            "padding-block-end: calc(var(--spacing) * <number>);",
        ),
        ("pbe-px", "padding-block-end: 1px;"),
        (
            "pbe-(<custom-property>)",
            "padding-block-end: var(<custom-property>);",
        ),
        ("pbe-[<value>]", "padding-block-end: <value>;"),
        (
            "pt-<number>",
            "padding-top: calc(var(--spacing) * <number>);",
        ),
        ("pt-px", "padding-top: 1px;"),
        (
            "pt-(<custom-property>)",
            "padding-top: var(<custom-property>);",
        ),
        ("pt-[<value>]", "padding-top: <value>;"),
        (
            "pr-<number>",
            "padding-right: calc(var(--spacing) * <number>);",
        ),
        ("pr-px", "padding-right: 1px;"),
        (
            "pr-(<custom-property>)",
            "padding-right: var(<custom-property>);",
        ),
        ("pr-[<value>]", "padding-right: <value>;"),
        (
            "pb-<number>",
            "padding-bottom: calc(var(--spacing) * <number>);",
        ),
        ("pb-px", "padding-bottom: 1px;"),
        (
            "pb-(<custom-property>)",
            "padding-bottom: var(<custom-property>);",
        ),
        ("pb-[<value>]", "padding-bottom: <value>;"),
        (
            "pl-<number>",
            "padding-left: calc(var(--spacing) * <number>);",
        ),
        ("pl-px", "padding-left: 1px;"),
        (
            "pl-(<custom-property>)",
            "padding-left: var(<custom-property>);",
        ),
        ("pl-[<value>]", "padding-left: <value>;"),
    ],
    examples: &[
        TwExample {
            title: "Basic example",
            prose: &[
                "Use p-<number> utilities like p-4 and p-8 to control the padding on all sides of an element:",
            ],
            snippet: r#"<div class="p-8 ...">p-8</div>"#,
            demo: el(
                "rounded-lg bg-sky-300 p-8",
                &[labeled(
                    "rounded-md bg-sky-500 px-4 py-3 text-center text-white",
                    "p-8",
                )],
            ),
        },
        TwExample {
            title: "Adding padding to one side",
            prose: &[
                "Use pt-<number>, pr-<number>, pb-<number>, and pl-<number> utilities like pt-6 and pr-4 to control the padding on one side of an element:",
            ],
            snippet: r#"<div class="pt-6 ...">pt-6</div>
<div class="pr-4 ...">pr-4</div>
<div class="pb-8 ...">pb-8</div>
<div class="pl-2 ...">pl-2</div>"#,
            demo: el(
                "flex flex-wrap items-start gap-4",
                &[
                    el(
                        "rounded-lg bg-purple-300 pt-6",
                        &[labeled(
                            "rounded-md bg-purple-500 px-4 py-3 text-white",
                            "pt-6",
                        )],
                    ),
                    el(
                        "rounded-lg bg-purple-300 pr-4",
                        &[labeled(
                            "rounded-md bg-purple-500 px-4 py-3 text-white",
                            "pr-4",
                        )],
                    ),
                    el(
                        "rounded-lg bg-purple-300 pb-8",
                        &[labeled(
                            "rounded-md bg-purple-500 px-4 py-3 text-white",
                            "pb-8",
                        )],
                    ),
                    el(
                        "rounded-lg bg-purple-300 pl-2",
                        &[labeled(
                            "rounded-md bg-purple-500 px-4 py-3 text-white",
                            "pl-2",
                        )],
                    ),
                ],
            ),
        },
        TwExample {
            title: "Adding horizontal padding",
            prose: &[
                "Use px-<number> utilities like px-4 and px-8 to control the horizontal padding of an element:",
            ],
            snippet: r#"<div class="px-8 ...">px-8</div>"#,
            demo: el(
                "rounded-lg bg-indigo-300 px-8",
                &[labeled(
                    "rounded-md bg-indigo-500 px-4 py-3 text-center text-white",
                    "px-8",
                )],
            ),
        },
        TwExample {
            title: "Adding vertical padding",
            prose: &[
                "Use py-<number> utilities like py-4 and py-8 to control the vertical padding of an element:",
            ],
            snippet: r#"<div class="py-8 ...">py-8</div>"#,
            demo: el(
                "rounded-lg bg-pink-300 py-8",
                &[labeled(
                    "rounded-md bg-pink-500 px-4 py-3 text-center text-white",
                    "py-8",
                )],
            ),
        },
        TwExample {
            title: "Using logical properties",
            prose: &[
                "Use ps-<number> or pe-<number> utilities like ps-4 and pe-8 to set the padding-inline-start and padding-inline-end logical properties, which map to either the left or right side based on the text direction:",
                "Left-to-right",
                "Right-to-left",
                "For more control, you can also use the LTR and RTL modifiers to conditionally apply specific styles depending on the current text direction.",
                "Use the pbs-<number> and pbe-<number> utilities to set the padding-block-start and padding-block-end logical properties, which map to either the top or bottom side based on the writing mode:",
            ],
            snippet: r#"<div>
<div dir="ltr">
<div class="ps-8 ...">ps-8</div>
<div class="pe-8 ...">pe-8</div>
</div>
<div dir="rtl">
<div class="ps-8 ...">ps-8</div>
<div class="pe-8 ...">pe-8</div>
</div>
</div>
<div class="pbs-8 ...">pbs-8</div>"#,
            demo: el(
                "flex flex-wrap items-start gap-4",
                &[
                    el(
                        "rounded-lg bg-indigo-300 ps-8",
                        &[labeled(
                            "rounded-md bg-indigo-500 px-4 py-3 text-white",
                            "ps-8",
                        )],
                    ),
                    el(
                        "rounded-lg bg-indigo-300 pe-8",
                        &[labeled(
                            "rounded-md bg-indigo-500 px-4 py-3 text-white",
                            "pe-8",
                        )],
                    ),
                ],
            ),
        },
        TwExample {
            title: "Using a custom value",
            prose: &[
                "Use utilities like p-[<value>],px-[<value>], and pb-[<value>] to set the padding based on a completely custom value:",
                "For CSS variables, you can also use the p-(<custom-property>) syntax:",
                "This is just a shorthand for p-[var(<custom-property>)] that adds the var() function for you automatically.",
            ],
            snippet: r#"<div class="p-[5px] ...">
<!-- ... -->
</div>
<div class="p-(--my-padding) ...">
<!-- ... -->
</div>"#,
            demo: el(
                "rounded-lg bg-sky-300 p-[5px]",
                &[labeled(
                    "rounded-md bg-sky-500 px-4 py-3 text-center text-white",
                    "p-[5px]",
                )],
            ),
        },
        TwExample {
            title: "Responsive design",
            prose: &[
                "Prefix a padding utility with a breakpoint variant like md: to only apply the utility at medium screen sizes and above:",
                "Learn more about using variants in the variants documentation.",
            ],
            snippet: r#"<div class="py-4 md:py-8 ...">
<!-- ... -->
</div>"#,
            demo: el(
                "rounded-lg bg-sky-300 py-4 md:py-8",
                &[labeled(
                    "rounded-md bg-sky-500 px-4 py-3 text-center text-white",
                    "py-4 md:py-8",
                )],
            ),
        },
    ],
};

/// <https://tailwindcss.com/docs/margin>
pub static MARGIN: TwPage = TwPage {
    slug: "margin",
    title: "Margin",
    section: TwSection::Spacing,
    description: "Utilities for controlling an element's margin.",
    reference: &[
        ("m-<number>", "margin: calc(var(--spacing) * <number>);"),
        ("-m-<number>", "margin: calc(var(--spacing) * -<number>);"),
        ("m-auto", "margin: auto;"),
        ("m-px", "margin: 1px;"),
        ("-m-px", "margin: -1px;"),
        ("m-(<custom-property>)", "margin: var(<custom-property>);"),
        ("m-[<value>]", "margin: <value>;"),
        (
            "mx-<number>",
            "margin-inline: calc(var(--spacing) * <number>);",
        ),
        (
            "-mx-<number>",
            "margin-inline: calc(var(--spacing) * -<number>);",
        ),
        ("mx-auto", "margin-inline: auto;"),
        ("mx-px", "margin-inline: 1px;"),
        ("-mx-px", "margin-inline: -1px;"),
        (
            "mx-(<custom-property>)",
            "margin-inline: var(<custom-property>);",
        ),
        ("mx-[<value>]", "margin-inline: <value>;"),
        (
            "my-<number>",
            "margin-block: calc(var(--spacing) * <number>);",
        ),
        (
            "-my-<number>",
            "margin-block: calc(var(--spacing) * -<number>);",
        ),
        ("my-auto", "margin-block: auto;"),
        ("my-px", "margin-block: 1px;"),
        ("-my-px", "margin-block: -1px;"),
        (
            "my-(<custom-property>)",
            "margin-block: var(<custom-property>);",
        ),
        ("my-[<value>]", "margin-block: <value>;"),
        (
            "ms-<number>",
            "margin-inline-start: calc(var(--spacing) * <number>);",
        ),
        (
            "-ms-<number>",
            "margin-inline-start: calc(var(--spacing) * -<number>);",
        ),
        ("ms-auto", "margin-inline-start: auto;"),
        ("ms-px", "margin-inline-start: 1px;"),
        ("-ms-px", "margin-inline-start: -1px;"),
        (
            "ms-(<custom-property>)",
            "margin-inline-start: var(<custom-property>);",
        ),
        ("ms-[<value>]", "margin-inline-start: <value>;"),
        (
            "me-<number>",
            "margin-inline-end: calc(var(--spacing) * <number>);",
        ),
        (
            "-me-<number>",
            "margin-inline-end: calc(var(--spacing) * -<number>);",
        ),
        ("me-auto", "margin-inline-end: auto;"),
        ("me-px", "margin-inline-end: 1px;"),
        ("-me-px", "margin-inline-end: -1px;"),
        (
            "me-(<custom-property>)",
            "margin-inline-end: var(<custom-property>);",
        ),
        ("me-[<value>]", "margin-inline-end: <value>;"),
        (
            "mbs-<number>",
            "margin-block-start: calc(var(--spacing) * <number>);",
        ),
        (
            "-mbs-<number>",
            "margin-block-start: calc(var(--spacing) * -<number>);",
        ),
        ("mbs-auto", "margin-block-start: auto;"),
        ("mbs-px", "margin-block-start: 1px;"),
        ("-mbs-px", "margin-block-start: -1px;"),
        (
            "mbs-(<custom-property>)",
            "margin-block-start: var(<custom-property>);",
        ),
        ("mbs-[<value>]", "margin-block-start: <value>;"),
        (
            "mbe-<number>",
            "margin-block-end: calc(var(--spacing) * <number>);",
        ),
        (
            "-mbe-<number>",
            "margin-block-end: calc(var(--spacing) * -<number>);",
        ),
        ("mbe-auto", "margin-block-end: auto;"),
        ("mbe-px", "margin-block-end: 1px;"),
        ("-mbe-px", "margin-block-end: -1px;"),
        (
            "mbe-(<custom-property>)",
            "margin-block-end: var(<custom-property>);",
        ),
        ("mbe-[<value>]", "margin-block-end: <value>;"),
        (
            "mt-<number>",
            "margin-top: calc(var(--spacing) * <number>);",
        ),
        (
            "-mt-<number>",
            "margin-top: calc(var(--spacing) * -<number>);",
        ),
        ("mt-auto", "margin-top: auto;"),
        ("mt-px", "margin-top: 1px;"),
        ("-mt-px", "margin-top: -1px;"),
        (
            "mt-(<custom-property>)",
            "margin-top: var(<custom-property>);",
        ),
        ("mt-[<value>]", "margin-top: <value>;"),
        (
            "mr-<number>",
            "margin-right: calc(var(--spacing) * <number>);",
        ),
        (
            "-mr-<number>",
            "margin-right: calc(var(--spacing) * -<number>);",
        ),
        ("mr-auto", "margin-right: auto;"),
        ("mr-px", "margin-right: 1px;"),
        ("-mr-px", "margin-right: -1px;"),
        (
            "mr-(<custom-property>)",
            "margin-right: var(<custom-property>);",
        ),
        ("mr-[<value>]", "margin-right: <value>;"),
        (
            "mb-<number>",
            "margin-bottom: calc(var(--spacing) * <number>);",
        ),
        (
            "-mb-<number>",
            "margin-bottom: calc(var(--spacing) * -<number>);",
        ),
        ("mb-auto", "margin-bottom: auto;"),
        ("mb-px", "margin-bottom: 1px;"),
        ("-mb-px", "margin-bottom: -1px;"),
        (
            "mb-(<custom-property>)",
            "margin-bottom: var(<custom-property>);",
        ),
        ("mb-[<value>]", "margin-bottom: <value>;"),
        (
            "ml-<number>",
            "margin-left: calc(var(--spacing) * <number>);",
        ),
        (
            "-ml-<number>",
            "margin-left: calc(var(--spacing) * -<number>);",
        ),
        ("ml-auto", "margin-left: auto;"),
        ("ml-px", "margin-left: 1px;"),
        ("-ml-px", "margin-left: -1px;"),
        (
            "ml-(<custom-property>)",
            "margin-left: var(<custom-property>);",
        ),
        ("ml-[<value>]", "margin-left: <value>;"),
        (
            "space-x-<number>",
            "& > :not(:last-child) {
  --tw-space-x-reverse: 0;
  margin-inline-start: calc(calc(var(--spacing) * <number>) * var(--tw-space-x-reverse));
  margin-inline-end: calc(calc(var(--spacing) * <number>) * calc(1 - var(--tw-space-x-reverse)));
};",
        ),
        (
            "-space-x-<number>",
            "& > :not(:last-child) {
  --tw-space-x-reverse: 0;
  margin-inline-start: calc(calc(var(--spacing) * -<number>) * var(--tw-space-x-reverse));
  margin-inline-end: calc(calc(var(--spacing) * -<number>) * calc(1 - var(--tw-space-x-reverse)));
};",
        ),
        (
            "space-x-px",
            "& > :not(:last-child) {
  --tw-space-x-reverse: 0;
  margin-inline-start: calc(1px * var(--tw-space-x-reverse));
  margin-inline-end: calc(1px * calc(1 - var(--tw-space-x-reverse)));
};",
        ),
        (
            "-space-x-px",
            "& > :not(:last-child) {
  --tw-space-x-reverse: 0;
  margin-inline-start: calc(-1px * var(--tw-space-x-reverse));
  margin-inline-end: calc(-1px * calc(1 - var(--tw-space-x-reverse)));
};",
        ),
        (
            "space-x-(<custom-property>)",
            "& > :not(:last-child) {
  --tw-space-x-reverse: 0;
  margin-inline-start: calc(var(<custom-property>) * var(--tw-space-x-reverse));
  margin-inline-end: calc(var(<custom-property>) * calc(1 - var(--tw-space-x-reverse)));
};",
        ),
        (
            "space-x-[<value>]",
            "& > :not(:last-child) {
  --tw-space-x-reverse: 0;
  margin-inline-start: calc(<value> * var(--tw-space-x-reverse));
  margin-inline-end: calc(<value> * calc(1 - var(--tw-space-x-reverse)));
};",
        ),
        (
            "space-y-<number>",
            "& > :not(:last-child) {
  --tw-space-y-reverse: 0;
  margin-block-start: calc(calc(var(--spacing) * <number>) * var(--tw-space-y-reverse));
  margin-block-end: calc(calc(var(--spacing) * <number>) * calc(1 - var(--tw-space-y-reverse)));
};",
        ),
        (
            "-space-y-<number>",
            "& > :not(:last-child) {
  --tw-space-y-reverse: 0;
  margin-block-start: calc(calc(var(--spacing) * -<number>) * var(--tw-space-y-reverse));
  margin-block-end: calc(calc(var(--spacing) * -<number>) * calc(1 - var(--tw-space-y-reverse)));
};",
        ),
        (
            "space-y-px",
            "& > :not(:last-child) {
  --tw-space-y-reverse: 0;
  margin-block-start: calc(1px * var(--tw-space-y-reverse));
  margin-block-end: calc(1px * calc(1 - var(--tw-space-y-reverse)));
};",
        ),
        (
            "-space-y-px",
            "& > :not(:last-child) {
  --tw-space-y-reverse: 0;
  margin-block-start: calc(-1px * var(--tw-space-y-reverse));
  margin-block-end: calc(-1px * calc(1 - var(--tw-space-y-reverse)));
};",
        ),
        (
            "space-y-(<custom-property>)",
            "& > :not(:last-child) {
  --tw-space-y-reverse: 0;
  margin-block-start: calc(var(<custom-property>) * var(--tw-space-y-reverse));
  margin-block-end: calc(var(<custom-property>) * calc(1 - var(--tw-space-y-reverse)));
};",
        ),
        (
            "space-y-[<value>]",
            "& > :not(:last-child) {
  --tw-space-y-reverse: 0;
  margin-block-start: calc(<value> * var(--tw-space-y-reverse));
  margin-block-end: calc(<value> * calc(1 - var(--tw-space-y-reverse)));
};",
        ),
        (
            "space-x-reverse",
            "& > :not(:last-child)) {
  --tw-space-x-reverse: 1;
}",
        ),
        (
            "space-y-reverse",
            "& > :not(:last-child)) {
  --tw-space-y-reverse: 1;
}",
        ),
    ],
    examples: &[
        TwExample {
            title: "Basic example",
            prose: &[
                "Use m-<number> utilities like m-4 and m-8 to control the margin on all sides of an element:",
            ],
            snippet: r#"<div class="m-8 ...">m-8</div>"#,
            demo: el(
                "rounded-lg bg-sky-300",
                &[labeled(
                    "m-8 rounded-md bg-sky-500 px-4 py-3 text-center text-white",
                    "m-8",
                )],
            ),
        },
        TwExample {
            title: "Adding margin to a single side",
            prose: &[
                "Use mt-<number>, mr-<number>, mb-<number>, and ml-<number> utilities like ml-2 and mt-6 to control the margin on one side of an element:",
            ],
            snippet: r#"<div class="mt-6 ...">mt-6</div>
<div class="mr-4 ...">mr-4</div>
<div class="mb-8 ...">mb-8</div>
<div class="ml-2 ...">ml-2</div>"#,
            demo: el(
                "flex flex-wrap items-start gap-4",
                &[
                    el(
                        "rounded-lg bg-purple-300",
                        &[labeled(
                            "mt-6 rounded-md bg-purple-500 px-4 py-3 text-white",
                            "mt-6",
                        )],
                    ),
                    el(
                        "rounded-lg bg-purple-300",
                        &[labeled(
                            "mr-4 rounded-md bg-purple-500 px-4 py-3 text-white",
                            "mr-4",
                        )],
                    ),
                    el(
                        "rounded-lg bg-purple-300",
                        &[labeled(
                            "mb-8 rounded-md bg-purple-500 px-4 py-3 text-white",
                            "mb-8",
                        )],
                    ),
                    el(
                        "rounded-lg bg-purple-300",
                        &[labeled(
                            "ml-2 rounded-md bg-purple-500 px-4 py-3 text-white",
                            "ml-2",
                        )],
                    ),
                ],
            ),
        },
        TwExample {
            title: "Adding horizontal margin",
            prose: &[
                "Use mx-<number> utilities like mx-4 and mx-8 to control the horizontal margin of an element:",
            ],
            snippet: r#"<div class="mx-8 ...">mx-8</div>"#,
            demo: el(
                "rounded-lg bg-indigo-300",
                &[labeled(
                    "mx-8 rounded-md bg-indigo-500 px-4 py-3 text-center text-white",
                    "mx-8",
                )],
            ),
        },
        TwExample {
            title: "Adding vertical margin",
            prose: &[
                "Use my-<number> utilities like my-4 and my-8 to control the vertical margin of an element:",
            ],
            snippet: r#"<div class="my-8 ...">my-8</div>"#,
            demo: el(
                "rounded-lg bg-pink-300",
                &[labeled(
                    "my-8 rounded-md bg-pink-500 px-4 py-3 text-center text-white",
                    "my-8",
                )],
            ),
        },
        TwExample {
            title: "Using negative values",
            prose: &[
                "To use a negative margin value, prefix the class name with a dash to convert it to a negative value:",
            ],
            snippet: r#"<div class="h-16 w-36 bg-sky-400 opacity-20 ...">
</div>
<div class="-mt-8 bg-sky-300 ...">-mt-8</div>"#,
            demo: el(
                "flex flex-col items-start",
                &[
                    el("h-16 w-36 rounded-lg bg-sky-400 opacity-20", &[]),
                    labeled("-mt-8 rounded-md bg-sky-300 px-4 py-3", "-mt-8"),
                ],
            ),
        },
        TwExample {
            title: "Using logical properties",
            prose: &[
                "Use ms-<number> or me-<number> utilities like ms-4 and me-8 to set the margin-inline-start and margin-inline-end logical properties:",
                "Left-to-right",
                "Right-to-left",
                "Use the mbs-<number> and mbe-<number> utilities to set the margin-block-start and margin-block-end logical properties, which map to either the top or bottom side based on the writing mode:",
            ],
            snippet: r#"<div>
<div dir="ltr">
<div class="ms-8 ...">ms-8</div>
<div class="me-8 ...">me-8</div>
</div>
<div dir="rtl">
<div class="ms-8 ...">ms-8</div>
<div class="me-8 ...">me-8</div>
</div>
</div>
<div class="mbs-8 ...">mbs-8</div>"#,
            demo: el(
                "flex flex-wrap items-start gap-4",
                &[
                    el(
                        "rounded-lg bg-indigo-300",
                        &[labeled(
                            "ms-8 rounded-md bg-indigo-500 px-4 py-3 text-white",
                            "ms-8",
                        )],
                    ),
                    el(
                        "rounded-lg bg-indigo-300",
                        &[labeled(
                            "me-8 rounded-md bg-indigo-500 px-4 py-3 text-white",
                            "me-8",
                        )],
                    ),
                ],
            ),
        },
        TwExample {
            title: "Adding space between children",
            prose: &[
                "Use space-x-<number> or space-y-<number> utilities like space-x-4 and space-y-8 to control the space between elements:",
                "If your elements are in reverse order (using say flex-row-reverse or flex-col-reverse), use the space-x-reverse or space-y-reverse utilities to ensure the space is added to the correct side of each element:",
                "The space utilities are really just a shortcut for adding margin to all-but-the-last-item in a group, and aren't designed to handle complex cases like grids, layouts that wrap, or situations where the children are rendered in a complex custom order rather than their natural DOM order.",
                "For those situations, it's better to use the gap utilities when possible, or add margin to every element with a matching negative margin on the parent.",
                "Additionally, the space utilities are not designed to work together with the divide utilities. For those situations, consider adding margin/padding utilities to the children instead.",
            ],
            snippet: r#"<div class="flex space-x-4 ...">
<div>01</div>
<div>02</div>
<div>03</div>
</div>
<div class="flex flex-row-reverse space-x-4 space-x-reverse ...">
<div>01</div>
<div>02</div>
<div>03</div>
</div>"#,
            demo: el(
                "flex space-x-4 rounded-lg bg-fuchsia-300 p-4",
                &[
                    labeled(
                        "flex h-14 w-14 items-center justify-center rounded-lg bg-fuchsia-500 text-white",
                        "01",
                    ),
                    labeled(
                        "flex h-14 w-14 items-center justify-center rounded-lg bg-fuchsia-500 text-white",
                        "02",
                    ),
                    labeled(
                        "flex h-14 w-14 items-center justify-center rounded-lg bg-fuchsia-500 text-white",
                        "03",
                    ),
                ],
            ),
        },
        TwExample {
            title: "Using a custom value",
            prose: &[
                "Use utilities like m-[<value>],mx-[<value>], and mb-[<value>] to set the margin based on a completely custom value:",
                "For CSS variables, you can also use the m-(<custom-property>) syntax:",
                "This is just a shorthand for m-[var(<custom-property>)] that adds the var() function for you automatically.",
            ],
            snippet: r#"<div class="m-[5px] ...">
<!-- ... -->
</div>
<div class="m-(--my-margin) ...">
<!-- ... -->
</div>"#,
            demo: el(
                "rounded-lg bg-sky-300",
                &[labeled(
                    "m-[5px] rounded-md bg-sky-500 px-4 py-3 text-center text-white",
                    "m-[5px]",
                )],
            ),
        },
        TwExample {
            title: "Responsive design",
            prose: &[
                "Prefix a margin utility with a breakpoint variant like md: to only apply the utility at medium screen sizes and above:",
                "Learn more about using variants in the variants documentation.",
            ],
            snippet: r#"<div class="mt-4 md:mt-8 ...">
<!-- ... -->
</div>"#,
            demo: el(
                "rounded-lg bg-sky-300",
                &[labeled(
                    "mt-4 md:mt-8 rounded-md bg-sky-500 px-4 py-3 text-center text-white",
                    "mt-4 md:mt-8",
                )],
            ),
        },
    ],
};
