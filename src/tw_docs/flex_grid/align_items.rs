//! <https://tailwindcss.com/docs/align-items>

use crate::tw_docs::demo::{el, labeled};

use crate::tw_docs::{TwExample, TwPage, TwSection};

/// <https://tailwindcss.com/docs/align-items>
pub static ALIGN_ITEMS: TwPage = TwPage {
    slug: "align-items",
    title: "Align-items",
    section: TwSection::FlexboxGrid,
    description: "Utilities for controlling how flex and grid items are positioned along a container's cross axis.",
    reference: &[
        ("items-start", "align-items: flex-start;"),
        ("items-end", "align-items: flex-end;"),
        ("items-end-safe", "align-items: safe flex-end;"),
        ("items-center", "align-items: center;"),
        ("items-center-safe", "align-items: safe center;"),
        ("items-baseline", "align-items: baseline;"),
        ("items-baseline-last", "align-items: last baseline;"),
        ("items-stretch", "align-items: stretch;"),
    ],
    examples: &[
        TwExample {
            title: "Stretch",
            prose: &[
                "Use the items-stretch utility to stretch items to fill the container's cross axis:",
            ],
            snippet: r#"<div class="flex items-stretch ...">
<div class="py-4">01</div>
<div class="py-12">02</div>
<div class="py-8">03</div>
</div>"#,
            demo: el(
                "flex w-full items-stretch gap-4 rounded-lg bg-purple-300 p-2",
                &[
                    labeled(
                        "w-14 py-4 flex items-center justify-center rounded-lg text-white bg-purple-500",
                        "01",
                    ),
                    labeled(
                        "w-14 py-12 flex items-center justify-center rounded-lg text-white bg-purple-500",
                        "02",
                    ),
                    labeled(
                        "w-14 py-8 flex items-center justify-center rounded-lg text-white bg-purple-500",
                        "03",
                    ),
                ],
            ),
        },
        TwExample {
            title: "Start",
            prose: &[
                "Use the items-start utility to align items to the start of the container's cross axis:",
            ],
            snippet: r#"<div class="flex items-start ...">
<div class="py-4">01</div>
<div class="py-12">02</div>
<div class="py-8">03</div>
</div>"#,
            demo: el(
                "flex w-full items-start gap-4 rounded-lg bg-purple-300 p-2",
                &[
                    labeled(
                        "w-14 py-4 flex items-center justify-center rounded-lg text-white bg-purple-500",
                        "01",
                    ),
                    labeled(
                        "w-14 py-12 flex items-center justify-center rounded-lg text-white bg-purple-500",
                        "02",
                    ),
                    labeled(
                        "w-14 py-8 flex items-center justify-center rounded-lg text-white bg-purple-500",
                        "03",
                    ),
                ],
            ),
        },
        TwExample {
            title: "Center",
            prose: &[
                "Use the items-center utility to align items along the center of the container's cross axis:",
            ],
            snippet: r#"<div class="flex items-center ...">
<div class="py-4">01</div>
<div class="py-12">02</div>
<div class="py-8">03</div>
</div>"#,
            demo: el(
                "flex w-full items-center gap-4 rounded-lg bg-purple-300 p-2",
                &[
                    labeled(
                        "w-14 py-4 flex items-center justify-center rounded-lg text-white bg-purple-500",
                        "01",
                    ),
                    labeled(
                        "w-14 py-12 flex items-center justify-center rounded-lg text-white bg-purple-500",
                        "02",
                    ),
                    labeled(
                        "w-14 py-8 flex items-center justify-center rounded-lg text-white bg-purple-500",
                        "03",
                    ),
                ],
            ),
        },
        TwExample {
            title: "End",
            prose: &[
                "Use the items-end utility to align items to the end of the container's cross axis:",
            ],
            snippet: r#"<div class="flex items-end ...">
<div class="py-4">01</div>
<div class="py-12">02</div>
<div class="py-8">03</div>
</div>"#,
            demo: el(
                "flex w-full items-end gap-4 rounded-lg bg-purple-300 p-2",
                &[
                    labeled(
                        "w-14 py-4 flex items-center justify-center rounded-lg text-white bg-purple-500",
                        "01",
                    ),
                    labeled(
                        "w-14 py-12 flex items-center justify-center rounded-lg text-white bg-purple-500",
                        "02",
                    ),
                    labeled(
                        "w-14 py-8 flex items-center justify-center rounded-lg text-white bg-purple-500",
                        "03",
                    ),
                ],
            ),
        },
        TwExample {
            title: "Baseline",
            prose: &[
                "Use the items-baseline utility to align items along the container's cross axis such that all of their baselines align:",
            ],
            snippet: r#"<div class="flex items-baseline ...">
<div class="pt-2 pb-6">01</div>
<div class="pt-8 pb-12">02</div>
<div class="pt-12 pb-4">03</div>
</div>"#,
            demo: el(
                "flex w-full items-baseline gap-4 rounded-lg bg-purple-300 p-2",
                &[
                    labeled(
                        "w-14 pt-2 pb-6 flex items-center justify-center rounded-lg text-white bg-purple-500",
                        "01",
                    ),
                    labeled(
                        "w-14 pt-8 pb-12 flex items-center justify-center rounded-lg text-white bg-purple-500",
                        "02",
                    ),
                    labeled(
                        "w-14 pt-12 pb-4 flex items-center justify-center rounded-lg text-white bg-purple-500",
                        "03",
                    ),
                ],
            ),
        },
        TwExample {
            title: "Last baseline",
            prose: &[
                "Use the items-baseline-last utility to align items along the container's cross axis such that all of their baselines align with the last baseline in the container:",
                "Working on the future of astronaut recruitment at Space Recruit.",
                "A multidisciplinary designer.",
                "This is useful for ensuring that text items align with each other, even if they have different heights.",
            ],
            snippet: r#"<div class="grid grid-cols-[1fr_auto] items-baseline-last">
<div>
<img src="img/spencer-sharp.jpg" />
<h4>Spencer Sharp</h4>
<p>Working on the future of astronaut recruitment at Space Recruit.</p>
</div>
<p>spacerecruit.com</p>
</div>"#,
            demo: el(
                "grid w-96 grid-cols-2 items-end gap-4 rounded-lg bg-purple-300 p-2",
                &[
                    el(
                        "flex flex-col gap-1 rounded-lg bg-purple-500 p-3 text-white",
                        &[
                            labeled("", "Spencer Sharp"),
                            labeled("", "Working on the future of astronaut recruitment."),
                        ],
                    ),
                    labeled(
                        "px-3 py-2 flex items-center justify-center rounded-lg text-white bg-purple-500",
                        "items-baseline-last ≈ items-end",
                    ),
                ],
            ),
        },
        TwExample {
            title: "Responsive design",
            prose: &[
                "Prefix an align-items utility with a breakpoint variant like md: to only apply the utility at medium screen sizes and above:",
                "Learn more about using variants in the variants documentation.",
            ],
            snippet: r#"<div class="flex items-stretch md:items-center ...">
<!-- ... -->
</div>"#,
            demo: el(
                "flex w-full items-stretch md:items-center gap-4 rounded-lg bg-purple-300 p-2",
                &[
                    labeled(
                        "w-14 py-4 flex items-center justify-center rounded-lg text-white bg-purple-500",
                        "01",
                    ),
                    labeled(
                        "w-14 py-12 flex items-center justify-center rounded-lg text-white bg-purple-500",
                        "02",
                    ),
                    labeled(
                        "w-14 py-8 flex items-center justify-center rounded-lg text-white bg-purple-500",
                        "03",
                    ),
                ],
            ),
        },
    ],
};
