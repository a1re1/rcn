//! <https://tailwindcss.com/docs/flex-wrap>

use crate::tw_docs::demo::{el, labeled};

use crate::tw_docs::{TwExample, TwPage, TwSection};

/// <https://tailwindcss.com/docs/flex-wrap>
pub static FLEX_WRAP: TwPage = TwPage {
    slug: "flex-wrap",
    title: "Flex-wrap",
    section: TwSection::FlexboxGrid,
    description: "Utilities for controlling how flex items wrap.",
    reference: &[
        ("flex-nowrap", "flex-wrap: nowrap;"),
        ("flex-wrap", "flex-wrap: wrap;"),
        ("flex-wrap-reverse", "flex-wrap: wrap-reverse;"),
    ],
    examples: &[
        TwExample {
            title: "Don't wrap",
            prose: &[
                "Use flex-nowrap to prevent flex items from wrapping, causing inflexible items to overflow the container if necessary:",
            ],
            snippet: r#"<div class="flex flex-nowrap">
<div>01</div>
<div>02</div>
<div>03</div>
</div>"#,
            demo: el(
                "flex w-80 flex-nowrap gap-4 rounded-lg bg-indigo-300 p-2",
                &[
                    labeled(
                        "h-14 w-40 shrink-0 flex items-center justify-center rounded-lg text-white bg-indigo-500",
                        "01",
                    ),
                    labeled(
                        "h-14 w-40 shrink-0 flex items-center justify-center rounded-lg text-white bg-indigo-500",
                        "02",
                    ),
                    labeled(
                        "h-14 w-40 shrink-0 flex items-center justify-center rounded-lg text-white bg-indigo-500",
                        "03",
                    ),
                ],
            ),
        },
        TwExample {
            title: "Wrap normally",
            prose: &["Use flex-wrap to allow flex items to wrap:"],
            snippet: r#"<div class="flex flex-wrap">
<div>01</div>
<div>02</div>
<div>03</div>
</div>"#,
            demo: el(
                "flex w-80 flex-wrap gap-4 rounded-lg bg-indigo-300 p-2",
                &[
                    labeled(
                        "h-14 w-40 flex items-center justify-center rounded-lg text-white bg-indigo-500",
                        "01",
                    ),
                    labeled(
                        "h-14 w-40 flex items-center justify-center rounded-lg text-white bg-indigo-500",
                        "02",
                    ),
                    labeled(
                        "h-14 w-40 flex items-center justify-center rounded-lg text-white bg-indigo-500",
                        "03",
                    ),
                ],
            ),
        },
        TwExample {
            title: "Wrap reversed",
            prose: &["Use flex-wrap-reverse to wrap flex items in the reverse direction:"],
            snippet: r#"<div class="flex flex-wrap-reverse">
<div>01</div>
<div>02</div>
<div>03</div>
</div>"#,
            demo: el(
                "flex w-80 flex-wrap-reverse gap-4 rounded-lg bg-indigo-300 p-2",
                &[
                    labeled(
                        "h-14 w-40 flex items-center justify-center rounded-lg text-white bg-indigo-500",
                        "01",
                    ),
                    labeled(
                        "h-14 w-40 flex items-center justify-center rounded-lg text-white bg-indigo-500",
                        "02",
                    ),
                    labeled(
                        "h-14 w-40 flex items-center justify-center rounded-lg text-white bg-indigo-500",
                        "03",
                    ),
                ],
            ),
        },
        TwExample {
            title: "Responsive design",
            prose: &[
                "Prefix a flex-wrap utility with a breakpoint variant like md: to only apply the utility at medium screen sizes and above:",
                "Learn more about using variants in the variants documentation.",
            ],
            snippet: r#"<div class="flex flex-wrap md:flex-wrap-reverse ...">
<!-- ... -->
</div>"#,
            demo: el(
                "flex w-80 flex-wrap md:flex-wrap-reverse gap-4 rounded-lg bg-indigo-300 p-2",
                &[
                    labeled(
                        "h-14 w-40 flex items-center justify-center rounded-lg text-white bg-indigo-500",
                        "01",
                    ),
                    labeled(
                        "h-14 w-40 flex items-center justify-center rounded-lg text-white bg-indigo-500",
                        "02",
                    ),
                    labeled(
                        "h-14 w-40 flex items-center justify-center rounded-lg text-white bg-indigo-500",
                        "03",
                    ),
                ],
            ),
        },
    ],
};
