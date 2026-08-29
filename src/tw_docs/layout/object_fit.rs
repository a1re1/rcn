//! <https://tailwindcss.com/docs/object-fit>

use crate::tw_docs::demo::{TODO_DEMO, el, image, labeled};

use crate::tw_docs::{Node, TwExample, TwPage, TwSection};

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
            demo: TODO_DEMO,
        },
        TwExample {
            title: "Containing within",
            prose: &[
                "Use the object-contain utility to resize an element's content to stay contained within its container:",
            ],
            snippet: r#"<img class="h-48 w-96 object-contain ..." src="/img/mountains.jpg" />"#,
            demo: TODO_DEMO,
        },
        TwExample {
            title: "Stretching to fit",
            prose: &[
                "Use the object-fill utility to stretch an element's content to fit its container:",
            ],
            snippet: r#"<img class="h-48 w-96 object-fill ..." src="/img/mountains.jpg" />"#,
            demo: TODO_DEMO,
        },
        TwExample {
            title: "Scaling down",
            prose: &[
                "Use the object-scale-down utility to display an element's content at its original size but scale it down to fit its container if necessary:",
            ],
            snippet: r#"<img class="h-48 w-96 object-scale-down ..." src="/img/mountains.jpg" />"#,
            demo: TODO_DEMO,
        },
        TwExample {
            title: "Using the original size",
            prose: &[
                "Use the object-none utility to display an element's content at its original size ignoring the container size:",
            ],
            snippet: r#"<img class="h-48 w-96 object-none ..." src="/img/mountains.jpg" />"#,
            demo: TODO_DEMO,
        },
        TwExample {
            title: "Responsive design",
            prose: &[
                "Prefix an object-fit utility with a breakpoint variant like md: to only apply the utility at medium screen sizes and above:",
                "Learn more about using variants in the variants documentation.",
            ],
            snippet: r#"<img class="object-contain md:object-cover" src="/img/mountains.jpg" />"#,
            demo: TODO_DEMO,
        },
    ],
};
