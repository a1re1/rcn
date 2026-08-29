//! <https://tailwindcss.com/docs/grid-auto-flow>

use crate::tw_docs::demo::{el, labeled};

use crate::tw_docs::{TwExample, TwPage, TwSection};

/// <https://tailwindcss.com/docs/grid-auto-flow>
pub static GRID_AUTO_FLOW: TwPage = TwPage {
    slug: "grid-auto-flow",
    title: "Grid-auto-flow",
    section: TwSection::FlexboxGrid,
    description: "Utilities for controlling how elements in a grid are auto-placed.",
    reference: &[
        ("grid-flow-row", "grid-auto-flow: row;"),
        ("grid-flow-col", "grid-auto-flow: column;"),
        ("grid-flow-dense", "grid-auto-flow: dense;"),
        ("grid-flow-row-dense", "grid-auto-flow: row dense;"),
        ("grid-flow-col-dense", "grid-auto-flow: column dense;"),
    ],
    examples: &[
        TwExample {
            title: "Basic example",
            prose: &[
                "Use utilities like grid-flow-col and grid-flow-row-dense to control how the auto-placement algorithm works for a grid layout:",
            ],
            snippet: r#"<div class="grid grid-flow-row-dense grid-cols-3 grid-rows-3 ...">
<div class="col-span-2">01</div>
<div class="col-span-2">02</div>
<div>03</div>
<div>04</div>
<div>05</div>
</div>"#,
            demo: el(
                "grid w-96 grid-flow-row-dense grid-cols-3 grid-rows-3 gap-4",
                &[
                    labeled(
                        "col-span-2 h-14 flex items-center justify-center rounded-lg text-white bg-purple-500",
                        "01",
                    ),
                    labeled(
                        "col-span-2 h-14 flex items-center justify-center rounded-lg text-white bg-purple-500",
                        "02",
                    ),
                    labeled(
                        "h-14 flex items-center justify-center rounded-lg text-white bg-purple-500",
                        "03",
                    ),
                    labeled(
                        "h-14 flex items-center justify-center rounded-lg text-white bg-purple-500",
                        "04",
                    ),
                    labeled(
                        "h-14 flex items-center justify-center rounded-lg text-white bg-purple-500",
                        "05",
                    ),
                ],
            ),
        },
        TwExample {
            title: "Responsive design",
            prose: &[
                "Prefix a grid-auto-flow utility with a breakpoint variant like md: to only apply the utility at medium screen sizes and above:",
                "Learn more about using variants in the variants documentation.",
            ],
            snippet: r#"<div class="grid grid-flow-col md:grid-flow-row ...">
<!-- ... -->
</div>"#,
            demo: el(
                "grid w-96 grid-flow-col md:grid-flow-row gap-4",
                &[
                    labeled(
                        "h-14 flex items-center justify-center rounded-lg text-white bg-purple-500",
                        "01",
                    ),
                    labeled(
                        "h-14 flex items-center justify-center rounded-lg text-white bg-purple-500",
                        "02",
                    ),
                    labeled(
                        "h-14 flex items-center justify-center rounded-lg text-white bg-purple-500",
                        "03",
                    ),
                ],
            ),
        },
    ],
};
