//! <https://tailwindcss.com/docs/justify-self>

use crate::tw_docs::demo::{el, labeled};

use crate::tw_docs::{TwExample, TwPage, TwSection};

/// <https://tailwindcss.com/docs/justify-self>
pub static JUSTIFY_SELF: TwPage = TwPage {
    slug: "justify-self",
    title: "Justify-self",
    section: TwSection::FlexboxGrid,
    description: "Utilities for controlling how an individual grid item is aligned along its inline axis.",
    reference: &[
        ("justify-self-auto", "justify-self: auto;"),
        ("justify-self-start", "justify-self: start;"),
        ("justify-self-center", "justify-self: center;"),
        ("justify-self-center-safe", "justify-self: safe center;"),
        ("justify-self-end", "justify-self: end;"),
        ("justify-self-end-safe", "justify-self: safe end;"),
        ("justify-self-stretch", "justify-self: stretch;"),
    ],
    examples: &[
        TwExample {
            title: "Auto",
            prose: &[
                "Use the justify-self-auto utility to align an item based on the value of the grid's justify-items property:",
            ],
            snippet: r#"<div class="grid justify-items-stretch ...">
<!-- ... -->
<div class="justify-self-auto ...">02</div>
<!-- ... -->
</div>"#,
            demo: el(
                "grid w-full grid-cols-3 justify-items-stretch gap-4 rounded-lg bg-violet-300 p-2",
                &[
                    labeled(
                        "h-14 flex items-center justify-center rounded-lg text-white bg-violet-500",
                        "01",
                    ),
                    labeled(
                        "h-14 flex items-center justify-center rounded-lg text-white bg-violet-500",
                        "justify-self-auto ≈ inherits",
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
            title: "Start",
            prose: &[
                "Use the justify-self-start utility to align a grid item to the start of its inline axis:",
            ],
            snippet: r#"<div class="grid justify-items-stretch ...">
<!-- ... -->
<div class="justify-self-start ...">02</div>
<!-- ... -->
</div>"#,
            demo: el(
                "grid w-full grid-cols-3 justify-items-stretch gap-4 rounded-lg bg-violet-300 p-2",
                &[
                    labeled(
                        "h-14 flex items-center justify-center rounded-lg text-white bg-violet-500",
                        "01",
                    ),
                    labeled(
                        "justify-self-start size-14 flex items-center justify-center rounded-lg text-white bg-violet-500",
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
            title: "Center",
            prose: &[
                "Use the justify-self-center or justify-self-center-safe utilities to align a grid item along the center of its inline axis:",
                "Resize the container to see the alignment behavior",
                "justify-self-center",
                "justify-self-center-safe",
                "When there is not enough space available, the justify-self-center-safe utility will align the item to the start of the container instead of the end.",
            ],
            snippet: r#"<div class="grid justify-items-stretch ...">
<!-- ... -->
<div class="justify-self-center ...">02</div>
<!-- ... -->
</div>
<div class="grid justify-items-stretch ...">
<!-- ... -->
<div class="justify-self-center-safe ...">02</div>
<!-- ... -->
</div>"#,
            demo: el(
                "grid w-full grid-cols-3 justify-items-stretch gap-4 rounded-lg bg-violet-300 p-2",
                &[
                    labeled(
                        "h-14 flex items-center justify-center rounded-lg text-white bg-violet-500",
                        "01",
                    ),
                    labeled(
                        "justify-self-center size-14 flex items-center justify-center rounded-lg text-white bg-violet-500",
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
            title: "End",
            prose: &[
                "Use the justify-self-end or justify-self-end-safe utilities to align a grid item to the end of its inline axis:",
                "Resize the container to see the alignment behavior",
                "justify-self-end",
                "justify-self-end-safe",
                "When there is not enough space available, the justify-self-end-safe utility will align the item to the start of the container instead of the end.",
            ],
            snippet: r#"<div class="grid justify-items-stretch ...">
<!-- ... -->
<div class="justify-self-end ...">02</div>
<!-- ... -->
</div>
<div class="grid justify-items-stretch ...">
<!-- ... -->
<div class="justify-self-end-safe ...">02</div>
<!-- ... -->
</div>"#,
            demo: el(
                "grid w-full grid-cols-3 justify-items-stretch gap-4 rounded-lg bg-violet-300 p-2",
                &[
                    labeled(
                        "h-14 flex items-center justify-center rounded-lg text-white bg-violet-500",
                        "01",
                    ),
                    labeled(
                        "justify-self-end size-14 flex items-center justify-center rounded-lg text-white bg-violet-500",
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
            title: "Stretch",
            prose: &[
                "Use the justify-self-stretch utility to stretch a grid item to fill the grid area on its inline axis:",
            ],
            snippet: r#"<div class="grid justify-items-start ...">
<!-- ... -->
<div class="justify-self-stretch ...">02</div>
<!-- ... -->
</div>"#,
            demo: el(
                "grid w-full grid-cols-3 justify-items-start gap-4 rounded-lg bg-violet-300 p-2",
                &[
                    labeled(
                        "size-14 flex items-center justify-center rounded-lg text-white bg-violet-500",
                        "01",
                    ),
                    labeled(
                        "justify-self-stretch h-14 flex items-center justify-center rounded-lg text-white bg-violet-500",
                        "02",
                    ),
                    labeled(
                        "size-14 flex items-center justify-center rounded-lg text-white bg-violet-500",
                        "03",
                    ),
                    labeled(
                        "size-14 flex items-center justify-center rounded-lg text-white bg-violet-500",
                        "04",
                    ),
                    labeled(
                        "size-14 flex items-center justify-center rounded-lg text-white bg-violet-500",
                        "05",
                    ),
                    labeled(
                        "size-14 flex items-center justify-center rounded-lg text-white bg-violet-500",
                        "06",
                    ),
                ],
            ),
        },
        TwExample {
            title: "Responsive design",
            prose: &[
                "Prefix a justify-self utility with a breakpoint variant like md: to only apply the utility at medium screen sizes and above:",
                "Learn more about using variants in the variants documentation.",
            ],
            snippet: r#"<div class="justify-self-start md:justify-self-end ...">
<!-- ... -->
</div>"#,
            demo: el(
                "grid w-full grid-cols-3 justify-items-stretch gap-4 rounded-lg bg-violet-300 p-2",
                &[
                    labeled(
                        "h-14 flex items-center justify-center rounded-lg text-white bg-violet-500",
                        "01",
                    ),
                    labeled(
                        "justify-self-start md:justify-self-end size-14 flex items-center justify-center rounded-lg text-white bg-violet-500",
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
    ],
};
