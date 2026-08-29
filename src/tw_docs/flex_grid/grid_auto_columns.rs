//! <https://tailwindcss.com/docs/grid-auto-columns>

use crate::tw_docs::demo::{el, labeled};

use crate::tw_docs::{TwExample, TwPage, TwSection};

/// <https://tailwindcss.com/docs/grid-auto-columns>
pub static GRID_AUTO_COLUMNS: TwPage = TwPage {
    slug: "grid-auto-columns",
    title: "Grid-auto-columns",
    section: TwSection::FlexboxGrid,
    description: "Utilities for controlling the size of implicitly-created grid columns.",
    reference: &[
        ("auto-cols-auto", "grid-auto-columns: auto;"),
        ("auto-cols-min", "grid-auto-columns: min-content;"),
        ("auto-cols-max", "grid-auto-columns: max-content;"),
        ("auto-cols-fr", "grid-auto-columns: minmax(0, 1fr);"),
        (
            "auto-cols-<number>",
            "grid-auto-columns: calc(var(--spacing) * <number>);",
        ),
        (
            "auto-cols-(<custom-property>)",
            "grid-auto-columns: var(<custom-property>);",
        ),
        ("auto-cols-[<value>]", "grid-auto-columns: <value>;"),
    ],
    examples: &[
        TwExample {
            title: "Basic example",
            prose: &[
                "Use utilities like auto-cols-min and auto-cols-max to control the size of implicitly-created grid columns:",
            ],
            snippet: r#"<div class="grid auto-cols-max grid-flow-col">
<div>01</div>
<div>02</div>
<div>03</div>
</div>"#,
            demo: el(
                "grid auto-cols-max grid-flow-col gap-4",
                &[
                    labeled(
                        "h-14 px-4 flex items-center justify-center rounded-lg text-white bg-emerald-500",
                        "01",
                    ),
                    labeled(
                        "h-14 px-4 flex items-center justify-center rounded-lg text-white bg-emerald-500",
                        "A much wider column",
                    ),
                    labeled(
                        "h-14 px-4 flex items-center justify-center rounded-lg text-white bg-emerald-500",
                        "03",
                    ),
                ],
            ),
        },
        TwExample {
            title: "Using a custom value",
            prose: &[
                "Use the auto-cols-[<value>] syntax to set the size of implicitly-created grid columns based on a completely custom value:",
                "For CSS variables, you can also use the auto-cols-(<custom-property>) syntax:",
                "This is just a shorthand for auto-cols-[var(<custom-property>)] that adds the var() function for you automatically.",
            ],
            snippet: r#"<div class="auto-cols-[minmax(0,2fr)] ...">
<!-- ... -->
</div>
<div class="auto-cols-(--my-auto-cols) ...">
<!-- ... -->
</div>"#,
            demo: el(
                "grid auto-cols-fr grid-flow-col gap-4 w-96",
                &[
                    labeled(
                        "h-14 flex items-center justify-center rounded-lg text-white bg-emerald-500",
                        "auto-cols-[minmax(0,2fr)] ≈ auto-cols-fr",
                    ),
                    labeled(
                        "h-14 flex items-center justify-center rounded-lg text-white bg-emerald-500",
                        "02",
                    ),
                ],
            ),
        },
        TwExample {
            title: "Responsive design",
            prose: &[
                "Prefix a grid-auto-columns utility with a breakpoint variant like md: to only apply the utility at medium screen sizes and above:",
                "Learn more about using variants in the variants documentation.",
            ],
            snippet: r#"<div class="grid grid-flow-col auto-cols-max md:auto-cols-min ...">
<!-- ... -->
</div>"#,
            demo: el(
                "grid grid-flow-col auto-cols-max md:auto-cols-min gap-4",
                &[
                    labeled(
                        "h-14 px-4 flex items-center justify-center rounded-lg text-white bg-emerald-500",
                        "01",
                    ),
                    labeled(
                        "h-14 px-4 flex items-center justify-center rounded-lg text-white bg-emerald-500",
                        "A much wider column",
                    ),
                    labeled(
                        "h-14 px-4 flex items-center justify-center rounded-lg text-white bg-emerald-500",
                        "03",
                    ),
                ],
            ),
        },
    ],
};
