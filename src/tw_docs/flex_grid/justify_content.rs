//! <https://tailwindcss.com/docs/justify-content>

use crate::tw_docs::demo::{el, labeled};

use crate::tw_docs::{TwExample, TwPage, TwSection};

/// <https://tailwindcss.com/docs/justify-content>
pub static JUSTIFY_CONTENT: TwPage = TwPage {
    slug: "justify-content",
    title: "Justify-content",
    section: TwSection::FlexboxGrid,
    description: "Utilities for controlling how flex and grid items are positioned along a container's main axis.",
    reference: &[
        ("justify-start", "justify-content: flex-start;"),
        ("justify-end", "justify-content: flex-end;"),
        ("justify-end-safe", "justify-content: safe flex-end;"),
        ("justify-center", "justify-content: center;"),
        ("justify-center-safe", "justify-content: safe center;"),
        ("justify-between", "justify-content: space-between;"),
        ("justify-around", "justify-content: space-around;"),
        ("justify-evenly", "justify-content: space-evenly;"),
        ("justify-stretch", "justify-content: stretch;"),
        ("justify-baseline", "justify-content: baseline;"),
        ("justify-normal", "justify-content: normal;"),
    ],
    examples: &[
        TwExample {
            title: "Start",
            prose: &[
                "Use the justify-start utility to justify items against the start of the container's main axis:",
            ],
            snippet: r#"<div class="flex justify-start ...">
<div>01</div>
<div>02</div>
<div>03</div>
</div>"#,
            demo: el(
                "flex w-full gap-4 rounded-lg bg-sky-300 p-2 justify-start",
                &[
                    labeled(
                        "size-14 flex items-center justify-center rounded-lg text-white bg-sky-500",
                        "01",
                    ),
                    labeled(
                        "size-14 flex items-center justify-center rounded-lg text-white bg-sky-500",
                        "02",
                    ),
                    labeled(
                        "size-14 flex items-center justify-center rounded-lg text-white bg-sky-500",
                        "03",
                    ),
                ],
            ),
        },
        TwExample {
            title: "Center",
            prose: &[
                "Use the justify-center or justify-center-safe utilities to justify items along the center of the container's main axis:",
                "Resize the container to see the alignment behavior",
                "justify-center",
                "justify-center-safe",
                "When there is not enough space available, the justify-center-safe utility will align items to the start of the container instead of the center.",
            ],
            snippet: r#"<div class="flex justify-center ...">
<div>01</div>
<div>02</div>
<div>03</div>
<div>04</div>
</div>
<div class="flex justify-center-safe ...">
<div>01</div>
<div>02</div>
<div>03</div>
<div>04</div>
</div>"#,
            demo: el(
                "flex w-full gap-4 rounded-lg bg-sky-300 p-2 justify-center",
                &[
                    labeled(
                        "size-14 flex items-center justify-center rounded-lg text-white bg-sky-500",
                        "01",
                    ),
                    labeled(
                        "size-14 flex items-center justify-center rounded-lg text-white bg-sky-500",
                        "02",
                    ),
                    labeled(
                        "size-14 flex items-center justify-center rounded-lg text-white bg-sky-500",
                        "03",
                    ),
                    labeled(
                        "size-14 flex items-center justify-center rounded-lg text-white bg-sky-500",
                        "04",
                    ),
                ],
            ),
        },
        TwExample {
            title: "End",
            prose: &[
                "Use the justify-end or justify-end-safe utilities to justify items against the end of the container's main axis:",
                "Resize the container to see the alignment behavior",
                "justify-end",
                "justify-end-safe",
                "When there is not enough space available, the justify-end-safe utility will align items to the start of the container instead of the end.",
            ],
            snippet: r#"<div class="flex justify-end ...">
<div>01</div>
<div>02</div>
<div>03</div>
<div>04</div>
</div>
<div class="flex justify-end-safe ...">
<div>01</div>
<div>02</div>
<div>03</div>
<div>04</div>
</div>"#,
            demo: el(
                "flex w-full gap-4 rounded-lg bg-sky-300 p-2 justify-end",
                &[
                    labeled(
                        "size-14 flex items-center justify-center rounded-lg text-white bg-sky-500",
                        "01",
                    ),
                    labeled(
                        "size-14 flex items-center justify-center rounded-lg text-white bg-sky-500",
                        "02",
                    ),
                    labeled(
                        "size-14 flex items-center justify-center rounded-lg text-white bg-sky-500",
                        "03",
                    ),
                    labeled(
                        "size-14 flex items-center justify-center rounded-lg text-white bg-sky-500",
                        "04",
                    ),
                ],
            ),
        },
        TwExample {
            title: "Space between",
            prose: &[
                "Use the justify-between utility to justify items along the container's main axis such that there is an equal amount of space between each item:",
            ],
            snippet: r#"<div class="flex justify-between ...">
<div>01</div>
<div>02</div>
<div>03</div>
</div>"#,
            demo: el(
                "flex w-full gap-4 rounded-lg bg-sky-300 p-2 justify-between",
                &[
                    labeled(
                        "size-14 flex items-center justify-center rounded-lg text-white bg-sky-500",
                        "01",
                    ),
                    labeled(
                        "size-14 flex items-center justify-center rounded-lg text-white bg-sky-500",
                        "02",
                    ),
                    labeled(
                        "size-14 flex items-center justify-center rounded-lg text-white bg-sky-500",
                        "03",
                    ),
                ],
            ),
        },
        TwExample {
            title: "Space around",
            prose: &[
                "Use the justify-around utility to justify items along the container's main axis such that there is an equal amount of space on each side of each item:",
            ],
            snippet: r#"<div class="flex justify-around ...">
<div>01</div>
<div>02</div>
<div>03</div>
</div>"#,
            demo: el(
                "flex w-full gap-4 rounded-lg bg-sky-300 p-2 justify-around",
                &[
                    labeled(
                        "size-14 flex items-center justify-center rounded-lg text-white bg-sky-500",
                        "01",
                    ),
                    labeled(
                        "size-14 flex items-center justify-center rounded-lg text-white bg-sky-500",
                        "02",
                    ),
                    labeled(
                        "size-14 flex items-center justify-center rounded-lg text-white bg-sky-500",
                        "03",
                    ),
                ],
            ),
        },
        TwExample {
            title: "Space evenly",
            prose: &[
                "Use the justify-evenly utility to justify items along the container's main axis such that there is an equal amount of space around each item, but also accounting for the doubling of space you would normally see between each item when using justify-around:",
            ],
            snippet: r#"<div class="flex justify-evenly ...">
<div>01</div>
<div>02</div>
<div>03</div>
</div>"#,
            demo: el(
                "flex w-full gap-4 rounded-lg bg-sky-300 p-2 justify-evenly",
                &[
                    labeled(
                        "size-14 flex items-center justify-center rounded-lg text-white bg-sky-500",
                        "01",
                    ),
                    labeled(
                        "size-14 flex items-center justify-center rounded-lg text-white bg-sky-500",
                        "02",
                    ),
                    labeled(
                        "size-14 flex items-center justify-center rounded-lg text-white bg-sky-500",
                        "03",
                    ),
                ],
            ),
        },
        TwExample {
            title: "Stretch",
            prose: &[
                "Use the justify-stretch utility to allow auto-sized content items to fill the available space along the container's main axis:",
            ],
            snippet: r#"<div class="grid grid-cols-[4rem_auto_4rem] justify-stretch ...">
<div>01</div>
<div>02</div>
<div>03</div>
</div>"#,
            demo: el(
                "grid w-full grid-cols-3 justify-stretch gap-4 rounded-lg bg-sky-300 p-2",
                &[
                    labeled(
                        "h-14 flex items-center justify-center rounded-lg text-white bg-sky-500",
                        "01",
                    ),
                    labeled(
                        "h-14 flex items-center justify-center rounded-lg text-white bg-sky-500",
                        "02",
                    ),
                    labeled(
                        "h-14 flex items-center justify-center rounded-lg text-white bg-sky-500",
                        "03",
                    ),
                ],
            ),
        },
        TwExample {
            title: "Normal",
            prose: &[
                "Use the justify-normal utility to pack content items in their default position as if no justify-content value was set:",
            ],
            snippet: r#"<div class="flex justify-normal ...">
<div>01</div>
<div>02</div>
<div>03</div>
</div>"#,
            demo: el(
                "flex w-full gap-4 rounded-lg bg-sky-300 p-2 justify-start",
                &[
                    labeled(
                        "size-14 flex items-center justify-center rounded-lg text-white bg-sky-500",
                        "justify-normal ≈ justify-start",
                    ),
                    labeled(
                        "size-14 flex items-center justify-center rounded-lg text-white bg-sky-500",
                        "01",
                    ),
                    labeled(
                        "size-14 flex items-center justify-center rounded-lg text-white bg-sky-500",
                        "02",
                    ),
                ],
            ),
        },
        TwExample {
            title: "Responsive design",
            prose: &[
                "Prefix a justify-content utility with a breakpoint variant like md: to only apply the utility at medium screen sizes and above:",
                "Learn more about using variants in the variants documentation.",
            ],
            snippet: r#"<div class="flex justify-start md:justify-between ...">
<!-- ... -->
</div>"#,
            demo: el(
                "flex w-full gap-4 rounded-lg bg-sky-300 p-2 justify-start md:justify-between",
                &[
                    labeled(
                        "size-14 flex items-center justify-center rounded-lg text-white bg-sky-500",
                        "01",
                    ),
                    labeled(
                        "size-14 flex items-center justify-center rounded-lg text-white bg-sky-500",
                        "02",
                    ),
                    labeled(
                        "size-14 flex items-center justify-center rounded-lg text-white bg-sky-500",
                        "03",
                    ),
                ],
            ),
        },
    ],
};
