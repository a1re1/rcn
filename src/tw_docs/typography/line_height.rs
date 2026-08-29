//! <https://tailwindcss.com/docs/line-height>

use crate::tw_docs::demo::{el, labeled};

use crate::tw_docs::{Node, TwExample, TwPage, TwSection};

/// <https://tailwindcss.com/docs/line-height>
pub static LINE_HEIGHT: TwPage = TwPage {
    slug: "line-height",
    title: "Line-height",
    section: TwSection::Typography,
    description: "Utilities for controlling the leading, or line height, of an element.",
    reference: &[
        (
            "text-<size>/<number>",
            "font-size: <size>;
line-height: calc(var(--spacing) * <number>);",
        ),
        (
            "text-<size>/(<custom-property>)",
            "font-size: <size>;
line-height: var(<custom-property>);",
        ),
        (
            "text-<size>/[<value>]",
            "font-size: <size>;
line-height: <value>;",
        ),
        ("leading-none", "line-height: 1;"),
        (
            "leading-<number>",
            "line-height: calc(var(--spacing) * <number>);",
        ),
        (
            "leading-(<custom-property>)",
            "line-height: var(<custom-property>);",
        ),
        ("leading-[<value>]", "line-height: <value>;"),
    ],
    examples: &[
        TwExample {
            title: "Basic example",
            prose: &[
                "Use font size utilities like text-sm/6 and text-lg/7 to set the font size and line-height of an element at the same time:",
                "So I started to walk into the water. I won't lie to you boys, I was terrified. But I pressed on, and as I made my way past the breakers a strange calm came over me. I don't know if it was divine intervention or the kinship of all living things but I tell you Jerry at that moment, I was a marine biologist.",
                "So I started to walk into the water. I won't lie to you boys, I was terrified. But I pressed on, and as I made my way past the breakers a strange calm came over me. I don't know if it was divine intervention or the kinship of all living things but I tell you Jerry at that moment, I was a marine biologist.",
                "So I started to walk into the water. I won't lie to you boys, I was terrified. But I pressed on, and as I made my way past the breakers a strange calm came over me. I don't know if it was divine intervention or the kinship of all living things but I tell you Jerry at that moment, I was a marine biologist.",
                "Each font size utility also sets a default line height when one isn't provided. You can learn more about these values and how to customize them in the font-size documentation.",
            ],
            snippet: r#"<p class="text-base/6 ...">So I started to walk into the water...</p>
<p class="text-base/7 ...">So I started to walk into the water...</p>
<p class="text-base/8 ...">So I started to walk into the water...</p>"#,
            demo: el(
                "flex w-96 flex-col gap-4",
                &[
                    labeled(
                        "text-xs text-slate-500",
                        "text-base/6 ≈ text-base leading-6",
                    ),
                    el(
                        "text-base leading-6",
                        &[Node::Prose(
                            "So I started to walk into the water. I won't lie to you boys, I was terrified. But I pressed on, and as I made my way past the breakers a strange calm came over me.",
                        )],
                    ),
                    el(
                        "text-base leading-7",
                        &[Node::Prose(
                            "So I started to walk into the water. I won't lie to you boys, I was terrified. But I pressed on, and as I made my way past the breakers a strange calm came over me.",
                        )],
                    ),
                    el(
                        "text-base leading-8",
                        &[Node::Prose(
                            "So I started to walk into the water. I won't lie to you boys, I was terrified. But I pressed on, and as I made my way past the breakers a strange calm came over me.",
                        )],
                    ),
                ],
            ),
        },
        TwExample {
            title: "Setting independently",
            prose: &[
                "Use leading-<number> utilities like leading-6 and leading-7 to set the line height of an element independent of the font-size:",
                "So I started to walk into the water. I won't lie to you boys, I was terrified. But I pressed on, and as I made my way past the breakers a strange calm came over me. I don't know if it was divine intervention or the kinship of all living things but I tell you Jerry at that moment, I was a marine biologist.",
                "So I started to walk into the water. I won't lie to you boys, I was terrified. But I pressed on, and as I made my way past the breakers a strange calm came over me. I don't know if it was divine intervention or the kinship of all living things but I tell you Jerry at that moment, I was a marine biologist.",
                "So I started to walk into the water. I won't lie to you boys, I was terrified. But I pressed on, and as I made my way past the breakers a strange calm came over me. I don't know if it was divine intervention or the kinship of all living things but I tell you Jerry at that moment, I was a marine biologist.",
            ],
            snippet: r#"<p class="text-sm leading-6">So I started to walk into the water...</p>
<p class="text-sm leading-7">So I started to walk into the water...</p>
<p class="text-sm leading-8">So I started to walk into the water...</p>"#,
            demo: el(
                "flex w-96 flex-col gap-4",
                &[
                    el(
                        "text-sm leading-6",
                        &[Node::Prose(
                            "So I started to walk into the water. I won't lie to you boys, I was terrified. But I pressed on, and as I made my way past the breakers a strange calm came over me.",
                        )],
                    ),
                    el(
                        "text-sm leading-7",
                        &[Node::Prose(
                            "So I started to walk into the water. I won't lie to you boys, I was terrified. But I pressed on, and as I made my way past the breakers a strange calm came over me.",
                        )],
                    ),
                    el(
                        "text-sm leading-8",
                        &[Node::Prose(
                            "So I started to walk into the water. I won't lie to you boys, I was terrified. But I pressed on, and as I made my way past the breakers a strange calm came over me.",
                        )],
                    ),
                ],
            ),
        },
        TwExample {
            title: "Removing the leading",
            prose: &[
                "Use the leading-none utility to set the line height of an element equal to its font size:",
                "The quick brown fox jumps over the lazy dog.",
            ],
            snippet: r#"<p class="text-2xl leading-none ...">The quick brown fox...</p>"#,
            demo: el(
                "flex w-full flex-col gap-3",
                &[el(
                    "text-2xl leading-none",
                    &[Node::Prose("The quick brown fox jumps over the lazy dog.")],
                )],
            ),
        },
        TwExample {
            title: "Using a custom value",
            prose: &[
                "Use the leading-[<value>] syntax to set the line height based on a completely custom value:",
                "For CSS variables, you can also use the leading-(<custom-property>) syntax:",
                "This is just a shorthand for leading-[var(<custom-property>)] that adds the var() function for you automatically.",
            ],
            snippet: r#"<p class="leading-[1.5] ...">  Lorem ipsum dolor sit amet...</p>
<p class="leading-(--my-line-height) ...">  Lorem ipsum dolor sit amet...</p>"#,
            demo: el(
                "flex w-96 flex-col gap-3",
                &[
                    labeled(
                        "text-xs text-slate-500",
                        "leading-[1.5] ≈ leading-6 (unitless arbitrary leading has no equivalent)",
                    ),
                    el(
                        "leading-6",
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
                "Prefix a line-height utility with a breakpoint variant like md: to only apply the utility at medium screen sizes and above:",
                "Learn more about using variants in the variants documentation.",
            ],
            snippet: r#"<p class="leading-5 md:leading-6 ...">  Lorem ipsum dolor sit amet...</p>"#,
            demo: el(
                "flex w-96 flex-col gap-3",
                &[el(
                    "leading-5 md:leading-6",
                    &[Node::Prose(
                        "Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod tempor incididunt ut labore et dolore magna aliqua.",
                    )],
                )],
            ),
        },
    ],
};
