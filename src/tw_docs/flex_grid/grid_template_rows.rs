//! <https://tailwindcss.com/docs/grid-template-rows>

use crate::tw_docs::demo::{el, labeled};

use crate::tw_docs::{TwExample, TwPage, TwSection};

/// <https://tailwindcss.com/docs/grid-template-rows>
pub static GRID_TEMPLATE_ROWS: TwPage = TwPage {
    slug: "grid-template-rows",
    title: "Grid-template-rows",
    section: TwSection::FlexboxGrid,
    description: "Utilities for specifying the rows in a grid layout.",
    reference: &[
        (
            "grid-rows-<number>",
            "grid-template-rows: repeat(<number>, minmax(0, 1fr));",
        ),
        ("grid-rows-none", "grid-template-rows: none;"),
        ("grid-rows-subgrid", "grid-template-rows: subgrid;"),
        ("grid-rows-[<value>]", "grid-template-rows: <value>;"),
        (
            "grid-rows-(<custom-property>)",
            "grid-template-rows: var(<custom-property>);",
        ),
    ],
    examples: &[
        TwExample {
            title: "Specifying the grid rows",
            prose: &[
                "Use grid-rows-<number> utilities like grid-rows-2 and grid-rows-4 to create grids with n equally sized rows:",
            ],
            snippet: r#"<div class="grid grid-flow-col grid-rows-4 gap-4">
<div>01</div>
<!-- ... -->
<div>09</div>
</div>"#,
            demo: el(
                "grid h-80 grid-flow-col grid-rows-4 gap-4",
                &[
                    labeled(
                        "w-16 flex items-center justify-center rounded-lg text-white bg-indigo-500",
                        "01",
                    ),
                    labeled(
                        "w-16 flex items-center justify-center rounded-lg text-white bg-indigo-500",
                        "02",
                    ),
                    labeled(
                        "w-16 flex items-center justify-center rounded-lg text-white bg-indigo-500",
                        "03",
                    ),
                    labeled(
                        "w-16 flex items-center justify-center rounded-lg text-white bg-indigo-500",
                        "04",
                    ),
                    labeled(
                        "w-16 flex items-center justify-center rounded-lg text-white bg-indigo-500",
                        "05",
                    ),
                    labeled(
                        "w-16 flex items-center justify-center rounded-lg text-white bg-indigo-500",
                        "06",
                    ),
                    labeled(
                        "w-16 flex items-center justify-center rounded-lg text-white bg-indigo-500",
                        "07",
                    ),
                    labeled(
                        "w-16 flex items-center justify-center rounded-lg text-white bg-indigo-500",
                        "08",
                    ),
                    labeled(
                        "w-16 flex items-center justify-center rounded-lg text-white bg-indigo-500",
                        "09",
                    ),
                ],
            ),
        },
        TwExample {
            title: "Implementing a subgrid",
            prose: &[
                "Use the grid-rows-subgrid utility to adopt the row tracks defined by the item's parent:",
            ],
            snippet: r#"<div class="grid grid-flow-col grid-rows-4 gap-4">
<div>01</div>
<!-- ... -->
<div>05</div>
<div class="row-span-3 grid grid-rows-subgrid gap-4">
<div class="row-start-2">06</div>
</div>
<div>07</div>
<!-- ... -->
<div>10</div>
</div>"#,
            demo: el(
                "grid h-80 grid-flow-col grid-rows-4 gap-4",
                &[
                    labeled(
                        "w-16 flex items-center justify-center rounded-lg text-white bg-indigo-500",
                        "01",
                    ),
                    labeled(
                        "w-16 flex items-center justify-center rounded-lg text-white bg-indigo-500",
                        "02",
                    ),
                    labeled(
                        "w-16 flex items-center justify-center rounded-lg text-white bg-indigo-500",
                        "03",
                    ),
                    labeled(
                        "w-16 flex items-center justify-center rounded-lg text-white bg-indigo-500",
                        "04",
                    ),
                    labeled(
                        "w-16 flex items-center justify-center rounded-lg text-white bg-indigo-500",
                        "05",
                    ),
                    el(
                        "row-span-3 grid grid-rows-3 gap-4 rounded-lg bg-indigo-300 p-2",
                        &[labeled(
                            "row-start-2 w-16 flex items-center justify-center rounded-lg text-white bg-indigo-500",
                            "06 (subgrid ≈ nested grid)",
                        )],
                    ),
                    labeled(
                        "w-16 flex items-center justify-center rounded-lg text-white bg-indigo-500",
                        "07",
                    ),
                    labeled(
                        "w-16 flex items-center justify-center rounded-lg text-white bg-indigo-500",
                        "08",
                    ),
                    labeled(
                        "w-16 flex items-center justify-center rounded-lg text-white bg-indigo-500",
                        "09",
                    ),
                    labeled(
                        "w-16 flex items-center justify-center rounded-lg text-white bg-indigo-500",
                        "10",
                    ),
                ],
            ),
        },
        TwExample {
            title: "Using a custom value",
            prose: &[
                "Use the grid-rows-[<value>] syntax to set the rows based on a completely custom value:",
                "For CSS variables, you can also use the grid-rows-(<custom-property>) syntax:",
                "This is just a shorthand for grid-rows-[var(<custom-property>)] that adds the var() function for you automatically.",
            ],
            snippet: r#"<div class="grid-rows-[200px_minmax(900px,1fr)_100px] ...">
<!-- ... -->
</div>
<div class="grid-rows-(--my-grid-rows) ...">
<!-- ... -->
</div>"#,
            demo: el(
                "grid h-64 grid-flow-col grid-rows-3 gap-4",
                &[
                    labeled(
                        "w-24 flex items-center justify-center rounded-lg text-white bg-indigo-500",
                        "grid-rows-[…] ≈ grid-rows-3",
                    ),
                    labeled(
                        "w-24 flex items-center justify-center rounded-lg text-white bg-indigo-500",
                        "01",
                    ),
                    labeled(
                        "w-24 flex items-center justify-center rounded-lg text-white bg-indigo-500",
                        "02",
                    ),
                ],
            ),
        },
        TwExample {
            title: "Responsive design",
            prose: &[
                "Prefix a grid-template-rows utility with a breakpoint variant like md: to only apply the utility at medium screen sizes and above:",
                "Learn more about using variants in the variants documentation.",
            ],
            snippet: r#"<div class="grid grid-rows-2 md:grid-rows-6 ...">
<!-- ... -->
</div>"#,
            demo: el(
                "grid h-64 grid-flow-col grid-rows-2 md:grid-rows-6 gap-4",
                &[
                    labeled(
                        "w-16 flex items-center justify-center rounded-lg text-white bg-indigo-500",
                        "01",
                    ),
                    labeled(
                        "w-16 flex items-center justify-center rounded-lg text-white bg-indigo-500",
                        "02",
                    ),
                    labeled(
                        "w-16 flex items-center justify-center rounded-lg text-white bg-indigo-500",
                        "03",
                    ),
                    labeled(
                        "w-16 flex items-center justify-center rounded-lg text-white bg-indigo-500",
                        "04",
                    ),
                    labeled(
                        "w-16 flex items-center justify-center rounded-lg text-white bg-indigo-500",
                        "05",
                    ),
                    labeled(
                        "w-16 flex items-center justify-center rounded-lg text-white bg-indigo-500",
                        "06",
                    ),
                ],
            ),
        },
    ],
};
