//! <https://tailwindcss.com/docs/grid-column>

use crate::tw_docs::demo::{el, labeled};

use crate::tw_docs::{TwExample, TwPage, TwSection};

/// <https://tailwindcss.com/docs/grid-column>
pub static GRID_COLUMN: TwPage = TwPage {
    slug: "grid-column",
    title: "Grid-column",
    section: TwSection::FlexboxGrid,
    description: "Utilities for controlling how elements are sized and placed across grid columns.",
    reference: &[
        (
            "col-span-<number>",
            "grid-column: span <number> / span <number>;",
        ),
        ("col-span-full", "grid-column: 1 / -1;"),
        (
            "col-span-(<custom-property>)",
            "grid-column: span var(<custom-property>) / span var(<custom-property>);",
        ),
        (
            "col-span-[<value>]",
            "grid-column: span <value> / span <value>;",
        ),
        ("col-start-<number>", "grid-column-start: <number>;"),
        (
            "-col-start-<number>",
            "grid-column-start: calc(<number> * -1);",
        ),
        ("col-start-auto", "grid-column-start: auto;"),
        (
            "col-start-(<custom-property>)",
            "grid-column-start: var(<custom-property>);",
        ),
        ("col-start-[<value>]", "grid-column-start: <value>;"),
        ("col-end-<number>", "grid-column-end: <number>;"),
        ("-col-end-<number>", "grid-column-end: calc(<number> * -1);"),
        ("col-end-auto", "grid-column-end: auto;"),
        (
            "col-end-(<custom-property>)",
            "grid-column-end: var(<custom-property>);",
        ),
        ("col-end-[<value>]", "grid-column-end: <value>;"),
        ("col-auto", "grid-column: auto;"),
        ("col-<number>", "grid-column: <number>;"),
        ("-col-<number>", "grid-column: calc(<number> * -1);"),
        (
            "col-(<custom-property>)",
            "grid-column: var(<custom-property>);",
        ),
        ("col-[<value>]", "grid-column: <value>;"),
    ],
    examples: &[
        TwExample {
            title: "Spanning columns",
            prose: &[
                "Use col-span-<number> utilities like col-span-2 and col-span-4 to make an element span n columns:",
            ],
            snippet: r#"<div class="grid grid-cols-3 gap-4">
<div class="...">01</div>
<div class="...">02</div>
<div class="...">03</div>
<div class="col-span-2 ...">04</div>
<div class="...">05</div>
<div class="...">06</div>
<div class="col-span-2 ...">07</div>
</div>"#,
            demo: el(
                "grid w-full grid-cols-3 gap-4",
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
                        "col-span-2 h-14 flex items-center justify-center rounded-lg text-white bg-violet-500",
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
                    labeled(
                        "col-span-2 h-14 flex items-center justify-center rounded-lg text-white bg-violet-500",
                        "07",
                    ),
                ],
            ),
        },
        TwExample {
            title: "Starting and ending lines",
            prose: &[
                "Use col-start-<number> or col-end-<number> utilities like col-start-2 and col-end-3 to make an element start or end at the nth grid line:",
                "These can also be combined with the col-span-<number> utilities to span a specific number of columns.",
            ],
            snippet: r#"<div class="grid grid-cols-6 gap-4">
<div class="col-span-4 col-start-2 ...">01</div>
<div class="col-start-1 col-end-3 ...">02</div>
<div class="col-span-2 col-end-7 ...">03</div>
<div class="col-start-1 col-end-7 ...">04</div>
</div>"#,
            demo: el(
                "grid w-full grid-cols-6 gap-4",
                &[
                    labeled(
                        "col-span-4 col-start-2 h-14 flex items-center justify-center rounded-lg text-white bg-violet-500",
                        "01",
                    ),
                    labeled(
                        "col-start-1 col-end-3 h-14 flex items-center justify-center rounded-lg text-white bg-violet-500",
                        "02",
                    ),
                    labeled(
                        "col-span-2 col-end-7 h-14 flex items-center justify-center rounded-lg text-white bg-violet-500",
                        "03",
                    ),
                    labeled(
                        "col-start-1 col-end-7 h-14 flex items-center justify-center rounded-lg text-white bg-violet-500",
                        "04",
                    ),
                ],
            ),
        },
        TwExample {
            title: "Using a custom value",
            prose: &[
                "Use utilities like col-[<value>],col-span-[<value>],col-start-[<value>], and col-end-[<value>] to set the grid column size and location based on a completely custom value:",
                "For CSS variables, you can also use the col-(<custom-property>) syntax:",
                "This is just a shorthand for col-[var(<custom-property>)] that adds the var() function for you automatically.",
            ],
            snippet: r#"<div class="col-[16_/_span_16] ...">
<!-- ... -->
</div>
<div class="col-(--my-columns) ...">
<!-- ... -->
</div>"#,
            demo: el(
                "grid w-full grid-cols-6 gap-4",
                &[
                    labeled(
                        "col-span-full h-14 flex items-center justify-center rounded-lg text-white bg-violet-500",
                        "col-[16_/_span_16] ≈ col-span-full",
                    ),
                    labeled(
                        "h-14 flex items-center justify-center rounded-lg text-white bg-violet-500",
                        "02",
                    ),
                ],
            ),
        },
        TwExample {
            title: "Responsive design",
            prose: &[
                "Prefix grid-column,grid-column-start, and grid-column-end utilities with a breakpoint variant like md: to only apply the utility at medium screen sizes and above:",
                "Learn more about using variants in the variants documentation.",
            ],
            snippet: r#"<div class="col-span-2 md:col-span-6 ...">
<!-- ... -->
</div>"#,
            demo: el(
                "grid w-full grid-cols-6 gap-4",
                &[
                    labeled(
                        "col-span-2 md:col-span-6 h-14 flex items-center justify-center rounded-lg text-white bg-violet-500",
                        "col-span-2 md:col-span-6",
                    ),
                    labeled(
                        "h-14 flex items-center justify-center rounded-lg text-white bg-violet-500",
                        "02",
                    ),
                ],
            ),
        },
    ],
};
