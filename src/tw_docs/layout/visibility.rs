//! <https://tailwindcss.com/docs/visibility>

use crate::tw_docs::demo::{TODO_DEMO, el, image, labeled};

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
            demo: TODO_DEMO,
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
            demo: TODO_DEMO,
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
            demo: TODO_DEMO,
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
            demo: TODO_DEMO,
        },
    ],
};
