# tw-docs tooling

Pipeline for porting Tailwind's utility docs into the storybook's Tailwind
section (`src/tw_docs/`). Ground truth lives on tailwindcss.com; these scripts
turn each page into data the storybook renders.

1. **Scrape** — `TW_DOCS_DIR=/tmp/twdocs scripts/tw-docs/scrape.sh` loads every
   utility page in the gstack `browse` headless browser and runs `extract.js`
   in it, writing `<slug>.json` with `{title, desc, rows: [[class, css]…],
   examples: [{title, prose[], code[]}]}`.
2. **Classify** — `TW_DOCS_DIR=/tmp/twdocs cargo test --lib dump_support_report
   -- --ignored --nocapture` prints, per page, how many quick-reference classes
   the parser supports. That decides scope (`docs/tw-docs-checklist.md`).
3. **Skeleton** — `TW_DOCS_DIR=/tmp/twdocs python3 scripts/tw-docs/gen_page.py
   <slug> <TwSection variant>` emits a `TwPage` with the reference table and
   every example's title/prose/snippet filled in and `demo: TODO_DEMO` left to
   hand-write in the `Node` DSL (`src/tw_docs/demo.rs`).
4. Register the page in `src/tw_docs/PAGES`; `cargo test --lib tw_docs` checks
   every demo class parses.
