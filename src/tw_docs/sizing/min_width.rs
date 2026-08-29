//! <https://tailwindcss.com/docs/min-width>

use crate::tw_docs::demo::{el, labeled};

use crate::tw_docs::{TwExample, TwPage, TwSection};

/// <https://tailwindcss.com/docs/min-width>
pub static MIN_WIDTH: TwPage = TwPage {
    slug: "min-width",
    title: "Min-width",
    section: TwSection::Sizing,
    description: "Utilities for setting the minimum width of an element.",
    reference: &[
        (
            "min-w-<number>",
            "min-width: calc(var(--spacing) * <number>);",
        ),
        ("min-w-<fraction>", "min-width: calc(<fraction> * 100%);"),
        (
            "min-w-3xs",
            "min-width: var(--container-3xs); /* 16rem (256px) */",
        ),
        (
            "min-w-2xs",
            "min-width: var(--container-2xs); /* 18rem (288px) */",
        ),
        (
            "min-w-xs",
            "min-width: var(--container-xs); /* 20rem (320px) */",
        ),
        (
            "min-w-sm",
            "min-width: var(--container-sm); /* 24rem (384px) */",
        ),
        (
            "min-w-md",
            "min-width: var(--container-md); /* 28rem (448px) */",
        ),
        (
            "min-w-lg",
            "min-width: var(--container-lg); /* 32rem (512px) */",
        ),
        (
            "min-w-xl",
            "min-width: var(--container-xl); /* 36rem (576px) */",
        ),
        (
            "min-w-2xl",
            "min-width: var(--container-2xl); /* 42rem (672px) */",
        ),
        (
            "min-w-3xl",
            "min-width: var(--container-3xl); /* 48rem (768px) */",
        ),
        (
            "min-w-4xl",
            "min-width: var(--container-4xl); /* 56rem (896px) */",
        ),
        (
            "min-w-5xl",
            "min-width: var(--container-5xl); /* 64rem (1024px) */",
        ),
        (
            "min-w-6xl",
            "min-width: var(--container-6xl); /* 72rem (1152px) */",
        ),
        (
            "min-w-7xl",
            "min-width: var(--container-7xl); /* 80rem (1280px) */",
        ),
        ("min-w-auto", "min-width: auto;"),
        ("min-w-px", "min-width: 1px;"),
        ("min-w-full", "min-width: 100%;"),
        ("min-w-screen", "min-width: 100vw;"),
        ("min-w-dvw", "min-width: 100dvw;"),
        ("min-w-dvh", "min-width: 100dvh;"),
        ("min-w-lvw", "min-width: 100lvw;"),
        ("min-w-lvh", "min-width: 100lvh;"),
        ("min-w-svw", "min-width: 100svw;"),
        ("min-w-svh", "min-width: 100svh;"),
        ("min-w-min", "min-width: min-content;"),
        ("min-w-max", "min-width: max-content;"),
        ("min-w-fit", "min-width: fit-content;"),
        (
            "min-w-(<custom-property>)",
            "min-width: var(<custom-property>);",
        ),
        ("min-w-[<value>]", "min-width: <value>;"),
    ],
    examples: &[
        TwExample {
            title: "Basic example",
            prose: &[
                "Use min-w-<number> utilities like min-w-24 and min-w-64 to set an element to a fixed minimum width based on the spacing scale:",
            ],
            snippet: r#"<div class="w-20 ...">
<div class="min-w-80 ...">min-w-80</div>
<div class="min-w-64 ...">min-w-64</div>
<div class="min-w-48 ...">min-w-48</div>
<div class="min-w-40 ...">min-w-40</div>
<div class="min-w-32 ...">min-w-32</div>
<div class="min-w-24 ...">min-w-24</div>
</div>"#,
            demo: el(
                "flex w-20 flex-col items-start gap-2",
                &[
                    labeled(
                        "min-w-80 px-4 py-2 flex items-center justify-center rounded-lg text-white bg-sky-500",
                        "min-w-80",
                    ),
                    labeled(
                        "min-w-64 px-4 py-2 flex items-center justify-center rounded-lg text-white bg-sky-500",
                        "min-w-64",
                    ),
                    labeled(
                        "min-w-48 px-4 py-2 flex items-center justify-center rounded-lg text-white bg-sky-500",
                        "min-w-48",
                    ),
                    labeled(
                        "min-w-40 px-4 py-2 flex items-center justify-center rounded-lg text-white bg-sky-500",
                        "min-w-40",
                    ),
                    labeled(
                        "min-w-32 px-4 py-2 flex items-center justify-center rounded-lg text-white bg-sky-500",
                        "min-w-32",
                    ),
                    labeled(
                        "min-w-24 px-4 py-2 flex items-center justify-center rounded-lg text-white bg-sky-500",
                        "min-w-24",
                    ),
                ],
            ),
        },
        TwExample {
            title: "Using a percentage",
            prose: &[
                "Use min-w-full or min-w-<fraction> utilities like min-w-1/2 and min-w-2/5 to give an element a percentage-based minimum width:",
            ],
            snippet: r#"<div class="flex ...">
<div class="min-w-3/4 ...">min-w-3/4</div>
<div class="w-full ...">w-full</div>
</div>"#,
            demo: el(
                "flex w-96 gap-2",
                &[
                    labeled(
                        "min-w-3/4 py-2 flex items-center justify-center rounded-lg text-white bg-indigo-500",
                        "min-w-3/4",
                    ),
                    labeled("w-full rounded-lg bg-indigo-300 py-2 text-center", "w-full"),
                ],
            ),
        },
        TwExample {
            title: "Using the container scale",
            prose: &[
                "Use utilities like min-w-sm and min-w-xl to set an element to a fixed minimum width based on the container scale:",
            ],
            snippet: r#"<div class="w-40 ...">
<div class="min-w-lg ...">min-w-lg</div>
<div class="min-w-md ...">min-w-md</div>
<div class="min-w-sm ...">min-w-sm</div>
<div class="min-w-xs ...">min-w-xs</div>
<div class="min-w-2xs ...">min-w-2xs</div>
<div class="min-w-3xs ...">min-w-3xs</div>
</div>"#,
            demo: el(
                "flex w-40 flex-col items-start gap-2",
                &[
                    labeled(
                        "min-w-[32rem] px-4 py-2 flex items-center justify-center rounded-lg text-white bg-purple-500",
                        "min-w-lg ≈ min-w-[32rem]",
                    ),
                    labeled(
                        "min-w-[28rem] px-4 py-2 flex items-center justify-center rounded-lg text-white bg-purple-500",
                        "min-w-md ≈ min-w-[28rem]",
                    ),
                    labeled(
                        "min-w-[24rem] px-4 py-2 flex items-center justify-center rounded-lg text-white bg-purple-500",
                        "min-w-sm ≈ min-w-[24rem]",
                    ),
                    labeled(
                        "min-w-[20rem] px-4 py-2 flex items-center justify-center rounded-lg text-white bg-purple-500",
                        "min-w-xs ≈ min-w-[20rem]",
                    ),
                    labeled(
                        "min-w-[16rem] px-4 py-2 flex items-center justify-center rounded-lg text-white bg-purple-500",
                        "min-w-2xs ≈ min-w-[16rem]",
                    ),
                    labeled(
                        "min-w-[12rem] px-4 py-2 flex items-center justify-center rounded-lg text-white bg-purple-500",
                        "min-w-3xs ≈ min-w-[12rem]",
                    ),
                ],
            ),
        },
        TwExample {
            title: "Using a custom value",
            prose: &[
                "Use the min-w-[<value>] syntax to set the minimum width based on a completely custom value:",
                "For CSS variables, you can also use the min-w-(<custom-property>) syntax:",
                "This is just a shorthand for min-w-[var(<custom-property>)] that adds the var() function for you automatically.",
            ],
            snippet: r#"<div class="min-w-[220px] ...">
<!-- ... -->
</div>
<div class="min-w-(--my-min-width) ...">
<!-- ... -->
</div>"#,
            demo: el(
                "flex flex-col items-start gap-2 ",
                &[labeled(
                    "min-w-[220px] px-4 py-2 flex items-center justify-center rounded-lg text-white bg-sky-500",
                    "min-w-[220px]",
                )],
            ),
        },
        TwExample {
            title: "Responsive design",
            prose: &[
                "Prefix a min-width utility with a breakpoint variant like md: to only apply the utility at medium screen sizes and above:",
                "Learn more about using variants in the variants documentation.",
            ],
            snippet: r#"<div class="w-24 min-w-full md:min-w-0 ...">
<!-- ... -->
</div>"#,
            demo: el(
                "w-96 rounded-lg bg-sky-300 p-2",
                &[labeled(
                    "w-24 min-w-full md:min-w-0 px-4 py-2 flex items-center justify-center rounded-lg text-white bg-sky-500",
                    "w-24 min-w-full md:min-w-0",
                )],
            ),
        },
    ],
};
