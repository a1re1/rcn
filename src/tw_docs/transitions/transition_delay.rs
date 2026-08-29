//! <https://tailwindcss.com/docs/transition-delay>

use crate::tw_docs::demo::{el, labeled};

use crate::tw_docs::{TwExample, TwPage, TwSection};

/// <https://tailwindcss.com/docs/transition-delay>
pub static TRANSITION_DELAY: TwPage = TwPage {
    slug: "transition-delay",
    title: "Transition-delay",
    section: TwSection::Transitions,
    description: "Utilities for controlling the delay of CSS transitions.",
    reference: &[
        ("delay-<number>", "transition-delay: <number>ms;"),
        (
            "delay-(<custom-property>)",
            "transition-delay: var(<custom-property>);",
        ),
        ("delay-[<value>]", "transition-delay: <value>;"),
    ],
    examples: &[
        TwExample {
            title: "Basic example",
            prose: &[
                "Use utilities like delay-150 and delay-700 to set the transition delay of an element in milliseconds:",
                "Hover each button to see the expected behavior",
                "delay-150",
                "delay-300",
                "delay-700",
            ],
            snippet: r#"<button class="transition delay-150 duration-300 ease-in-out ...">Button A</button>
<button class="transition delay-300 duration-300 ease-in-out ...">Button B</button>
<button class="transition delay-700 duration-300 ease-in-out ...">Button C</button>"#,
            demo: el(
                "flex flex-wrap items-center gap-4",
                &[
                    labeled(
                        "rounded-md px-4 py-2 font-semibold text-white bg-blue-500 transition delay-150 duration-300 ease-in-out hover:bg-indigo-500",
                        "Button A",
                    ),
                    labeled(
                        "rounded-md px-4 py-2 font-semibold text-white bg-blue-500 transition delay-300 duration-300 ease-in-out hover:bg-indigo-500",
                        "Button B",
                    ),
                    labeled(
                        "rounded-md px-4 py-2 font-semibold text-white bg-blue-500 transition delay-700 duration-300 ease-in-out hover:bg-indigo-500",
                        "Button C",
                    ),
                ],
            ),
        },
        TwExample {
            title: "Supporting reduced motion",
            prose: &[
                "For situations where the user has specified that they prefer reduced motion, you can conditionally apply animations and transitions using the motion-safe and motion-reduce variants:",
            ],
            snippet: r#"<button type="button" class="delay-300 motion-reduce:delay-0 ...">
<!-- ... -->
</button>"#,
            demo: el(
                "flex flex-col gap-3",
                &[
                    el(
                        "flex flex-wrap items-center gap-4",
                        &[labeled(
                            "rounded-md px-4 py-2 font-semibold text-white bg-blue-500 transition delay-300 duration-300 hover:bg-indigo-500",
                            "Save Changes",
                        )],
                    ),
                    labeled(
                        "text-xs text-slate-500",
                        "motion-reduce:delay-0 has no equivalent",
                    ),
                ],
            ),
        },
        TwExample {
            title: "Using a custom value",
            prose: &[
                "Use the delay-[<value>] syntax to set the transition delay based on a completely custom value:",
                "For CSS variables, you can also use the delay-(<custom-property>) syntax:",
                "This is just a shorthand for delay-[var(<custom-property>)] that adds the var() function for you automatically.",
            ],
            snippet: r#"<button class="delay-[1s,250ms] ...">
<!-- ... -->
</button>
<button class="delay-(--my-delay) ...">
<!-- ... -->
</button>"#,
            demo: el(
                "flex flex-col gap-3",
                &[
                    el(
                        "flex flex-wrap items-center gap-4",
                        &[labeled(
                            "rounded-md px-4 py-2 font-semibold text-white bg-blue-500 transition delay-1000 duration-300 hover:bg-indigo-500",
                            "Save Changes",
                        )],
                    ),
                    labeled("text-xs text-slate-500", "delay-[1s,250ms] ≈ delay-1000"),
                ],
            ),
        },
        TwExample {
            title: "Responsive design",
            prose: &[
                "Prefix a transition-delay utility with a breakpoint variant like md: to only apply the utility at medium screen sizes and above:",
                "Learn more about using variants in the variants documentation.",
            ],
            snippet: r#"<button class="delay-150 md:delay-300 ...">
<!-- ... -->
</button>"#,
            demo: el(
                "flex flex-wrap items-center gap-4",
                &[labeled(
                    "rounded-md px-4 py-2 font-semibold text-white bg-blue-500 transition delay-150 md:delay-300 duration-300 hover:bg-indigo-500",
                    "Save Changes",
                )],
            ),
        },
    ],
};
