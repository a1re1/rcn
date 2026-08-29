//! <https://tailwindcss.com/docs/visibility>

use crate::tw_docs::demo::{el, labeled};

use crate::tw_docs::{Node, TwExample, TwPage, TwSection};

/// <https://tailwindcss.com/docs/visibility>
pub static VISIBILITY: TwPage = TwPage {
    slug: "visibility",
    title: "Visibility",
    section: TwSection::Layout,
    description: "Utilities for controlling the visibility of an element.",
    reference: &[
        ("visible", "visibility: visible;"),
        ("invisible", "visibility: hidden;"),
        ("collapse", "visibility: collapse;"),
        ("#100", "Pendant Publishing"),
        ("#101", "Kruger Industrial Smoothing"),
        ("#102", "J. Peterman"),
        ("#100", "Pendant Publishing"),
        ("#101", "Kruger Industrial Smoothing"),
        ("#102", "J. Peterman"),
        ("#100", "Pendant Publishing"),
        ("#101", "Kruger Industrial Smoothing"),
        ("#102", "J. Peterman"),
    ],
    examples: &[
        TwExample {
            title: "Making elements invisible",
            prose: &[
                "Use the invisible utility to hide an element, but still maintain its place in the document, affecting the layout of other elements:",
                "To completely remove an element from the document, use the display property instead.",
            ],
            snippet: r#"<div class="grid grid-cols-3 gap-4">
<div>01</div>
<div class="invisible ...">02</div>
<div>03</div>
</div>"#,
            demo: el(
                "grid grid-cols-3 gap-4",
                &[
                    labeled(
                        "size-14 flex items-center justify-center rounded-lg text-white bg-sky-500",
                        "01",
                    ),
                    labeled(
                        "invisible size-14 flex items-center justify-center rounded-lg text-white bg-sky-500",
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
            title: "Collapsing elements",
            prose: &[
                "Use the collapse utility to hide table rows, row groups, columns, and column groups as if they were set to display: none, but without impacting the size of other rows and columns:",
                "This makes it possible to dynamically toggle rows and columns without affecting the table layout.",
            ],
            snippet: r#"<table>
<thead>
<tr>
<th>Invoice #</th>
<th>Client</th>
<th>Amount</th>
</tr>
</thead>
<tbody>
<tr>
<td>#100</td>
<td>Pendant Publishing</td>
<td>$2,000.00</td>
</tr>
<tr class="collapse">
<td>#101</td>
<td>Kruger Industrial Smoothing</td>
<td>$545.00</td>
</tr>
<tr>
<td>#102</td>
<td>J. Peterman</td>
<td>$10,000.25</td>
</tr>
</tbody>
</table>"#,
            demo: el(
                "flex flex-col gap-2",
                &[
                    el(
                        "grid w-96 grid-cols-3 gap-2",
                        &[
                            el("font-semibold", &[Node::Prose("Invoice #")]),
                            el("font-semibold", &[Node::Prose("Client")]),
                            el("font-semibold", &[Node::Prose("Amount")]),
                            Node::Prose("#100"),
                            Node::Prose("Pendant Publishing"),
                            Node::Prose("$2,000.00"),
                            el("hidden", &[Node::Prose("#101")]),
                            el("hidden", &[Node::Prose("Kruger Industrial Smoothing")]),
                            el("hidden", &[Node::Prose("$545.00")]),
                            Node::Prose("#102"),
                            Node::Prose("Petrossian"),
                            Node::Prose("$1,200.00"),
                        ],
                    ),
                    labeled(
                        "text-xs text-slate-500",
                        "collapse has no equivalent — the #101 row is hidden",
                    ),
                ],
            ),
        },
        TwExample {
            title: "Making elements visible",
            prose: &[
                "Use the visible utility to make an element visible:",
                "This is mostly useful for undoing the invisible utility at different screen sizes.",
            ],
            snippet: r#"<div class="grid grid-cols-3 gap-4">
<div>01</div>
<div class="visible ...">02</div>
<div>03</div>
</div>"#,
            demo: el(
                "grid grid-cols-3 gap-4",
                &[
                    labeled(
                        "size-14 flex items-center justify-center rounded-lg text-white bg-sky-500",
                        "01",
                    ),
                    labeled(
                        "visible size-14 flex items-center justify-center rounded-lg text-white bg-sky-500",
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
            title: "Responsive design",
            prose: &[
                "Prefix a visibility utility with a breakpoint variant like md: to only apply the utility at medium screen sizes and above:",
                "Learn more about using variants in the variants documentation.",
            ],
            snippet: r#"<div class="visible md:invisible ...">
<!-- ... -->
</div>"#,
            demo: el(
                "grid grid-cols-3 gap-4",
                &[
                    labeled(
                        "size-14 flex items-center justify-center rounded-lg text-white bg-sky-500",
                        "01",
                    ),
                    labeled(
                        "visible md:invisible size-14 flex items-center justify-center rounded-lg text-white bg-sky-500",
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
