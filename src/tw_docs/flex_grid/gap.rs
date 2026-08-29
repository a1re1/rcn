//! <https://tailwindcss.com/docs/gap>

use crate::tw_docs::demo::{el, labeled};

use crate::tw_docs::{TwExample, TwPage, TwSection};

/// <https://tailwindcss.com/docs/gap>
pub static GAP: TwPage = TwPage {
    slug: "gap",
    title: "Gap",
    section: TwSection::FlexboxGrid,
    description: "Utilities for controlling gutters between grid and flexbox items.",
    reference: &[
        ("gap-<number>", "gap: calc(var(--spacing) * <value>);"),
        ("gap-px", "gap: 1px;"),
        ("gap-(<custom-property>)", "gap: var(<custom-property>);"),
        ("gap-[<value>]", "gap: <value>;"),
        (
            "gap-x-<number>",
            "column-gap: calc(var(--spacing) * <value>);",
        ),
        ("gap-x-px", "column-gap: 1px;"),
        (
            "gap-x-(<custom-property>)",
            "column-gap: var(<custom-property>);",
        ),
        ("gap-x-[<value>]", "column-gap: <value>;"),
        ("gap-y-<number>", "row-gap: calc(var(--spacing) * <value>);"),
        ("gap-y-px", "row-gap: 1px;"),
        (
            "gap-y-(<custom-property>)",
            "row-gap: var(<custom-property>);",
        ),
        ("gap-y-[<value>]", "row-gap: <value>;"),
    ],
    examples: &[
        TwExample {
            title: "Basic example",
            prose: &[
                "Use gap-<number> utilities like gap-2 and gap-4 to change the gap between both rows and columns in grid and flexbox layouts:",
            ],
            snippet: r#"<div class="grid grid-cols-2 gap-4">
<div>01</div>
<div>02</div>
<div>03</div>
<div>04</div>
</div>"#,
            demo: el(
                "grid w-96 grid-cols-2 gap-4",
                &[
                    labeled(
                        "h-14 flex items-center justify-center rounded-lg text-white bg-violet-500",
                        "01",
                    ),
                    labeled(
                        "h-14 flex items-center justify-center rounded-lg text-white bg-violet-500",
                        "02",
                    ),
                    labeled(
                        "h-14 flex items-center justify-center rounded-lg text-white bg-violet-500",
                        "03",
                    ),
                    labeled(
                        "h-14 flex items-center justify-center rounded-lg text-white bg-violet-500",
                        "04",
                    ),
                ],
            ),
        },
        TwExample {
            title: "Changing row and column gaps independently",
            prose: &[
                "Use gap-x-<number> or gap-y-<number> utilities like gap-x-8 and gap-y-4 to change the gap between columns and rows independently:",
            ],
            snippet: r#"<div class="grid grid-cols-3 gap-x-8 gap-y-4">
<div>01</div>
<div>02</div>
<div>03</div>
<div>04</div>
<div>05</div>
<div>06</div>
</div>"#,
            demo: el(
                "grid w-96 grid-cols-3 gap-x-8 gap-y-4",
                &[
                    labeled(
                        "h-14 flex items-center justify-center rounded-lg text-white bg-violet-500",
                        "01",
                    ),
                    labeled(
                        "h-14 flex items-center justify-center rounded-lg text-white bg-violet-500",
                        "02",
                    ),
                    labeled(
                        "h-14 flex items-center justify-center rounded-lg text-white bg-violet-500",
                        "03",
                    ),
                    labeled(
                        "h-14 flex items-center justify-center rounded-lg text-white bg-violet-500",
                        "04",
                    ),
                    labeled(
                        "h-14 flex items-center justify-center rounded-lg text-white bg-violet-500",
                        "05",
                    ),
                    labeled(
                        "h-14 flex items-center justify-center rounded-lg text-white bg-violet-500",
                        "06",
                    ),
                ],
            ),
        },
        TwExample {
            title: "Using a custom value",
            prose: &[
                "Use utilities like gap-[<value>],gap-x-[<value>], and gap-y-[<value>] to set the gap based on a completely custom value:",
                "For CSS variables, you can also use the gap-(<custom-property>) syntax:",
                "This is just a shorthand for gap-[var(<custom-property>)] that adds the var() function for you automatically.",
            ],
            snippet: r#"<div class="gap-[10vw] ...">
<!-- ... -->
</div>
<div class="gap-(--my-gap) ...">
<!-- ... -->
</div>"#,
            demo: el(
                "grid w-96 grid-cols-2 gap-[10px]",
                &[
                    labeled(
                        "h-14 flex items-center justify-center rounded-lg text-white bg-violet-500",
                        "gap-[10vw] ≈ gap-[10px]",
                    ),
                    labeled(
                        "h-14 flex items-center justify-center rounded-lg text-white bg-violet-500",
                        "01",
                    ),
                    labeled(
                        "h-14 flex items-center justify-center rounded-lg text-white bg-violet-500",
                        "02",
                    ),
                    labeled(
                        "h-14 flex items-center justify-center rounded-lg text-white bg-violet-500",
                        "03",
                    ),
                ],
            ),
        },
        TwExample {
            title: "Responsive design",
            prose: &[
                "Prefix gap,column-gap, and row-gap utilities with a breakpoint variant like md: to only apply the utility at medium screen sizes and above:",
                "Learn more about using variants in the variants documentation.",
            ],
            snippet: r#"<div class="grid gap-4 md:gap-6 ...">
<!-- ... -->
</div>"#,
            demo: el(
                "grid w-96 grid-cols-2 gap-4 md:gap-6",
                &[
                    labeled(
                        "h-14 flex items-center justify-center rounded-lg text-white bg-violet-500",
                        "01",
                    ),
                    labeled(
                        "h-14 flex items-center justify-center rounded-lg text-white bg-violet-500",
                        "02",
                    ),
                    labeled(
                        "h-14 flex items-center justify-center rounded-lg text-white bg-violet-500",
                        "03",
                    ),
                    labeled(
                        "h-14 flex items-center justify-center rounded-lg text-white bg-violet-500",
                        "04",
                    ),
                ],
            ),
        },
    ],
};
