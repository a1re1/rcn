//! <https://tailwindcss.com/docs/flex-grow>

use crate::tw_docs::demo::{el, labeled};

use crate::tw_docs::{TwExample, TwPage, TwSection};

/// <https://tailwindcss.com/docs/flex-grow>
pub static FLEX_GROW: TwPage = TwPage {
    slug: "flex-grow",
    title: "Flex-grow",
    section: TwSection::FlexboxGrid,
    description: "Utilities for controlling how flex items grow.",
    reference: &[
        ("grow", "flex-grow: 1;"),
        ("grow-<number>", "flex-grow: <number>;"),
        ("grow-[<value>]", "flex-grow: <value>;"),
        (
            "grow-(<custom-property>)",
            "flex-grow: var(<custom-property>);",
        ),
    ],
    examples: &[
        TwExample {
            title: "Allowing items to grow",
            prose: &["Use grow to allow a flex item to grow to fill any available space:"],
            snippet: r#"<div class="flex ...">
<div class="size-14 flex-none ...">01</div>
<div class="size-14 grow ...">02</div>
<div class="size-14 flex-none ...">03</div>
</div>"#,
            demo: el(
                "flex w-full gap-4",
                &[
                    labeled(
                        "size-14 flex-none flex items-center justify-center rounded-lg text-white bg-indigo-500",
                        "01",
                    ),
                    labeled(
                        "size-14 grow flex items-center justify-center rounded-lg text-white bg-indigo-500",
                        "02",
                    ),
                    labeled(
                        "size-14 flex-none flex items-center justify-center rounded-lg text-white bg-indigo-500",
                        "03",
                    ),
                ],
            ),
        },
        TwExample {
            title: "Growing items based on factor",
            prose: &[
                "Use grow-<number> utilities like grow-3 to make flex items grow proportionally based on their growth factor, allowing them to fill the available space relative to each other:",
            ],
            snippet: r#"<div class="flex ...">
<div class="size-14 grow-3 ...">01</div>
<div class="size-14 grow-7 ...">02</div>
<div class="size-14 grow-3 ...">03</div>
</div>"#,
            demo: el(
                "flex w-full gap-4",
                &[
                    labeled(
                        "size-14 grow-3 flex items-center justify-center rounded-lg text-white bg-indigo-500",
                        "01",
                    ),
                    labeled(
                        "size-14 grow-7 flex items-center justify-center rounded-lg text-white bg-indigo-500",
                        "02",
                    ),
                    labeled(
                        "size-14 grow-3 flex items-center justify-center rounded-lg text-white bg-indigo-500",
                        "03",
                    ),
                ],
            ),
        },
        TwExample {
            title: "Preventing items from growing",
            prose: &["Use grow-0 to prevent a flex item from growing:"],
            snippet: r#"<div class="flex ...">
<div class="size-14 grow ...">01</div>
<div class="size-14 grow-0 ...">02</div>
<div class="size-14 grow ...">03</div>
</div>"#,
            demo: el(
                "flex w-full gap-4",
                &[
                    labeled(
                        "size-14 grow flex items-center justify-center rounded-lg text-white bg-indigo-500",
                        "01",
                    ),
                    labeled(
                        "size-14 grow-0 flex items-center justify-center rounded-lg text-white bg-indigo-500",
                        "02",
                    ),
                    labeled(
                        "size-14 grow flex items-center justify-center rounded-lg text-white bg-indigo-500",
                        "03",
                    ),
                ],
            ),
        },
        TwExample {
            title: "Using a custom value",
            prose: &[
                "Use the grow-[<value>] syntax to set the flex grow factor based on a completely custom value:",
                "For CSS variables, you can also use the grow-(<custom-property>) syntax:",
                "This is just a shorthand for grow-[var(<custom-property>)] that adds the var() function for you automatically.",
            ],
            snippet: r#"<div class="grow-[25vw] ...">
<!-- ... -->
</div>
<div class="grow-(--my-grow) ...">
<!-- ... -->
</div>"#,
            demo: el(
                "flex w-full gap-4",
                &[
                    labeled(
                        "size-14 grow-2 flex items-center justify-center rounded-lg text-white bg-indigo-500",
                        "grow-[25vw] ≈ grow-2",
                    ),
                    labeled(
                        "size-14 grow flex items-center justify-center rounded-lg text-white bg-indigo-500",
                        "grow",
                    ),
                ],
            ),
        },
        TwExample {
            title: "Responsive design",
            prose: &[
                "Prefix a flex-grow utility with a breakpoint variant like md: to only apply the utility at medium screen sizes and above:",
                "Learn more about using variants in the variants documentation.",
            ],
            snippet: r#"<div class="grow md:grow-0 ...">
<!-- ... -->
</div>"#,
            demo: el(
                "flex w-full gap-4",
                &[
                    labeled(
                        "size-14 flex-none flex items-center justify-center rounded-lg text-white bg-indigo-500",
                        "01",
                    ),
                    labeled(
                        "size-14 grow md:grow-0 flex items-center justify-center rounded-lg text-white bg-indigo-500",
                        "grow md:grow-0",
                    ),
                    labeled(
                        "size-14 flex-none flex items-center justify-center rounded-lg text-white bg-indigo-500",
                        "03",
                    ),
                ],
            ),
        },
    ],
};
