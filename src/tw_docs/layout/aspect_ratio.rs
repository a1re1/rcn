//! <https://tailwindcss.com/docs/aspect-ratio>

use crate::tw_docs::demo::{el, image, labeled};

use crate::tw_docs::{TwExample, TwPage, TwSection};

/// <https://tailwindcss.com/docs/aspect-ratio>
pub static ASPECT_RATIO: TwPage = TwPage {
    slug: "aspect-ratio",
    title: "Aspect-ratio",
    section: TwSection::Layout,
    description: "Utilities for controlling the aspect ratio of an element.",
    reference: &[
        ("aspect-<ratio>", "aspect-ratio: <ratio>;"),
        ("aspect-square", "aspect-ratio: 1 / 1;"),
        (
            "aspect-video",
            "aspect-ratio: var(--aspect-video); /* 16 / 9 */",
        ),
        ("aspect-auto", "aspect-ratio: auto;"),
        (
            "aspect-(<custom-property>)",
            "aspect-ratio: var(<custom-property>);",
        ),
        ("aspect-[<value>]", "aspect-ratio: <value>;"),
    ],
    examples: &[
        TwExample {
            title: "Basic example",
            prose: &[
                "Use aspect-<ratio> utilities like aspect-3/2 to give an element a specific aspect ratio:",
                "Resize the example to see the expected behavior",
            ],
            snippet: r#"<img class="aspect-3/2 object-cover ..." src="/img/villas.jpg" />"#,
            demo: el(
                "flex flex-col gap-2",
                &[
                    image(
                        "aspect-video w-96 rounded-lg object-cover",
                        "https://images.unsplash.com/photo-1554629947-334ff61d85dc?w=640&h=360&fit=crop",
                    ),
                    labeled(
                        "text-xs text-slate-500",
                        "aspect-3/2 ≈ aspect-video (custom ratios have no equivalent)",
                    ),
                ],
            ),
        },
        TwExample {
            title: "Using a video aspect ratio",
            prose: &[
                "Use the aspect-video utility to give a video element a 16 / 9 aspect ratio:",
                "Resize the example to see the expected behavior",
            ],
            snippet: r#"<iframe class="aspect-video ..." src="https://www.youtube.com/embed/dQw4w9WgXcQ">
</iframe>"#,
            demo: el(
                "flex flex-col gap-2",
                &[image(
                    "aspect-video w-96 rounded-lg object-cover",
                    "https://images.unsplash.com/photo-1554629947-334ff61d85dc?w=640&h=360&fit=crop",
                )],
            ),
        },
        TwExample {
            title: "Using a custom value",
            prose: &[
                "Use the aspect-[<value>] syntax to set the aspect ratio based on a completely custom value:",
                "For CSS variables, you can also use the aspect-(<custom-property>) syntax:",
                "This is just a shorthand for aspect-[var(<custom-property>)] that adds the var() function for you automatically.",
            ],
            snippet: r#"<img class="aspect-[calc(4*3+1)/3] ..." src="/img/villas.jpg" />
<img class="aspect-(--my-aspect-ratio) ..." src="/img/villas.jpg" />"#,
            demo: el(
                "flex flex-col gap-2",
                &[
                    labeled(
                        "aspect-square w-32 flex items-center justify-center rounded-lg text-white bg-sky-500",
                        "aspect-square",
                    ),
                    labeled(
                        "text-xs text-slate-500",
                        "aspect-[calc(4*3+1)/3] has no equivalent — aspect-square shown",
                    ),
                ],
            ),
        },
        TwExample {
            title: "Responsive design",
            prose: &[
                "Prefix an aspect-ratio utility with a breakpoint variant like md: to only apply the utility at medium screen sizes and above:",
                "Learn more about using variants in the variants documentation.",
            ],
            snippet: r#"<iframe class="aspect-video md:aspect-square ..." src="https://www.youtube.com/embed/dQw4w9WgXcQ">
</iframe>"#,
            demo: el(
                "flex flex-col gap-2",
                &[image(
                    "aspect-video md:aspect-square w-64 rounded-lg object-cover",
                    "https://images.unsplash.com/photo-1554629947-334ff61d85dc?w=640&h=360&fit=crop",
                )],
            ),
        },
    ],
};
