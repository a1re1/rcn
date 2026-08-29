//! <https://tailwindcss.com/docs/border-style>

use crate::tw_docs::demo::{el, labeled};

use crate::tw_docs::{TwExample, TwPage, TwSection};

/// <https://tailwindcss.com/docs/border-style>
pub static BORDER_STYLE: TwPage = TwPage {
    slug: "border-style",
    title: "Border-style",
    section: TwSection::Borders,
    description: "Utilities for controlling the style of an element's borders.",
    reference: &[
        ("border-solid", "border-style: solid;"),
        ("border-dashed", "border-style: dashed;"),
        ("border-dotted", "border-style: dotted;"),
        ("border-double", "border-style: double;"),
        ("border-hidden", "border-style: hidden;"),
        ("border-none", "border-style: none;"),
        (
            "divide-solid",
            "& > :not(:last-child) {
  border-style: solid;
}",
        ),
        (
            "divide-dashed",
            "& > :not(:last-child) {
  border-style: dashed;
}",
        ),
        (
            "divide-dotted",
            "& > :not(:last-child) {
  border-style: dotted;
}",
        ),
        (
            "divide-double",
            "& > :not(:last-child) {
  border-style: double;
}",
        ),
        (
            "divide-hidden",
            "& > :not(:last-child) {
  border-style: hidden;
}",
        ),
        (
            "divide-none",
            "& > :not(:last-child) {
  border-style: none;
}",
        ),
    ],
    examples: &[
        TwExample {
            title: "Basic example",
            prose: &[
                "Use utilities like border-solid and border-dotted to control an element's border style:",
                "border-solid",
                "border-dashed",
                "border-dotted",
                "border-double",
            ],
            snippet: r#"<div class="border-2 border-solid ...">
</div>
<div class="border-2 border-dashed ...">
</div>
<div class="border-2 border-dotted ...">
</div>
<div class="border-4 border-double ...">
</div>"#,
            demo: el(
                "flex flex-col gap-3",
                &[
                    el(
                        "flex flex-wrap items-center gap-4",
                        &[
                            labeled(
                                "size-16 rounded-lg bg-white border-2 border-solid border-indigo-500",
                                "",
                            ),
                            labeled(
                                "size-16 rounded-lg bg-white border-2 border-dashed border-indigo-500",
                                "",
                            ),
                        ],
                    ),
                    labeled(
                        "text-xs text-slate-500",
                        "border-dotted and border-double have no equivalent",
                    ),
                ],
            ),
        },
        TwExample {
            title: "Removing a border",
            prose: &[
                "Use the border-none utility to remove an existing border from an element:",
                "This is most commonly used to remove a border style that was applied at a smaller breakpoint.",
            ],
            snippet: r#"<button class="border-none ...">Save Changes</button>"#,
            demo: el(
                "flex flex-wrap items-center gap-4",
                &[labeled(
                    "rounded-md px-4 py-2 font-semibold text-white bg-sky-500 border-none",
                    "Save Changes",
                )],
            ),
        },
        TwExample {
            title: "Setting the divider style",
            prose: &[
                "Use utilities like divide-dashed and divide-dotted to control the border style between child elements:",
            ],
            snippet: r#"<div class="grid grid-cols-3 divide-x-3 divide-dashed divide-indigo-500">
<div>01</div>
<div>02</div>
<div>03</div>
</div>"#,
            demo: el(
                "flex flex-col gap-3",
                &[
                    el(
                        "grid w-96 grid-cols-3 divide-x-4 divide-indigo-500 rounded-lg bg-indigo-100",
                        &[
                            labeled("h-14 flex items-center justify-center", "01"),
                            labeled("h-14 flex items-center justify-center", "02"),
                            labeled("h-14 flex items-center justify-center", "03"),
                        ],
                    ),
                    labeled(
                        "text-xs text-slate-500",
                        "divide-dashed has no equivalent — a solid divider is shown",
                    ),
                ],
            ),
        },
        TwExample {
            title: "Responsive design",
            prose: &[
                "Prefix a border-style utility with a breakpoint variant like md: to only apply the utility at medium screen sizes and above:",
                "Learn more about using variants in the variants documentation.",
            ],
            snippet: r#"<div class="border-solid md:border-dotted ...">
<!-- ... -->
</div>"#,
            demo: el(
                "flex flex-col gap-3",
                &[
                    el(
                        "flex flex-wrap items-center gap-4",
                        &[labeled(
                            "size-16 rounded-lg bg-white border-2 border-solid md:border-dashed border-indigo-500",
                            "",
                        )],
                    ),
                    labeled(
                        "text-xs text-slate-500",
                        "md:border-dotted ≈ md:border-dashed",
                    ),
                ],
            ),
        },
    ],
};
