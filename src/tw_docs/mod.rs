//! Tailwind utility docs, ported page-for-page into the storybook's
//! "Tailwind" section so the `tw` parser's output can be seen live.
//!
//! Each [`TwPage`] mirrors one page of <https://tailwindcss.com/docs>: the
//! quick-reference table (class → CSS, with rcn's support status probed from
//! the real parser at render time) and the page's examples. An example's demo
//! is written in a tiny [`Node`] DSL that transcribes the docs' HTML snippet
//! one element per node, so every class string in a demo is data the tests
//! can feed through the parser — a demo may not contain classes the parser
//! does not understand.
//!
//! Pages live in one module per Tailwind docs section (`spacing`, `sizing`,
//! …) and are registered in [`PAGES`] in the docs' own order.

pub mod demo;
pub mod flex_grid;
pub mod layout;
pub mod sizing;
pub mod spacing;
pub mod typography;

pub use demo::Node;

/// A Tailwind docs sidebar section.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TwSection {
    Layout,
    FlexboxGrid,
    Spacing,
    Sizing,
    Typography,
    Backgrounds,
    Borders,
    Effects,
    Filters,
    Transitions,
    Transforms,
    Interactivity,
    Svg,
}

impl TwSection {
    /// Docs order.
    pub const ALL: [TwSection; 13] = [
        TwSection::Layout,
        TwSection::FlexboxGrid,
        TwSection::Spacing,
        TwSection::Sizing,
        TwSection::Typography,
        TwSection::Backgrounds,
        TwSection::Borders,
        TwSection::Effects,
        TwSection::Filters,
        TwSection::Transitions,
        TwSection::Transforms,
        TwSection::Interactivity,
        TwSection::Svg,
    ];

    pub fn label(self) -> &'static str {
        match self {
            TwSection::Layout => "Layout",
            TwSection::FlexboxGrid => "Flexbox & Grid",
            TwSection::Spacing => "Spacing",
            TwSection::Sizing => "Sizing",
            TwSection::Typography => "Typography",
            TwSection::Backgrounds => "Backgrounds",
            TwSection::Borders => "Borders",
            TwSection::Effects => "Effects",
            TwSection::Filters => "Filters",
            TwSection::Transitions => "Transitions & Animation",
            TwSection::Transforms => "Transforms",
            TwSection::Interactivity => "Interactivity",
            TwSection::Svg => "SVG",
        }
    }
}

/// One row of a page's quick-reference table: `(class, css)`.
pub type Reference = (&'static str, &'static str);

/// One example on a docs page.
pub struct TwExample {
    /// The docs heading, e.g. "Adding padding to one side".
    pub title: &'static str,
    /// The prose paragraphs under the heading, in order.
    pub prose: &'static [&'static str],
    /// The HTML snippet the docs show, verbatim (rendered as a code block).
    pub snippet: &'static str,
    /// The live demo, transcribed from the snippet.
    pub demo: Node,
}

/// One ported Tailwind docs page.
pub struct TwPage {
    /// The docs URL slug, e.g. `padding` → tailwindcss.com/docs/padding.
    pub slug: &'static str,
    /// Sidebar / canvas title, e.g. "Padding".
    pub title: &'static str,
    pub section: TwSection,
    /// The one-line description under the docs title.
    pub description: &'static str,
    /// The quick-reference table, in docs order.
    pub reference: &'static [Reference],
    pub examples: &'static [TwExample],
}

impl TwPage {
    pub fn url(&self) -> String {
        format!("https://tailwindcss.com/docs/{}", self.slug)
    }
}

/// Every ported page, in docs order. Indexed by `Story::Tailwind(usize)`.
pub static PAGES: &[&TwPage] = &[
    &layout::ASPECT_RATIO,
    &layout::DISPLAY,
    &layout::OBJECT_FIT,
    &layout::OVERFLOW,
    &layout::POSITION,
    &layout::TOP_RIGHT_BOTTOM_LEFT,
    &layout::VISIBILITY,
    &flex_grid::FLEX_BASIS,
    &flex_grid::FLEX_DIRECTION,
    &flex_grid::FLEX_WRAP,
    &flex_grid::FLEX,
    &flex_grid::FLEX_GROW,
    &flex_grid::FLEX_SHRINK,
    &flex_grid::GRID_TEMPLATE_COLUMNS,
    &flex_grid::GRID_COLUMN,
    &flex_grid::GRID_TEMPLATE_ROWS,
    &flex_grid::GRID_ROW,
    &flex_grid::GRID_AUTO_FLOW,
    &flex_grid::GRID_AUTO_COLUMNS,
    &flex_grid::GRID_AUTO_ROWS,
    &flex_grid::GAP,
    &flex_grid::JUSTIFY_CONTENT,
    &flex_grid::JUSTIFY_ITEMS,
    &flex_grid::JUSTIFY_SELF,
    &flex_grid::ALIGN_CONTENT,
    &flex_grid::ALIGN_ITEMS,
    &flex_grid::ALIGN_SELF,
    &flex_grid::PLACE_CONTENT,
    &flex_grid::PLACE_ITEMS,
    &flex_grid::PLACE_SELF,
    &spacing::PADDING,
    &spacing::MARGIN,
    &sizing::WIDTH,
    &sizing::MIN_WIDTH,
    &sizing::MAX_WIDTH,
    &sizing::HEIGHT,
    &sizing::MIN_HEIGHT,
    &sizing::MAX_HEIGHT,
    &typography::FONT_SIZE,
    &typography::FONT_STYLE,
    &typography::FONT_WEIGHT,
    &typography::FONT_VARIANT_NUMERIC,
    &typography::LINE_CLAMP,
    &typography::LINE_HEIGHT,
    &typography::TEXT_ALIGN,
    &typography::COLOR,
    &typography::TEXT_DECORATION_LINE,
    &typography::TEXT_DECORATION_COLOR,
    &typography::TEXT_DECORATION_STYLE,
    &typography::TEXT_DECORATION_THICKNESS,
    &typography::TEXT_OVERFLOW,
    &typography::TEXT_WRAP,
    &typography::WHITE_SPACE,
];

/// Pages of one section, with their indices into [`PAGES`].
pub fn pages_in(section: TwSection) -> impl Iterator<Item = (usize, &'static TwPage)> {
    PAGES
        .iter()
        .enumerate()
        .filter(move |(_, page)| page.section == section)
        .map(|(i, page)| (i, *page))
}

/// Turn a quick-reference class like `p-<number>` into a concrete token the
/// parser can be asked about (`p-4`).
pub fn sample_class(reference_class: &str) -> String {
    let mut c = reference_class.trim().to_string();
    for (placeholder, value) in [
        ("<number>", "4"),
        ("<fraction>", "1/2"),
        ("<ratio>", "16/9"),
        ("<color>", "red-500"),
        ("<size>", "sm"),
        ("<percentage>", "50%"),
        ("<angle>", "45"),
        ("[<value>]", "[10px]"),
        ("(<custom-property>)", "(--x)"),
        ("(family-name:<custom-property>)", "(--x)"),
        ("(image:<custom-property>)", "(--x)"),
    ] {
        c = c.replace(placeholder, value);
    }
    // Anything still angle-bracketed is a placeholder we do not model.
    while let (Some(a), Some(b)) = (c.find('<'), c.find('>')) {
        if a < b {
            c.replace_range(a..=b, "x");
        } else {
            break;
        }
    }
    c
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::theme::Theme;
    use crate::tw::{Support, probe};
    use std::collections::BTreeSet;

    #[test]
    fn slugs_are_unique_and_titles_nonempty() {
        let mut seen = BTreeSet::new();
        for page in PAGES {
            assert!(seen.insert(page.slug), "duplicate page slug {}", page.slug);
            assert!(!page.title.is_empty());
            assert!(!page.examples.is_empty(), "{} has no examples", page.slug);
        }
    }

    /// Every class used by a demo must be understood by the parser: demos
    /// exist to show what `tw` renders, so an unknown or skipped class would
    /// silently show nothing.
    #[test]
    fn demo_classes_parse() {
        let theme = Theme::light();
        let mut failures = Vec::new();
        for page in PAGES {
            for example in page.examples {
                example.demo.walk_classes(&mut |classes| {
                    for class in classes.split_whitespace() {
                        match probe(&theme, class) {
                            Support::Supported | Support::Extended => {}
                            other => failures.push(format!(
                                "{} / {}: `{class}` is {other:?}",
                                page.slug, example.title
                            )),
                        }
                    }
                });
            }
        }
        assert!(
            failures.is_empty(),
            "unparsed demo classes:\n{}",
            failures.join("\n")
        );
    }

    /// Each page must document at least one supported class — otherwise it
    /// does not belong in the section.
    #[test]
    fn pages_have_supported_reference_rows() {
        let theme = Theme::light();
        for page in PAGES {
            let supported = page
                .reference
                .iter()
                .filter(|(class, _)| {
                    matches!(
                        probe(&theme, &sample_class(class)),
                        Support::Supported | Support::Extended
                    )
                })
                .count();
            assert!(
                supported > 0,
                "{} has no supported reference rows",
                page.slug
            );
        }
    }

    /// Generated pages start with `demo::TODO_DEMO` placeholders; a page is
    /// only registered once every example has a real demo.
    #[test]
    fn no_placeholder_demos_registered() {
        for page in PAGES {
            for example in page.examples {
                assert!(
                    !matches!(example.demo, Node::Text("TODO: demo")),
                    "{} / {} still has a placeholder demo",
                    page.slug,
                    example.title
                );
            }
        }
    }

    #[test]
    fn sample_class_substitutes_placeholders() {
        assert_eq!(sample_class("p-<number>"), "p-4");
        assert_eq!(sample_class("p-[<value>]"), "p-[10px]");
        assert_eq!(sample_class("p-(<custom-property>)"), "p-(--x)");
        assert_eq!(sample_class("bg-<color>"), "bg-red-500");
        assert_eq!(sample_class("text-<size>/<number>"), "text-sm/4");
    }

    /// Dev tool: classify every quick-reference row of every scraped docs
    /// page (JSON files with `{"rows": [[class, css], …]}`) and print a
    /// markdown summary — used to regenerate docs/tw-docs-checklist.md.
    ///
    /// `TW_DOCS_DIR=/path/to/dir cargo test -p rcn dump_support_report -- --ignored --nocapture`
    #[test]
    #[ignore]
    fn dump_support_report() {
        let Ok(dir) = std::env::var("TW_DOCS_DIR") else {
            eprintln!("TW_DOCS_DIR not set");
            return;
        };
        let theme = Theme::light();
        let mut entries: Vec<_> = std::fs::read_dir(&dir).unwrap().flatten().collect();
        entries.sort_by_key(|e| e.file_name());
        for entry in entries {
            let path = entry.path();
            if path.extension().and_then(|e| e.to_str()) != Some("json") {
                continue;
            }
            let json: serde_json::Value =
                serde_json::from_str(&std::fs::read_to_string(&path).unwrap()).unwrap();
            let rows = json["rows"].as_array().cloned().unwrap_or_default();
            let mut supported = BTreeSet::new();
            let mut total = 0;
            for row in &rows {
                let Some(class) = row.get(0).and_then(|c| c.as_str()) else {
                    continue;
                };
                total += 1;
                if matches!(
                    probe(&theme, &sample_class(class)),
                    Support::Supported | Support::Extended
                ) {
                    supported.insert(class.to_string());
                }
            }
            let slug = path.file_stem().unwrap().to_string_lossy();
            println!(
                "{}\t{}\t{}\t{}",
                slug,
                supported.len(),
                total,
                supported
                    .iter()
                    .take(6)
                    .cloned()
                    .collect::<Vec<_>>()
                    .join(" ")
            );
        }
    }
}
