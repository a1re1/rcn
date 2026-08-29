//! <https://tailwindcss.com/docs/position>

use crate::tw_docs::demo::{el, labeled};

use crate::tw_docs::{Node, TwExample, TwPage, TwSection};

/// <https://tailwindcss.com/docs/position>
pub static POSITION: TwPage = TwPage {
    slug: "position",
    title: "Position",
    section: TwSection::Layout,
    description: "Utilities for controlling how an element is positioned in the document.",
    reference: &[
        ("static", "position: static;"),
        ("fixed", "position: fixed;"),
        ("absolute", "position: absolute;"),
        ("relative", "position: relative;"),
        ("sticky", "position: sticky;"),
    ],
    examples: &[
        TwExample {
            title: "Statically positioning elements",
            prose: &[
                "Use the static utility to position an element according to the normal flow of the document:",
                "Static parent",
                "Absolute child",
                "With statically positioned elements, any offsets will be ignored and the element will not act as a position reference for absolutely positioned children.",
            ],
            snippet: r#"<div class="static ...">
<p>Static parent</p>
<div class="absolute bottom-0 left-0 ...">
<p>Absolute child</p>
</div>
</div>"#,
            demo: el(
                "h-32 w-72 rounded-lg bg-sky-300 p-4",
                &[
                    Node::Prose("Static parent"),
                    labeled(
                        "absolute bottom-0 left-0 rounded-md bg-sky-500 px-3 py-1 text-white",
                        "Absolute child",
                    ),
                ],
            ),
        },
        TwExample {
            title: "Relatively positioning elements",
            prose: &[
                "Use the relative utility to position an element according to the normal flow of the document:",
                "Relative parent",
                "Absolute child",
                "With relatively position elements, any offsets are calculated relative to the element's normal position and the element will act as a position reference for absolutely positioned children.",
            ],
            snippet: r#"<div class="relative ...">
<p>Relative parent</p>
<div class="absolute bottom-0 left-0 ...">
<p>Absolute child</p>
</div>
</div>"#,
            demo: el(
                "relative h-32 w-72 rounded-lg bg-sky-300 p-4",
                &[
                    Node::Prose("Relative parent"),
                    labeled(
                        "absolute bottom-0 left-0 rounded-md bg-sky-500 px-3 py-1 text-white",
                        "Absolute child",
                    ),
                ],
            ),
        },
        TwExample {
            title: "Absolutely positioning elements",
            prose: &[
                "Use the absolute utility to position an element outside of the normal flow of the document, causing neighboring elements to act as if the element doesn't exist:",
                "With static positioning",
                "Relative parent",
                "Static parent",
                "Static child?",
                "Static sibling",
                "With absolute positioning",
                "Relative parent",
                "Static parent",
                "Absolute child",
                "Static sibling",
                "With absolutely positioned elements, any offsets are calculated relative to the nearest parent that has a position other than static, and the element will act as a position reference for other absolutely positioned children.",
            ],
            snippet: r#"<div class="static ...">
<!-- Static parent -->
<div class="static ...">
<p>Static child</p>
</div>
<div class="inline-block ...">
<p>Static sibling</p>
</div>
<!-- Static parent -->
<div class="absolute ...">
<p>Absolute child</p>
</div>
<div class="inline-block ...">
<p>Static sibling</p>
</div>
</div>"#,
            demo: el(
                "relative h-40 w-80 rounded-lg bg-indigo-300 p-4",
                &[
                    el(
                        "flex gap-2",
                        &[
                            labeled(
                                "rounded-md bg-indigo-500 px-3 py-1 text-white",
                                "Static child",
                            ),
                            labeled(
                                "rounded-md bg-indigo-400 px-3 py-1 text-white",
                                "Static sibling",
                            ),
                        ],
                    ),
                    labeled(
                        "absolute top-0 right-0 rounded-md bg-indigo-600 px-3 py-1 text-white",
                        "Absolute child",
                    ),
                ],
            ),
        },
        TwExample {
            title: "Fixed positioning elements",
            prose: &[
                "Use the fixed utility to position an element relative to the browser window:",
                "Scroll this element to see the fixed positioning in action",
                "With fixed positioned elements, any offsets are calculated relative to the viewport and the element will act as a position reference for absolutely positioned children:",
            ],
            snippet: r#"<div class="relative">
<div class="fixed top-0 right-0 left-0">Contacts</div>
<div>
<div>
<img src="/img/andrew.jpg" />
<strong>Andrew Alfred</strong>
</div>
<div>
<img src="/img/debra.jpg" />
<strong>Debra Houston</strong>
</div>
<!-- ... -->
</div>
</div>"#,
            demo: el(
                "flex flex-col gap-2",
                &[
                    el(
                        "relative h-48 w-72 overflow-y-auto rounded-lg bg-slate-100",
                        &[
                            labeled("rounded-t-lg bg-slate-300 px-3 py-2", "Contacts"),
                            el(
                                "flex flex-col gap-2 p-3",
                                &[
                                    Node::Prose("Andrew Alfred"),
                                    Node::Prose("Debra Houston"),
                                    Node::Prose("Jane Cooper"),
                                    Node::Prose("Guy Hawkins"),
                                    Node::Prose("Brooklyn Simmons"),
                                ],
                            ),
                        ],
                    ),
                    labeled(
                        "text-xs text-slate-500",
                        "fixed has no equivalent — the header is shown as a static row",
                    ),
                ],
            ),
        },
        TwExample {
            title: "Sticky positioning elements",
            prose: &[
                "Use the sticky utility to position an element as relative until it crosses a specified threshold, then treat it as fixed until its parent is off screen:",
                "Scroll this element to see the sticky positioning in action",
                "With sticky positioned elements, any offsets are calculated relative to the element's normal position and the element will act as a position reference for absolutely positioned children.",
            ],
            snippet: r#"<div>
<div>
<div class="sticky top-0 ...">A</div>
<div>
<div>
<img src="/img/andrew.jpg" />
<strong>Andrew Alfred</strong>
</div>
<div>
<img src="/img/aisha.jpg" />
<strong>Aisha Houston</strong>
</div>
<!-- ... -->
</div>
</div>
<div>
<div class="sticky top-0">B</div>
<div>
<div>
<img src="/img/bob.jpg" />
<strong>Bob Alfred</strong>
</div>
<!-- ... -->
</div>
</div>
<!-- ... -->
</div>"#,
            demo: el(
                "flex flex-col gap-2",
                &[
                    el(
                        "relative h-48 w-72 overflow-y-auto rounded-lg bg-slate-100",
                        &[
                            labeled("bg-slate-300 px-3 py-1", "A"),
                            el(
                                "flex flex-col gap-2 p-3",
                                &[
                                    Node::Prose("Andrew Alfred"),
                                    Node::Prose("Aisha Houston"),
                                    Node::Prose("Anna Roberts"),
                                ],
                            ),
                            labeled("bg-slate-300 px-3 py-1", "B"),
                            el(
                                "flex flex-col gap-2 p-3",
                                &[Node::Prose("Bob Alfred"), Node::Prose("Bianca Houston")],
                            ),
                        ],
                    ),
                    labeled(
                        "text-xs text-slate-500",
                        "sticky has no equivalent — section headers scroll with the list",
                    ),
                ],
            ),
        },
        TwExample {
            title: "Responsive design",
            prose: &[
                "Prefix a position utility with a breakpoint variant like md: to only apply the utility at medium screen sizes and above:",
                "Learn more about using variants in the variants documentation.",
            ],
            snippet: r#"<div class="relative md:absolute ...">
<!-- ... -->
</div>"#,
            demo: el(
                "relative h-32 w-72 rounded-lg bg-sky-300 p-4",
                &[
                    Node::Prose("Parent"),
                    labeled(
                        "relative md:absolute bottom-0 left-0 rounded-md bg-sky-500 px-3 py-1 text-white",
                        "relative md:absolute",
                    ),
                ],
            ),
        },
    ],
};
