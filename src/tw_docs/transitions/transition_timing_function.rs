//! <https://tailwindcss.com/docs/transition-timing-function>

use crate::tw_docs::demo::{el, labeled};

use crate::tw_docs::{TwExample, TwPage, TwSection};

/// <https://tailwindcss.com/docs/transition-timing-function>
pub static TRANSITION_TIMING_FUNCTION: TwPage = TwPage {
    slug: "transition-timing-function",
    title: "Transition-timing-function",
    section: TwSection::Transitions,
    description: "Utilities for controlling the easing of CSS transitions.",
    reference: &[
        ("ease-linear", "transition-timing-function: linear;"),
        (
            "ease-in",
            "transition-timing-function: var(--ease-in); /* cubic-bezier(0.4, 0, 1, 1) */",
        ),
        (
            "ease-out",
            "transition-timing-function: var(--ease-out); /* cubic-bezier(0, 0, 0.2, 1) */",
        ),
        (
            "ease-in-out",
            "transition-timing-function: var(--ease-in-out); /* cubic-bezier(0.4, 0, 0.2, 1) */",
        ),
        ("ease-initial", "transition-timing-function: initial;"),
        (
            "ease-(<custom-property>)",
            "transition-timing-function: var(<custom-property>);",
        ),
        ("ease-[<value>]", "transition-timing-function: <value>;"),
    ],
    examples: &[
        TwExample {
            title: "Basic example",
            prose: &[
                "Use utilities like ease-in and ease-out to control the easing curve of an element's transition:",
                "Hover each button to see the expected behavior",
                "ease-in",
                "ease-out",
                "ease-in-out",
            ],
            snippet: r#"<button class="duration-300 ease-in ...">Button A</button>
<button class="duration-300 ease-out ...">Button B</button>
<button class="duration-300 ease-in-out ...">Button C</button>"#,
            demo: el(
                "flex flex-wrap items-center gap-4",
                &[
                    labeled(
                        "rounded-md px-4 py-2 font-semibold text-white bg-blue-500 transition duration-300 ease-in hover:bg-indigo-500",
                        "Button A",
                    ),
                    labeled(
                        "rounded-md px-4 py-2 font-semibold text-white bg-blue-500 transition duration-300 ease-out hover:bg-indigo-500",
                        "Button B",
                    ),
                    labeled(
                        "rounded-md px-4 py-2 font-semibold text-white bg-blue-500 transition duration-300 ease-in-out hover:bg-indigo-500",
                        "Button C",
                    ),
                ],
            ),
        },
        TwExample {
            title: "Using a custom value",
            prose: &[
                "Use the ease-[<value>] syntax to set the transition timing function based on a completely custom value:",
                "For CSS variables, you can also use the ease-(<custom-property>) syntax:",
                "This is just a shorthand for ease-[var(<custom-property>)] that adds the var() function for you automatically.",
            ],
            snippet: r#"<button class="ease-[cubic-bezier(0.95,0.05,0.795,0.035)] ...">
<!-- ... -->
</button>
<button class="ease-(--my-ease) ...">
<!-- ... -->
</button>"#,
            demo: el(
                "flex flex-col gap-3",
                &[
                    el(
                        "flex flex-wrap items-center gap-4",
                        &[labeled(
                            "rounded-md px-4 py-2 font-semibold text-white bg-blue-500 transition duration-300 ease-in-out hover:bg-indigo-500",
                            "Save Changes",
                        )],
                    ),
                    labeled(
                        "text-xs text-slate-500",
                        "ease-[cubic-bezier(…)] has no equivalent — ease-in-out shown",
                    ),
                ],
            ),
        },
        TwExample {
            title: "Responsive design",
            prose: &[
                "Prefix a transition-timing-function utility with a breakpoint variant like md: to only apply the utility at medium screen sizes and above:",
                "Learn more about using variants in the variants documentation.",
            ],
            snippet: r#"<button class="ease-out md:ease-in ...">
<!-- ... -->
</button>"#,
            demo: el(
                "flex flex-wrap items-center gap-4",
                &[labeled(
                    "rounded-md px-4 py-2 font-semibold text-white bg-blue-500 transition duration-300 ease-out md:ease-in hover:bg-indigo-500",
                    "Save Changes",
                )],
            ),
        },
    ],
};
