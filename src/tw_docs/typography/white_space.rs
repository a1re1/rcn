//! <https://tailwindcss.com/docs/white-space>

use crate::tw_docs::demo::{el, labeled};

use crate::tw_docs::{Node, TwExample, TwPage, TwSection};

/// <https://tailwindcss.com/docs/white-space>
pub static WHITE_SPACE: TwPage = TwPage {
    slug: "white-space",
    title: "White-space",
    section: TwSection::Typography,
    description: "Utilities for controlling an element's white-space property.",
    reference: &[
        ("whitespace-normal", "white-space: normal;"),
        ("whitespace-nowrap", "white-space: nowrap;"),
        ("whitespace-pre", "white-space: pre;"),
        ("whitespace-pre-line", "white-space: pre-line;"),
        ("whitespace-pre-wrap", "white-space: pre-wrap;"),
        ("whitespace-break-spaces", "white-space: break-spaces;"),
    ],
    examples: &[
        TwExample {
            title: "Normal",
            prose: &[
                "Use the whitespace-normal utility to cause text to wrap normally within an element. Newlines and spaces will be collapsed:",
                "Hey everyone!

It’s almost 2022       and we still don’t know if there             are aliens living among us, or do we? Maybe the person writing this is an alien.

You will never know.",
            ],
            snippet: r#"<p class="whitespace-normal">Hey everyone!It's almost 2022       and we still don't know if there             are aliens living among us, or do we? Maybe the person writing this is an alien.You will never know.</p>"#,
            demo: el("flex w-64 flex-col", &[
                el("whitespace-normal", &[Node::Prose("Hey everyone! It's almost 2022 and we still don't know if there are aliens living among us, or do we? Maybe the person writing this is an alien. You will never know.")]),
            ]),
        },
        TwExample {
            title: "No Wrap",
            prose: &[
                "Use the whitespace-nowrap utility to prevent text from wrapping within an element. Newlines and spaces will be collapsed:",
                "Hey everyone!

It’s almost 2022       and we still don’t know if there             are aliens living among us, or do we? Maybe the person writing this is an alien.

You will never know.",
            ],
            snippet: r#"<p class="overflow-auto whitespace-nowrap">Hey everyone!It's almost 2022       and we still don't know if there             are aliens living among us, or do we? Maybe the person writing this is an alien.You will never know.</p>"#,
            demo: el("flex w-64 flex-col", &[
                el("overflow-auto whitespace-nowrap", &[Node::Prose("Hey everyone! It's almost 2022 and we still don't know if there are aliens living among us, or do we? Maybe the person writing this is an alien. You will never know.")]),
            ]),
        },
        TwExample {
            title: "Pre",
            prose: &[
                "Use the whitespace-pre utility to preserve newlines and spaces within an element. Text will not be wrapped:",
                "Hey everyone!

It’s almost 2022       and we still don’t know if there             are aliens living among us, or do we? Maybe the person writing this is an alien.

You will never know.",
            ],
            snippet: r#"<p class="overflow-auto whitespace-pre">Hey everyone!It's almost 2022       and we still don't know if there             are aliens living among us, or do we? Maybe the person writing this is an alien.You will never know.</p>"#,
            demo: el("flex w-64 flex-col gap-2", &[
                el("whitespace-nowrap overflow-hidden", &[Node::Prose("Hey everyone! It's almost 2022 and we still don't know if there are aliens living among us, or do we? Maybe the person writing this is an alien. You will never know.")]),
                labeled("text-xs text-slate-500", "whitespace-pre has no equivalent — nowrap shown"),
            ]),
        },
        TwExample {
            title: "Pre Line",
            prose: &[
                "Use the whitespace-pre-line utility to preserve newlines but not spaces within an element. Text will be wrapped normally:",
                "Hey everyone!

It’s almost 2022       and we still don’t know if there             are aliens living among us, or do we? Maybe the person writing this is an alien.

You will never know.",
            ],
            snippet: r#"<p class="whitespace-pre-line">Hey everyone!It's almost 2022       and we still don't know if there             are aliens living among us, or do we? Maybe the person writing this is an alien.You will never know.</p>"#,
            demo: el("flex w-64 flex-col gap-2", &[
                el("whitespace-normal", &[Node::Prose("Hey everyone! It's almost 2022 and we still don't know if there are aliens living among us, or do we? Maybe the person writing this is an alien. You will never know.")]),
                labeled("text-xs text-slate-500", "whitespace-pre-line has no equivalent — normal shown"),
            ]),
        },
        TwExample {
            title: "Pre Wrap",
            prose: &[
                "Use the whitespace-pre-wrap utility to preserve newlines and spaces within an element. Text will be wrapped normally:",
                "Hey everyone!

It’s almost 2022       and we still don’t know if there             are aliens living among us, or do we? Maybe the person writing this is an alien.

You will never know.",
            ],
            snippet: r#"<p class="whitespace-pre-wrap">Hey everyone!It's almost 2022       and we still don't know if there             are aliens living among us, or do we? Maybe the person writing this is an alien.You will never know.</p>"#,
            demo: el("flex w-64 flex-col gap-2", &[
                el("whitespace-normal", &[Node::Prose("Hey everyone! It's almost 2022 and we still don't know if there are aliens living among us, or do we? Maybe the person writing this is an alien. You will never know.")]),
                labeled("text-xs text-slate-500", "whitespace-pre-wrap has no equivalent — normal shown"),
            ]),
        },
        TwExample {
            title: "Break Spaces",
            prose: &[
                "Use the whitespace-break-spaces utility to preserve newlines and spaces within an element. White space at the end of lines will not hang, but will wrap to the next line:",
                "Hey everyone!

It’s almost 2022       and we still don’t know if there             are aliens living among us, or do we? Maybe the person writing this is an alien.

You will never know.",
            ],
            snippet: r#"<p class="whitespace-break-spaces">Hey everyone!It's almost 2022       and we still don't know if there             are aliens living among us, or do we? Maybe the person writing this is an alien.You will never know.</p>"#,
            demo: el("flex w-64 flex-col gap-2", &[
                el("whitespace-normal", &[Node::Prose("Hey everyone! It's almost 2022 and we still don't know if there are aliens living among us, or do we? Maybe the person writing this is an alien. You will never know.")]),
                labeled("text-xs text-slate-500", "whitespace-break-spaces has no equivalent — normal shown"),
            ]),
        },
        TwExample {
            title: "Responsive design",
            prose: &[
                "Prefix a white-space utility with a breakpoint variant like md: to only apply the utility at medium screen sizes and above:",
                "Learn more about using variants in the variants documentation.",
            ],
            snippet: r#"<p class="whitespace-pre md:whitespace-normal ...">  Lorem ipsum dolor sit amet...</p>"#,
            demo: el("flex w-64 flex-col gap-2", &[
                el("whitespace-nowrap md:whitespace-normal overflow-hidden", &[Node::Prose("Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod tempor incididunt ut labore et dolore magna aliqua.")]),
                labeled("text-xs text-slate-500", "whitespace-pre ≈ whitespace-nowrap"),
            ]),
        },
    ],
};
