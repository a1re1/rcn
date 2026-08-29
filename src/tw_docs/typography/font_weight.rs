//! <https://tailwindcss.com/docs/font-weight>

use crate::tw_docs::demo::{el, labeled};

use crate::tw_docs::{Node, TwExample, TwPage, TwSection};

/// <https://tailwindcss.com/docs/font-weight>
pub static FONT_WEIGHT: TwPage = TwPage {
    slug: "font-weight",
    title: "Font-weight",
    section: TwSection::Typography,
    description: "Utilities for controlling the font weight of an element.",
    reference: &[
        ("font-thin", "font-weight: 100;"),
        ("font-extralight", "font-weight: 200;"),
        ("font-light", "font-weight: 300;"),
        ("font-normal", "font-weight: 400;"),
        ("font-medium", "font-weight: 500;"),
        ("font-semibold", "font-weight: 600;"),
        ("font-bold", "font-weight: 700;"),
        ("font-extrabold", "font-weight: 800;"),
        ("font-black", "font-weight: 900;"),
        (
            "font-(<custom-property>)",
            "font-weight: var(<custom-property>);",
        ),
        ("font-[<value>]", "font-weight: <value>;"),
    ],
    examples: &[
        TwExample {
            title: "Basic example",
            prose: &[
                "Use utilities like font-thin and font-bold to set the font weight of an element:",
                "The quick brown fox jumps over the lazy dog.",
                "The quick brown fox jumps over the lazy dog.",
                "The quick brown fox jumps over the lazy dog.",
                "The quick brown fox jumps over the lazy dog.",
                "The quick brown fox jumps over the lazy dog.",
            ],
            snippet: r#"<p class="font-light ...">The quick brown fox ...</p>
<p class="font-normal ...">The quick brown fox ...</p>
<p class="font-medium ...">The quick brown fox ...</p>
<p class="font-semibold ...">The quick brown fox ...</p>
<p class="font-bold ...">The quick brown fox ...</p>"#,
            demo: el(
                "flex w-full flex-col gap-3",
                &[
                    el(
                        "font-light",
                        &[Node::Prose("The quick brown fox jumps over the lazy dog.")],
                    ),
                    el(
                        "font-normal",
                        &[Node::Prose("The quick brown fox jumps over the lazy dog.")],
                    ),
                    el(
                        "font-medium",
                        &[Node::Prose("The quick brown fox jumps over the lazy dog.")],
                    ),
                    el(
                        "font-semibold",
                        &[Node::Prose("The quick brown fox jumps over the lazy dog.")],
                    ),
                    el(
                        "font-bold",
                        &[Node::Prose("The quick brown fox jumps over the lazy dog.")],
                    ),
                    el(
                        "font-extrabold",
                        &[Node::Prose("The quick brown fox jumps over the lazy dog.")],
                    ),
                    el(
                        "font-black",
                        &[Node::Prose("The quick brown fox jumps over the lazy dog.")],
                    ),
                ],
            ),
        },
        TwExample {
            title: "Using a custom value",
            prose: &[
                "Use the font-[<value>] syntax to set the font weight based on a completely custom value:",
                "For CSS variables, you can also use the font-(weight:<custom-property>) syntax:",
                "This is just a shorthand for font-[weight:var(<custom-property>)] that adds the var() function for you automatically.",
            ],
            snippet: r#"<p class="font-[1000] ...">  Lorem ipsum dolor sit amet...</p>
<p class="font-(weight:--my-font-weight) ...">  Lorem ipsum dolor sit amet...</p>"#,
            demo: el(
                "flex w-full flex-col gap-3",
                &[
                    el(
                        "font-black",
                        &[Node::Prose(
                            "Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod tempor incididunt ut labore et dolore magna aliqua.",
                        )],
                    ),
                    labeled(
                        "text-xs text-slate-500",
                        "font-[1000] ≈ font-black (arbitrary weights have no equivalent)",
                    ),
                ],
            ),
        },
        TwExample {
            title: "Responsive design",
            prose: &[
                "Prefix a font-weight utility with a breakpoint variant like md: to only apply the utility at medium screen sizes and above:",
                "Learn more about using variants in the variants documentation.",
            ],
            snippet: r#"<p class="font-normal md:font-bold ...">  Lorem ipsum dolor sit amet...</p>"#,
            demo: el(
                "flex w-full flex-col gap-3",
                &[el(
                    "font-normal md:font-bold",
                    &[Node::Prose(
                        "Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod tempor incididunt ut labore et dolore magna aliqua.",
                    )],
                )],
            ),
        },
    ],
};
