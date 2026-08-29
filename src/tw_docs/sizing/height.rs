//! <https://tailwindcss.com/docs/height>

use crate::tw_docs::demo::{el, labeled};

use crate::tw_docs::{TwExample, TwPage, TwSection};

/// <https://tailwindcss.com/docs/height>
pub static HEIGHT: TwPage = TwPage {
    slug: "height",
    title: "Height",
    section: TwSection::Sizing,
    description: "Utilities for setting the height of an element.",
    reference: &[
        ("h-<number>", "height: calc(var(--spacing) * <number>);"),
        ("h-<fraction>", "height: calc(<fraction> * 100%);"),
        ("h-auto", "height: auto;"),
        ("h-px", "height: 1px;"),
        ("h-full", "height: 100%;"),
        ("h-screen", "height: 100vh;"),
        ("h-dvh", "height: 100dvh;"),
        ("h-dvw", "height: 100dvw;"),
        ("h-lvh", "height: 100lvh;"),
        ("h-lvw", "height: 100lvw;"),
        ("h-svh", "height: 100svh;"),
        ("h-svw", "height: 100svw;"),
        ("h-min", "height: min-content;"),
        ("h-max", "height: max-content;"),
        ("h-fit", "height: fit-content;"),
        ("h-lh", "height: 1lh;"),
        ("h-(<custom-property>)", "height: var(<custom-property>);"),
        ("h-[<value>]", "height: <value>;"),
        (
            "size-<number>",
            "width: calc(var(--spacing) * <number>);
height: calc(var(--spacing) * <number>);",
        ),
        (
            "size-<fraction>",
            "width: calc(<fraction> * 100%);
height: calc(<fraction> * 100%);",
        ),
        (
            "size-auto",
            "width: auto;
height: auto;",
        ),
        (
            "size-px",
            "width: 1px;
height: 1px;",
        ),
        (
            "size-full",
            "width: 100%;
height: 100%;",
        ),
        (
            "size-dvw",
            "width: 100dvw;
height: 100dvw;",
        ),
        (
            "size-dvh",
            "width: 100dvh;
height: 100dvh;",
        ),
        (
            "size-lvw",
            "width: 100lvw;
height: 100lvw;",
        ),
        (
            "size-lvh",
            "width: 100lvh;
height: 100lvh;",
        ),
        (
            "size-svw",
            "width: 100svw;
height: 100svw;",
        ),
        (
            "size-svh",
            "width: 100svh;
height: 100svh;",
        ),
        (
            "size-min",
            "width: min-content;
height: min-content;",
        ),
        (
            "size-max",
            "width: max-content;
height: max-content;",
        ),
        (
            "size-fit",
            "width: fit-content;
height: fit-content;",
        ),
        (
            "size-(<custom-property>)",
            "width: var(<custom-property>);
height: var(<custom-property>);",
        ),
        (
            "size-[<value>]",
            "width: <value>;
height: <value>;",
        ),
    ],
    examples: &[
        TwExample {
            title: "Basic example",
            prose: &[
                "Use h-<number> utilities like h-24 and h-64 to set an element to a fixed height based on the spacing scale:",
            ],
            snippet: r#"<div class="h-96 ...">h-96</div>
<div class="h-80 ...">h-80</div>
<div class="h-64 ...">h-64</div>
<div class="h-48 ...">h-48</div>
<div class="h-40 ...">h-40</div>
<div class="h-32 ...">h-32</div>
<div class="h-24 ...">h-24</div>"#,
            demo: el(
                "flex items-end gap-4 ",
                &[
                    labeled(
                        "h-96 w-16 text-xs flex items-center justify-center rounded-lg text-white bg-sky-500",
                        "h-96",
                    ),
                    labeled(
                        "h-80 w-16 text-xs flex items-center justify-center rounded-lg text-white bg-sky-500",
                        "h-80",
                    ),
                    labeled(
                        "h-64 w-16 text-xs flex items-center justify-center rounded-lg text-white bg-sky-500",
                        "h-64",
                    ),
                    labeled(
                        "h-48 w-16 text-xs flex items-center justify-center rounded-lg text-white bg-sky-500",
                        "h-48",
                    ),
                    labeled(
                        "h-40 w-16 text-xs flex items-center justify-center rounded-lg text-white bg-sky-500",
                        "h-40",
                    ),
                    labeled(
                        "h-32 w-16 text-xs flex items-center justify-center rounded-lg text-white bg-sky-500",
                        "h-32",
                    ),
                    labeled(
                        "h-24 w-16 text-xs flex items-center justify-center rounded-lg text-white bg-sky-500",
                        "h-24",
                    ),
                ],
            ),
        },
        TwExample {
            title: "Using a percentage",
            prose: &[
                "Use h-full or h-<fraction> utilities like h-1/2 and h-2/5 to give an element a percentage-based height:",
            ],
            snippet: r#"<div class="h-full ...">h-full</div>
<div class="h-9/10 ...">h-9/10</div>
<div class="h-3/4 ...">h-3/4</div>
<div class="h-1/2 ...">h-1/2</div>
<div class="h-1/3 ...">h-1/3</div>"#,
            demo: el(
                "flex h-96 items-end gap-4",
                &[
                    labeled(
                        "h-full w-16 text-xs flex items-center justify-center rounded-lg text-white bg-indigo-500",
                        "h-full",
                    ),
                    labeled(
                        "h-9/10 w-16 text-xs flex items-center justify-center rounded-lg text-white bg-indigo-500",
                        "h-9/10",
                    ),
                    labeled(
                        "h-3/4 w-16 text-xs flex items-center justify-center rounded-lg text-white bg-indigo-500",
                        "h-3/4",
                    ),
                    labeled(
                        "h-1/2 w-16 text-xs flex items-center justify-center rounded-lg text-white bg-indigo-500",
                        "h-1/2",
                    ),
                    labeled(
                        "h-1/3 w-16 text-xs flex items-center justify-center rounded-lg text-white bg-indigo-500",
                        "h-1/3",
                    ),
                ],
            ),
        },
        TwExample {
            title: "Matching viewport",
            prose: &[
                "Use the h-screen utility to make an element span the entire height of the viewport:",
            ],
            snippet: r#"<div class="h-screen">
<!-- ... -->
</div>"#,
            demo: el(
                "h-48 w-full overflow-hidden rounded-lg",
                &[labeled(
                    "h-screen w-full flex items-center justify-center rounded-lg text-white bg-sky-500",
                    "h-screen",
                )],
            ),
        },
        TwExample {
            title: "Matching dynamic viewport",
            prose: &[
                "Use the h-dvh utility to make an element span the entire height of the viewport, which changes as the browser UI expands or contracts:",
                "Scroll the viewport to see the viewport height change",
                "h-dvh",
            ],
            snippet: r#"<div class="h-dvh">
<!-- ... -->
</div>"#,
            demo: el(
                "h-48 w-full overflow-hidden rounded-lg",
                &[labeled(
                    "h-dvh w-full flex items-center justify-center rounded-lg text-white bg-sky-500",
                    "h-dvh",
                )],
            ),
        },
        TwExample {
            title: "Matching large viewport",
            prose: &[
                "Use the h-lvh utility to set an element's height to the largest possible height of the viewport:",
                "Scroll the viewport to see the viewport height change",
                "h-lvh",
            ],
            snippet: r#"<div class="h-lvh">
<!-- ... -->
</div>"#,
            demo: el(
                "h-48 w-full overflow-hidden rounded-lg",
                &[labeled(
                    "h-lvh w-full flex items-center justify-center rounded-lg text-white bg-sky-500",
                    "h-lvh",
                )],
            ),
        },
        TwExample {
            title: "Matching small viewport",
            prose: &[
                "Use the h-svh utility to set an element's height to the smallest possible height of the viewport:",
                "Scroll the viewport to see the viewport height change",
                "h-svh",
            ],
            snippet: r#"<div class="h-svh">
<!-- ... -->
</div>"#,
            demo: el(
                "h-48 w-full overflow-hidden rounded-lg",
                &[labeled(
                    "h-svh w-full flex items-center justify-center rounded-lg text-white bg-sky-500",
                    "h-svh",
                )],
            ),
        },
        TwExample {
            title: "Setting both width and height",
            prose: &[
                "Use utilities like size-px, size-4, and size-full to set both the width and height of an element at the same time:",
            ],
            snippet: r#"<div class="size-16 ...">size-16</div>
<div class="size-20 ...">size-20</div>
<div class="size-24 ...">size-24</div>
<div class="size-32 ...">size-32</div>
<div class="size-40 ...">size-40</div>"#,
            demo: el(
                "flex items-end gap-4 ",
                &[
                    labeled(
                        "size-16 flex items-center justify-center rounded-lg text-white bg-purple-500",
                        "size-16",
                    ),
                    labeled(
                        "size-20 flex items-center justify-center rounded-lg text-white bg-purple-500",
                        "size-20",
                    ),
                    labeled(
                        "size-24 flex items-center justify-center rounded-lg text-white bg-purple-500",
                        "size-24",
                    ),
                    labeled(
                        "size-32 flex items-center justify-center rounded-lg text-white bg-purple-500",
                        "size-32",
                    ),
                    labeled(
                        "size-40 flex items-center justify-center rounded-lg text-white bg-purple-500",
                        "size-40",
                    ),
                ],
            ),
        },
        TwExample {
            title: "Using a custom value",
            prose: &[
                "Use the h-[<value>] syntax to set the height based on a completely custom value:",
                "For CSS variables, you can also use the h-(<custom-property>) syntax:",
                "This is just a shorthand for h-[var(<custom-property>)] that adds the var() function for you automatically.",
            ],
            snippet: r#"<div class="h-[32rem] ...">
<!-- ... -->
</div>
<div class="h-(--my-height) ...">
<!-- ... -->
</div>"#,
            demo: el(
                "h-64 w-full overflow-hidden rounded-lg",
                &[labeled(
                    "h-[32rem] w-full flex items-center justify-center rounded-lg text-white bg-sky-500",
                    "h-[32rem]",
                )],
            ),
        },
        TwExample {
            title: "Responsive design",
            prose: &[
                "Prefix a height utility with a breakpoint variant like md: to only apply the utility at medium screen sizes and above:",
                "Learn more about using variants in the variants documentation.",
            ],
            snippet: r#"<div class="h-1/2 md:h-full ...">
<!-- ... -->
</div>"#,
            demo: el(
                "flex h-48 items-end rounded-lg bg-sky-300 p-2",
                &[labeled(
                    "h-1/2 md:h-full w-32 text-xs flex items-center justify-center rounded-lg text-white bg-sky-500",
                    "h-1/2 md:h-full",
                )],
            ),
        },
    ],
};
