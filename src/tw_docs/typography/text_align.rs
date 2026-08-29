//! <https://tailwindcss.com/docs/text-align>

use crate::tw_docs::demo::{el, labeled};

use crate::tw_docs::{Node, TwExample, TwPage, TwSection};

/// <https://tailwindcss.com/docs/text-align>
pub static TEXT_ALIGN: TwPage = TwPage {
    slug: "text-align",
    title: "Text-align",
    section: TwSection::Typography,
    description: "Utilities for controlling the alignment of text.",
    reference: &[
        ("text-left", "text-align: left;"),
        ("text-center", "text-align: center;"),
        ("text-right", "text-align: right;"),
        ("text-justify", "text-align: justify;"),
        ("text-start", "text-align: start;"),
        ("text-end", "text-align: end;"),
    ],
    examples: &[
        TwExample {
            title: "Left aligning text",
            prose: &[
                "Use the text-left utility to left align the text of an element:",
                "So I started to walk into the water. I won't lie to you boys, I was terrified. But I pressed on, and as I made my way past the breakers a strange calm came over me. I don't know if it was divine intervention or the kinship of all living things but I tell you Jerry at that moment, I was a marine biologist.",
            ],
            snippet: r#"<p class="text-left">So I started to walk into the water...</p>"#,
            demo: el(
                "flex w-96 flex-col",
                &[el(
                    "text-left",
                    &[Node::Prose(
                        "So I started to walk into the water. I won't lie to you boys, I was terrified. But I pressed on, and as I made my way past the breakers a strange calm came over me.",
                    )],
                )],
            ),
        },
        TwExample {
            title: "Right aligning text",
            prose: &[
                "Use the text-right utility to right align the text of an element:",
                "So I started to walk into the water. I won't lie to you boys, I was terrified. But I pressed on, and as I made my way past the breakers a strange calm came over me. I don't know if it was divine intervention or the kinship of all living things but I tell you Jerry at that moment, I was a marine biologist.",
            ],
            snippet: r#"<p class="text-right">So I started to walk into the water...</p>"#,
            demo: el(
                "flex w-96 flex-col",
                &[el(
                    "text-right",
                    &[Node::Prose(
                        "So I started to walk into the water. I won't lie to you boys, I was terrified. But I pressed on, and as I made my way past the breakers a strange calm came over me.",
                    )],
                )],
            ),
        },
        TwExample {
            title: "Centering text",
            prose: &[
                "Use the text-center utility to center the text of an element:",
                "So I started to walk into the water. I won't lie to you boys, I was terrified. But I pressed on, and as I made my way past the breakers a strange calm came over me. I don't know if it was divine intervention or the kinship of all living things but I tell you Jerry at that moment, I was a marine biologist.",
            ],
            snippet: r#"<p class="text-center">So I started to walk into the water...</p>"#,
            demo: el(
                "flex w-96 flex-col",
                &[el(
                    "text-center",
                    &[Node::Prose(
                        "So I started to walk into the water. I won't lie to you boys, I was terrified. But I pressed on, and as I made my way past the breakers a strange calm came over me.",
                    )],
                )],
            ),
        },
        TwExample {
            title: "Justifying text",
            prose: &[
                "Use the text-justify utility to justify the text of an element:",
                "So I started to walk into the water. I won't lie to you boys, I was terrified. But I pressed on, and as I made my way past the breakers a strange calm came over me. I don't know if it was divine intervention or the kinship of all living things but I tell you Jerry at that moment, I was a marine biologist.",
            ],
            snippet: r#"<p class="text-justify">So I started to walk into the water...</p>"#,
            demo: el(
                "flex w-96 flex-col gap-2",
                &[
                    el(
                        "text-left",
                        &[Node::Prose(
                            "So I started to walk into the water. I won't lie to you boys, I was terrified. But I pressed on, and as I made my way past the breakers a strange calm came over me.",
                        )],
                    ),
                    labeled("text-xs text-slate-500", "text-justify has no equivalent"),
                ],
            ),
        },
        TwExample {
            title: "Using logical properties",
            prose: &[
                "Use the text-start and text-end utilities, which use logical properties to map to either the left or right side based on the text direction:",
                "بدأتُ أسير نحو الماء. لن أكذب عليكم يا رفاق، كنتُ مرعوبًا. لكنني واصلتُ المسير، وبينما كنتُ أشق طريقي عبر الأمواج، غمرني هدوءٌ غريب. لا أعلم إن كان ذلك تدخّلًا إلهيًا أم صلة قرابة بين جميع الكائنات الحية، لكنني أقول لك يا جيري، في تلك اللحظة، كنتُ عالم أحياء بحرية.",
            ],
            snippet: r#"<div dir="rtl" lang="ar">
<p class="text-end">فبدأت بالسير نحو الماء...</p>
</div>"#,
            demo: el(
                "flex w-96 flex-col gap-2",
                &[
                    el(
                        "text-end",
                        &[Node::Prose(
                            "So I started to walk into the water. I won't lie to you boys, I was terrified. But I pressed on, and as I made my way past the breakers a strange calm came over me.",
                        )],
                    ),
                    labeled(
                        "text-xs text-slate-500",
                        "logical properties map to LTR: text-end ≈ text-right",
                    ),
                ],
            ),
        },
        TwExample {
            title: "Responsive design",
            prose: &[
                "Prefix a text-align utility with a breakpoint variant like md: to only apply the utility at medium screen sizes and above:",
                "Learn more about using variants in the variants documentation.",
            ],
            snippet: r#"<p class="text-left md:text-center ...">  Lorem ipsum dolor sit amet...</p>"#,
            demo: el(
                "flex w-96 flex-col",
                &[el(
                    "text-left md:text-center",
                    &[Node::Prose(
                        "Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod tempor incididunt ut labore et dolore magna aliqua.",
                    )],
                )],
            ),
        },
    ],
};
