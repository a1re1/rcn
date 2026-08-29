//! <https://tailwindcss.com/docs/place-items>

use crate::tw_docs::demo::{el, labeled};

use crate::tw_docs::{TwExample, TwPage, TwSection};

/// <https://tailwindcss.com/docs/place-items>
pub static PLACE_ITEMS: TwPage = TwPage {
    slug: "place-items",
    title: "Place-items",
    section: TwSection::FlexboxGrid,
    description: "Utilities for controlling how items are justified and aligned at the same time.",
    reference: &[
        ("place-items-start", "place-items: start;"),
        ("place-items-end", "place-items: end;"),
        ("place-items-end-safe", "place-items: safe end;"),
        ("place-items-center", "place-items: center;"),
        ("place-items-center-safe", "place-items: safe center;"),
        ("place-items-baseline", "place-items: baseline;"),
        ("place-items-stretch", "place-items: stretch;"),
    ],
    examples: &[
        TwExample {
            title: "Start",
            prose: &[
                "Use place-items-start to place grid items on the start of their grid areas on both axes:",
            ],
            snippet: r#"<div class="grid grid-cols-3 place-items-start gap-4 ...">
<div>01</div>
<div>02</div>
<div>03</div>
<div>04</div>
<div>05</div>
<div>06</div>
</div>"#,
            demo: el(
                "grid w-full grid-cols-3 place-items-start gap-4 rounded-lg bg-cyan-300 p-2",
                &[
                    labeled(
                        "size-14 flex items-center justify-center rounded-lg text-white bg-cyan-500",
                        "01",
                    ),
                    labeled(
                        "size-14 flex items-center justify-center rounded-lg text-white bg-cyan-500",
                        "02",
                    ),
                    labeled(
                        "size-14 flex items-center justify-center rounded-lg text-white bg-cyan-500",
                        "03",
                    ),
                    labeled(
                        "size-14 flex items-center justify-center rounded-lg text-white bg-cyan-500",
                        "04",
                    ),
                    labeled(
                        "size-14 flex items-center justify-center rounded-lg text-white bg-cyan-500",
                        "05",
                    ),
                    labeled(
                        "size-14 flex items-center justify-center rounded-lg text-white bg-cyan-500",
                        "06",
                    ),
                ],
            ),
        },
        TwExample {
            title: "End",
            prose: &[
                "Use place-items-end to place grid items on the end of their grid areas on both axes:",
            ],
            snippet: r#"<div class="grid h-56 grid-cols-3 place-items-end gap-4 ...">
<div>01</div>
<div>02</div>
<div>03</div>
<div>04</div>
<div>05</div>
<div>06</div>
</div>"#,
            demo: el(
                "grid h-56 w-full grid-cols-3 place-items-end gap-4 rounded-lg bg-cyan-300 p-2",
                &[
                    labeled(
                        "size-14 flex items-center justify-center rounded-lg text-white bg-cyan-500",
                        "01",
                    ),
                    labeled(
                        "size-14 flex items-center justify-center rounded-lg text-white bg-cyan-500",
                        "02",
                    ),
                    labeled(
                        "size-14 flex items-center justify-center rounded-lg text-white bg-cyan-500",
                        "03",
                    ),
                    labeled(
                        "size-14 flex items-center justify-center rounded-lg text-white bg-cyan-500",
                        "04",
                    ),
                    labeled(
                        "size-14 flex items-center justify-center rounded-lg text-white bg-cyan-500",
                        "05",
                    ),
                    labeled(
                        "size-14 flex items-center justify-center rounded-lg text-white bg-cyan-500",
                        "06",
                    ),
                ],
            ),
        },
        TwExample {
            title: "Center",
            prose: &[
                "Use place-items-center to place grid items on the center of their grid areas on both axes:",
            ],
            snippet: r#"<div class="grid h-56 grid-cols-3 place-items-center gap-4 ...">
<div>01</div>
<div>02</div>
<div>03</div>
<div>04</div>
<div>05</div>
<div>06</div>
</div>"#,
            demo: el(
                "grid h-56 w-full grid-cols-3 place-items-center gap-4 rounded-lg bg-cyan-300 p-2",
                &[
                    labeled(
                        "size-14 flex items-center justify-center rounded-lg text-white bg-cyan-500",
                        "01",
                    ),
                    labeled(
                        "size-14 flex items-center justify-center rounded-lg text-white bg-cyan-500",
                        "02",
                    ),
                    labeled(
                        "size-14 flex items-center justify-center rounded-lg text-white bg-cyan-500",
                        "03",
                    ),
                    labeled(
                        "size-14 flex items-center justify-center rounded-lg text-white bg-cyan-500",
                        "04",
                    ),
                    labeled(
                        "size-14 flex items-center justify-center rounded-lg text-white bg-cyan-500",
                        "05",
                    ),
                    labeled(
                        "size-14 flex items-center justify-center rounded-lg text-white bg-cyan-500",
                        "06",
                    ),
                ],
            ),
        },
        TwExample {
            title: "Stretch",
            prose: &[
                "Use place-items-stretch to stretch items along their grid areas on both axes:",
            ],
            snippet: r#"<div class="grid h-56 grid-cols-3 place-items-stretch gap-4 ...">
<div>01</div>
<div>02</div>
<div>03</div>
<div>04</div>
<div>05</div>
<div>06</div>
</div>"#,
            demo: el(
                "grid h-56 w-full grid-cols-3 place-items-stretch gap-4 rounded-lg bg-cyan-300 p-2",
                &[
                    labeled(
                        " flex items-center justify-center rounded-lg text-white bg-cyan-500",
                        "01",
                    ),
                    labeled(
                        " flex items-center justify-center rounded-lg text-white bg-cyan-500",
                        "02",
                    ),
                    labeled(
                        " flex items-center justify-center rounded-lg text-white bg-cyan-500",
                        "03",
                    ),
                    labeled(
                        " flex items-center justify-center rounded-lg text-white bg-cyan-500",
                        "04",
                    ),
                    labeled(
                        " flex items-center justify-center rounded-lg text-white bg-cyan-500",
                        "05",
                    ),
                    labeled(
                        " flex items-center justify-center rounded-lg text-white bg-cyan-500",
                        "06",
                    ),
                ],
            ),
        },
    ],
};
