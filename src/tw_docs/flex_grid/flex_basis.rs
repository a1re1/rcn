//! <https://tailwindcss.com/docs/flex-basis>

use crate::tw_docs::demo::{el, labeled};

use crate::tw_docs::{TwExample, TwPage, TwSection};

/// <https://tailwindcss.com/docs/flex-basis>
pub static FLEX_BASIS: TwPage = TwPage {
    slug: "flex-basis",
    title: "Flex-basis",
    section: TwSection::FlexboxGrid,
    description: "Utilities for controlling the initial size of flex items.",
    reference: &[
        (
            "basis-<number>",
            "flex-basis: calc(var(--spacing) * <number>);",
        ),
        ("basis-<fraction>", "flex-basis: calc(<fraction> * 100%);"),
        ("basis-full", "flex-basis: 100%;"),
        ("basis-auto", "flex-basis: auto;"),
        ("basis-px", "flex-basis: 1px;"),
        (
            "basis-3xs",
            "flex-basis: var(--container-3xs); /* 16rem (256px) */",
        ),
        (
            "basis-2xs",
            "flex-basis: var(--container-2xs); /* 18rem (288px) */",
        ),
        (
            "basis-xs",
            "flex-basis: var(--container-xs); /* 20rem (320px) */",
        ),
        (
            "basis-sm",
            "flex-basis: var(--container-sm); /* 24rem (384px) */",
        ),
        (
            "basis-md",
            "flex-basis: var(--container-md); /* 28rem (448px) */",
        ),
        (
            "basis-lg",
            "flex-basis: var(--container-lg); /* 32rem (512px) */",
        ),
        (
            "basis-xl",
            "flex-basis: var(--container-xl); /* 36rem (576px) */",
        ),
        (
            "basis-2xl",
            "flex-basis: var(--container-2xl); /* 42rem (672px) */",
        ),
        (
            "basis-3xl",
            "flex-basis: var(--container-3xl); /* 48rem (768px) */",
        ),
        (
            "basis-4xl",
            "flex-basis: var(--container-4xl); /* 56rem (896px) */",
        ),
        (
            "basis-5xl",
            "flex-basis: var(--container-5xl); /* 64rem (1024px) */",
        ),
        (
            "basis-6xl",
            "flex-basis: var(--container-6xl); /* 72rem (1152px) */",
        ),
        (
            "basis-7xl",
            "flex-basis: var(--container-7xl); /* 80rem (1280px) */",
        ),
        (
            "basis-(<custom-property>)",
            "flex-basis: var(<custom-property>);",
        ),
        ("basis-[<value>]", "flex-basis: <value>;"),
    ],
    examples: &[
        TwExample {
            title: "Using the spacing scale",
            prose: &[
                "Use basis-<number> utilities like basis-64 and basis-128 to set the initial size of flex items based on the spacing scale:",
            ],
            snippet: r#"<div class="flex flex-row">
<div class="basis-64">01</div>
<div class="basis-64">02</div>
<div class="basis-128">03</div>
</div>"#,
            demo: el(
                "flex w-full flex-row gap-4",
                &[
                    labeled(
                        "basis-64 h-14 flex items-center justify-center rounded-lg text-white bg-fuchsia-500",
                        "01",
                    ),
                    labeled(
                        "basis-64 h-14 flex items-center justify-center rounded-lg text-white bg-fuchsia-500",
                        "02",
                    ),
                    labeled(
                        "basis-128 h-14 flex items-center justify-center rounded-lg text-white bg-fuchsia-500",
                        "03",
                    ),
                ],
            ),
        },
        TwExample {
            title: "Using the container scale",
            prose: &[
                "Use utilities like basis-xs and basis-sm to set the initial size of flex items based on the container scale:",
            ],
            snippet: r#"<div class="flex flex-row">
<div class="basis-3xs">01</div>
<div class="basis-2xs">02</div>
<div class="basis-xs">03</div>
<div class="basis-sm">04</div>
</div>"#,
            demo: el(
                "flex w-full flex-row gap-4",
                &[
                    labeled(
                        "basis-[12rem] h-14 flex items-center justify-center rounded-lg text-white bg-fuchsia-500",
                        "basis-3xs ≈ basis-[12rem]",
                    ),
                    labeled(
                        "basis-[16rem] h-14 flex items-center justify-center rounded-lg text-white bg-fuchsia-500",
                        "basis-2xs ≈ basis-[16rem]",
                    ),
                    labeled(
                        "basis-[20rem] h-14 flex items-center justify-center rounded-lg text-white bg-fuchsia-500",
                        "basis-xs ≈ basis-[20rem]",
                    ),
                    labeled(
                        "basis-[24rem] h-14 flex items-center justify-center rounded-lg text-white bg-fuchsia-500",
                        "basis-sm ≈ basis-[24rem]",
                    ),
                ],
            ),
        },
        TwExample {
            title: "Using percentages",
            prose: &[
                "Use basis-<fraction> utilities like basis-1/2 and basis-2/3 to set the initial size of flex items:",
            ],
            snippet: r#"<div class="flex flex-row">
<div class="basis-1/3">01</div>
<div class="basis-2/3">02</div>
</div>"#,
            demo: el(
                "flex w-full flex-row gap-4",
                &[
                    labeled(
                        "basis-1/3 h-14 flex items-center justify-center rounded-lg text-white bg-fuchsia-500",
                        "basis-1/3",
                    ),
                    labeled(
                        "basis-2/3 h-14 flex items-center justify-center rounded-lg text-white bg-fuchsia-500",
                        "basis-2/3",
                    ),
                ],
            ),
        },
        TwExample {
            title: "Using a custom value",
            prose: &[
                "Use the basis-[<value>] syntax to set the basis based on a completely custom value:",
                "For CSS variables, you can also use the basis-(<custom-property>) syntax:",
                "This is just a shorthand for basis-[var(<custom-property>)] that adds the var() function for you automatically.",
            ],
            snippet: r#"<div class="basis-[30vw] ...">
<!-- ... -->
</div>
<div class="basis-(--my-basis) ...">
<!-- ... -->
</div>"#,
            demo: el(
                "flex w-full flex-row gap-4",
                &[
                    labeled(
                        "basis-[240px] h-14 flex items-center justify-center rounded-lg text-white bg-fuchsia-500",
                        "basis-[30vw] ≈ basis-[240px]",
                    ),
                    labeled(
                        "flex-1 h-14 flex items-center justify-center rounded-lg text-white bg-fuchsia-500",
                        "flex-1",
                    ),
                ],
            ),
        },
        TwExample {
            title: "Responsive design",
            prose: &[
                "Prefix a flex-basis utility with a breakpoint variant like md: to only apply the utility at medium screen sizes and above:",
                "Learn more about using variants in the variants documentation.",
            ],
            snippet: r#"<div class="flex flex-row">
<div class="basis-1/4 md:basis-1/3">01</div>
<div class="basis-1/4 md:basis-1/3">02</div>
<div class="basis-1/2 md:basis-1/3">03</div>
</div>"#,
            demo: el(
                "flex w-full flex-row gap-4",
                &[
                    labeled(
                        "basis-1/4 md:basis-1/3 h-14 flex items-center justify-center rounded-lg text-white bg-fuchsia-500",
                        "01",
                    ),
                    labeled(
                        "basis-1/4 md:basis-1/3 h-14 flex items-center justify-center rounded-lg text-white bg-fuchsia-500",
                        "02",
                    ),
                    labeled(
                        "basis-1/2 md:basis-1/3 h-14 flex items-center justify-center rounded-lg text-white bg-fuchsia-500",
                        "03",
                    ),
                ],
            ),
        },
    ],
};
