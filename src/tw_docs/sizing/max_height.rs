//! <https://tailwindcss.com/docs/max-height>

use crate::tw_docs::demo::{el, labeled};

use crate::tw_docs::{TwExample, TwPage, TwSection};

/// <https://tailwindcss.com/docs/max-height>
pub static MAX_HEIGHT: TwPage = TwPage {
    slug: "max-height",
    title: "Max-height",
    section: TwSection::Sizing,
    description: "Utilities for setting the maximum height of an element.",
    reference: &[
        (
            "max-h-<number>",
            "max-height: calc(var(--spacing) * <number>);",
        ),
        ("max-h-<fraction>", "max-height: calc(<fraction> * 100%);"),
        ("max-h-none", "max-height: none;"),
        ("max-h-px", "max-height: 1px;"),
        ("max-h-full", "max-height: 100%;"),
        ("max-h-screen", "max-height: 100vh;"),
        ("max-h-dvh", "max-height: 100dvh;"),
        ("max-h-dvw", "max-height: 100dvw;"),
        ("max-h-lvh", "max-height: 100lvh;"),
        ("max-h-lvw", "max-height: 100lvw;"),
        ("max-h-svh", "max-height: 100svh;"),
        ("max-h-svw", "max-height: 100svw;"),
        ("max-h-min", "max-height: min-content;"),
        ("max-h-max", "max-height: max-content;"),
        ("max-h-fit", "max-height: fit-content;"),
        ("max-h-lh", "max-height: 1lh;"),
        (
            "max-h-(<custom-property>)",
            "max-height: var(<custom-property>);",
        ),
        ("max-h-[<value>]", "max-height: <value>;"),
    ],
    examples: &[
        TwExample {
            title: "Basic example",
            prose: &[
                "Use max-h-<number> utilities like max-h-24 and max-h-64 to set an element to a fixed maximum height based on the spacing scale:",
            ],
            snippet: r#"<div class="h-96 ...">
<div class="h-full max-h-80 ...">max-h-80</div>
<div class="h-full max-h-64 ...">max-h-64</div>
<div class="h-full max-h-48 ...">max-h-48</div>
<div class="h-full max-h-40 ...">max-h-40</div>
<div class="h-full max-h-32 ...">max-h-32</div>
<div class="h-full max-h-24 ...">max-h-24</div>
</div>"#,
            demo: el(
                "flex h-96 items-start gap-4",
                &[
                    labeled(
                        "h-full max-h-80 w-16 text-xs flex items-center justify-center rounded-lg text-white bg-sky-500",
                        "max-h-80",
                    ),
                    labeled(
                        "h-full max-h-64 w-16 text-xs flex items-center justify-center rounded-lg text-white bg-sky-500",
                        "max-h-64",
                    ),
                    labeled(
                        "h-full max-h-48 w-16 text-xs flex items-center justify-center rounded-lg text-white bg-sky-500",
                        "max-h-48",
                    ),
                    labeled(
                        "h-full max-h-40 w-16 text-xs flex items-center justify-center rounded-lg text-white bg-sky-500",
                        "max-h-40",
                    ),
                    labeled(
                        "h-full max-h-32 w-16 text-xs flex items-center justify-center rounded-lg text-white bg-sky-500",
                        "max-h-32",
                    ),
                    labeled(
                        "h-full max-h-24 w-16 text-xs flex items-center justify-center rounded-lg text-white bg-sky-500",
                        "max-h-24",
                    ),
                ],
            ),
        },
        TwExample {
            title: "Using a percentage",
            prose: &[
                "Use max-h-full or max-h-<fraction> utilities like max-h-1/2 and max-h-2/5 to give an element a percentage-based maximum height:",
            ],
            snippet: r#"<div class="h-96 ...">
<div class="h-full max-h-9/10 ...">max-h-9/10</div>
<div class="h-full max-h-3/4 ...">max-h-3/4</div>
<div class="h-full max-h-1/2 ...">max-h-1/2</div>
<div class="h-full max-h-1/4 ...">max-h-1/4</div>
<div class="h-full max-h-full ...">max-h-full</div>
</div>"#,
            demo: el(
                "flex h-96 items-start gap-4",
                &[
                    labeled(
                        "h-full max-h-9/10 w-16 text-xs flex items-center justify-center rounded-lg text-white bg-indigo-500",
                        "max-h-9/10",
                    ),
                    labeled(
                        "h-full max-h-3/4 w-16 text-xs flex items-center justify-center rounded-lg text-white bg-indigo-500",
                        "max-h-3/4",
                    ),
                    labeled(
                        "h-full max-h-1/2 w-16 text-xs flex items-center justify-center rounded-lg text-white bg-indigo-500",
                        "max-h-1/2",
                    ),
                    labeled(
                        "h-full max-h-1/4 w-16 text-xs flex items-center justify-center rounded-lg text-white bg-indigo-500",
                        "max-h-1/4",
                    ),
                    labeled(
                        "h-full max-h-full w-16 text-xs flex items-center justify-center rounded-lg text-white bg-indigo-500",
                        "max-h-full",
                    ),
                ],
            ),
        },
        TwExample {
            title: "Using a custom value",
            prose: &[
                "Use the max-h-[<value>] syntax to set the maximum height based on a completely custom value:",
                "For CSS variables, you can also use the max-h-(<custom-property>) syntax:",
                "This is just a shorthand for max-h-[var(<custom-property>)] that adds the var() function for you automatically.",
            ],
            snippet: r#"<div class="max-h-[220px] ...">
<!-- ... -->
</div>
<div class="max-h-(--my-max-height) ...">
<!-- ... -->
</div>"#,
            demo: el(
                "flex h-96 items-start",
                &[labeled(
                    "h-full max-h-[220px] w-32 flex items-center justify-center rounded-lg text-white bg-sky-500",
                    "max-h-[220px]",
                )],
            ),
        },
        TwExample {
            title: "Responsive design",
            prose: &[
                "Prefix a max-height utility with a breakpoint variant like md: to only apply the utility at medium screen sizes and above:",
                "Learn more about using variants in the variants documentation.",
            ],
            snippet: r#"<div class="h-48 max-h-full md:max-h-screen ...">
<!-- ... -->
</div>"#,
            demo: el(
                "flex h-64 items-start rounded-lg bg-sky-300 p-2",
                &[labeled(
                    "h-48 max-h-full md:max-h-screen w-40 text-xs flex items-center justify-center rounded-lg text-white bg-sky-500",
                    "h-48 max-h-full md:max-h-screen",
                )],
            ),
        },
    ],
};
