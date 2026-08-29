//! <https://tailwindcss.com/docs/flex-shrink>

use crate::tw_docs::demo::{el, labeled};

use crate::tw_docs::{TwExample, TwPage, TwSection};

/// <https://tailwindcss.com/docs/flex-shrink>
pub static FLEX_SHRINK: TwPage = TwPage {
    slug: "flex-shrink",
    title: "Flex-shrink",
    section: TwSection::FlexboxGrid,
    description: "Utilities for controlling how flex items shrink.",
    reference: &[
        ("shrink", "flex-shrink: 1;"),
        ("shrink-<number>", "flex-shrink: <number>;"),
        ("shrink-[<value>]", "flex-shrink: <value>;"),
        (
            "shrink-(<custom-property>)",
            "flex-shrink: var(<custom-property>);",
        ),
    ],
    examples: &[
        TwExample {
            title: "Allowing flex items to shrink",
            prose: &["Use shrink to allow a flex item to shrink if needed:"],
            snippet: r#"<div class="flex ...">
<div class="h-14 w-14 flex-none ...">01</div>
<div class="h-14 w-64 shrink ...">02</div>
<div class="h-14 w-14 flex-none ...">03</div>
</div>"#,
            demo: el(
                "flex w-80 gap-4",
                &[
                    labeled(
                        "h-14 w-14 flex-none flex items-center justify-center rounded-lg text-white bg-sky-500",
                        "01",
                    ),
                    labeled(
                        "h-14 w-64 shrink flex items-center justify-center rounded-lg text-white bg-sky-500",
                        "02",
                    ),
                    labeled(
                        "h-14 w-14 flex-none flex items-center justify-center rounded-lg text-white bg-sky-500",
                        "03",
                    ),
                ],
            ),
        },
        TwExample {
            title: "Preventing items from shrinking",
            prose: &["Use shrink-0 to prevent a flex item from shrinking:"],
            snippet: r#"<div class="flex ...">
<div class="h-16 flex-1 ...">01</div>
<div class="h-16 w-32 shrink-0 ...">02</div>
<div class="h-16 flex-1 ...">03</div>
</div>"#,
            demo: el(
                "flex w-80 gap-4",
                &[
                    labeled(
                        "h-16 flex-1 flex items-center justify-center rounded-lg text-white bg-sky-500",
                        "01",
                    ),
                    labeled(
                        "h-16 w-32 shrink-0 flex items-center justify-center rounded-lg text-white bg-sky-500",
                        "02",
                    ),
                    labeled(
                        "h-16 flex-1 flex items-center justify-center rounded-lg text-white bg-sky-500",
                        "03",
                    ),
                ],
            ),
        },
        TwExample {
            title: "Using a custom value",
            prose: &[
                "Use the shrink-[<value>] syntax to set the flex shrink factor based on a completely custom value:",
                "For CSS variables, you can also use the shrink-(<custom-property>) syntax:",
                "This is just a shorthand for shrink-[var(<custom-property>)] that adds the var() function for you automatically.",
            ],
            snippet: r#"<div class="shrink-[calc(100vw-var(--sidebar))] ...">
<!-- ... -->
</div>
<div class="shrink-(--my-shrink) ...">
<!-- ... -->
</div>"#,
            demo: el(
                "flex w-80 gap-4",
                &[
                    labeled(
                        "h-14 w-64 shrink-0 flex items-center justify-center rounded-lg text-white bg-sky-500",
                        "shrink-[calc(…)] ≈ shrink-0",
                    ),
                    labeled(
                        "h-14 flex-1 flex items-center justify-center rounded-lg text-white bg-sky-500",
                        "flex-1",
                    ),
                ],
            ),
        },
        TwExample {
            title: "Responsive design",
            prose: &[
                "Prefix a flex-shrink utility with a breakpoint variant like md: to only apply the utility at medium screen sizes and above:",
                "Learn more about using variants in the variants documentation.",
            ],
            snippet: r#"<div class="shrink md:shrink-0 ...">
<!-- ... -->
</div>"#,
            demo: el(
                "flex w-80 gap-4",
                &[
                    labeled(
                        "h-14 w-14 flex-none flex items-center justify-center rounded-lg text-white bg-sky-500",
                        "01",
                    ),
                    labeled(
                        "h-14 w-64 shrink md:shrink-0 flex items-center justify-center rounded-lg text-white bg-sky-500",
                        "shrink md:shrink-0",
                    ),
                    labeled(
                        "h-14 w-14 flex-none flex items-center justify-center rounded-lg text-white bg-sky-500",
                        "03",
                    ),
                ],
            ),
        },
    ],
};
