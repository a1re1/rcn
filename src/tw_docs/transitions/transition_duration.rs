//! <https://tailwindcss.com/docs/transition-duration>

use crate::tw_docs::demo::{el, labeled};

use crate::tw_docs::{TwExample, TwPage, TwSection};

/// <https://tailwindcss.com/docs/transition-duration>
pub static TRANSITION_DURATION: TwPage = TwPage {
    slug: "transition-duration",
    title: "Transition-duration",
    section: TwSection::Transitions,
    description: "Utilities for controlling the duration of CSS transitions.",
    reference: &[
        ("duration-<number>", "transition-duration: <number>ms;"),
        ("duration-initial", "transition-duration: initial;"),
        (
            "duration-(<custom-property>)",
            "transition-duration: var(<custom-property>);",
        ),
        ("duration-[<value>]", "transition-duration: <value>;"),
    ],
    examples: &[
        TwExample {
            title: "Basic example",
            prose: &[
                "Use utilities like duration-150 and duration-700 to set the transition duration of an element in milliseconds:",
                "Hover each button to see the expected behavior",
                "duration-150",
                "duration-300",
                "duration-700",
            ],
            snippet: r#"<button class="transition duration-150 ease-in-out ...">Button A</button>
<button class="transition duration-300 ease-in-out ...">Button B</button>
<button class="transition duration-700 ease-in-out ...">Button C</button>"#,
            demo: el(
                "flex flex-wrap items-center gap-4",
                &[
                    labeled(
                        "rounded-md px-4 py-2 font-semibold text-white bg-blue-500 transition duration-150 ease-in-out hover:bg-indigo-500",
                        "Button A",
                    ),
                    labeled(
                        "rounded-md px-4 py-2 font-semibold text-white bg-blue-500 transition duration-300 ease-in-out hover:bg-indigo-500",
                        "Button B",
                    ),
                    labeled(
                        "rounded-md px-4 py-2 font-semibold text-white bg-blue-500 transition duration-700 ease-in-out hover:bg-indigo-500",
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
            snippet: r#"<button type="button" class="duration-300 motion-reduce:duration-0 ...">
<!-- ... -->
</button>"#,
            demo: el(
                "flex flex-col gap-3",
                &[
                    el(
                        "flex flex-wrap items-center gap-4",
                        &[labeled(
                            "rounded-md px-4 py-2 font-semibold text-white bg-blue-500 transition duration-300 hover:bg-indigo-500",
                            "Save Changes",
                        )],
                    ),
                    labeled(
                        "text-xs text-slate-500",
                        "motion-reduce:duration-0 has no equivalent",
                    ),
                ],
            ),
        },
        TwExample {
            title: "Using a custom value",
            prose: &[
                "Use the duration-[<value>] syntax to set the transition duration based on a completely custom value:",
                "For CSS variables, you can also use the duration-(<custom-property>) syntax:",
                "This is just a shorthand for duration-[var(<custom-property>)] that adds the var() function for you automatically.",
            ],
            snippet: r#"<button class="duration-[1s,15s] ...">
<!-- ... -->
</button>
<button class="duration-(--my-duration) ...">
<!-- ... -->
</button>"#,
            demo: el(
                "flex flex-col gap-3",
                &[
                    el(
                        "flex flex-wrap items-center gap-4",
                        &[labeled(
                            "rounded-md px-4 py-2 font-semibold text-white bg-blue-500 transition duration-1000 hover:bg-indigo-500",
                            "Save Changes",
                        )],
                    ),
                    labeled(
                        "text-xs text-slate-500",
                        "duration-[1s,15s] ≈ duration-1000",
                    ),
                ],
            ),
        },
        TwExample {
            title: "Responsive design",
            prose: &[
                "Prefix a transition-duration utility with a breakpoint variant like md: to only apply the utility at medium screen sizes and above:",
                "Learn more about using variants in the variants documentation.",
            ],
            snippet: r#"<button class="duration-0 md:duration-150 ...">
<!-- ... -->
</button>"#,
            demo: el(
                "flex flex-wrap items-center gap-4",
                &[labeled(
                    "rounded-md px-4 py-2 font-semibold text-white bg-blue-500 transition duration-0 md:duration-150 hover:bg-indigo-500",
                    "Save Changes",
                )],
            ),
        },
    ],
};
