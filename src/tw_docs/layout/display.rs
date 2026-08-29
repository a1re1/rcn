//! <https://tailwindcss.com/docs/display>

use crate::tw_docs::demo::{el, image, labeled};

use crate::tw_docs::{Node, TwExample, TwPage, TwSection};

/// <https://tailwindcss.com/docs/display>
pub static DISPLAY: TwPage = TwPage {
    slug: "display",
    title: "Display",
    section: TwSection::Layout,
    description: "Utilities for controlling the display box type of an element.",
    reference: &[
        ("inline", "display: inline;"),
        ("block", "display: block;"),
        ("inline-block", "display: inline-block;"),
        ("flow-root", "display: flow-root;"),
        ("flex", "display: flex;"),
        ("inline-flex", "display: inline-flex;"),
        ("grid", "display: grid;"),
        ("inline-grid", "display: inline-grid;"),
        ("contents", "display: contents;"),
        ("table", "display: table;"),
        ("inline-table", "display: inline-table;"),
        ("table-caption", "display: table-caption;"),
        ("table-cell", "display: table-cell;"),
        ("table-column", "display: table-column;"),
        ("table-column-group", "display: table-column-group;"),
        ("table-footer-group", "display: table-footer-group;"),
        ("table-header-group", "display: table-header-group;"),
        ("table-row-group", "display: table-row-group;"),
        ("table-row", "display: table-row;"),
        ("list-item", "display: list-item;"),
        ("hidden", "display: none;"),
        (
            "sr-only",
            "position: absolute;
width: 1px;
height: 1px;
padding: 0;
margin: -1px;
overflow: hidden;
clip-path: inset(50%);
white-space: nowrap;
border-width: 0;",
        ),
        (
            "not-sr-only",
            "position: static;
width: auto;
height: auto;
padding: 0;
margin: 0;
overflow: visible;
clip-path: none;
white-space: normal;",
        ),
    ],
    examples: &[
        TwExample {
            title: "Block and Inline",
            prose: &[
                "Use the inline, inline-block, and block utilities to control the flow of text and elements:",
            ],
            snippet: r#"<p>  When controlling the flow of text, using the CSS property <span class="inline">display: inline</span> will cause the  text inside the element to wrap normally.</p>
<p>  While using the property <span class="inline-block">display: inline-block</span> will wrap the element to prevent the  text inside from extending beyond its parent.</p>
<p>  Lastly, using the property <span class="block">display: block</span> will put the element on its own line and fill its  parent.</p>"#,
            demo: el(
                "flex flex-col gap-2",
                &[
                    el(
                        "flex w-96 flex-wrap items-center gap-1",
                        &[
                            Node::Prose(
                                "When controlling the flow of text, using the CSS property",
                            ),
                            labeled("rounded bg-sky-200 px-1", "display: inline"),
                            Node::Prose("will cause the text inside the element to wrap normally."),
                        ],
                    ),
                    labeled(
                        "text-xs text-slate-500",
                        "inline / inline-block have no equivalent — shown as a wrapping flex run",
                    ),
                ],
            ),
        },
        TwExample {
            title: "Flow Root",
            prose: &[
                "Use the flow-root utility to create a block-level element with its own block formatting context:",
            ],
            snippet: r#"<div class="p-4">
<div class="flow-root ...">
<div class="my-4 ...">Well, let me tell you something, ...</div>
</div>
<div class="flow-root ...">
<div class="my-4 ...">Sure, go ahead, laugh if you want...</div>
</div>
</div>"#,
            demo: el(
                "flex flex-col gap-2",
                &[
                    el(
                        "flex flex-col gap-4 rounded-lg bg-slate-200 p-4",
                        &[
                            el(
                                "rounded-lg bg-slate-50 p-4",
                                &[Node::Prose(
                                    "Well, let me tell you something, funny boy. You know that little stamp, the one that says New York Public Library?",
                                )],
                            ),
                            el(
                                "rounded-lg bg-slate-50 p-4",
                                &[Node::Prose(
                                    "Sure, go ahead, laugh if you want to. I have seen your type before: flashy, making the scene.",
                                )],
                            ),
                        ],
                    ),
                    labeled(
                        "text-xs text-slate-500",
                        "flow-root has no equivalent — gpui margins never collapse, so plain blocks behave the same",
                    ),
                ],
            ),
        },
        TwExample {
            title: "Flex",
            prose: &["Use the flex utility to create a block-level flex container:"],
            snippet: r#"<div class="flex items-center">
<img src="path/to/image.jpg" />
<div>
<strong>Andrew Alfred</strong>
<span>Technical advisor</span>
</div>
</div>"#,
            demo: el(
                "flex items-center gap-3",
                &[
                    image(
                        "size-12 rounded-full object-cover",
                        "https://images.unsplash.com/photo-1554629947-334ff61d85dc?w=640&h=360&fit=crop",
                    ),
                    el(
                        "flex flex-col",
                        &[
                            el("font-semibold", &[Node::Prose("Andrew Alfred")]),
                            el(
                                "text-sm text-slate-500",
                                &[Node::Prose("Technical advisor")],
                            ),
                        ],
                    ),
                ],
            ),
        },
        TwExample {
            title: "Inline Flex",
            prose: &[
                "Use the inline-flex utility to create an inline flex container that flows with text:",
                "Today I spent most of the day researching ways to take advantage of the fact that bottles can be returned for 10 cents in Michigan, but only 5 cents here. Kramer keeps telling me there is no way to make it work, that he has run the numbers on every possible approach, but I just have to believe there's a way to make it work, there's simply too much opportunity here.",
            ],
            snippet: r#"<p>  Today I spent most of the day researching ways to ...  <span class="inline-flex items-baseline">
<img src="/img/kramer.jpg" class="mx-1 size-5 self-center rounded-full" />
<span>Kramer</span>
</span>  keeps telling me there is no way to make it work, that ...</p>"#,
            demo: el(
                "flex w-96 flex-wrap items-center gap-1",
                &[
                    Node::Prose(
                        "Today I spent most of the day researching ways to make this work, but",
                    ),
                    el(
                        "inline-flex items-baseline",
                        &[
                            image(
                                "mx-1 size-5 self-center rounded-full object-cover",
                                "https://images.unsplash.com/photo-1554629947-334ff61d85dc?w=640&h=360&fit=crop",
                            ),
                            Node::Prose("Kramer"),
                        ],
                    ),
                    Node::Prose("keeps telling me there is no way to make it work."),
                ],
            ),
        },
        TwExample {
            title: "Grid",
            prose: &["Use the grid utility to create a grid container:"],
            snippet: r#"<div class="grid grid-cols-3 grid-rows-3 gap-4">
<!-- ... -->
</div>"#,
            demo: el(
                "grid grid-cols-3 grid-rows-3 gap-4",
                &[
                    labeled(
                        "size-14 flex items-center justify-center rounded-lg text-white bg-violet-500",
                        "01",
                    ),
                    labeled(
                        "size-14 flex items-center justify-center rounded-lg text-white bg-violet-500",
                        "02",
                    ),
                    labeled(
                        "size-14 flex items-center justify-center rounded-lg text-white bg-violet-500",
                        "03",
                    ),
                    labeled(
                        "size-14 flex items-center justify-center rounded-lg text-white bg-violet-500",
                        "04",
                    ),
                    labeled(
                        "size-14 flex items-center justify-center rounded-lg text-white bg-violet-500",
                        "05",
                    ),
                    labeled(
                        "size-14 flex items-center justify-center rounded-lg text-white bg-violet-500",
                        "06",
                    ),
                    labeled(
                        "size-14 flex items-center justify-center rounded-lg text-white bg-violet-500",
                        "07",
                    ),
                    labeled(
                        "size-14 flex items-center justify-center rounded-lg text-white bg-violet-500",
                        "08",
                    ),
                    labeled(
                        "size-14 flex items-center justify-center rounded-lg text-white bg-violet-500",
                        "09",
                    ),
                ],
            ),
        },
        TwExample {
            title: "Inline Grid",
            prose: &["Use the inline-grid utility to create an inline grid container:"],
            snippet: r#"<span class="inline-grid grid-cols-3 gap-4">
<span>01</span>
<span>02</span>
<span>03</span>
<span>04</span>
<span>05</span>
<span>06</span>
</span>
<span class="inline-grid grid-cols-3 gap-4">
<span>01</span>
<span>02</span>
<span>03</span>
<span>04</span>
<span>05</span>
<span>06</span>
</span>"#,
            demo: el(
                "flex gap-6",
                &[
                    el(
                        "inline-grid grid-cols-3 gap-4",
                        &[
                            labeled(
                                "size-14 flex items-center justify-center rounded-lg text-white bg-violet-500",
                                "01",
                            ),
                            labeled(
                                "size-14 flex items-center justify-center rounded-lg text-white bg-violet-500",
                                "02",
                            ),
                            labeled(
                                "size-14 flex items-center justify-center rounded-lg text-white bg-violet-500",
                                "03",
                            ),
                            labeled(
                                "size-14 flex items-center justify-center rounded-lg text-white bg-violet-500",
                                "04",
                            ),
                            labeled(
                                "size-14 flex items-center justify-center rounded-lg text-white bg-violet-500",
                                "05",
                            ),
                            labeled(
                                "size-14 flex items-center justify-center rounded-lg text-white bg-violet-500",
                                "06",
                            ),
                        ],
                    ),
                    el(
                        "inline-grid grid-cols-3 gap-4",
                        &[
                            labeled(
                                "size-14 flex items-center justify-center rounded-lg text-white bg-violet-500",
                                "01",
                            ),
                            labeled(
                                "size-14 flex items-center justify-center rounded-lg text-white bg-violet-500",
                                "02",
                            ),
                            labeled(
                                "size-14 flex items-center justify-center rounded-lg text-white bg-violet-500",
                                "03",
                            ),
                            labeled(
                                "size-14 flex items-center justify-center rounded-lg text-white bg-violet-500",
                                "04",
                            ),
                            labeled(
                                "size-14 flex items-center justify-center rounded-lg text-white bg-violet-500",
                                "05",
                            ),
                            labeled(
                                "size-14 flex items-center justify-center rounded-lg text-white bg-violet-500",
                                "06",
                            ),
                        ],
                    ),
                ],
            ),
        },
        TwExample {
            title: "Contents",
            prose: &[
                "Use the contents utility to create a \"phantom\" container whose children act like direct children of the parent:",
            ],
            snippet: r#"<div class="flex ...">
<div class="flex-1 ...">01</div>
<div class="contents">
<div class="flex-1 ...">02</div>
<div class="flex-1 ...">03</div>
</div>
<div class="flex-1 ...">04</div>
</div>"#,
            demo: el(
                "flex flex-col gap-2",
                &[
                    el(
                        "flex w-full gap-4",
                        &[
                            labeled(
                                "h-14 flex-1 flex items-center justify-center rounded-lg text-white bg-sky-500",
                                "01",
                            ),
                            labeled(
                                "h-14 flex-1 flex items-center justify-center rounded-lg text-white bg-sky-500",
                                "02",
                            ),
                            labeled(
                                "h-14 flex-1 flex items-center justify-center rounded-lg text-white bg-sky-500",
                                "03",
                            ),
                            labeled(
                                "h-14 flex-1 flex items-center justify-center rounded-lg text-white bg-sky-500",
                                "04",
                            ),
                        ],
                    ),
                    labeled(
                        "text-xs text-slate-500",
                        "contents has no equivalent — the wrapped children are shown flattened into the parent",
                    ),
                ],
            ),
        },
        TwExample {
            title: "Table",
            prose: &[
                "Use the table, table-row, table-cell, table-caption, table-column, table-column-group, table-header-group, table-row-group, and table-footer-group utilities to create elements that behave like their respective table elements:",
            ],
            snippet: r#"<div class="table w-full ...">
<div class="table-header-group ...">
<div class="table-row">
<div class="table-cell text-left ...">Song</div>
<div class="table-cell text-left ...">Artist</div>
<div class="table-cell text-left ...">Year</div>
</div>
</div>
<div class="table-row-group">
<div class="table-row">
<div class="table-cell ...">The Sliding Mr. Bones (Next Stop, Pottersville)</div>
<div class="table-cell ...">Malcolm Lockyer</div>
<div class="table-cell ...">1961</div>
</div>
<div class="table-row">
<div class="table-cell ...">Witchy Woman</div>
<div class="table-cell ...">The Eagles</div>
<div class="table-cell ...">1972</div>
</div>
<div class="table-row">
<div class="table-cell ...">Shining Star</div>
<div class="table-cell ...">Earth, Wind, and Fire</div>
<div class="table-cell ...">1975</div>
</div>
</div>
</div>"#,
            demo: el(
                "flex flex-col gap-2",
                &[
                    el(
                        "grid w-full grid-cols-3 gap-2",
                        &[
                            el("font-semibold", &[Node::Prose("Song")]),
                            el("font-semibold", &[Node::Prose("Artist")]),
                            el("font-semibold", &[Node::Prose("Year")]),
                            Node::Prose("The Sliding Mr. Bones (Next Stop, Pottersville)"),
                            Node::Prose("Malcolm Lockyer"),
                            Node::Prose("1961"),
                            Node::Prose("Witchy Woman"),
                            Node::Prose("The Eagles"),
                            Node::Prose("1972"),
                            Node::Prose("Shining Star"),
                            Node::Prose("Earth, Wind, and Fire"),
                            Node::Prose("1975"),
                        ],
                    ),
                    labeled(
                        "text-xs text-slate-500",
                        "table / table-row / table-cell have no equivalent — a grid is shown",
                    ),
                ],
            ),
        },
        TwExample {
            title: "Hidden",
            prose: &[
                "Use the hidden utility to remove an element from the document:",
                "To visually hide an element but keep it in the document, use the visibility property instead.",
            ],
            snippet: r#"<div class="flex ...">
<div class="hidden ...">01</div>
<div>02</div>
<div>03</div>
</div>"#,
            demo: el(
                "flex gap-4",
                &[
                    labeled(
                        "hidden size-14 flex items-center justify-center rounded-lg text-white bg-sky-500",
                        "01",
                    ),
                    labeled(
                        "size-14 flex items-center justify-center rounded-lg text-white bg-sky-500",
                        "02",
                    ),
                    labeled(
                        "size-14 flex items-center justify-center rounded-lg text-white bg-sky-500",
                        "03",
                    ),
                ],
            ),
        },
        TwExample {
            title: "Screen-reader only",
            prose: &[
                "Use sr-only to hide an element visually without hiding it from screen readers:",
                "Use not-sr-only to undo sr-only, making an element visible to sighted users as well as screen readers:",
                "This can be useful when you want to visually hide something on small screens but show it on larger screens for example.",
            ],
            snippet: "<a href=\"#\">
<svg>
<!-- ... -->
</svg>
<span class=\"sr-only\">Settings</span>
</a>
<a href=\"#\">
<svg>
<!-- ... -->
</svg>
<span class=\"sr-only sm:not-sr-only\">Settings</span>
</a>",
            demo: el(
                "flex flex-col gap-2",
                &[
                    el(
                        "flex items-center gap-2",
                        &[
                            labeled("size-6 rounded-full bg-slate-400", ""),
                            labeled("hidden", "Settings"),
                        ],
                    ),
                    labeled(
                        "text-xs text-slate-500",
                        "sr-only has no equivalent — the label is hidden",
                    ),
                ],
            ),
        },
        TwExample {
            title: "Responsive design",
            prose: &[
                "Prefix a display utility with a breakpoint variant like md: to only apply the utility at medium screen sizes and above:",
                "Learn more about using variants in the variants documentation.",
            ],
            snippet: r#"<div class="flex md:inline-flex ...">
<!-- ... -->
</div>"#,
            demo: el(
                "flex md:inline-flex gap-4",
                &[
                    labeled(
                        "size-14 flex items-center justify-center rounded-lg text-white bg-sky-500",
                        "01",
                    ),
                    labeled(
                        "size-14 flex items-center justify-center rounded-lg text-white bg-sky-500",
                        "02",
                    ),
                    labeled(
                        "size-14 flex items-center justify-center rounded-lg text-white bg-sky-500",
                        "03",
                    ),
                ],
            ),
        },
    ],
};
