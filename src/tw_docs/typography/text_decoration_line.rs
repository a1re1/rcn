//! <https://tailwindcss.com/docs/text-decoration-line>

use crate::tw_docs::demo::{el, labeled};

use crate::tw_docs::{Node, TwExample, TwPage, TwSection};

/// <https://tailwindcss.com/docs/text-decoration-line>
pub static TEXT_DECORATION_LINE: TwPage = TwPage {
    slug: "text-decoration-line",
    title: "Text-decoration-line",
    section: TwSection::Typography,
    description: "Utilities for controlling the decoration of text.",
    reference: &[
        ("underline", "text-decoration-line: underline;"),
        ("overline", "text-decoration-line: overline;"),
        ("line-through", "text-decoration-line: line-through;"),
        ("no-underline", "text-decoration-line: none;"),
    ],
    examples: &[
        TwExample {
            title: "Underling text",
            prose: &[
                "Use the underline utility to add an underline to the text of an element:",
                "The quick brown fox jumps over the lazy dog.",
            ],
            snippet: r#"<p class="underline">The quick brown fox...</p>"#,
            demo: el(
                "flex w-full flex-col gap-3",
                &[el(
                    "underline",
                    &[Node::Prose("The quick brown fox jumps over the lazy dog.")],
                )],
            ),
        },
        TwExample {
            title: "Adding an overline to text",
            prose: &[
                "Use the overline utility to add an overline to the text of an element:",
                "The quick brown fox jumps over the lazy dog.",
            ],
            snippet: r#"<p class="overline">The quick brown fox...</p>"#,
            demo: el(
                "flex w-full flex-col gap-3",
                &[
                    el(
                        "no-underline",
                        &[Node::Prose("The quick brown fox jumps over the lazy dog.")],
                    ),
                    labeled("text-xs text-slate-500", "overline has no equivalent"),
                ],
            ),
        },
        TwExample {
            title: "Adding a line through text",
            prose: &[
                "Use the line-through utility to add a line through the text of an element:",
                "The quick brown fox jumps over the lazy dog.",
            ],
            snippet: r#"<p class="line-through">The quick brown fox...</p>"#,
            demo: el(
                "flex w-full flex-col gap-3",
                &[el(
                    "line-through",
                    &[Node::Prose("The quick brown fox jumps over the lazy dog.")],
                )],
            ),
        },
        TwExample {
            title: "Removing a line from text",
            prose: &[
                "Use the no-underline utility to remove a line from the text of an element:",
                "The quick brown fox jumps over the lazy dog.",
            ],
            snippet: r#"<p class="no-underline">The quick brown fox...</p>"#,
            demo: el(
                "flex w-full flex-col gap-3",
                &[el(
                    "no-underline",
                    &[Node::Prose("The quick brown fox jumps over the lazy dog.")],
                )],
            ),
        },
        TwExample {
            title: "Applying on hover",
            prose: &[
                "Prefix a text-decoration-line utility with a variant like hover:* to only apply the utility in that state:",
                "Hover over the text to see the expected behavior",
                "Learn more about using variants in the variants documentation.",
            ],
            snippet: r#"<p>The <a href="..." class="no-underline hover:underline ...">quick brown fox</a> jumps over the lazy dog.</p>"#,
            demo: el(
                "flex flex-wrap gap-1",
                &[
                    Node::Prose("The"),
                    el(
                        "no-underline hover:underline",
                        &[Node::Prose("quick brown fox")],
                    ),
                    Node::Prose("jumps over the lazy dog."),
                ],
            ),
        },
        TwExample {
            title: "Responsive design",
            prose: &[
                "Prefix a text-decoration-line utility with a breakpoint variant like md: to only apply the utility at medium screen sizes and above:",
                "Learn more about using variants in the variants documentation.",
            ],
            snippet: r#"<a class="no-underline md:underline ..." href="...">
<!-- ... -->
</a>"#,
            demo: el(
                "flex w-full flex-col gap-3",
                &[el(
                    "no-underline md:underline",
                    &[Node::Prose("The quick brown fox jumps over the lazy dog.")],
                )],
            ),
        },
    ],
};
