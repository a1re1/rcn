//! <https://tailwindcss.com/docs/overflow>

use crate::tw_docs::demo::{TODO_DEMO, el, image, labeled};

use crate::tw_docs::{Node, TwExample, TwPage, TwSection};

/// <https://tailwindcss.com/docs/overflow>
pub static OVERFLOW: TwPage = TwPage {
    slug: "overflow",
    title: "Overflow",
    section: TwSection::Layout,
    description: "Utilities for controlling how an element handles content that is too large for the container.",
    reference: &[
        ("overflow-auto", "overflow: auto;"),
        ("overflow-hidden", "overflow: hidden;"),
        ("overflow-clip", "overflow: clip;"),
        ("overflow-visible", "overflow: visible;"),
        ("overflow-scroll", "overflow: scroll;"),
        ("overflow-x-auto", "overflow-x: auto;"),
        ("overflow-y-auto", "overflow-y: auto;"),
        ("overflow-x-hidden", "overflow-x: hidden;"),
        ("overflow-y-hidden", "overflow-y: hidden;"),
        ("overflow-x-clip", "overflow-x: clip;"),
        ("overflow-y-clip", "overflow-y: clip;"),
        ("overflow-x-visible", "overflow-x: visible;"),
        ("overflow-y-visible", "overflow-y: visible;"),
        ("overflow-x-scroll", "overflow-x: scroll;"),
        ("overflow-y-scroll", "overflow-y: scroll;"),
    ],
    examples: &[
        TwExample {
            title: "Showing content that overflows",
            prose: &[
                "Use the overflow-visible utility to prevent content within an element from being clipped:",
                "Note that any content that overflows the bounds of the element will then be visible.",
            ],
            snippet: r#"<div class="overflow-visible ...">
<!-- ... -->
</div>"#,
            demo: TODO_DEMO,
        },
        TwExample {
            title: "Hiding content that overflows",
            prose: &[
                "Use the overflow-hidden utility to clip any content within an element that overflows the bounds of that element:",
            ],
            snippet: r#"<div class="overflow-hidden ...">
<!-- ... -->
</div>"#,
            demo: TODO_DEMO,
        },
        TwExample {
            title: "Scrolling if needed",
            prose: &[
                "Use the overflow-auto utility to add scrollbars to an element in the event that its content overflows the bounds of that element:",
                "Scroll vertically",
                "Unlike overflow-scroll, which always shows scrollbars, this utility will only show them if scrolling is necessary.",
            ],
            snippet: r#"<div class="overflow-auto ...">
<!-- ... -->
</div>"#,
            demo: TODO_DEMO,
        },
        TwExample {
            title: "Scrolling horizontally if needed",
            prose: &[
                "Use the overflow-x-auto utility to allow horizontal scrolling if needed:",
                "Scroll horizontally",
            ],
            snippet: r#"<div class="overflow-x-auto ...">
<!-- ... -->
</div>"#,
            demo: TODO_DEMO,
        },
        TwExample {
            title: "Scrolling vertically if needed",
            prose: &[
                "Use the overflow-y-auto utility to allow vertical scrolling if needed:",
                "Scroll vertically",
            ],
            snippet: r#"<div class="h-32 overflow-y-auto ...">
<!-- ... -->
</div>"#,
            demo: TODO_DEMO,
        },
        TwExample {
            title: "Scrolling horizontally always",
            prose: &[
                "Use the overflow-x-scroll utility to allow horizontal scrolling and always show scrollbars unless always-visible scrollbars are disabled by the operating system:",
                "Scroll horizontally",
            ],
            snippet: r#"<div class="overflow-x-scroll ...">
<!-- ... -->
</div>"#,
            demo: TODO_DEMO,
        },
        TwExample {
            title: "Scrolling vertically always",
            prose: &[
                "Use the overflow-y-scroll utility to allow vertical scrolling and always show scrollbars unless always-visible scrollbars are disabled by the operating system:",
                "Scroll vertically",
            ],
            snippet: r#"<div class="overflow-y-scroll ...">
<!-- ... -->
</div>"#,
            demo: TODO_DEMO,
        },
        TwExample {
            title: "Scrolling in all directions",
            prose: &[
                "Use the overflow-scroll utility to add scrollbars to an element:",
                "Scroll vertically and horizontally",
                "Unlike overflow-auto, which only shows scrollbars if they are necessary, this utility always shows them. Note that some operating systems (like macOS) hide unnecessary scrollbars regardless of this setting.",
            ],
            snippet: r#"<div class="overflow-scroll ...">
<!-- ... -->
</div>"#,
            demo: TODO_DEMO,
        },
        TwExample {
            title: "Responsive design",
            prose: &[
                "Prefix an overflow utility with a breakpoint variant like md: to only apply the utility at medium screen sizes and above:",
                "Learn more about using variants in the variants documentation.",
            ],
            snippet: r#"<div class="overflow-auto md:overflow-scroll ...">
<!-- ... -->
</div>"#,
            demo: TODO_DEMO,
        },
    ],
};
