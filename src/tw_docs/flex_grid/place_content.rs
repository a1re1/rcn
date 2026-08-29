//! <https://tailwindcss.com/docs/place-content>

use crate::tw_docs::demo::{el, labeled};

use crate::tw_docs::{TwExample, TwPage, TwSection};

/// <https://tailwindcss.com/docs/place-content>
pub static PLACE_CONTENT: TwPage = TwPage {
    slug: "place-content",
    title: "Place-content",
    section: TwSection::FlexboxGrid,
    description: "Utilities for controlling how content is justified and aligned at the same time.",
    reference: &[
        ("place-content-center", "place-content: center;"),
        ("place-content-center-safe", "place-content: safe center;"),
        ("place-content-start", "place-content: start;"),
        ("place-content-end", "place-content: end;"),
        ("place-content-end-safe", "place-content: safe end;"),
        ("place-content-between", "place-content: space-between;"),
        ("place-content-around", "place-content: space-around;"),
        ("place-content-evenly", "place-content: space-evenly;"),
        ("place-content-baseline", "place-content: baseline;"),
        ("place-content-stretch", "place-content: stretch;"),
    ],
    examples: &[
        TwExample {
            title: "Center",
            prose: &[
                "Use place-content-center to pack items in the center of the inline and block axes:",
            ],
            snippet: r#"<div class="grid h-48 grid-cols-2 place-content-center gap-4 ...">
<div>01</div>
<div>02</div>
<div>03</div>
<div>04</div>
</div>"#,
            demo: el(
                "grid h-48 w-full grid-cols-2 place-content-center gap-4 rounded-lg bg-emerald-300 p-2",
                &[
                    labeled(
                        "size-14 flex items-center justify-center rounded-lg text-white bg-emerald-500",
                        "01",
                    ),
                    labeled(
                        "size-14 flex items-center justify-center rounded-lg text-white bg-emerald-500",
                        "02",
                    ),
                    labeled(
                        "size-14 flex items-center justify-center rounded-lg text-white bg-emerald-500",
                        "03",
                    ),
                    labeled(
                        "size-14 flex items-center justify-center rounded-lg text-white bg-emerald-500",
                        "04",
                    ),
                ],
            ),
        },
        TwExample {
            title: "Start",
            prose: &[
                "Use place-content-start to pack items against the start of the inline and block axes:",
            ],
            snippet: r#"<div class="grid h-48 grid-cols-2 place-content-start gap-4 ...">
<div>01</div>
<div>02</div>
<div>03</div>
<div>04</div>
</div>"#,
            demo: el(
                "grid h-48 w-full grid-cols-2 place-content-start gap-4 rounded-lg bg-emerald-300 p-2",
                &[
                    labeled(
                        "size-14 flex items-center justify-center rounded-lg text-white bg-emerald-500",
                        "01",
                    ),
                    labeled(
                        "size-14 flex items-center justify-center rounded-lg text-white bg-emerald-500",
                        "02",
                    ),
                    labeled(
                        "size-14 flex items-center justify-center rounded-lg text-white bg-emerald-500",
                        "03",
                    ),
                    labeled(
                        "size-14 flex items-center justify-center rounded-lg text-white bg-emerald-500",
                        "04",
                    ),
                ],
            ),
        },
        TwExample {
            title: "End",
            prose: &[
                "Use place-content-end to pack items against the end of the inline and block axes:",
            ],
            snippet: r#"<div class="grid h-48 grid-cols-2 place-content-end gap-4 ...">
<div>01</div>
<div>02</div>
<div>03</div>
<div>04</div>
</div>"#,
            demo: el(
                "grid h-48 w-full grid-cols-2 place-content-end gap-4 rounded-lg bg-emerald-300 p-2",
                &[
                    labeled(
                        "size-14 flex items-center justify-center rounded-lg text-white bg-emerald-500",
                        "01",
                    ),
                    labeled(
                        "size-14 flex items-center justify-center rounded-lg text-white bg-emerald-500",
                        "02",
                    ),
                    labeled(
                        "size-14 flex items-center justify-center rounded-lg text-white bg-emerald-500",
                        "03",
                    ),
                    labeled(
                        "size-14 flex items-center justify-center rounded-lg text-white bg-emerald-500",
                        "04",
                    ),
                ],
            ),
        },
        TwExample {
            title: "Space between",
            prose: &[
                "Use place-content-between to distribute grid items along the inline and block axes so that there is an equal amount of space between each row and column on each axis respectively:",
            ],
            snippet: r#"<div class="grid h-48 grid-cols-2 place-content-between gap-4 ...">
<div>01</div>
<div>02</div>
<div>03</div>
<div>04</div>
</div>"#,
            demo: el(
                "grid h-48 w-full grid-cols-2 place-content-between gap-4 rounded-lg bg-emerald-300 p-2",
                &[
                    labeled(
                        "size-14 flex items-center justify-center rounded-lg text-white bg-emerald-500",
                        "01",
                    ),
                    labeled(
                        "size-14 flex items-center justify-center rounded-lg text-white bg-emerald-500",
                        "02",
                    ),
                    labeled(
                        "size-14 flex items-center justify-center rounded-lg text-white bg-emerald-500",
                        "03",
                    ),
                    labeled(
                        "size-14 flex items-center justify-center rounded-lg text-white bg-emerald-500",
                        "04",
                    ),
                ],
            ),
        },
        TwExample {
            title: "Space around",
            prose: &[
                "Use place-content-around to distribute grid items along the inline and block axes so that there is an equal amount of space around each row and column on each axis respectively:",
            ],
            snippet: r#"<div class="grid h-48 grid-cols-2 place-content-around gap-4 ...">
<div>01</div>
<div>02</div>
<div>03</div>
<div>04</div>
</div>"#,
            demo: el(
                "grid h-48 w-full grid-cols-2 place-content-around gap-4 rounded-lg bg-emerald-300 p-2",
                &[
                    labeled(
                        "size-14 flex items-center justify-center rounded-lg text-white bg-emerald-500",
                        "01",
                    ),
                    labeled(
                        "size-14 flex items-center justify-center rounded-lg text-white bg-emerald-500",
                        "02",
                    ),
                    labeled(
                        "size-14 flex items-center justify-center rounded-lg text-white bg-emerald-500",
                        "03",
                    ),
                    labeled(
                        "size-14 flex items-center justify-center rounded-lg text-white bg-emerald-500",
                        "04",
                    ),
                ],
            ),
        },
        TwExample {
            title: "Space evenly",
            prose: &[
                "Use place-content-evenly to distribute grid items such that they are evenly spaced on the inline and block axes:",
            ],
            snippet: r#"<div class="grid h-48 grid-cols-2 place-content-evenly gap-4 ...">
<div>01</div>
<div>02</div>
<div>03</div>
<div>04</div>
</div>"#,
            demo: el(
                "grid h-48 w-full grid-cols-2 place-content-evenly gap-4 rounded-lg bg-emerald-300 p-2",
                &[
                    labeled(
                        "size-14 flex items-center justify-center rounded-lg text-white bg-emerald-500",
                        "01",
                    ),
                    labeled(
                        "size-14 flex items-center justify-center rounded-lg text-white bg-emerald-500",
                        "02",
                    ),
                    labeled(
                        "size-14 flex items-center justify-center rounded-lg text-white bg-emerald-500",
                        "03",
                    ),
                    labeled(
                        "size-14 flex items-center justify-center rounded-lg text-white bg-emerald-500",
                        "04",
                    ),
                ],
            ),
        },
        TwExample {
            title: "Stretch",
            prose: &[
                "Use place-content-stretch to stretch grid items along their grid areas on the inline and block axes:",
            ],
            snippet: r#"<div class="grid h-48 grid-cols-2 place-content-stretch gap-4 ...">
<div>01</div>
<div>02</div>
<div>03</div>
<div>04</div>
</div>"#,
            demo: el(
                "grid h-48 w-full grid-cols-2 place-content-stretch gap-4 rounded-lg bg-emerald-300 p-2",
                &[
                    labeled(
                        " flex items-center justify-center rounded-lg text-white bg-emerald-500",
                        "01",
                    ),
                    labeled(
                        " flex items-center justify-center rounded-lg text-white bg-emerald-500",
                        "02",
                    ),
                    labeled(
                        " flex items-center justify-center rounded-lg text-white bg-emerald-500",
                        "03",
                    ),
                    labeled(
                        " flex items-center justify-center rounded-lg text-white bg-emerald-500",
                        "04",
                    ),
                ],
            ),
        },
        TwExample {
            title: "Responsive design",
            prose: &[
                "Prefix a place-content utility with a breakpoint variant like md: to only apply the utility at medium screen sizes and above:",
                "Learn more about using variants in the variants documentation.",
            ],
            snippet: r#"<div class="grid place-content-start md:place-content-center ...">
<!-- ... -->
</div>"#,
            demo: el(
                "grid h-48 w-full grid-cols-2 place-content-start md:place-content-center gap-4 rounded-lg bg-emerald-300 p-2",
                &[
                    labeled(
                        "size-14 flex items-center justify-center rounded-lg text-white bg-emerald-500",
                        "01",
                    ),
                    labeled(
                        "size-14 flex items-center justify-center rounded-lg text-white bg-emerald-500",
                        "02",
                    ),
                    labeled(
                        "size-14 flex items-center justify-center rounded-lg text-white bg-emerald-500",
                        "03",
                    ),
                    labeled(
                        "size-14 flex items-center justify-center rounded-lg text-white bg-emerald-500",
                        "04",
                    ),
                ],
            ),
        },
    ],
};
