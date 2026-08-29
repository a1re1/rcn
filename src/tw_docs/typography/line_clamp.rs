//! <https://tailwindcss.com/docs/line-clamp>

use crate::tw_docs::demo::{el, labeled};

use crate::tw_docs::{Node, TwExample, TwPage, TwSection};

/// <https://tailwindcss.com/docs/line-clamp>
pub static LINE_CLAMP: TwPage = TwPage {
    slug: "line-clamp",
    title: "Line-clamp",
    section: TwSection::Typography,
    description: "Utilities for clamping text to a specific number of lines.",
    reference: &[
        (
            "line-clamp-<number>",
            "overflow: hidden;
display: -webkit-box;
-webkit-box-orient: vertical;
-webkit-line-clamp: <number>;",
        ),
        (
            "line-clamp-none",
            "overflow: visible;
display: block;
-webkit-box-orient: horizontal;
-webkit-line-clamp: unset;",
        ),
        (
            "line-clamp-(<custom-property>)",
            "overflow: hidden;
display: -webkit-box;
-webkit-box-orient: vertical;
-webkit-line-clamp: var(<custom-property>);",
        ),
        (
            "line-clamp-[<value>]",
            "overflow: hidden;
display: -webkit-box;
-webkit-box-orient: vertical;
-webkit-line-clamp: <value>;",
        ),
    ],
    examples: &[
        TwExample {
            title: "Basic example",
            prose: &[
                "Use line-clamp-<number> utilities like line-clamp-2 and line-clamp-3 to truncate multi-line text after a specific number of lines:",
                "Nulla dolor velit adipisicing duis excepteur esse in duis nostrud occaecat mollit incididunt deserunt sunt. Ut ut sunt laborum ex occaecat eu tempor labore enim adipisicing minim ad. Est in quis eu dolore occaecat excepteur fugiat dolore nisi aliqua fugiat enim ut cillum. Labore enim duis nostrud eu. Est ut eiusmod consequat irure quis deserunt ex. Enim laboris dolor magna pariatur. Dolor et ad sint voluptate sunt elit mollit officia ad enim sit consectetur enim.",
            ],
            snippet: r#"<article>
<time>Mar 10, 2020</time>
<h2>Boost your conversion rate</h2>
<p class="line-clamp-3">    Nulla dolor velit adipisicing duis excepteur esse in duis nostrud occaecat mollit incididunt deserunt sunt. Ut ut    sunt laborum ex occaecat eu tempor labore enim adipisicing minim ad. Est in quis eu dolore occaecat excepteur fugiat    dolore nisi aliqua fugiat enim ut cillum. Labore enim duis nostrud eu. Est ut eiusmod consequat irure quis deserunt    ex. Enim laboris dolor magna pariatur. Dolor et ad sint voluptate sunt elit mollit officia ad enim sit consectetur    enim.  </p>
<div>
<img src="/img/lindsay.jpg" />    Lindsay Walton  </div>
</article>"#,
            demo: el(
                "flex w-96 flex-col gap-1",
                &[
                    el("text-xs text-slate-500", &[Node::Prose("Mar 10, 2020")]),
                    el(
                        "font-semibold",
                        &[Node::Prose("Boost your conversion rate")],
                    ),
                    el(
                        "line-clamp-3 text-sm text-slate-600",
                        &[Node::Prose(
                            "Nulla dolor velit adipisicing duis excepteur esse in duis nostrud occaecat mollit incididunt deserunt sunt. Ut ut sunt laborum ex occaecat eu tempor labore enim adipisicing minim ad. Est in quis eu dolore occaecat excepteur fugiat dolore nisi aliqua fugiat enim ut cillum. Aliqua veniam incididunt consequat sunt nostrud aliqua ea sit.",
                        )],
                    ),
                ],
            ),
        },
        TwExample {
            title: "Undoing line clamping",
            prose: &["Use line-clamp-none to undo a previously applied line clamp utility:"],
            snippet: r#"<p class="line-clamp-3 lg:line-clamp-none">
<!-- ... -->
</p>"#,
            demo: el(
                "flex w-96 flex-col gap-2",
                &[
                    el(
                        "line-clamp-3 text-sm",
                        &[Node::Prose(
                            "Nulla dolor velit adipisicing duis excepteur esse in duis nostrud occaecat mollit incididunt deserunt sunt. Ut ut sunt laborum ex occaecat eu tempor labore enim adipisicing minim ad. Est in quis eu dolore occaecat excepteur fugiat dolore nisi aliqua fugiat enim ut cillum. Aliqua veniam incididunt consequat sunt nostrud aliqua ea sit.",
                        )],
                    ),
                    labeled(
                        "text-xs text-slate-500",
                        "lg:line-clamp-none has no equivalent — the clamp stays on",
                    ),
                ],
            ),
        },
        TwExample {
            title: "Using a custom value",
            prose: &[
                "Use the line-clamp-[<value>] syntax to set the number of lines based on a completely custom value:",
                "For CSS variables, you can also use the line-clamp-(<custom-property>) syntax:",
                "This is just a shorthand for line-clamp-[var(<custom-property>)] that adds the var() function for you automatically.",
            ],
            snippet: r#"<p class="line-clamp-[calc(var(--characters)/100)] ...">  Lorem ipsum dolor sit amet...</p>
<p class="line-clamp-(--my-line-count) ...">  Lorem ipsum dolor sit amet...</p>"#,
            demo: el(
                "flex w-96 flex-col gap-2",
                &[
                    el(
                        "line-clamp-2 text-sm",
                        &[Node::Prose(
                            "Nulla dolor velit adipisicing duis excepteur esse in duis nostrud occaecat mollit incididunt deserunt sunt. Ut ut sunt laborum ex occaecat eu tempor labore enim adipisicing minim ad. Est in quis eu dolore occaecat excepteur fugiat dolore nisi aliqua fugiat enim ut cillum. Aliqua veniam incididunt consequat sunt nostrud aliqua ea sit.",
                        )],
                    ),
                    labeled(
                        "text-xs text-slate-500",
                        "line-clamp-[calc(…)] ≈ line-clamp-2",
                    ),
                ],
            ),
        },
        TwExample {
            title: "Responsive design",
            prose: &[
                "Prefix a line-clamp utility with a breakpoint variant like md: to only apply the utility at medium screen sizes and above:",
                "Learn more about using variants in the variants documentation.",
            ],
            snippet: r#"<div class="line-clamp-3 md:line-clamp-4 ...">
<!-- ... -->
</div>"#,
            demo: el(
                "flex w-96 flex-col gap-2",
                &[el(
                    "line-clamp-3 md:line-clamp-4 text-sm",
                    &[Node::Prose(
                        "Nulla dolor velit adipisicing duis excepteur esse in duis nostrud occaecat mollit incididunt deserunt sunt. Ut ut sunt laborum ex occaecat eu tempor labore enim adipisicing minim ad. Est in quis eu dolore occaecat excepteur fugiat dolore nisi aliqua fugiat enim ut cillum. Aliqua veniam incididunt consequat sunt nostrud aliqua ea sit.",
                    )],
                )],
            ),
        },
    ],
};
