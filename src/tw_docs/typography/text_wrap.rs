//! <https://tailwindcss.com/docs/text-wrap>

use crate::tw_docs::demo::{el, labeled};

use crate::tw_docs::{Node, TwExample, TwPage, TwSection};

/// <https://tailwindcss.com/docs/text-wrap>
pub static TEXT_WRAP: TwPage = TwPage {
    slug: "text-wrap",
    title: "Text-wrap",
    section: TwSection::Typography,
    description: "Utilities for controlling how text wraps within an element.",
    reference: &[
        ("text-wrap", "text-wrap: wrap;"),
        ("text-nowrap", "text-wrap: nowrap;"),
        ("text-balance", "text-wrap: balance;"),
        ("text-pretty", "text-wrap: pretty;"),
    ],
    examples: &[
        TwExample {
            title: "Allowing text to wrap",
            prose: &[
                "Use the text-wrap utility to wrap overflowing text onto multiple lines at logical points in the text:",
                "New Yorkers are facing the winter chill with less warmth this year as the city's most revered soup stand unexpectedly shutters, following a series of events that have left the community puzzled.",
            ],
            snippet: r#"<article class="text-wrap">
<h3>Beloved Manhattan soup stand closes</h3>
<p>New Yorkers are facing the winter chill...</p>
</article>"#,
            demo: el(
                "flex w-64 flex-col gap-1 text-wrap",
                &[
                    el(
                        "font-semibold",
                        &[Node::Prose("Beloved Manhattan soup stand closes")],
                    ),
                    el(
                        "text-wrap text-sm",
                        &[Node::Prose(
                            "New Yorkers are facing the winter chill with less warmth this year as the city's most revered soup stand unexpectedly shutters.",
                        )],
                    ),
                ],
            ),
        },
        TwExample {
            title: "Preventing text from wrapping",
            prose: &[
                "Use the text-nowrap utility to prevent text from wrapping, allowing it to overflow if necessary:",
                "New Yorkers are facing the winter chill with less warmth this year as the city's most revered soup stand unexpectedly shutters, following a series of events that have left the community puzzled.",
            ],
            snippet: r#"<article class="text-nowrap">
<h3>Beloved Manhattan soup stand closes</h3>
<p>New Yorkers are facing the winter chill...</p>
</article>"#,
            demo: el(
                "flex w-64 flex-col gap-1 overflow-hidden",
                &[
                    el(
                        "font-semibold",
                        &[Node::Prose("Beloved Manhattan soup stand closes")],
                    ),
                    el(
                        "text-nowrap text-sm",
                        &[Node::Prose(
                            "New Yorkers are facing the winter chill with less warmth this year.",
                        )],
                    ),
                ],
            ),
        },
        TwExample {
            title: "Balanced text wrapping",
            prose: &[
                "Use the text-balance utility to distribute the text evenly across each line:",
                "New Yorkers are facing the winter chill with less warmth this year as the city's most revered soup stand unexpectedly shutters, following a series of events that have left the community puzzled.",
                "For performance reasons browsers limit text balancing to blocks that are ~6 lines or less, making it best suited for headings.",
            ],
            snippet: r#"<article>
<h3 class="text-balance">Beloved Manhattan soup stand closes</h3>
<p>New Yorkers are facing the winter chill...</p>
</article>"#,
            demo: el(
                "flex w-64 flex-col gap-1",
                &[
                    el(
                        "font-semibold text-wrap",
                        &[Node::Prose("Beloved Manhattan soup stand closes")],
                    ),
                    labeled(
                        "text-xs text-slate-500",
                        "text-balance has no equivalent — text-wrap shown",
                    ),
                ],
            ),
        },
        TwExample {
            title: "Pretty text wrapping",
            prose: &[
                "Use the text-pretty utility to prefer better text wrapping and layout at the expense of speed. Behavior varies across browsers but often involves approaches like preventing orphans (a single word on its own line) at the end of a text block:",
                "New Yorkers are facing the winter chill with less warmth this year as the city's most revered soup stand unexpectedly shutters, following a series of events that have left the community puzzled.",
            ],
            snippet: r#"<article>
<h3 class="text-pretty">Beloved Manhattan soup stand closes</h3>
<p>New Yorkers are facing the winter chill...</p>
</article>"#,
            demo: el(
                "flex w-64 flex-col gap-1",
                &[
                    el(
                        "font-semibold text-wrap",
                        &[Node::Prose("Beloved Manhattan soup stand closes")],
                    ),
                    labeled(
                        "text-xs text-slate-500",
                        "text-pretty has no equivalent — text-wrap shown",
                    ),
                ],
            ),
        },
        TwExample {
            title: "Responsive design",
            prose: &[
                "Prefix a text-wrap utility with a breakpoint variant like md: to only apply the utility at medium screen sizes and above:",
                "Learn more about using variants in the variants documentation.",
            ],
            snippet: r#"<h1 class="text-pretty md:text-balance ...">
<!-- ... -->
</h1>"#,
            demo: el(
                "flex w-64 flex-col gap-1",
                &[
                    el(
                        "text-lg font-semibold text-wrap",
                        &[Node::Prose("Beloved Manhattan soup stand closes")],
                    ),
                    labeled(
                        "text-xs text-slate-500",
                        "text-pretty md:text-balance have no equivalent",
                    ),
                ],
            ),
        },
    ],
};
