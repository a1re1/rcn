//! <https://tailwindcss.com/docs/flex>

use crate::tw_docs::demo::{el, labeled};

use crate::tw_docs::{TwExample, TwPage, TwSection};

/// <https://tailwindcss.com/docs/flex>
pub static FLEX: TwPage = TwPage {
    slug: "flex",
    title: "Flex",
    section: TwSection::FlexboxGrid,
    description: "Utilities for controlling how flex items both grow and shrink.",
    reference: &[
        ("flex-<number>", "flex: <number>;"),
        ("flex-<fraction>", "flex: calc(<fraction> * 100%);"),
        ("flex-auto", "flex: auto;"),
        ("flex-initial", "flex: 0 auto;"),
        ("flex-none", "flex: none;"),
        ("flex-(<custom-property>)", "flex: var(<custom-property>);"),
        ("flex-[<value>]", "flex: <value>;"),
    ],
    examples: &[
        TwExample {
            title: "Basic example",
            prose: &[
                "Use flex-<number> utilities like flex-1 to allow a flex item to grow and shrink as needed, ignoring its initial size:",
            ],
            snippet: r#"<div class="flex">
<div class="w-14 flex-none ...">01</div>
<div class="w-64 flex-1 ...">02</div>
<div class="w-32 flex-1 ...">03</div>
</div>"#,
            demo: el(
                "flex w-full gap-4",
                &[
                    labeled(
                        "h-14 w-14 flex-none flex items-center justify-center rounded-lg text-white bg-violet-500",
                        "01",
                    ),
                    labeled(
                        "h-14 w-64 flex-1 flex items-center justify-center rounded-lg text-white bg-violet-500",
                        "02",
                    ),
                    labeled(
                        "h-14 w-32 flex-1 flex items-center justify-center rounded-lg text-white bg-violet-500",
                        "03",
                    ),
                ],
            ),
        },
        TwExample {
            title: "Initial",
            prose: &[
                "Use flex-initial to allow a flex item to shrink but not grow, taking into account its initial size:",
            ],
            snippet: r#"<div class="flex">
<div class="w-14 flex-none ...">01</div>
<div class="w-64 flex-initial ...">02</div>
<div class="w-32 flex-initial ...">03</div>
</div>"#,
            demo: el(
                "flex w-full gap-4",
                &[
                    labeled(
                        "h-14 w-14 flex-none flex items-center justify-center rounded-lg text-white bg-violet-500",
                        "01",
                    ),
                    labeled(
                        "h-14 w-64 flex-initial flex items-center justify-center rounded-lg text-white bg-violet-500",
                        "02",
                    ),
                    labeled(
                        "h-14 w-32 flex-initial flex items-center justify-center rounded-lg text-white bg-violet-500",
                        "03",
                    ),
                ],
            ),
        },
        TwExample {
            title: "Auto",
            prose: &[
                "Use flex-auto to allow a flex item to grow and shrink, taking into account its initial size:",
            ],
            snippet: r#"<div class="flex ...">
<div class="w-14 flex-none ...">01</div>
<div class="w-64 flex-auto ...">02</div>
<div class="w-32 flex-auto ...">03</div>
</div>"#,
            demo: el(
                "flex w-full gap-4",
                &[
                    labeled(
                        "h-14 w-14 flex-none flex items-center justify-center rounded-lg text-white bg-violet-500",
                        "01",
                    ),
                    labeled(
                        "h-14 w-64 flex-auto flex items-center justify-center rounded-lg text-white bg-violet-500",
                        "02",
                    ),
                    labeled(
                        "h-14 w-32 flex-auto flex items-center justify-center rounded-lg text-white bg-violet-500",
                        "03",
                    ),
                ],
            ),
        },
        TwExample {
            title: "None",
            prose: &["Use flex-none to prevent a flex item from growing or shrinking:"],
            snippet: r#"<div class="flex ...">
<div class="w-14 flex-none ...">01</div>
<div class="w-32 flex-none ...">02</div>
<div class="flex-1 ...">03</div>
</div>"#,
            demo: el(
                "flex w-full gap-4",
                &[
                    labeled(
                        "h-14 w-14 flex-none flex items-center justify-center rounded-lg text-white bg-violet-500",
                        "01",
                    ),
                    labeled(
                        "h-14 w-32 flex-none flex items-center justify-center rounded-lg text-white bg-violet-500",
                        "02",
                    ),
                    labeled(
                        "h-14 flex-1 flex items-center justify-center rounded-lg text-white bg-violet-500",
                        "03",
                    ),
                ],
            ),
        },
        TwExample {
            title: "Using a custom value",
            prose: &[
                "Use the flex-[<value>] syntax to set the flex shorthand property based on a completely custom value:",
                "For CSS variables, you can also use the flex-(<custom-property>) syntax:",
                "This is just a shorthand for flex-[var(<custom-property>)] that adds the var() function for you automatically.",
            ],
            snippet: r#"<div class="flex-[3_1_auto] ...">
<!-- ... -->
</div>
<div class="flex-(--my-flex) ...">
<!-- ... -->
</div>"#,
            demo: el(
                "flex w-full gap-4",
                &[
                    labeled(
                        "h-14 flex-3 flex items-center justify-center rounded-lg text-white bg-violet-500",
                        "flex-[3_1_auto] ≈ flex-3",
                    ),
                    labeled(
                        "h-14 flex-1 flex items-center justify-center rounded-lg text-white bg-violet-500",
                        "flex-1",
                    ),
                ],
            ),
        },
        TwExample {
            title: "Responsive design",
            prose: &[
                "Prefix a flex utility with a breakpoint variant like md: to only apply the utility at medium screen sizes and above:",
                "Learn more about using variants in the variants documentation.",
            ],
            snippet: r#"<div class="flex-none md:flex-1 ...">
<!-- ... -->
</div>"#,
            demo: el(
                "flex w-full gap-4",
                &[
                    labeled(
                        "h-14 w-14 flex-none md:flex-1 flex items-center justify-center rounded-lg text-white bg-violet-500",
                        "flex-none md:flex-1",
                    ),
                    labeled(
                        "h-14 flex-1 flex items-center justify-center rounded-lg text-white bg-violet-500",
                        "flex-1",
                    ),
                ],
            ),
        },
    ],
};
