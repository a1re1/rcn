//! <https://tailwindcss.com/docs/transition-property>

use crate::tw_docs::demo::{el, labeled};

use crate::tw_docs::{TwExample, TwPage, TwSection};

/// <https://tailwindcss.com/docs/transition-property>
pub static TRANSITION_PROPERTY: TwPage = TwPage {
    slug: "transition-property",
    title: "Transition-property",
    section: TwSection::Transitions,
    description: "Utilities for controlling which CSS properties transition.",
    reference: &[
        ("transition", "transition-property: color, background-color, border-color, outline-color, text-decoration-color, fill, stroke, --tw-gradient-from, --tw-gradient-via, --tw-gradient-to, opacity, box-shadow, transform, translate, scale, rotate, filter, -webkit-backdrop-filter, backdrop-filter, display, content-visibility, overlay, pointer-events;
transition-timing-function: var(--default-transition-timing-function); /* cubic-bezier(0.4, 0, 0.2, 1) */
transition-duration: var(--default-transition-duration); /* 150ms */"),
        ("transition-all", "transition-property: all;
transition-timing-function: var(--default-transition-timing-function); /* cubic-bezier(0.4, 0, 0.2, 1) */
transition-duration: var(--default-transition-duration); /* 150ms */"),
        ("transition-colors", "transition-property: color, background-color, border-color, outline-color, text-decoration-color, fill, stroke, --tw-gradient-from, --tw-gradient-via, --tw-gradient-to;
transition-timing-function: var(--default-transition-timing-function); /* cubic-bezier(0.4, 0, 0.2, 1) */
transition-duration: var(--default-transition-duration); /* 150ms */"),
        ("transition-opacity", "transition-property: opacity;
transition-timing-function: var(--default-transition-timing-function); /* cubic-bezier(0.4, 0, 0.2, 1) */
transition-duration: var(--default-transition-duration); /* 150ms */"),
        ("transition-shadow", "transition-property: box-shadow;
transition-timing-function: var(--default-transition-timing-function); /* cubic-bezier(0.4, 0, 0.2, 1) */
transition-duration: var(--default-transition-duration); /* 150ms */"),
        ("transition-transform", "transition-property: transform, translate, scale, rotate;
transition-timing-function: var(--default-transition-timing-function); /* cubic-bezier(0.4, 0, 0.2, 1) */
transition-duration: var(--default-transition-duration); /* 150ms */"),
        ("transition-none", "transition-property: none;"),
        ("transition-(<custom-property>)", "transition-property: var(<custom-property>);
transition-timing-function: var(--default-transition-timing-function); /* cubic-bezier(0.4, 0, 0.2, 1) */
transition-duration: var(--default-transition-duration); /* 150ms */"),
        ("transition-[<value>]", "transition-property: <value>;
transition-timing-function: var(--default-transition-timing-function); /* cubic-bezier(0.4, 0, 0.2, 1) */
transition-duration: var(--default-transition-duration); /* 150ms */"),
    ],
    examples: &[
        TwExample {
            title: "Basic example",
            prose: &[
                "Use utilities like transition and transition-colors to specify which properties should transition when they change:",
                "Hover the button to see the expected behavior",
            ],
            snippet: r#"<button class="bg-blue-500 transition delay-150 duration-300 ease-in-out hover:-translate-y-1 hover:scale-110 hover:bg-indigo-500 ...">  Save Changes</button>"#,
            demo: el("flex flex-col gap-3", &[
                el("flex flex-wrap items-center gap-4", &[
                labeled("rounded-md px-4 py-2 font-semibold text-white bg-blue-500 transition delay-150 duration-300 ease-in-out hover:bg-indigo-500", "Save Changes"),
            ]),
                labeled("text-xs text-slate-500", "hover:-translate-y-1 and hover:scale-110 have no equivalent — the color transition is shown (hover it)"),
            ]),
        },
        TwExample {
            title: "Supporting reduced motion",
            prose: &[
                "For situations where the user has specified that they prefer reduced motion, you can conditionally apply animations and transitions using the motion-safe and motion-reduce variants:",
            ],
            snippet: r#"<button class="transform transition hover:-translate-y-1 motion-reduce:transition-none motion-reduce:hover:transform-none ...">
<!-- ... -->
</button>"#,
            demo: el("flex flex-col gap-3", &[
                el("flex flex-wrap items-center gap-4", &[
                labeled("rounded-md px-4 py-2 font-semibold text-white bg-blue-500 transition hover:bg-indigo-500", "Save Changes"),
            ]),
                labeled("text-xs text-slate-500", "motion-reduce: variants have no equivalent"),
            ]),
        },
        TwExample {
            title: "Using a custom value",
            prose: &[
                "Use the transition-[<value>] syntax to set the transition properties based on a completely custom value:",
                "For CSS variables, you can also use the transition-(<custom-property>) syntax:",
                "This is just a shorthand for transition-[var(<custom-property>)] that adds the var() function for you automatically.",
            ],
            snippet: r#"<button class="transition-[height] ...">
<!-- ... -->
</button>
<button class="transition-(--my-properties) ...">
<!-- ... -->
</button>"#,
            demo: el("flex flex-col gap-3", &[
                el("flex flex-wrap items-center gap-4", &[
                labeled("rounded-md px-4 py-2 font-semibold text-white bg-blue-500 transition hover:bg-indigo-500", "Save Changes"),
            ]),
                labeled("text-xs text-slate-500", "transition-[height] has no equivalent — transition (colors) shown"),
            ]),
        },
        TwExample {
            title: "Responsive design",
            prose: &[
                "Prefix a transition-property utility with a breakpoint variant like md: to only apply the utility at medium screen sizes and above:",
                "Learn more about using variants in the variants documentation.",
            ],
            snippet: r#"<button class="transition-none md:transition-all ...">
<!-- ... -->
</button>"#,
            demo: el("flex flex-wrap items-center gap-4", &[
                labeled("rounded-md px-4 py-2 font-semibold text-white bg-blue-500 transition-none md:transition-all hover:bg-indigo-500", "Save Changes"),
            ]),
        },
    ],
};
