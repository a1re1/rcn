//! <https://tailwindcss.com/docs/cursor>

use crate::tw_docs::demo::{el, labeled};
use crate::tw_docs::{TwExample, TwPage, TwSection};

/// <https://tailwindcss.com/docs/cursor>
pub static CURSOR: TwPage = TwPage {
    slug: "cursor",
    title: "Cursor",
    section: TwSection::Interactivity,
    description: "Utilities for controlling the cursor style when hovering over an element.",
    reference: &[
        ("cursor-auto", "cursor: auto;"),
        ("cursor-default", "cursor: default;"),
        ("cursor-pointer", "cursor: pointer;"),
        ("cursor-default", "cursor: wait;"),
        ("cursor-text", "cursor: text;"),
        ("cursor-move", "cursor: move;"),
        ("cursor-help", "cursor: help;"),
        ("cursor-not-allowed", "cursor: not-allowed;"),
        ("cursor-none", "cursor: none;"),
        ("cursor-context-menu", "cursor: context-menu;"),
        ("cursor-progress", "cursor: progress;"),
        ("cursor-cell", "cursor: cell;"),
        ("cursor-crosshair", "cursor: crosshair;"),
        ("cursor-vertical-text", "cursor: vertical-text;"),
        ("cursor-alias", "cursor: alias;"),
        ("cursor-copy", "cursor: copy;"),
        ("cursor-no-drop", "cursor: no-drop;"),
        ("cursor-grab", "cursor: grab;"),
        ("cursor-grabbing", "cursor: grabbing;"),
        ("cursor-all-scroll", "cursor: all-scroll;"),
        ("cursor-col-resize", "cursor: col-resize;"),
        ("cursor-row-resize", "cursor: row-resize;"),
        ("cursor-n-resize", "cursor: n-resize;"),
        ("cursor-e-resize", "cursor: e-resize;"),
        ("cursor-s-resize", "cursor: s-resize;"),
        ("cursor-w-resize", "cursor: w-resize;"),
        ("cursor-ne-resize", "cursor: ne-resize;"),
        ("cursor-nw-resize", "cursor: nw-resize;"),
        ("cursor-se-resize", "cursor: se-resize;"),
        ("cursor-sw-resize", "cursor: sw-resize;"),
        ("cursor-ew-resize", "cursor: ew-resize;"),
        ("cursor-ns-resize", "cursor: ns-resize;"),
        ("cursor-nesw-resize", "cursor: nesw-resize;"),
        ("cursor-nwse-resize", "cursor: nwse-resize;"),
        ("cursor-zoom-in", "cursor: zoom-in;"),
        ("cursor-zoom-out", "cursor: zoom-out;"),
        (
            "cursor-(<custom-property>)",
            "cursor: var(<custom-property>);",
        ),
        ("cursor-[<value>]", "cursor: <value>;"),
    ],
    examples: &[
        TwExample {
            title: "Basic example",
            prose: &[
                "Use utilities like cursor-pointer and cursor-grab to control which cursor is displayed when hovering over an element:",
                "Hover over each button to see the cursor change",
            ],
            snippet: r#"<button class="cursor-pointer ...">Submit</button>
<button class="cursor-progress ...">Saving...</button>
<button class="cursor-not-allowed ..." disabled>Confirm</button>"#,
            demo: el(
                "flex flex-col gap-3",
                &[
                    el(
                        "flex flex-wrap items-center gap-4",
                        &[
                            labeled(
                                "rounded-md px-4 py-2 font-semibold text-white bg-indigo-500 cursor-pointer",
                                "Submit",
                            ),
                            labeled(
                                "rounded-md px-4 py-2 font-semibold text-white bg-indigo-500 cursor-default",
                                "Saving...",
                            ),
                            labeled(
                                "rounded-md px-4 py-2 font-semibold text-white bg-indigo-300 cursor-not-allowed",
                                "Confirm",
                            ),
                        ],
                    ),
                    labeled("text-xs text-slate-500", "cursor-progress ≈ cursor-default"),
                ],
            ),
        },
        TwExample {
            title: "Using a custom value",
            prose: &[
                "Use the cursor-[<value>] syntax to set the cursor based on a completely custom value:",
                "For CSS variables, you can also use the cursor-(<custom-property>) syntax:",
                "This is just a shorthand for cursor-[var(<custom-property>)] that adds the var() function for you automatically.",
            ],
            snippet: r#"<button class="cursor-[url(hand.cur),_pointer] ...">
<!-- ... -->
</button>
<button class="cursor-(--my-cursor) ...">
<!-- ... -->
</button>"#,
            demo: el(
                "flex flex-col gap-3",
                &[
                    el(
                        "flex flex-wrap items-center gap-4",
                        &[labeled(
                            "rounded-md px-4 py-2 font-semibold text-white bg-indigo-500 cursor-pointer",
                            "Submit",
                        )],
                    ),
                    labeled(
                        "text-xs text-slate-500",
                        "cursor-[url(hand.cur),_pointer] has no equivalent — cursor-pointer shown",
                    ),
                ],
            ),
        },
        TwExample {
            title: "Responsive design",
            prose: &[
                "Prefix a cursor utility with a breakpoint variant like md: to only apply the utility at medium screen sizes and above:",
                "Learn more about using variants in the variants documentation.",
            ],
            snippet: r#"<button class="cursor-not-allowed md:cursor-auto ...">
<!-- ... -->
</button>"#,
            demo: el(
                "flex flex-col gap-3",
                &[
                    el(
                        "flex flex-wrap items-center gap-4",
                        &[labeled(
                            "rounded-md px-4 py-2 font-semibold text-white bg-indigo-500 cursor-not-allowed md:cursor-default",
                            "Submit",
                        )],
                    ),
                    labeled(
                        "text-xs text-slate-500",
                        "md:cursor-auto ≈ md:cursor-default",
                    ),
                ],
            ),
        },
    ],
};
