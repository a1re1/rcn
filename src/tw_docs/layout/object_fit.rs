//! <https://tailwindcss.com/docs/object-fit>

use crate::tw_docs::demo::{el, image};

use crate::tw_docs::{TwExample, TwPage, TwSection};

/// <https://tailwindcss.com/docs/object-fit>
pub static OBJECT_FIT: TwPage = TwPage {
    slug: "object-fit",
    title: "Object-fit",
    section: TwSection::Layout,
    description: "Utilities for controlling how a replaced element's content should be resized.",
    reference: &[
        ("object-contain", "object-fit: contain;"),
        ("object-cover", "object-fit: cover;"),
        ("object-fill", "object-fit: fill;"),
        ("object-none", "object-fit: none;"),
        ("object-scale-down", "object-fit: scale-down;"),
    ],
    examples: &[
        TwExample {
            title: "Resizing to cover",
            prose: &[
                "Use the object-cover utility to resize an element's content to cover its container:",
            ],
            snippet: r#"<img class="h-48 w-96 object-cover ..." src="/img/mountains.jpg" />"#,
            demo: el(
                "h-48 w-96 overflow-hidden rounded-lg bg-slate-200",
                &[image(
                    "h-full w-full object-cover",
                    "https://images.unsplash.com/photo-1554629947-334ff61d85dc?w=640&h=360&fit=crop",
                )],
            ),
        },
        TwExample {
            title: "Containing within",
            prose: &[
                "Use the object-contain utility to resize an element's content to stay contained within its container:",
            ],
            snippet: r#"<img class="h-48 w-96 object-contain ..." src="/img/mountains.jpg" />"#,
            demo: el(
                "h-48 w-96 overflow-hidden rounded-lg bg-slate-200",
                &[image(
                    "h-full w-full object-contain",
                    "https://images.unsplash.com/photo-1554629947-334ff61d85dc?w=640&h=360&fit=crop",
                )],
            ),
        },
        TwExample {
            title: "Stretching to fit",
            prose: &[
                "Use the object-fill utility to stretch an element's content to fit its container:",
            ],
            snippet: r#"<img class="h-48 w-96 object-fill ..." src="/img/mountains.jpg" />"#,
            demo: el(
                "h-48 w-96 overflow-hidden rounded-lg bg-slate-200",
                &[image(
                    "h-full w-full object-fill",
                    "https://images.unsplash.com/photo-1554629947-334ff61d85dc?w=640&h=360&fit=crop",
                )],
            ),
        },
        TwExample {
            title: "Scaling down",
            prose: &[
                "Use the object-scale-down utility to display an element's content at its original size but scale it down to fit its container if necessary:",
            ],
            snippet: r#"<img class="h-48 w-96 object-scale-down ..." src="/img/mountains.jpg" />"#,
            demo: el(
                "h-48 w-96 overflow-hidden rounded-lg bg-slate-200",
                &[image(
                    "h-full w-full object-scale-down",
                    "https://images.unsplash.com/photo-1554629947-334ff61d85dc?w=640&h=360&fit=crop",
                )],
            ),
        },
        TwExample {
            title: "Using the original size",
            prose: &[
                "Use the object-none utility to display an element's content at its original size ignoring the container size:",
            ],
            snippet: r#"<img class="h-48 w-96 object-none ..." src="/img/mountains.jpg" />"#,
            demo: el(
                "h-48 w-96 overflow-hidden rounded-lg bg-slate-200",
                &[image(
                    "h-full w-full object-none",
                    "https://images.unsplash.com/photo-1554629947-334ff61d85dc?w=640&h=360&fit=crop",
                )],
            ),
        },
        TwExample {
            title: "Responsive design",
            prose: &[
                "Prefix an object-fit utility with a breakpoint variant like md: to only apply the utility at medium screen sizes and above:",
                "Learn more about using variants in the variants documentation.",
            ],
            snippet: r#"<img class="object-contain md:object-cover" src="/img/mountains.jpg" />"#,
            demo: el(
                "h-48 w-96 overflow-hidden rounded-lg bg-slate-200",
                &[image(
                    "h-full w-full object-contain md:object-cover",
                    "https://images.unsplash.com/photo-1554629947-334ff61d85dc?w=640&h=360&fit=crop",
                )],
            ),
        },
    ],
};
