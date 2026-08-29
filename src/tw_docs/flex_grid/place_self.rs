//! <https://tailwindcss.com/docs/place-self>

use crate::tw_docs::demo::{el, labeled};

use crate::tw_docs::{TwExample, TwPage, TwSection};

/// <https://tailwindcss.com/docs/place-self>
pub static PLACE_SELF: TwPage = TwPage {
    slug: "place-self",
    title: "Place-self",
    section: TwSection::FlexboxGrid,
    description: "Utilities for controlling how an individual item is justified and aligned at the same time.",
    reference: &[
        ("place-self-auto", "place-self: auto;"),
        ("place-self-start", "place-self: start;"),
        ("place-self-end", "place-self: end;"),
        ("place-self-end-safe", "place-self: safe end;"),
        ("place-self-center", "place-self: center;"),
        ("place-self-center-safe", "place-self: safe center;"),
        ("place-self-stretch", "place-self: stretch;"),
    ],
    examples: &[
        TwExample {
            title: "Auto",
            prose: &[
                "Use place-self-auto to align an item based on the value of the container's place-items property:",
            ],
            snippet: r#"<div class="grid grid-cols-3 gap-4 ...">
<div>01</div>
<div class="place-self-auto ...">02</div>
<div>03</div>
<div>04</div>
<div>05</div>
<div>06</div>
</div>"#,
            demo: el(
                "grid h-56 w-full grid-cols-3 gap-4 rounded-lg bg-amber-300 p-2",
                &[
                    labeled(
                        " flex items-center justify-center rounded-lg text-white bg-amber-500",
                        "01",
                    ),
                    labeled(
                        " flex items-center justify-center rounded-lg text-white bg-amber-500",
                        "place-self-auto ≈ inherits",
                    ),
                    labeled(
                        " flex items-center justify-center rounded-lg text-white bg-amber-500",
                        "03",
                    ),
                    labeled(
                        " flex items-center justify-center rounded-lg text-white bg-amber-500",
                        "04",
                    ),
                    labeled(
                        " flex items-center justify-center rounded-lg text-white bg-amber-500",
                        "05",
                    ),
                    labeled(
                        " flex items-center justify-center rounded-lg text-white bg-amber-500",
                        "06",
                    ),
                ],
            ),
        },
        TwExample {
            title: "Start",
            prose: &["Use place-self-start to align an item to the start on both axes:"],
            snippet: r#"<div class="grid grid-cols-3 gap-4 ...">
<div>01</div>
<div class="place-self-start ...">02</div>
<div>03</div>
<div>04</div>
<div>05</div>
<div>06</div>
</div>"#,
            demo: el(
                "grid h-56 w-full grid-cols-3 gap-4 rounded-lg bg-amber-300 p-2",
                &[
                    labeled(
                        " flex items-center justify-center rounded-lg text-white bg-amber-500",
                        "01",
                    ),
                    labeled(
                        "place-self-start size-14 flex items-center justify-center rounded-lg text-white bg-amber-500",
                        "02",
                    ),
                    labeled(
                        " flex items-center justify-center rounded-lg text-white bg-amber-500",
                        "03",
                    ),
                    labeled(
                        " flex items-center justify-center rounded-lg text-white bg-amber-500",
                        "04",
                    ),
                    labeled(
                        " flex items-center justify-center rounded-lg text-white bg-amber-500",
                        "05",
                    ),
                    labeled(
                        " flex items-center justify-center rounded-lg text-white bg-amber-500",
                        "06",
                    ),
                ],
            ),
        },
        TwExample {
            title: "Center",
            prose: &["Use place-self-center to align an item at the center on both axes:"],
            snippet: r#"<div class="grid grid-cols-3 gap-4 ...">
<div>01</div>
<div class="place-self-center ...">02</div>
<div>03</div>
<div>04</div>
<div>05</div>
<div>06</div>
</div>"#,
            demo: el(
                "grid h-56 w-full grid-cols-3 gap-4 rounded-lg bg-amber-300 p-2",
                &[
                    labeled(
                        " flex items-center justify-center rounded-lg text-white bg-amber-500",
                        "01",
                    ),
                    labeled(
                        "place-self-center size-14 flex items-center justify-center rounded-lg text-white bg-amber-500",
                        "02",
                    ),
                    labeled(
                        " flex items-center justify-center rounded-lg text-white bg-amber-500",
                        "03",
                    ),
                    labeled(
                        " flex items-center justify-center rounded-lg text-white bg-amber-500",
                        "04",
                    ),
                    labeled(
                        " flex items-center justify-center rounded-lg text-white bg-amber-500",
                        "05",
                    ),
                    labeled(
                        " flex items-center justify-center rounded-lg text-white bg-amber-500",
                        "06",
                    ),
                ],
            ),
        },
        TwExample {
            title: "End",
            prose: &["Use place-self-end to align an item to the end on both axes:"],
            snippet: r#"<div class="grid grid-cols-3 gap-4 ...">
<div>01</div>
<div class="place-self-end ...">02</div>
<div>03</div>
<div>04</div>
<div>05</div>
<div>06</div>
</div>"#,
            demo: el(
                "grid h-56 w-full grid-cols-3 gap-4 rounded-lg bg-amber-300 p-2",
                &[
                    labeled(
                        " flex items-center justify-center rounded-lg text-white bg-amber-500",
                        "01",
                    ),
                    labeled(
                        "place-self-end size-14 flex items-center justify-center rounded-lg text-white bg-amber-500",
                        "02",
                    ),
                    labeled(
                        " flex items-center justify-center rounded-lg text-white bg-amber-500",
                        "03",
                    ),
                    labeled(
                        " flex items-center justify-center rounded-lg text-white bg-amber-500",
                        "04",
                    ),
                    labeled(
                        " flex items-center justify-center rounded-lg text-white bg-amber-500",
                        "05",
                    ),
                    labeled(
                        " flex items-center justify-center rounded-lg text-white bg-amber-500",
                        "06",
                    ),
                ],
            ),
        },
        TwExample {
            title: "Stretch",
            prose: &["Use place-self-stretch to stretch an item on both axes:"],
            snippet: r#"<div class="grid grid-cols-3 gap-4 ...">
<div>01</div>
<div class="place-self-stretch ...">02</div>
<div>03</div>
<div>04</div>
<div>05</div>
<div>06</div>
</div>"#,
            demo: el(
                "grid h-56 w-full grid-cols-3 place-items-start gap-4 rounded-lg bg-amber-300 p-2",
                &[
                    labeled(
                        "size-14 flex items-center justify-center rounded-lg text-white bg-amber-500",
                        "01",
                    ),
                    labeled(
                        "place-self-stretch flex items-center justify-center rounded-lg text-white bg-amber-500",
                        "02",
                    ),
                    labeled(
                        "size-14 flex items-center justify-center rounded-lg text-white bg-amber-500",
                        "03",
                    ),
                    labeled(
                        "size-14 flex items-center justify-center rounded-lg text-white bg-amber-500",
                        "04",
                    ),
                    labeled(
                        "size-14 flex items-center justify-center rounded-lg text-white bg-amber-500",
                        "05",
                    ),
                    labeled(
                        "size-14 flex items-center justify-center rounded-lg text-white bg-amber-500",
                        "06",
                    ),
                ],
            ),
        },
        TwExample {
            title: "Responsive design",
            prose: &[
                "Prefix a place-self utility with a breakpoint variant like md: to only apply the utility at medium screen sizes and above:",
                "Learn more about using variants in the variants documentation.",
            ],
            snippet: r#"<div class="place-self-start md:place-self-end ...">
<!-- ... -->
</div>"#,
            demo: el(
                "grid h-56 w-full grid-cols-3 gap-4 rounded-lg bg-amber-300 p-2",
                &[
                    labeled(
                        " flex items-center justify-center rounded-lg text-white bg-amber-500",
                        "01",
                    ),
                    labeled(
                        "place-self-start md:place-self-end size-14 flex items-center justify-center rounded-lg text-white bg-amber-500",
                        "02",
                    ),
                    labeled(
                        " flex items-center justify-center rounded-lg text-white bg-amber-500",
                        "03",
                    ),
                    labeled(
                        " flex items-center justify-center rounded-lg text-white bg-amber-500",
                        "04",
                    ),
                    labeled(
                        " flex items-center justify-center rounded-lg text-white bg-amber-500",
                        "05",
                    ),
                    labeled(
                        " flex items-center justify-center rounded-lg text-white bg-amber-500",
                        "06",
                    ),
                ],
            ),
        },
    ],
};
