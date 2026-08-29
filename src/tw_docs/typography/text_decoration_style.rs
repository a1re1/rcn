//! <https://tailwindcss.com/docs/text-decoration-style>

use crate::tw_docs::demo::{el, labeled};

use crate::tw_docs::{Node, TwExample, TwPage, TwSection};

/// <https://tailwindcss.com/docs/text-decoration-style>
pub static TEXT_DECORATION_STYLE: TwPage = TwPage {
    slug: "text-decoration-style",
    title: "Text-decoration-style",
    section: TwSection::Typography,
    description: "Utilities for controlling the style of text decorations.",
    reference: &[
        ("decoration-solid", "text-decoration-style: solid;"),
        ("decoration-double", "text-decoration-style: double;"),
        ("decoration-dotted", "text-decoration-style: dotted;"),
        ("decoration-dashed", "text-decoration-style: dashed;"),
        ("decoration-wavy", "text-decoration-style: wavy;"),
    ],
    examples: &[
        TwExample {
            title: "Basic example",
            prose: &[
                "Use utilities like decoration-dotted and decoration-dashed to change the text decoration style of an element:",
                "The quick brown fox jumps over the lazy dog.",
                "The quick brown fox jumps over the lazy dog.",
                "The quick brown fox jumps over the lazy dog.",
                "The quick brown fox jumps over the lazy dog.",
                "The quick brown fox jumps over the lazy dog.",
            ],
            snippet: r#"<p class="underline decoration-solid">The quick brown fox...</p>
<p class="underline decoration-double">The quick brown fox...</p>
<p class="underline decoration-dotted">The quick brown fox...</p>
<p class="underline decoration-dashed">The quick brown fox...</p>
<p class="underline decoration-wavy">The quick brown fox...</p>"#,
            demo: el(
                "flex w-full flex-col gap-3",
                &[
                    el(
                        "underline",
                        &[Node::Prose("The quick brown fox jumps over the lazy dog.")],
                    ),
                    el(
                        "underline decoration-wavy",
                        &[Node::Prose("The quick brown fox jumps over the lazy dog.")],
                    ),
                    labeled(
                        "text-xs text-slate-500",
                        "decoration-solid/double/dotted/dashed have no equivalent; wavy is supported",
                    ),
                ],
            ),
        },
        TwExample {
            title: "Responsive design",
            prose: &[
                "Prefix a text-decoration-style utility with a breakpoint variant like md: to only apply the utility at medium screen sizes and above:",
                "Learn more about using variants in the variants documentation.",
            ],
            snippet: r#"<p class="underline md:decoration-dashed ...">  Lorem ipsum dolor sit amet...</p>"#,
            demo: el(
                "flex w-full flex-col gap-3",
                &[
                    el(
                        "underline md:decoration-wavy",
                        &[Node::Prose(
                            "Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod tempor incididunt ut labore et dolore magna aliqua.",
                        )],
                    ),
                    labeled(
                        "text-xs text-slate-500",
                        "md:decoration-dashed ≈ md:decoration-wavy",
                    ),
                ],
            ),
        },
    ],
};
