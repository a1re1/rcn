//! <https://tailwindcss.com/docs/text-decoration-thickness>

use crate::tw_docs::demo::{el, labeled};

use crate::tw_docs::{Node, TwExample, TwPage, TwSection};

/// <https://tailwindcss.com/docs/text-decoration-thickness>
pub static TEXT_DECORATION_THICKNESS: TwPage = TwPage {
    slug: "text-decoration-thickness",
    title: "Text-decoration-thickness",
    section: TwSection::Typography,
    description: "Utilities for controlling the thickness of text decorations.",
    reference: &[
        (
            "decoration-<number>",
            "text-decoration-thickness: <number>px;",
        ),
        (
            "decoration-from-font",
            "text-decoration-thickness: from-font;",
        ),
        ("decoration-auto", "text-decoration-thickness: auto;"),
        (
            "decoration-(length:<custom-property>)",
            "text-decoration-thickness: var(<custom-property>);",
        ),
        (
            "decoration-[<value>]",
            "text-decoration-thickness: <value>;",
        ),
    ],
    examples: &[
        TwExample {
            title: "Basic example",
            prose: &[
                "Use decoration-<number> utilities like decoration-2 and decoration-4 to change the text decoration thickness of an element:",
                "The quick brown fox jumps over the lazy dog.",
                "The quick brown fox jumps over the lazy dog.",
                "The quick brown fox jumps over the lazy dog.",
            ],
            snippet: r#"<p class="underline decoration-1">The quick brown fox...</p>
<p class="underline decoration-2">The quick brown fox...</p>
<p class="underline decoration-4">The quick brown fox...</p>"#,
            demo: el(
                "flex w-full flex-col gap-3",
                &[
                    el(
                        "underline decoration-1",
                        &[Node::Prose("The quick brown fox jumps over the lazy dog.")],
                    ),
                    el(
                        "underline decoration-2",
                        &[Node::Prose("The quick brown fox jumps over the lazy dog.")],
                    ),
                    el(
                        "underline decoration-4",
                        &[Node::Prose("The quick brown fox jumps over the lazy dog.")],
                    ),
                ],
            ),
        },
        TwExample {
            title: "Using a custom value",
            prose: &[
                "Use the decoration-[<value>] syntax to set the text decoration thickness based on a completely custom value:",
                "For CSS variables, you can also use the decoration-(length:<custom-property>) syntax:",
                "This is just a shorthand for decoration-[length:var(<custom-property>)] that adds the var() function for you automatically.",
            ],
            snippet: r#"<p class="decoration-[0.25rem] ...">  Lorem ipsum dolor sit amet...</p>
<p class="decoration-(length:--my-decoration-thickness) ...">  Lorem ipsum dolor sit amet...</p>"#,
            demo: el(
                "flex w-full flex-col gap-3",
                &[
                    labeled(
                        "text-xs text-slate-500",
                        "decoration-[0.25rem] ≈ decoration-4",
                    ),
                    el(
                        "underline decoration-4",
                        &[Node::Prose(
                            "Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod tempor incididunt ut labore et dolore magna aliqua.",
                        )],
                    ),
                ],
            ),
        },
        TwExample {
            title: "Responsive design",
            prose: &[
                "Prefix a text-decoration-thickness utility with a breakpoint variant like md: to only apply the utility at medium screen sizes and above:",
                "Learn more about using variants in the variants documentation.",
            ],
            snippet: r#"<p class="underline md:decoration-4 ...">  Lorem ipsum dolor sit amet...</p>"#,
            demo: el(
                "flex w-full flex-col gap-3",
                &[el(
                    "underline md:decoration-4",
                    &[Node::Prose(
                        "Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod tempor incididunt ut labore et dolore magna aliqua.",
                    )],
                )],
            ),
        },
    ],
};
