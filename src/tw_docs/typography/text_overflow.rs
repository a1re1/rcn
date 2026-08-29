//! <https://tailwindcss.com/docs/text-overflow>

use crate::tw_docs::demo::{el, labeled};

use crate::tw_docs::{Node, TwExample, TwPage, TwSection};

/// <https://tailwindcss.com/docs/text-overflow>
pub static TEXT_OVERFLOW: TwPage = TwPage {
    slug: "text-overflow",
    title: "Text-overflow",
    section: TwSection::Typography,
    description: "Utilities for controlling how the text of an element overflows.",
    reference: &[
        (
            "truncate",
            "overflow: hidden;
text-overflow: ellipsis;
white-space: nowrap;",
        ),
        ("text-ellipsis", "text-overflow: ellipsis;"),
        ("text-clip", "text-overflow: clip;"),
    ],
    examples: &[
        TwExample {
            title: "Truncating text",
            prose: &[
                "Use the truncate utility to prevent text from wrapping and truncate overflowing text with an ellipsis (…) if needed:",
                "The longest word in any of the major English language dictionaries is pneumonoultramicroscopicsilicovolcanoconiosis, a word that refers to a lung disease contracted from the inhalation of very fine silica particles, specifically from a volcano; medically, it is the same as silicosis.",
            ],
            snippet: r#"<p class="truncate">The longest word in any of the major...</p>"#,
            demo: el(
                "flex w-64 flex-col",
                &[el(
                    "truncate",
                    &[Node::Prose(
                        "The longest word in any of the major English language dictionaries is pneumonoultramicroscopicsilicovolcanoconiosis.",
                    )],
                )],
            ),
        },
        TwExample {
            title: "Adding an ellipsis",
            prose: &[
                "Use the text-ellipsis utility to truncate overflowing text with an ellipsis (…) if needed:",
                "The longest word in any of the major English language dictionaries is pneumonoultramicroscopicsilicovolcanoconiosis, a word that refers to a lung disease contracted from the inhalation of very fine silica particles, specifically from a volcano; medically, it is the same as silicosis.",
            ],
            snippet: r#"<p class="overflow-hidden text-ellipsis">The longest word in any of the major...</p>"#,
            demo: el(
                "flex w-64 flex-col",
                &[el(
                    "overflow-hidden text-ellipsis",
                    &[Node::Prose(
                        "The longest word in any of the major English language dictionaries is pneumonoultramicroscopicsilicovolcanoconiosis.",
                    )],
                )],
            ),
        },
        TwExample {
            title: "Clipping text",
            prose: &[
                "Use the text-clip utility to truncate the text at the limit of the content area:",
                "The longest word in any of the major English language dictionaries is pneumonoultramicroscopicsilicovolcanoconiosis, a word that refers to a lung disease contracted from the inhalation of very fine silica particles, specifically from a volcano; medically, it is the same as silicosis.",
                "This is the default browser behavior.",
            ],
            snippet: r#"<p class="overflow-hidden text-clip">The longest word in any of the major...</p>"#,
            demo: el(
                "flex w-64 flex-col gap-2",
                &[
                    el(
                        "overflow-hidden whitespace-nowrap",
                        &[Node::Prose(
                            "The longest word in any of the major English language dictionaries is pneumonoultramicroscopicsilicovolcanoconiosis.",
                        )],
                    ),
                    labeled(
                        "text-xs text-slate-500",
                        "text-clip has no equivalent — plain overflow-hidden shown",
                    ),
                ],
            ),
        },
        TwExample {
            title: "Responsive design",
            prose: &[
                "Prefix a text-overflow utility with a breakpoint variant like md: to only apply the utility at medium screen sizes and above:",
                "Learn more about using variants in the variants documentation.",
            ],
            snippet: r#"<p class="text-ellipsis md:text-clip ...">  Lorem ipsum dolor sit amet...</p>"#,
            demo: el(
                "flex w-64 flex-col gap-2",
                &[
                    el(
                        "text-ellipsis overflow-hidden",
                        &[Node::Prose(
                            "Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod tempor incididunt ut labore et dolore magna aliqua.",
                        )],
                    ),
                    labeled("text-xs text-slate-500", "md:text-clip has no equivalent"),
                ],
            ),
        },
    ],
};
