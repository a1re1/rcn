//! <https://tailwindcss.com/docs/background-image>

use crate::tw_docs::demo::{el, image, labeled};
use crate::tw_docs::{TwExample, TwPage, TwSection};

/// <https://tailwindcss.com/docs/background-image>
pub static BACKGROUND_IMAGE: TwPage = TwPage {
    slug: "background-image",
    title: "Background-image",
    section: TwSection::Backgrounds,
    description: "Utilities for controlling an element's background image.",
    reference: &[
        ("bg-[<value>]", "background-image: <value>;"),
        (
            "bg-(image:<custom-property>)",
            "background-image: var(<custom-property>);",
        ),
        ("bg-none", "background-image: none;"),
        (
            "bg-linear-to-t",
            "background-image: linear-gradient(to top, var(--tw-gradient-stops));",
        ),
        (
            "bg-linear-to-tr",
            "background-image: linear-gradient(to top right, var(--tw-gradient-stops));",
        ),
        (
            "bg-linear-to-r",
            "background-image: linear-gradient(to right, var(--tw-gradient-stops));",
        ),
        (
            "bg-linear-to-br",
            "background-image: linear-gradient(to bottom right, var(--tw-gradient-stops));",
        ),
        (
            "bg-linear-to-b",
            "background-image: linear-gradient(to bottom, var(--tw-gradient-stops));",
        ),
        (
            "bg-linear-to-bl",
            "background-image: linear-gradient(to bottom left, var(--tw-gradient-stops));",
        ),
        (
            "bg-linear-to-l",
            "background-image: linear-gradient(to left, var(--tw-gradient-stops));",
        ),
        (
            "bg-linear-to-tl",
            "background-image: linear-gradient(to top left, var(--tw-gradient-stops));",
        ),
        (
            "bg-linear-<angle>",
            "background-image: linear-gradient(<angle> in oklab, var(--tw-gradient-stops));",
        ),
        (
            "-bg-linear-<angle>",
            "background-image: linear-gradient(-<angle> in oklab, var(--tw-gradient-stops));",
        ),
        (
            "bg-linear-(<custom-property>)",
            "background-image: linear-gradient(var(--tw-gradient-stops, var(<custom-property>)));",
        ),
        (
            "bg-linear-[<value>]",
            "background-image: linear-gradient(var(--tw-gradient-stops, <value>));",
        ),
        (
            "bg-radial",
            "background-image: radial-gradient(in oklab, var(--tw-gradient-stops));",
        ),
        (
            "bg-radial-(<custom-property>)",
            "background-image: radial-gradient(var(--tw-gradient-stops,  var(<custom-property>)));",
        ),
        (
            "bg-radial-[<value>]",
            "background-image: radial-gradient(var(--tw-gradient-stops, <value>));",
        ),
        (
            "bg-conic-<angle>",
            "background-image: conic-gradient(from <angle> in oklab, var(--tw-gradient-stops));",
        ),
        (
            "-bg-conic-<angle>",
            "background-image: conic-gradient(from -<angle> in oklab, var(--tw-gradient-stops));",
        ),
        (
            "bg-conic-(<custom-property>)",
            "background-image: var(<custom-property>);",
        ),
        ("bg-conic-[<value>]", "background-image: <value>;"),
        ("from-<color>", "--tw-gradient-from: <color>;"),
        (
            "from-<percentage>",
            "--tw-gradient-from-position: <percentage>;",
        ),
        (
            "from-(<custom-property>)",
            "--tw-gradient-from: var(<custom-property>);",
        ),
        ("from-[<value>]", "--tw-gradient-from: <value>;"),
        ("via-<color>", "--tw-gradient-via: <color>;"),
        (
            "via-<percentage>",
            "--tw-gradient-via-position: <percentage>;",
        ),
        (
            "via-(<custom-property>)",
            "--tw-gradient-via: var(<custom-property>);",
        ),
        ("via-[<value>]", "--tw-gradient-via: <value>;"),
        ("to-<color>", "--tw-gradient-to: <color>;"),
        (
            "to-<percentage>",
            "--tw-gradient-to-position: <percentage>;",
        ),
        (
            "to-(<custom-property>)",
            "--tw-gradient-to: var(<custom-property>);",
        ),
        ("to-[<value>]", "--tw-gradient-to: <value>;"),
    ],
    examples: &[
        TwExample {
            title: "Basic example",
            prose: &["Use the bg-[<value>] syntax to set the background image of an element:"],
            snippet: r#"<div class="bg-[url(/img/mountains.jpg)] ...">
</div>"#,
            demo: el(
                "flex flex-col gap-3",
                &[
                    image(
                        "h-48 w-96 rounded-lg object-cover",
                        "https://images.unsplash.com/photo-1554629947-334ff61d85dc?w=640&h=360&fit=crop",
                    ),
                    labeled(
                        "text-xs text-slate-500",
                        "bg-[url(…)] has no equivalent — an img is shown",
                    ),
                ],
            ),
        },
        TwExample {
            title: "Adding a linear gradient",
            prose: &[
                "Use utilities like bg-linear-to-r and bg-linear-<angle> with the color stop utilities to add a linear gradient to an element:",
            ],
            snippet: r#"<div class="h-14 bg-linear-to-r from-cyan-500 to-blue-500">
</div>
<div class="h-14 bg-linear-to-t from-sky-500 to-indigo-500">
</div>
<div class="h-14 bg-linear-to-bl from-violet-500 to-fuchsia-500">
</div>
<div class="h-14 bg-linear-65 from-purple-500 to-pink-500">
</div>"#,
            demo: el(
                "flex flex-col gap-3",
                &[
                    el(
                        "h-14 w-96 rounded-lg bg-linear-to-r from-cyan-500 to-blue-500",
                        &[],
                    ),
                    el(
                        "h-14 w-96 rounded-lg bg-linear-to-t from-sky-500 to-indigo-500",
                        &[],
                    ),
                    el(
                        "h-14 w-96 rounded-lg bg-linear-to-bl from-violet-500 to-fuchsia-500",
                        &[],
                    ),
                    el(
                        "h-14 w-96 rounded-lg bg-linear-to-tr from-purple-500 to-pink-500",
                        &[],
                    ),
                    labeled(
                        "text-xs text-slate-500",
                        "bg-linear-65 ≈ bg-linear-to-tr (arbitrary angles have no equivalent)",
                    ),
                ],
            ),
        },
        TwExample {
            title: "Adding a radial gradient",
            prose: &[
                "Use the bg-radial and bg-radial-[<position>] utilities with the color stop utilities to add a radial gradient to an element:",
            ],
            snippet: r#"<div class="size-18 rounded-full bg-radial from-pink-400 from-40% to-fuchsia-700">
</div>
<div class="size-18 rounded-full bg-radial-[at_50%_75%] from-sky-200 via-blue-400 to-indigo-900 to-90%">
</div>
<div class="size-18 rounded-full bg-radial-[at_25%_25%] from-white to-zinc-900 to-75%">
</div>"#,
            demo: el(
                "flex flex-col gap-3",
                &[
                    el(
                        "flex flex-wrap items-center gap-4",
                        &[
                            el(
                                "size-24 rounded-full bg-linear-to-br from-pink-400 to-fuchsia-700",
                                &[],
                            ),
                            el(
                                "size-24 rounded-full bg-linear-to-br from-sky-200 to-indigo-900",
                                &[],
                            ),
                        ],
                    ),
                    labeled(
                        "text-xs text-slate-500",
                        "bg-radial has no equivalent — linear gradients shown",
                    ),
                ],
            ),
        },
        TwExample {
            title: "Adding a conic gradient",
            prose: &[
                "Use the bg-conic and bg-conic-<angle> utilities with the color stop utilities to add a conic gradient to an element:",
            ],
            snippet: r#"<div class="size-24 rounded-full bg-conic from-blue-600 to-sky-400 to-50%">
</div>
<div class="size-24 rounded-full bg-conic-180 from-indigo-600 via-indigo-50 to-indigo-600">
</div>
<div class="size-24 rounded-full bg-conic/decreasing from-violet-700 via-lime-300 to-violet-700">
</div>"#,
            demo: el(
                "flex flex-col gap-3",
                &[
                    el(
                        "flex flex-wrap items-center gap-4",
                        &[
                            el(
                                "size-24 rounded-full bg-linear-to-r from-blue-600 to-sky-400",
                                &[],
                            ),
                            el(
                                "size-24 rounded-full bg-linear-to-r from-indigo-600 to-indigo-50",
                                &[],
                            ),
                        ],
                    ),
                    labeled(
                        "text-xs text-slate-500",
                        "bg-conic has no equivalent — linear gradients shown",
                    ),
                ],
            ),
        },
        TwExample {
            title: "Setting gradient color stops",
            prose: &[
                "Use utilities like from-indigo-500, via-purple-500, and to-pink-500 to set the colors of the gradient stops:",
            ],
            snippet: r#"<div class="bg-linear-to-r from-indigo-500 via-purple-500 to-pink-500 ...">
</div>"#,
            demo: el(
                "flex flex-col gap-3",
                &[
                    el(
                        "h-14 w-96 rounded-lg bg-linear-to-r from-indigo-500 to-pink-500",
                        &[],
                    ),
                    labeled(
                        "text-xs text-slate-500",
                        "via-* stops have no equivalent — gradients are two-stop (from/to)",
                    ),
                ],
            ),
        },
        TwExample {
            title: "Setting gradient stop positions",
            prose: &[
                "Use utilities like from-10%, via-30%, and to-90% to set more precise positions for the gradient color stops:",
            ],
            snippet: r#"<div class="bg-linear-to-r from-indigo-500 from-10% via-sky-500 via-30% to-emerald-500 to-90% ...">
</div>"#,
            demo: el(
                "flex flex-col gap-3",
                &[
                    el(
                        "h-14 w-96 rounded-lg bg-linear-to-r from-indigo-500 to-emerald-500",
                        &[],
                    ),
                    labeled(
                        "text-xs text-slate-500",
                        "from-10% / via-30% / to-90% stop positions have no equivalent",
                    ),
                ],
            ),
        },
        TwExample {
            title: "Changing interpolation mode",
            prose: &[
                "Use the interpolation modifier to control the interpolation mode of a gradient:",
                "srgb",
                "hsl",
                "oklab",
                "oklch",
                "longer",
                "shorter",
                "increasing",
                "decreasing",
                "By default gradients are interpolated in the oklab color space.",
            ],
            snippet: r#"<div class="bg-linear-to-r/srgb from-indigo-500 to-teal-400">
</div>
<div class="bg-linear-to-r/hsl from-indigo-500 to-teal-400">
</div>
<div class="bg-linear-to-r/oklab from-indigo-500 to-teal-400">
</div>
<div class="bg-linear-to-r/oklch from-indigo-500 to-teal-400">
</div>
<div class="bg-linear-to-r/longer from-indigo-500 to-teal-400">
</div>
<div class="bg-linear-to-r/shorter from-indigo-500 to-teal-400">
</div>
<div class="bg-linear-to-r/increasing from-indigo-500 to-teal-400">
</div>
<div class="bg-linear-to-r/decreasing from-indigo-500 to-teal-400">
</div>"#,
            demo: el(
                "flex flex-col gap-3",
                &[
                    el(
                        "h-14 w-96 rounded-lg bg-linear-to-r from-indigo-500 to-teal-400",
                        &[],
                    ),
                    labeled(
                        "text-xs text-slate-500",
                        "/srgb /hsl /oklab /oklch interpolation modes have no equivalent — one gradient shown",
                    ),
                ],
            ),
        },
        TwExample {
            title: "Removing background images",
            prose: &[
                "Use the bg-none utility to remove an existing background image from an element:",
            ],
            snippet: r#"<div class="bg-none">
</div>"#,
            demo: el(
                "flex flex-col gap-3",
                &[
                    el("h-14 w-96 rounded-lg border border-slate-300", &[]),
                    labeled(
                        "text-xs text-slate-500",
                        "bg-none has no equivalent — a box without a background is shown",
                    ),
                ],
            ),
        },
        TwExample {
            title: "Using a custom value",
            prose: &[
                "Use utilities like bg-linear-[<value>] and from-[<value>] to set the gradient based on a completely custom value:",
                "For CSS variables, you can also use the bg-linear-(<custom-property>) syntax:",
                "This is just a shorthand for bg-linear-[var(<custom-property>)] that adds the var() function for you automatically.",
            ],
            snippet: r#"<div class="bg-linear-[25deg,red_5%,yellow_60%,lime_90%,teal] ...">
<!-- ... -->
</div>
<div class="bg-linear-(--my-gradient) ...">
<!-- ... -->
</div>"#,
            demo: el(
                "flex flex-col gap-3",
                &[
                    el(
                        "h-14 w-96 rounded-lg bg-linear-to-r from-red-500 to-teal-500",
                        &[],
                    ),
                    labeled(
                        "text-xs text-slate-500",
                        "bg-linear-[25deg,red 5%,…] has no equivalent — a two-stop gradient shown",
                    ),
                ],
            ),
        },
        TwExample {
            title: "Responsive design",
            prose: &[
                "Prefix a background-image utility with a breakpoint variant like md: to only apply the utility at medium screen sizes and above:",
                "Learn more about using variants in the variants documentation.",
            ],
            snippet: r#"<div class="from-purple-400 md:from-yellow-500 ...">
<!-- ... -->
</div>"#,
            demo: el(
                "flex flex-col gap-3",
                &[el(
                    "h-14 w-96 rounded-lg bg-linear-to-r from-purple-400 md:from-yellow-500 to-pink-500",
                    &[],
                )],
            ),
        },
    ],
};
