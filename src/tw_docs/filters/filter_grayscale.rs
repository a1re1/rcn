//! <https://tailwindcss.com/docs/filter-grayscale>

use crate::tw_docs::demo::{el, image, labeled};
use crate::tw_docs::{TwExample, TwPage, TwSection};

/// <https://tailwindcss.com/docs/filter-grayscale>
pub static FILTER_GRAYSCALE: TwPage = TwPage {
    slug: "filter-grayscale",
    title: "Filter: grayscale()",
    section: TwSection::Filters,
    description: "Utilities for applying grayscale filters to an element.",
    reference: &[
        ("grayscale", "filter: grayscale(100%);"),
        ("grayscale-<number>", "filter: grayscale(<number>%);"),
        (
            "grayscale-(<custom-property>)",
            "filter: grayscale(var(<custom-property>));",
        ),
        ("grayscale-[<value>]", "filter: grayscale(<value>);"),
    ],
    examples: &[
        TwExample {
            title: "Basic example",
            prose: &[
                "Use utilities like grayscale and grayscale-75 to control the amount of grayscale effect applied to an element:",
                "grayscale-0",
                "grayscale-25",
                "grayscale-50",
                "grayscale",
            ],
            snippet: r#"<img class="grayscale-0 ..." src="/img/mountains.jpg" />
<img class="grayscale-25 ..." src="/img/mountains.jpg" />
<img class="grayscale-50 ..." src="/img/mountains.jpg" />
<img class="grayscale ..." src="/img/mountains.jpg" />"#,
            demo: el(
                "flex flex-col gap-3",
                &[
                    el(
                        "flex flex-wrap items-center gap-4",
                        &[
                            image(
                                "h-24 w-40 rounded-lg object-cover",
                                "https://images.unsplash.com/photo-1554629947-334ff61d85dc?w=640&h=360&fit=crop",
                            ),
                            image(
                                "h-24 w-40 rounded-lg object-cover grayscale",
                                "https://images.unsplash.com/photo-1554629947-334ff61d85dc?w=640&h=360&fit=crop",
                            ),
                        ],
                    ),
                    labeled(
                        "text-xs text-slate-500",
                        "grayscale-0/25/50 have no equivalent — only full grayscale; the first image is unfiltered",
                    ),
                ],
            ),
        },
        TwExample {
            title: "Using a custom value",
            prose: &[
                "Use the grayscale-[<value>] syntax to set the grayscale based on a completely custom value:",
                "For CSS variables, you can also use the grayscale-(<custom-property>) syntax:",
                "This is just a shorthand for grayscale-[var(<custom-property>)] that adds the var() function for you automatically.",
            ],
            snippet: r#"<img class="grayscale-[0.5] ..." src="/img/mountains.jpg" />
<img class="grayscale-(--my-grayscale) ..." src="/img/mountains.jpg" />"#,
            demo: el(
                "flex flex-col gap-3",
                &[
                    el(
                        "flex flex-wrap items-center gap-4",
                        &[image(
                            "h-24 w-40 rounded-lg object-cover grayscale",
                            "https://images.unsplash.com/photo-1554629947-334ff61d85dc?w=640&h=360&fit=crop",
                        )],
                    ),
                    labeled(
                        "text-xs text-slate-500",
                        "grayscale-[0.5] has no equivalent — full grayscale shown",
                    ),
                ],
            ),
        },
        TwExample {
            title: "Responsive design",
            prose: &[
                "Prefix a filter: grayscale() utility with a breakpoint variant like md: to only apply the utility at medium screen sizes and above:",
                "Learn more about using variants in the variants documentation.",
            ],
            snippet: r#"<img class="grayscale md:grayscale-0 ..." src="/img/mountains.jpg" />"#,
            demo: el(
                "flex flex-col gap-3",
                &[
                    el(
                        "flex flex-wrap items-center gap-4",
                        &[image(
                            "h-24 w-40 rounded-lg object-cover grayscale",
                            "https://images.unsplash.com/photo-1554629947-334ff61d85dc?w=640&h=360&fit=crop",
                        )],
                    ),
                    labeled(
                        "text-xs text-slate-500",
                        "md:grayscale-0 has no equivalent — grayscale stays on",
                    ),
                ],
            ),
        },
    ],
};
