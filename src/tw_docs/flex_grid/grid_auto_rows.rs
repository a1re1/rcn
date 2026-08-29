//! <https://tailwindcss.com/docs/grid-auto-rows>

use crate::tw_docs::demo::{el, labeled};

use crate::tw_docs::{TwExample, TwPage, TwSection};

/// <https://tailwindcss.com/docs/grid-auto-rows>
pub static GRID_AUTO_ROWS: TwPage = TwPage {
    slug: "grid-auto-rows",
    title: "Grid-auto-rows",
    section: TwSection::FlexboxGrid,
    description: "Utilities for controlling the size of implicitly-created grid rows.",
    reference: &[
        ("auto-rows-auto", "grid-auto-rows: auto;"),
        ("auto-rows-min", "grid-auto-rows: min-content;"),
        ("auto-rows-max", "grid-auto-rows: max-content;"),
        ("auto-rows-fr", "grid-auto-rows: minmax(0, 1fr);"),
        (
            "auto-rows-<number>",
            "grid-auto-rows: calc(var(--spacing) * <number>);",
        ),
        (
            "auto-rows-(<custom-property>)",
            "grid-auto-rows: var(<custom-property>);",
        ),
        ("auto-rows-[<value>]", "grid-auto-rows: <value>;"),
    ],
    examples: &[
        TwExample {
            title: "Basic example",
            prose: &[
                "Use utilities like auto-rows-min and auto-rows-max to control the size of implicitly-created grid rows:",
            ],
            snippet: r#"<div class="grid grid-flow-row auto-rows-max">
<div>01</div>
<div>02</div>
<div>03</div>
</div>"#,
            demo: el(
                "grid grid-flow-row auto-rows-max gap-4 w-64",
                &[
                    labeled(
                        "py-2 flex items-center justify-center rounded-lg text-white bg-emerald-500",
                        "01",
                    ),
                    labeled(
                        "py-8 flex items-center justify-center rounded-lg text-white bg-emerald-500",
                        "A much taller row",
                    ),
                    labeled(
                        "py-2 flex items-center justify-center rounded-lg text-white bg-emerald-500",
                        "03",
                    ),
                ],
            ),
        },
        TwExample {
            title: "Using a custom value",
            prose: &[
                "Use the auto-rows-[<value>] syntax to set the size of implicitly-created grid rows based on a completely custom value:",
                "For CSS variables, you can also use the auto-rows-(<custom-property>) syntax:",
                "This is just a shorthand for auto-rows-[var(<custom-property>)] that adds the var() function for you automatically.",
            ],
            snippet: r#"<div class="auto-rows-[minmax(0,2fr)] ...">
<!-- ... -->
</div>
<div class="auto-rows-(--my-auto-rows) ...">
<!-- ... -->
</div>"#,
            demo: el(
                "grid grid-flow-row auto-rows-fr gap-4 h-64 w-64",
                &[
                    labeled(
                        " flex items-center justify-center rounded-lg text-white bg-emerald-500",
                        "auto-rows-[minmax(0,2fr)] ≈ auto-rows-fr",
                    ),
                    labeled(
                        " flex items-center justify-center rounded-lg text-white bg-emerald-500",
                        "02",
                    ),
                ],
            ),
        },
        TwExample {
            title: "Responsive design",
            prose: &[
                "Prefix a grid-auto-rows utility with a breakpoint variant like md: to only apply the utility at medium screen sizes and above:",
                "Learn more about using variants in the variants documentation.",
            ],
            snippet: r#"<div class="grid grid-flow-row auto-rows-max md:auto-rows-min ...">
<!-- ... -->
</div>"#,
            demo: el(
                "grid grid-flow-row auto-rows-max md:auto-rows-min gap-4 w-64",
                &[
                    labeled(
                        "py-2 flex items-center justify-center rounded-lg text-white bg-emerald-500",
                        "01",
                    ),
                    labeled(
                        "py-8 flex items-center justify-center rounded-lg text-white bg-emerald-500",
                        "A much taller row",
                    ),
                    labeled(
                        "py-2 flex items-center justify-center rounded-lg text-white bg-emerald-500",
                        "03",
                    ),
                ],
            ),
        },
    ],
};
