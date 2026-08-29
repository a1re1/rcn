//! <https://tailwindcss.com/docs/grid-template-columns>

use crate::tw_docs::demo::{el, labeled};

use crate::tw_docs::{TwExample, TwPage, TwSection};

/// <https://tailwindcss.com/docs/grid-template-columns>
pub static GRID_TEMPLATE_COLUMNS: TwPage = TwPage {
    slug: "grid-template-columns",
    title: "Grid-template-columns",
    section: TwSection::FlexboxGrid,
    description: "Utilities for specifying the columns in a grid layout.",
    reference: &[
        (
            "grid-cols-<number>",
            "grid-template-columns: repeat(<number>, minmax(0, 1fr));",
        ),
        ("grid-cols-none", "grid-template-columns: none;"),
        ("grid-cols-subgrid", "grid-template-columns: subgrid;"),
        ("grid-cols-[<value>]", "grid-template-columns: <value>;"),
        (
            "grid-cols-(<custom-property>)",
            "grid-template-columns: var(<custom-property>);",
        ),
    ],
    examples: &[
        TwExample {
            title: "Specifying the grid columns",
            prose: &[
                "Use grid-cols-<number> utilities like grid-cols-2 and grid-cols-4 to create grids with n equally sized columns:",
            ],
            snippet: r#"<div class="grid grid-cols-4 gap-4">
<div>01</div>
<!-- ... -->
<div>09</div>
</div>"#,
            demo: el(
                "grid w-full grid-cols-4 gap-4",
                &[
                    labeled(
                        "h-14 flex items-center justify-center rounded-lg text-white bg-fuchsia-500",
                        "01",
                    ),
                    labeled(
                        "h-14 flex items-center justify-center rounded-lg text-white bg-fuchsia-500",
                        "02",
                    ),
                    labeled(
                        "h-14 flex items-center justify-center rounded-lg text-white bg-fuchsia-500",
                        "03",
                    ),
                    labeled(
                        "h-14 flex items-center justify-center rounded-lg text-white bg-fuchsia-500",
                        "04",
                    ),
                    labeled(
                        "h-14 flex items-center justify-center rounded-lg text-white bg-fuchsia-500",
                        "05",
                    ),
                    labeled(
                        "h-14 flex items-center justify-center rounded-lg text-white bg-fuchsia-500",
                        "06",
                    ),
                    labeled(
                        "h-14 flex items-center justify-center rounded-lg text-white bg-fuchsia-500",
                        "07",
                    ),
                    labeled(
                        "h-14 flex items-center justify-center rounded-lg text-white bg-fuchsia-500",
                        "08",
                    ),
                    labeled(
                        "h-14 flex items-center justify-center rounded-lg text-white bg-fuchsia-500",
                        "09",
                    ),
                ],
            ),
        },
        TwExample {
            title: "Implementing a subgrid",
            prose: &[
                "Use the grid-cols-subgrid utility to adopt the column tracks defined by the item's parent:",
            ],
            snippet: r#"<div class="grid grid-cols-4 gap-4">
<div>01</div>
<!-- ... -->
<div>05</div>
<div class="col-span-3 grid grid-cols-subgrid gap-4">
<div class="col-start-2">06</div>
</div>
</div>"#,
            demo: el(
                "grid w-full grid-cols-4 gap-4",
                &[
                    labeled(
                        "h-14 flex items-center justify-center rounded-lg text-white bg-fuchsia-500",
                        "01",
                    ),
                    labeled(
                        "h-14 flex items-center justify-center rounded-lg text-white bg-fuchsia-500",
                        "02",
                    ),
                    labeled(
                        "h-14 flex items-center justify-center rounded-lg text-white bg-fuchsia-500",
                        "03",
                    ),
                    labeled(
                        "h-14 flex items-center justify-center rounded-lg text-white bg-fuchsia-500",
                        "04",
                    ),
                    labeled(
                        "h-14 flex items-center justify-center rounded-lg text-white bg-fuchsia-500",
                        "05",
                    ),
                    el(
                        "col-span-3 grid grid-cols-3 gap-4 rounded-lg bg-fuchsia-300 p-2",
                        &[labeled(
                            "col-start-2 h-14 flex items-center justify-center rounded-lg text-white bg-fuchsia-500",
                            "06 (subgrid ≈ nested grid-cols-3)",
                        )],
                    ),
                ],
            ),
        },
        TwExample {
            title: "Using a custom value",
            prose: &[
                "Use the grid-cols-[<value>] syntax to set the columns based on a completely custom value:",
                "For CSS variables, you can also use the grid-cols-(<custom-property>) syntax:",
                "This is just a shorthand for grid-cols-[var(<custom-property>)] that adds the var() function for you automatically.",
            ],
            snippet: r#"<div class="grid-cols-[200px_minmax(900px,_1fr)_100px] ...">
<!-- ... -->
</div>
<div class="grid-cols-(--my-grid-cols) ...">
<!-- ... -->
</div>"#,
            demo: el(
                "grid w-full grid-cols-3 gap-4",
                &[
                    labeled(
                        "h-14 flex items-center justify-center rounded-lg text-white bg-fuchsia-500",
                        "grid-cols-[…] ≈ grid-cols-3",
                    ),
                    labeled(
                        "h-14 flex items-center justify-center rounded-lg text-white bg-fuchsia-500",
                        "01",
                    ),
                    labeled(
                        "h-14 flex items-center justify-center rounded-lg text-white bg-fuchsia-500",
                        "02",
                    ),
                ],
            ),
        },
        TwExample {
            title: "Responsive design",
            prose: &[
                "Prefix a grid-template-columns utility with a breakpoint variant like md: to only apply the utility at medium screen sizes and above:",
                "Learn more about using variants in the variants documentation.",
            ],
            snippet: r#"<div class="grid grid-cols-1 md:grid-cols-6 ...">
<!-- ... -->
</div>"#,
            demo: el(
                "grid w-full grid-cols-1 md:grid-cols-6 gap-4",
                &[
                    labeled(
                        "h-14 flex items-center justify-center rounded-lg text-white bg-fuchsia-500",
                        "01",
                    ),
                    labeled(
                        "h-14 flex items-center justify-center rounded-lg text-white bg-fuchsia-500",
                        "02",
                    ),
                    labeled(
                        "h-14 flex items-center justify-center rounded-lg text-white bg-fuchsia-500",
                        "03",
                    ),
                    labeled(
                        "h-14 flex items-center justify-center rounded-lg text-white bg-fuchsia-500",
                        "04",
                    ),
                    labeled(
                        "h-14 flex items-center justify-center rounded-lg text-white bg-fuchsia-500",
                        "05",
                    ),
                    labeled(
                        "h-14 flex items-center justify-center rounded-lg text-white bg-fuchsia-500",
                        "06",
                    ),
                ],
            ),
        },
    ],
};
