//! <https://tailwindcss.com/docs/align-content>

use crate::tw_docs::demo::{el, labeled};

use crate::tw_docs::{TwExample, TwPage, TwSection};

/// <https://tailwindcss.com/docs/align-content>
pub static ALIGN_CONTENT: TwPage = TwPage {
    slug: "align-content",
    title: "Align-content",
    section: TwSection::FlexboxGrid,
    description: "Utilities for controlling how rows are positioned in multi-row flex and grid containers.",
    reference: &[
        ("content-normal", "align-content: normal;"),
        ("content-center", "align-content: center;"),
        ("content-start", "align-content: flex-start;"),
        ("content-end", "align-content: flex-end;"),
        ("content-between", "align-content: space-between;"),
        ("content-around", "align-content: space-around;"),
        ("content-evenly", "align-content: space-evenly;"),
        ("content-baseline", "align-content: baseline;"),
        ("content-stretch", "align-content: stretch;"),
    ],
    examples: &[
        TwExample {
            title: "Start",
            prose: &[
                "Use content-start to pack rows in a container against the start of the cross axis:",
            ],
            snippet: r#"<div class="grid h-56 grid-cols-3 content-start gap-4 ...">
<div>01</div>
<div>02</div>
<div>03</div>
<div>04</div>
<div>05</div>
</div>"#,
            demo: el(
                "grid h-56 w-full grid-cols-3 content-start gap-4 rounded-lg bg-fuchsia-300 p-2",
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
                ],
            ),
        },
        TwExample {
            title: "Center",
            prose: &[
                "Use content-center to pack rows in a container in the center of the cross axis:",
            ],
            snippet: r#"<div class="grid h-56 grid-cols-3 content-center gap-4 ...">
<div>01</div>
<div>02</div>
<div>03</div>
<div>04</div>
<div>05</div>
</div>"#,
            demo: el(
                "grid h-56 w-full grid-cols-3 content-center gap-4 rounded-lg bg-fuchsia-300 p-2",
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
                ],
            ),
        },
        TwExample {
            title: "End",
            prose: &[
                "Use content-end to pack rows in a container against the end of the cross axis:",
            ],
            snippet: r#"<div class="grid h-56 grid-cols-3 content-end gap-4 ...">
<div>01</div>
<div>02</div>
<div>03</div>
<div>04</div>
<div>05</div>
</div>"#,
            demo: el(
                "grid h-56 w-full grid-cols-3 content-end gap-4 rounded-lg bg-fuchsia-300 p-2",
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
                ],
            ),
        },
        TwExample {
            title: "Space between",
            prose: &[
                "Use content-between to distribute rows in a container such that there is an equal amount of space between each line:",
            ],
            snippet: r#"<div class="grid h-56 grid-cols-3 content-between gap-4 ...">
<div>01</div>
<div>02</div>
<div>03</div>
<div>04</div>
<div>05</div>
</div>"#,
            demo: el(
                "grid h-56 w-full grid-cols-3 content-between gap-4 rounded-lg bg-fuchsia-300 p-2",
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
                ],
            ),
        },
        TwExample {
            title: "Space around",
            prose: &[
                "Use content-around to distribute rows in a container such that there is an equal amount of space around each line:",
            ],
            snippet: r#"<div class="grid h-56 grid-cols-3 content-around gap-4 ...">
<div>01</div>
<div>02</div>
<div>03</div>
<div>04</div>
<div>05</div>
</div>"#,
            demo: el(
                "grid h-56 w-full grid-cols-3 content-around gap-4 rounded-lg bg-fuchsia-300 p-2",
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
                ],
            ),
        },
        TwExample {
            title: "Space evenly",
            prose: &[
                "Use content-evenly to distribute rows in a container such that there is an equal amount of space around each item, but also accounting for the doubling of space you would normally see between each item when using content-around:",
            ],
            snippet: r#"<div class="grid h-56 grid-cols-3 content-evenly gap-4 ...">
<div>01</div>
<div>02</div>
<div>03</div>
<div>04</div>
<div>05</div>
</div>"#,
            demo: el(
                "grid h-56 w-full grid-cols-3 content-evenly gap-4 rounded-lg bg-fuchsia-300 p-2",
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
                ],
            ),
        },
        TwExample {
            title: "Stretch",
            prose: &[
                "Use content-stretch to allow content items to fill the available space along the container’s cross axis:",
            ],
            snippet: r#"<div class="grid h-56 grid-cols-3 content-stretch gap-4 ...">
<div>01</div>
<div>02</div>
<div>03</div>
<div>04</div>
<div>05</div>
</div>"#,
            demo: el(
                "grid h-56 w-full grid-cols-3 content-stretch gap-4 rounded-lg bg-fuchsia-300 p-2",
                &[
                    labeled(
                        " flex items-center justify-center rounded-lg text-white bg-fuchsia-500",
                        "01",
                    ),
                    labeled(
                        " flex items-center justify-center rounded-lg text-white bg-fuchsia-500",
                        "02",
                    ),
                    labeled(
                        " flex items-center justify-center rounded-lg text-white bg-fuchsia-500",
                        "03",
                    ),
                    labeled(
                        " flex items-center justify-center rounded-lg text-white bg-fuchsia-500",
                        "04",
                    ),
                    labeled(
                        " flex items-center justify-center rounded-lg text-white bg-fuchsia-500",
                        "05",
                    ),
                ],
            ),
        },
        TwExample {
            title: "Normal",
            prose: &[
                "Use content-normal to pack content items in their default position as if no align-content value was set:",
            ],
            snippet: r#"<div class="grid h-56 grid-cols-3 content-normal gap-4 ...">
<div>01</div>
<div>02</div>
<div>03</div>
<div>04</div>
<div>05</div>
</div>"#,
            demo: el(
                "grid h-56 w-full grid-cols-3 content-start gap-4 rounded-lg bg-fuchsia-300 p-2",
                &[
                    labeled(
                        "h-14 flex items-center justify-center rounded-lg text-white bg-fuchsia-500",
                        "content-normal ≈ content-start",
                    ),
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
                ],
            ),
        },
        TwExample {
            title: "Responsive design",
            prose: &[
                "Prefix an align-content utility with a breakpoint variant like md: to only apply the utility at medium screen sizes and above:",
                "Learn more about using variants in the variants documentation.",
            ],
            snippet: r#"<div class="grid content-start md:content-around ...">
<!-- ... -->
</div>"#,
            demo: el(
                "grid h-56 w-full grid-cols-3 content-start md:content-around gap-4 rounded-lg bg-fuchsia-300 p-2",
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
                ],
            ),
        },
    ],
};
