//! <https://tailwindcss.com/docs/align-self>

use crate::tw_docs::demo::{el, labeled};

use crate::tw_docs::{TwExample, TwPage, TwSection};

/// <https://tailwindcss.com/docs/align-self>
pub static ALIGN_SELF: TwPage = TwPage {
    slug: "align-self",
    title: "Align-self",
    section: TwSection::FlexboxGrid,
    description: "Utilities for controlling how an individual flex or grid item is positioned along its container's cross axis.",
    reference: &[
        ("self-auto", "align-self: auto;"),
        ("self-start", "align-self: flex-start;"),
        ("self-end", "align-self: flex-end;"),
        ("self-end-safe", "align-self: safe flex-end;"),
        ("self-center", "align-self: center;"),
        ("self-center-safe", "align-self: safe center;"),
        ("self-stretch", "align-self: stretch;"),
        ("self-baseline", "align-self: baseline;"),
        ("self-baseline-last", "align-self: last baseline;"),
    ],
    examples: &[
        TwExample {
            title: "Auto",
            prose: &[
                "Use the self-auto utility to align an item based on the value of the container's align-items property:",
            ],
            snippet: r#"<div class="flex items-stretch ...">
<div>01</div>
<div class="self-auto ...">02</div>
<div>03</div>
</div>"#,
            demo: el(
                "flex h-40 w-full items-stretch gap-4 rounded-lg bg-pink-300 p-2",
                &[
                    labeled(
                        "w-14 flex items-center justify-center rounded-lg text-white bg-pink-500",
                        "01",
                    ),
                    labeled(
                        "w-14 flex items-center justify-center rounded-lg text-white bg-pink-500",
                        "self-auto ≈ inherits",
                    ),
                    labeled(
                        "w-14 flex items-center justify-center rounded-lg text-white bg-pink-500",
                        "03",
                    ),
                ],
            ),
        },
        TwExample {
            title: "Start",
            prose: &[
                "Use the self-start utility to align an item to the start of the container's cross axis, despite the container's align-items value:",
            ],
            snippet: r#"<div class="flex items-stretch ...">
<div>01</div>
<div class="self-start ...">02</div>
<div>03</div>
</div>"#,
            demo: el(
                "flex h-40 w-full items-stretch gap-4 rounded-lg bg-pink-300 p-2",
                &[
                    labeled(
                        "w-14 flex items-center justify-center rounded-lg text-white bg-pink-500",
                        "01",
                    ),
                    labeled(
                        "w-14 self-start py-4 flex items-center justify-center rounded-lg text-white bg-pink-500",
                        "02",
                    ),
                    labeled(
                        "w-14 flex items-center justify-center rounded-lg text-white bg-pink-500",
                        "03",
                    ),
                ],
            ),
        },
        TwExample {
            title: "Center",
            prose: &[
                "Use the self-center utility to align an item along the center of the container's cross axis, despite the container's align-items value:",
            ],
            snippet: r#"<div class="flex items-stretch ...">
<div>01</div>
<div class="self-center ...">02</div>
<div>03</div>
</div>"#,
            demo: el(
                "flex h-40 w-full items-stretch gap-4 rounded-lg bg-pink-300 p-2",
                &[
                    labeled(
                        "w-14 flex items-center justify-center rounded-lg text-white bg-pink-500",
                        "01",
                    ),
                    labeled(
                        "w-14 self-center py-4 flex items-center justify-center rounded-lg text-white bg-pink-500",
                        "02",
                    ),
                    labeled(
                        "w-14 flex items-center justify-center rounded-lg text-white bg-pink-500",
                        "03",
                    ),
                ],
            ),
        },
        TwExample {
            title: "End",
            prose: &[
                "Use the self-end utility to align an item to the end of the container's cross axis, despite the container's align-items value:",
            ],
            snippet: r#"<div class="flex items-stretch ...">
<div>01</div>
<div class="self-end ...">02</div>
<div>03</div>
</div>"#,
            demo: el(
                "flex h-40 w-full items-stretch gap-4 rounded-lg bg-pink-300 p-2",
                &[
                    labeled(
                        "w-14 flex items-center justify-center rounded-lg text-white bg-pink-500",
                        "01",
                    ),
                    labeled(
                        "w-14 self-end py-4 flex items-center justify-center rounded-lg text-white bg-pink-500",
                        "02",
                    ),
                    labeled(
                        "w-14 flex items-center justify-center rounded-lg text-white bg-pink-500",
                        "03",
                    ),
                ],
            ),
        },
        TwExample {
            title: "Stretch",
            prose: &[
                "Use the self-stretch utility to stretch an item to fill the container's cross axis, despite the container's align-items value:",
            ],
            snippet: r#"<div class="flex items-stretch ...">
<div>01</div>
<div class="self-stretch ...">02</div>
<div>03</div>
</div>"#,
            demo: el(
                "flex h-40 w-full items-start gap-4 rounded-lg bg-pink-300 p-2",
                &[
                    labeled(
                        "w-14 py-4 flex items-center justify-center rounded-lg text-white bg-pink-500",
                        "01",
                    ),
                    labeled(
                        "w-14 self-stretch flex items-center justify-center rounded-lg text-white bg-pink-500",
                        "02",
                    ),
                    labeled(
                        "w-14 py-4 flex items-center justify-center rounded-lg text-white bg-pink-500",
                        "03",
                    ),
                ],
            ),
        },
        TwExample {
            title: "Baseline",
            prose: &[
                "Use the self-baseline utility to align an item such that its baseline aligns with the baseline of the flex container's cross axis:",
            ],
            snippet: r#"<div class="flex ...">
<div class="self-baseline pt-2 pb-6">01</div>
<div class="self-baseline pt-8 pb-12">02</div>
<div class="self-baseline pt-12 pb-4">03</div>
</div>"#,
            demo: el(
                "flex w-full gap-4 rounded-lg bg-pink-300 p-2",
                &[
                    labeled(
                        "w-14 self-baseline pt-2 pb-6 flex items-center justify-center rounded-lg text-white bg-pink-500",
                        "01",
                    ),
                    labeled(
                        "w-14 self-baseline pt-8 pb-12 flex items-center justify-center rounded-lg text-white bg-pink-500",
                        "02",
                    ),
                    labeled(
                        "w-14 self-baseline pt-12 pb-4 flex items-center justify-center rounded-lg text-white bg-pink-500",
                        "03",
                    ),
                ],
            ),
        },
        TwExample {
            title: "Last baseline",
            prose: &[
                "Use the self-baseline-last utility to align an item along the container's cross axis such that its baseline aligns with the last baseline in the container:",
                "Working on the future of astronaut recruitment at Space Recruit.",
                "A multidisciplinary designer.",
                "This is useful for ensuring that text items align with each other, even if they have different heights.",
            ],
            snippet: r#"<div class="grid grid-cols-[1fr_auto]">
<div>
<img src="img/spencer-sharp.jpg" />
<h4>Spencer Sharp</h4>
<p class="self-baseline-last">Working on the future of astronaut recruitment at Space Recruit.</p>
</div>
<p class="self-baseline-last">spacerecruit.com</p>
</div>"#,
            demo: el(
                "grid w-96 grid-cols-2 gap-4 rounded-lg bg-pink-300 p-2",
                &[
                    el(
                        "flex flex-col gap-1 rounded-lg bg-pink-500 p-3 text-white",
                        &[
                            labeled("", "Spencer Sharp"),
                            labeled("self-end", "self-baseline-last ≈ self-end"),
                        ],
                    ),
                    labeled(
                        "self-end px-3 py-2 flex items-center justify-center rounded-lg text-white bg-pink-500",
                        "spacerecruit.com",
                    ),
                ],
            ),
        },
        TwExample {
            title: "Responsive design",
            prose: &[
                "Prefix an align-self utility with a breakpoint variant like md: to only apply the utility at medium screen sizes and above:",
                "Learn more about using variants in the variants documentation.",
            ],
            snippet: r#"<div class="self-auto md:self-end ...">
<!-- ... -->
</div>"#,
            demo: el(
                "flex h-40 w-full items-stretch gap-4 rounded-lg bg-pink-300 p-2",
                &[
                    labeled(
                        "w-14 flex items-center justify-center rounded-lg text-white bg-pink-500",
                        "01",
                    ),
                    labeled(
                        "w-14 md:self-end py-4 flex items-center justify-center rounded-lg text-white bg-pink-500",
                        "self-auto md:self-end",
                    ),
                    labeled(
                        "w-14 flex items-center justify-center rounded-lg text-white bg-pink-500",
                        "03",
                    ),
                ],
            ),
        },
    ],
};
