//! <https://tailwindcss.com/docs/justify-items>

use crate::tw_docs::demo::{el, labeled};

use crate::tw_docs::{TwExample, TwPage, TwSection};

/// <https://tailwindcss.com/docs/justify-items>
pub static JUSTIFY_ITEMS: TwPage = TwPage {
    slug: "justify-items",
    title: "Justify-items",
    section: TwSection::FlexboxGrid,
    description: "Utilities for controlling how grid items are aligned along their inline axis.",
    reference: &[
        ("justify-items-start", "justify-items: start;"),
        ("justify-items-end", "justify-items: end;"),
        ("justify-items-end-safe", "justify-items: safe end;"),
        ("justify-items-center", "justify-items: center;"),
        ("justify-items-center-safe", "justify-items: safe center;"),
        ("justify-items-stretch", "justify-items: stretch;"),
        ("justify-items-normal", "justify-items: normal;"),
    ],
    examples: &[
        TwExample {
            title: "Start",
            prose: &[
                "Use the justify-items-start utility to justify grid items against the start of their inline axis:",
            ],
            snippet: r#"<div class="grid justify-items-start ...">
<div>01</div>
<div>02</div>
<div>03</div>
<div>04</div>
<div>05</div>
<div>06</div>
</div>"#,
            demo: el(
                "grid w-full grid-cols-3 justify-items-start gap-4 rounded-lg bg-indigo-300 p-2",
                &[
                    labeled(
                        "size-14 flex items-center justify-center rounded-lg text-white bg-indigo-500",
                        "01",
                    ),
                    labeled(
                        "size-14 flex items-center justify-center rounded-lg text-white bg-indigo-500",
                        "02",
                    ),
                    labeled(
                        "size-14 flex items-center justify-center rounded-lg text-white bg-indigo-500",
                        "03",
                    ),
                    labeled(
                        "size-14 flex items-center justify-center rounded-lg text-white bg-indigo-500",
                        "04",
                    ),
                    labeled(
                        "size-14 flex items-center justify-center rounded-lg text-white bg-indigo-500",
                        "05",
                    ),
                    labeled(
                        "size-14 flex items-center justify-center rounded-lg text-white bg-indigo-500",
                        "06",
                    ),
                ],
            ),
        },
        TwExample {
            title: "End",
            prose: &[
                "Use the justify-items-end or justify-items-end-safe utilities to justify grid items against the end of their inline axis:",
                "Resize the container to see the alignment behavior",
                "justify-items-end",
                "justify-items-end-safe",
                "When there is not enough space available, the justify-items-end-safe utility will align items to the start of the container instead of the end.",
            ],
            snippet: r#"<div class="grid grid-flow-col justify-items-end ...">
<div>01</div>
<div>02</div>
<div>03</div>
</div>
<div class="grid grid-flow-col justify-items-end-safe ...">
<div>01</div>
<div>02</div>
<div>03</div>
</div>"#,
            demo: el(
                "grid w-full grid-flow-col justify-items-end gap-4 rounded-lg bg-indigo-300 p-2",
                &[
                    labeled(
                        "size-14 flex items-center justify-center rounded-lg text-white bg-indigo-500",
                        "01",
                    ),
                    labeled(
                        "size-14 flex items-center justify-center rounded-lg text-white bg-indigo-500",
                        "02",
                    ),
                    labeled(
                        "size-14 flex items-center justify-center rounded-lg text-white bg-indigo-500",
                        "03",
                    ),
                ],
            ),
        },
        TwExample {
            title: "Center",
            prose: &[
                "Use the justify-items-center or justify-items-center-safe utilities to justify grid items against the end of their inline axis:",
                "Resize the container to see the alignment behavior",
                "justify-items-center",
                "justify-items-center-safe",
                "When there is not enough space available, the justify-items-center-safe utility will align items to the start of the container instead of the center.",
            ],
            snippet: r#"<div class="grid grid-flow-col justify-items-center ...">
<div>01</div>
<div>02</div>
<div>03</div>
</div>
<div class="grid grid-flow-col justify-items-center-safe ...">
<div>01</div>
<div>02</div>
<div>03</div>
</div>"#,
            demo: el(
                "grid w-full grid-flow-col justify-items-center gap-4 rounded-lg bg-indigo-300 p-2",
                &[
                    labeled(
                        "size-14 flex items-center justify-center rounded-lg text-white bg-indigo-500",
                        "01",
                    ),
                    labeled(
                        "size-14 flex items-center justify-center rounded-lg text-white bg-indigo-500",
                        "02",
                    ),
                    labeled(
                        "size-14 flex items-center justify-center rounded-lg text-white bg-indigo-500",
                        "03",
                    ),
                ],
            ),
        },
        TwExample {
            title: "Stretch",
            prose: &[
                "Use the justify-items-stretch utility to stretch items along their inline axis:",
            ],
            snippet: r#"<div class="grid justify-items-stretch ...">
<div>01</div>
<div>02</div>
<div>03</div>
<div>04</div>
<div>05</div>
<div>06</div>
</div>"#,
            demo: el(
                "grid w-full grid-cols-3 justify-items-stretch gap-4 rounded-lg bg-indigo-300 p-2",
                &[
                    labeled(
                        "h-14 flex items-center justify-center rounded-lg text-white bg-indigo-500",
                        "01",
                    ),
                    labeled(
                        "h-14 flex items-center justify-center rounded-lg text-white bg-indigo-500",
                        "02",
                    ),
                    labeled(
                        "h-14 flex items-center justify-center rounded-lg text-white bg-indigo-500",
                        "03",
                    ),
                    labeled(
                        "h-14 flex items-center justify-center rounded-lg text-white bg-indigo-500",
                        "04",
                    ),
                    labeled(
                        "h-14 flex items-center justify-center rounded-lg text-white bg-indigo-500",
                        "05",
                    ),
                    labeled(
                        "h-14 flex items-center justify-center rounded-lg text-white bg-indigo-500",
                        "06",
                    ),
                ],
            ),
        },
        TwExample {
            title: "Responsive design",
            prose: &[
                "Prefix a justify-items utility with a breakpoint variant like md: to only apply the utility at medium screen sizes and above:",
                "Learn more about using variants in the variants documentation.",
            ],
            snippet: r#"<div class="grid justify-items-start md:justify-items-center ...">
<!-- ... -->
</div>"#,
            demo: el(
                "grid w-full grid-cols-3 justify-items-start md:justify-items-center gap-4 rounded-lg bg-indigo-300 p-2",
                &[
                    labeled(
                        "size-14 flex items-center justify-center rounded-lg text-white bg-indigo-500",
                        "01",
                    ),
                    labeled(
                        "size-14 flex items-center justify-center rounded-lg text-white bg-indigo-500",
                        "02",
                    ),
                    labeled(
                        "size-14 flex items-center justify-center rounded-lg text-white bg-indigo-500",
                        "03",
                    ),
                    labeled(
                        "size-14 flex items-center justify-center rounded-lg text-white bg-indigo-500",
                        "04",
                    ),
                    labeled(
                        "size-14 flex items-center justify-center rounded-lg text-white bg-indigo-500",
                        "05",
                    ),
                    labeled(
                        "size-14 flex items-center justify-center rounded-lg text-white bg-indigo-500",
                        "06",
                    ),
                ],
            ),
        },
    ],
};
