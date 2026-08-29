//! <https://tailwindcss.com/docs/opacity>

use crate::tw_docs::demo::{el, labeled};

use crate::tw_docs::{TwExample, TwPage, TwSection};

/// <https://tailwindcss.com/docs/opacity>
pub static OPACITY: TwPage = TwPage {
    slug: "opacity",
    title: "Opacity",
    section: TwSection::Effects,
    description: "Utilities for controlling the opacity of an element.",
    reference: &[
        ("opacity-<number>", "opacity: <number>%;"),
        (
            "opacity-(<custom-property>)",
            "opacity: var(<custom-property>);",
        ),
        ("opacity-[<value>]", "opacity: <value>;"),
    ],
    examples: &[
        TwExample {
            title: "Basic example",
            prose: &[
                "Use opacity-<number> utilities like opacity-25 and opacity-100 to set the opacity of an element:",
                "opacity-100",
                "opacity-75",
                "opacity-50",
                "opacity-25",
            ],
            snippet: r#"<button class="bg-indigo-500 opacity-100 ...">
</button>
<button class="bg-indigo-500 opacity-75 ...">
</button>
<button class="bg-indigo-500 opacity-50 ...">
</button>
<button class="bg-indigo-500 opacity-25 ...">
</button>"#,
            demo: el(
                "flex flex-wrap items-center gap-4",
                &[
                    labeled("size-16 rounded-lg bg-indigo-500 opacity-100", ""),
                    labeled("size-16 rounded-lg bg-indigo-500 opacity-75", ""),
                    labeled("size-16 rounded-lg bg-indigo-500 opacity-50", ""),
                    labeled("size-16 rounded-lg bg-indigo-500 opacity-25", ""),
                ],
            ),
        },
        TwExample {
            title: "Applying conditionally",
            prose: &[
                "Prefix an opacity utility with a variant like disabled:* to only apply the utility in that state:",
                "Learn more about using variants in the variants documentation.",
            ],
            snippet: r#"<input class="opacity-100 disabled:opacity-75 ..." type="text" />"#,
            demo: el(
                "flex flex-col gap-3",
                &[
                    labeled(
                        "w-64 rounded-md border border-slate-300 px-3 py-2 opacity-100",
                        "opacity-100 disabled:opacity-75",
                    ),
                    labeled(
                        "text-xs text-slate-500",
                        "disabled: styles apply through tw_stateful on real controls; this box is never disabled",
                    ),
                ],
            ),
        },
        TwExample {
            title: "Using a custom value",
            prose: &[
                "Use the opacity-[<value>] syntax to set the opacity based on a completely custom value:",
                "For CSS variables, you can also use the opacity-(<custom-property>) syntax:",
                "This is just a shorthand for opacity-[var(<custom-property>)] that adds the var() function for you automatically.",
            ],
            snippet: r#"<button class="opacity-[.67] ...">
<!-- ... -->
</button>
<button class="opacity-(--my-opacity) ...">
<!-- ... -->
</button>"#,
            demo: el(
                "flex flex-col gap-3",
                &[
                    el(
                        "flex flex-wrap items-center gap-4",
                        &[labeled("size-16 rounded-lg bg-indigo-500 opacity-70", "")],
                    ),
                    labeled("text-xs text-slate-500", "opacity-[.67] ≈ opacity-70"),
                ],
            ),
        },
        TwExample {
            title: "Responsive design",
            prose: &[
                "Prefix an opacity utility with a breakpoint variant like md: to only apply the utility at medium screen sizes and above:",
                "Learn more about using variants in the variants documentation.",
            ],
            snippet: r#"<button class="opacity-50 md:opacity-100 ...">
<!-- ... -->
</button>"#,
            demo: el(
                "flex flex-wrap items-center gap-4",
                &[labeled(
                    "size-16 rounded-lg bg-indigo-500 opacity-50 md:opacity-100",
                    "",
                )],
            ),
        },
    ],
};
