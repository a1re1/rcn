//! <https://tailwindcss.com/docs/font-style>

use crate::tw_docs::demo::el;

use crate::tw_docs::{Node, TwExample, TwPage, TwSection};

/// <https://tailwindcss.com/docs/font-style>
pub static FONT_STYLE: TwPage = TwPage {
    slug: "font-style",
    title: "Font-style",
    section: TwSection::Typography,
    description: "Utilities for controlling the style of text.",
    reference: &[
        ("italic", "font-style: italic;"),
        ("not-italic", "font-style: normal;"),
    ],
    examples: &[
        TwExample {
            title: "Italicizing text",
            prose: &[
                "Use the italic utility to make text italic:",
                "The quick brown fox jumps over the lazy dog.",
            ],
            snippet: r#"<p class="italic ...">The quick brown fox ...</p>"#,
            demo: el(
                "flex w-full flex-col gap-3",
                &[el(
                    "italic",
                    &[Node::Prose("The quick brown fox jumps over the lazy dog.")],
                )],
            ),
        },
        TwExample {
            title: "Displaying text normally",
            prose: &[
                "Use the not-italic utility to display text normally:",
                "The quick brown fox jumps over the lazy dog.",
            ],
            snippet: r#"<p class="not-italic ...">The quick brown fox ...</p>"#,
            demo: el(
                "flex w-full flex-col gap-3",
                &[el(
                    "not-italic",
                    &[Node::Prose("The quick brown fox jumps over the lazy dog.")],
                )],
            ),
        },
        TwExample {
            title: "Responsive design",
            prose: &[
                "Prefix a font-style utility with a breakpoint variant like md: to only apply the utility at medium screen sizes and above:",
                "Learn more about using variants in the variants documentation.",
            ],
            snippet: r#"<p class="italic md:not-italic ...">  Lorem ipsum dolor sit amet...</p>"#,
            demo: el(
                "flex w-full flex-col gap-3",
                &[el(
                    "italic md:not-italic",
                    &[Node::Prose(
                        "Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod tempor incididunt ut labore et dolore magna aliqua.",
                    )],
                )],
            ),
        },
    ],
};
