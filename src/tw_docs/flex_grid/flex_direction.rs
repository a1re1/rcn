//! <https://tailwindcss.com/docs/flex-direction>

use crate::tw_docs::demo::{el, labeled};

use crate::tw_docs::{TwExample, TwPage, TwSection};

/// <https://tailwindcss.com/docs/flex-direction>
pub static FLEX_DIRECTION: TwPage = TwPage {
    slug: "flex-direction",
    title: "Flex-direction",
    section: TwSection::FlexboxGrid,
    description: "Utilities for controlling the direction of flex items.",
    reference: &[
        ("flex-row", "flex-direction: row;"),
        ("flex-row-reverse", "flex-direction: row-reverse;"),
        ("flex-col", "flex-direction: column;"),
        ("flex-col-reverse", "flex-direction: column-reverse;"),
    ],
    examples: &[
        TwExample {
            title: "Row",
            prose: &[
                "Use flex-row to position flex items horizontally in the same direction as text:",
            ],
            snippet: r#"<div class="flex flex-row ...">
<div>01</div>
<div>02</div>
<div>03</div>
</div>"#,
            demo: el(
                "flex flex-row gap-4",
                &[
                    labeled(
                        "size-14 flex items-center justify-center rounded-lg text-white bg-violet-500",
                        "01",
                    ),
                    labeled(
                        "size-14 flex items-center justify-center rounded-lg text-white bg-violet-500",
                        "02",
                    ),
                    labeled(
                        "size-14 flex items-center justify-center rounded-lg text-white bg-violet-500",
                        "03",
                    ),
                ],
            ),
        },
        TwExample {
            title: "Row reversed",
            prose: &[
                "Use flex-row-reverse to position flex items horizontally in the opposite direction:",
            ],
            snippet: r#"<div class="flex flex-row-reverse ...">
<div>01</div>
<div>02</div>
<div>03</div>
</div>"#,
            demo: el(
                "flex flex-row-reverse gap-4",
                &[
                    labeled(
                        "size-14 flex items-center justify-center rounded-lg text-white bg-violet-500",
                        "01",
                    ),
                    labeled(
                        "size-14 flex items-center justify-center rounded-lg text-white bg-violet-500",
                        "02",
                    ),
                    labeled(
                        "size-14 flex items-center justify-center rounded-lg text-white bg-violet-500",
                        "03",
                    ),
                ],
            ),
        },
        TwExample {
            title: "Column",
            prose: &["Use flex-col to position flex items vertically:"],
            snippet: r#"<div class="flex flex-col ...">
<div>01</div>
<div>02</div>
<div>03</div>
</div>"#,
            demo: el(
                "flex flex-col gap-4",
                &[
                    labeled(
                        "size-14 flex items-center justify-center rounded-lg text-white bg-violet-500",
                        "01",
                    ),
                    labeled(
                        "size-14 flex items-center justify-center rounded-lg text-white bg-violet-500",
                        "02",
                    ),
                    labeled(
                        "size-14 flex items-center justify-center rounded-lg text-white bg-violet-500",
                        "03",
                    ),
                ],
            ),
        },
        TwExample {
            title: "Column reversed",
            prose: &[
                "Use flex-col-reverse to position flex items vertically in the opposite direction:",
            ],
            snippet: r#"<div class="flex flex-col-reverse ...">
<div>01</div>
<div>02</div>
<div>03</div>
</div>"#,
            demo: el(
                "flex flex-col-reverse gap-4",
                &[
                    labeled(
                        "size-14 flex items-center justify-center rounded-lg text-white bg-violet-500",
                        "01",
                    ),
                    labeled(
                        "size-14 flex items-center justify-center rounded-lg text-white bg-violet-500",
                        "02",
                    ),
                    labeled(
                        "size-14 flex items-center justify-center rounded-lg text-white bg-violet-500",
                        "03",
                    ),
                ],
            ),
        },
        TwExample {
            title: "Responsive design",
            prose: &[
                "Prefix a flex-direction utility with a breakpoint variant like md: to only apply the utility at medium screen sizes and above:",
                "Learn more about using variants in the variants documentation.",
            ],
            snippet: r#"<div class="flex flex-col md:flex-row ...">
<!-- ... -->
</div>"#,
            demo: el(
                "flex flex-col md:flex-row gap-4",
                &[
                    labeled(
                        "size-14 flex items-center justify-center rounded-lg text-white bg-violet-500",
                        "01",
                    ),
                    labeled(
                        "size-14 flex items-center justify-center rounded-lg text-white bg-violet-500",
                        "02",
                    ),
                    labeled(
                        "size-14 flex items-center justify-center rounded-lg text-white bg-violet-500",
                        "03",
                    ),
                ],
            ),
        },
    ],
};
