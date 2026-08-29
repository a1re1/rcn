# Tailwind docs → storybook checklist

The storybook's **Tailwind** section ports Tailwind's utility docs page-for-page so the `tw` parser's output can be seen live (`src/tw_docs/`). A page is in scope when at least one of its quick-reference classes parses as `Supported`/`Extended` — decided by the parser itself, not by hand (`cargo test dump_support_report -- --ignored --nocapture` with `TW_DOCS_JSON_DIR` pointing at scraped pages; see `scripts/tw-docs/`).

Legend: ☑ ported · ☐ in scope, not yet ported · — out of scope (no supported class). Counts are *supported / total* quick-reference rows.

**Progress: 2 / 69 pages ported.**


## Layout (7 in scope of 19)

| | page | supported rows | PR |
|---|---|---|---|
| ☐ | [aspect-ratio](https://tailwindcss.com/docs/aspect-ratio) | 2 / 6 |  |
| — | [columns](https://tailwindcss.com/docs/columns) | 0 / 17 | |
| — | [break-after](https://tailwindcss.com/docs/break-after) | 0 / 8 | |
| — | [break-before](https://tailwindcss.com/docs/break-before) | 0 / 8 | |
| — | [break-inside](https://tailwindcss.com/docs/break-inside) | 0 / 4 | |
| — | [box-decoration-break](https://tailwindcss.com/docs/box-decoration-break) | 0 / 2 | |
| — | [box-sizing](https://tailwindcss.com/docs/box-sizing) | 0 / 2 | |
| ☐ | [display](https://tailwindcss.com/docs/display) | 6 / 23 |  |
| — | [float](https://tailwindcss.com/docs/float) | 0 / 5 | |
| — | [clear](https://tailwindcss.com/docs/clear) | 0 / 6 | |
| — | [isolation](https://tailwindcss.com/docs/isolation) | 0 / 2 | |
| ☐ | [object-fit](https://tailwindcss.com/docs/object-fit) | 5 / 5 |  |
| — | [object-position](https://tailwindcss.com/docs/object-position) | 0 / 11 | |
| ☐ | [overflow](https://tailwindcss.com/docs/overflow) | 15 / 15 |  |
| — | [overscroll-behavior](https://tailwindcss.com/docs/overscroll-behavior) | 0 / 9 | |
| ☐ | [position](https://tailwindcss.com/docs/position) | 2 / 5 |  |
| ☐ | [top-right-bottom-left](https://tailwindcss.com/docs/top-right-bottom-left) | 70 / 121 |  |
| ☐ | [visibility](https://tailwindcss.com/docs/visibility) | 2 / 12 |  |
| — | [z-index](https://tailwindcss.com/docs/z-index) | 0 / 4 | |

## Flexbox & Grid (23 in scope of 24)

| | page | supported rows | PR |
|---|---|---|---|
| ☐ | [flex-basis](https://tailwindcss.com/docs/flex-basis) | 6 / 20 |  |
| ☐ | [flex-direction](https://tailwindcss.com/docs/flex-direction) | 4 / 4 |  |
| ☐ | [flex-wrap](https://tailwindcss.com/docs/flex-wrap) | 3 / 3 |  |
| ☐ | [flex](https://tailwindcss.com/docs/flex) | 4 / 7 |  |
| ☐ | [flex-grow](https://tailwindcss.com/docs/flex-grow) | 2 / 4 |  |
| ☐ | [flex-shrink](https://tailwindcss.com/docs/flex-shrink) | 2 / 4 |  |
| — | [order](https://tailwindcss.com/docs/order) | 0 / 6 | |
| ☐ | [grid-template-columns](https://tailwindcss.com/docs/grid-template-columns) | 1 / 5 |  |
| ☐ | [grid-column](https://tailwindcss.com/docs/grid-column) | 11 / 19 |  |
| ☐ | [grid-template-rows](https://tailwindcss.com/docs/grid-template-rows) | 1 / 5 |  |
| ☐ | [grid-row](https://tailwindcss.com/docs/grid-row) | 11 / 19 |  |
| ☐ | [grid-auto-flow](https://tailwindcss.com/docs/grid-auto-flow) | 5 / 5 |  |
| ☐ | [grid-auto-columns](https://tailwindcss.com/docs/grid-auto-columns) | 4 / 7 |  |
| ☐ | [grid-auto-rows](https://tailwindcss.com/docs/grid-auto-rows) | 4 / 7 |  |
| ☐ | [gap](https://tailwindcss.com/docs/gap) | 9 / 12 |  |
| ☐ | [justify-content](https://tailwindcss.com/docs/justify-content) | 7 / 11 |  |
| ☐ | [justify-items](https://tailwindcss.com/docs/justify-items) | 4 / 7 |  |
| ☐ | [justify-self](https://tailwindcss.com/docs/justify-self) | 4 / 7 |  |
| ☐ | [align-content](https://tailwindcss.com/docs/align-content) | 7 / 9 |  |
| ☐ | [align-items](https://tailwindcss.com/docs/align-items) | 5 / 8 |  |
| ☐ | [align-self](https://tailwindcss.com/docs/align-self) | 5 / 9 |  |
| ☐ | [place-content](https://tailwindcss.com/docs/place-content) | 7 / 10 |  |
| ☐ | [place-items](https://tailwindcss.com/docs/place-items) | 5 / 7 |  |
| ☐ | [place-self](https://tailwindcss.com/docs/place-self) | 4 / 7 |  |

## Spacing (2 in scope of 2)

| | page | supported rows | PR |
|---|---|---|---|
| ☑ | [padding](https://tailwindcss.com/docs/padding) | 27 / 44 | #TBD |
| ☑ | [margin](https://tailwindcss.com/docs/margin) | 64 / 91 | #TBD |

## Sizing (6 in scope of 12)

| | page | supported rows | PR |
|---|---|---|---|
| ☐ | [width](https://tailwindcss.com/docs/width) | 25 / 46 |  |
| ☐ | [min-width](https://tailwindcss.com/docs/min-width) | 13 / 30 |  |
| ☐ | [max-width](https://tailwindcss.com/docs/max-width) | 13 / 31 |  |
| ☐ | [height](https://tailwindcss.com/docs/height) | 25 / 34 |  |
| ☐ | [min-height](https://tailwindcss.com/docs/min-height) | 13 / 18 |  |
| ☐ | [max-height](https://tailwindcss.com/docs/max-height) | 13 / 18 |  |
| — | [inline-size](https://tailwindcss.com/docs/inline-size) | 0 / 30 | |
| — | [min-inline-size](https://tailwindcss.com/docs/min-inline-size) | 0 / 30 | |
| — | [max-inline-size](https://tailwindcss.com/docs/max-inline-size) | 0 / 30 | |
| — | [block-size](https://tailwindcss.com/docs/block-size) | 0 / 18 | |
| — | [min-block-size](https://tailwindcss.com/docs/min-block-size) | 0 / 18 | |
| — | [max-block-size](https://tailwindcss.com/docs/max-block-size) | 0 / 18 | |

## Typography (15 in scope of 32)

| | page | supported rows | PR |
|---|---|---|---|
| — | [font-family](https://tailwindcss.com/docs/font-family) | 0 / 5 | |
| ☐ | [font-size](https://tailwindcss.com/docs/font-size) | 13 / 15 |  |
| — | [font-smoothing](https://tailwindcss.com/docs/font-smoothing) | 0 / 2 | |
| ☐ | [font-style](https://tailwindcss.com/docs/font-style) | 2 / 2 |  |
| ☐ | [font-weight](https://tailwindcss.com/docs/font-weight) | 9 / 11 |  |
| — | [font-stretch](https://tailwindcss.com/docs/font-stretch) | 0 / 12 | |
| ☐ | [font-variant-numeric](https://tailwindcss.com/docs/font-variant-numeric) | 8 / 9 |  |
| — | [font-feature-settings](https://tailwindcss.com/docs/font-feature-settings) | 0 / 2 | |
| — | [letter-spacing](https://tailwindcss.com/docs/letter-spacing) | 0 / 8 | |
| ☐ | [line-clamp](https://tailwindcss.com/docs/line-clamp) | 1 / 4 |  |
| ☐ | [line-height](https://tailwindcss.com/docs/line-height) | 3 / 7 |  |
| — | [list-style-image](https://tailwindcss.com/docs/list-style-image) | 0 / 3 | |
| — | [list-style-position](https://tailwindcss.com/docs/list-style-position) | 0 / 2 | |
| — | [list-style-type](https://tailwindcss.com/docs/list-style-type) | 0 / 5 | |
| ☐ | [text-align](https://tailwindcss.com/docs/text-align) | 5 / 6 |  |
| ☐ | [color](https://tailwindcss.com/docs/color) | 245 / 293 |  |
| ☐ | [text-decoration-line](https://tailwindcss.com/docs/text-decoration-line) | 3 / 4 |  |
| ☐ | [text-decoration-color](https://tailwindcss.com/docs/text-decoration-color) | 246 / 293 |  |
| ☐ | [text-decoration-style](https://tailwindcss.com/docs/text-decoration-style) | 1 / 5 |  |
| ☐ | [text-decoration-thickness](https://tailwindcss.com/docs/text-decoration-thickness) | 2 / 5 |  |
| — | [text-underline-offset](https://tailwindcss.com/docs/text-underline-offset) | 0 / 5 | |
| — | [text-transform](https://tailwindcss.com/docs/text-transform) | 0 / 4 | |
| ☐ | [text-overflow](https://tailwindcss.com/docs/text-overflow) | 2 / 3 |  |
| ☐ | [text-wrap](https://tailwindcss.com/docs/text-wrap) | 2 / 4 |  |
| — | [text-indent](https://tailwindcss.com/docs/text-indent) | 0 / 6 | |
| — | [tab-size](https://tailwindcss.com/docs/tab-size) | 0 / 3 | |
| — | [vertical-align](https://tailwindcss.com/docs/vertical-align) | 0 / 10 | |
| ☐ | [white-space](https://tailwindcss.com/docs/white-space) | 2 / 6 |  |
| — | [word-break](https://tailwindcss.com/docs/word-break) | 0 / 3 | |
| — | [overflow-wrap](https://tailwindcss.com/docs/overflow-wrap) | 0 / 3 | |
| — | [hyphens](https://tailwindcss.com/docs/hyphens) | 0 / 3 | |
| — | [content](https://tailwindcss.com/docs/content) | 0 / 3 | |

## Backgrounds (2 in scope of 8)

| | page | supported rows | PR |
|---|---|---|---|
| — | [background-attachment](https://tailwindcss.com/docs/background-attachment) | 0 / 3 | |
| — | [background-clip](https://tailwindcss.com/docs/background-clip) | 0 / 4 | |
| ☐ | [background-color](https://tailwindcss.com/docs/background-color) | 245 / 293 |  |
| ☐ | [background-image](https://tailwindcss.com/docs/background-image) | 10 / 34 |  |
| — | [background-origin](https://tailwindcss.com/docs/background-origin) | 0 / 3 | |
| — | [background-position](https://tailwindcss.com/docs/background-position) | 0 / 11 | |
| — | [background-repeat](https://tailwindcss.com/docs/background-repeat) | 0 / 6 | |
| — | [background-size](https://tailwindcss.com/docs/background-size) | 0 / 5 | |

## Borders (4 in scope of 8)

| | page | supported rows | PR |
|---|---|---|---|
| ☐ | [border-radius](https://tailwindcss.com/docs/border-radius) | 165 / 186 |  |
| ☐ | [border-width](https://tailwindcss.com/docs/border-width) | 31 / 54 |  |
| ☐ | [border-color](https://tailwindcss.com/docs/border-color) | 499 / 3516 |  |
| ☐ | [border-style](https://tailwindcss.com/docs/border-style) | 3 / 12 |  |
| — | [outline-width](https://tailwindcss.com/docs/outline-width) | 0 / 4 | |
| — | [outline-color](https://tailwindcss.com/docs/outline-color) | 0 / 293 | |
| — | [outline-style](https://tailwindcss.com/docs/outline-style) | 0 / 6 | |
| — | [outline-offset](https://tailwindcss.com/docs/outline-offset) | 0 / 4 | |

## Effects (2 in scope of 14)

| | page | supported rows | PR |
|---|---|---|---|
| ☐ | [box-shadow](https://tailwindcss.com/docs/box-shadow) | 507 / 1189 |  |
| — | [text-shadow](https://tailwindcss.com/docs/text-shadow) | 0 / 300 | |
| ☐ | [opacity](https://tailwindcss.com/docs/opacity) | 1 / 3 |  |
| — | [mix-blend-mode](https://tailwindcss.com/docs/mix-blend-mode) | 0 / 18 | |
| — | [background-blend-mode](https://tailwindcss.com/docs/background-blend-mode) | 0 / 16 | |
| — | [mask-clip](https://tailwindcss.com/docs/mask-clip) | 0 / 7 | |
| — | [mask-composite](https://tailwindcss.com/docs/mask-composite) | 0 / 4 | |
| — | [mask-image](https://tailwindcss.com/docs/mask-image) | 0 / 115 | |
| — | [mask-mode](https://tailwindcss.com/docs/mask-mode) | 0 / 3 | |
| — | [mask-origin](https://tailwindcss.com/docs/mask-origin) | 0 / 6 | |
| — | [mask-position](https://tailwindcss.com/docs/mask-position) | 0 / 11 | |
| — | [mask-repeat](https://tailwindcss.com/docs/mask-repeat) | 0 / 6 | |
| — | [mask-size](https://tailwindcss.com/docs/mask-size) | 0 / 5 | |
| — | [mask-type](https://tailwindcss.com/docs/mask-type) | 0 / 2 | |

## Filters (1 in scope of 20)

| | page | supported rows | PR |
|---|---|---|---|
| — | [filter](https://tailwindcss.com/docs/filter) | 0 / 3 | |
| — | [filter-blur](https://tailwindcss.com/docs/filter-blur) | 0 / 10 | |
| — | [filter-brightness](https://tailwindcss.com/docs/filter-brightness) | 0 / 3 | |
| — | [filter-contrast](https://tailwindcss.com/docs/filter-contrast) | 0 / 3 | |
| — | [filter-drop-shadow](https://tailwindcss.com/docs/filter-drop-shadow) | 0 / 301 | |
| ☐ | [filter-grayscale](https://tailwindcss.com/docs/filter-grayscale) | 1 / 4 |  |
| — | [filter-hue-rotate](https://tailwindcss.com/docs/filter-hue-rotate) | 0 / 4 | |
| — | [filter-invert](https://tailwindcss.com/docs/filter-invert) | 0 / 4 | |
| — | [filter-saturate](https://tailwindcss.com/docs/filter-saturate) | 0 / 3 | |
| — | [filter-sepia](https://tailwindcss.com/docs/filter-sepia) | 0 / 4 | |
| — | [backdrop-filter](https://tailwindcss.com/docs/backdrop-filter) | 0 / 3 | |
| — | [backdrop-filter-blur](https://tailwindcss.com/docs/backdrop-filter-blur) | 0 / 10 | |
| — | [backdrop-filter-brightness](https://tailwindcss.com/docs/backdrop-filter-brightness) | 0 / 3 | |
| — | [backdrop-filter-contrast](https://tailwindcss.com/docs/backdrop-filter-contrast) | 0 / 3 | |
| — | [backdrop-filter-grayscale](https://tailwindcss.com/docs/backdrop-filter-grayscale) | 0 / 4 | |
| — | [backdrop-filter-hue-rotate](https://tailwindcss.com/docs/backdrop-filter-hue-rotate) | 0 / 4 | |
| — | [backdrop-filter-invert](https://tailwindcss.com/docs/backdrop-filter-invert) | 0 / 4 | |
| — | [backdrop-filter-opacity](https://tailwindcss.com/docs/backdrop-filter-opacity) | 0 / 3 | |
| — | [backdrop-filter-saturate](https://tailwindcss.com/docs/backdrop-filter-saturate) | 0 / 3 | |
| — | [backdrop-filter-sepia](https://tailwindcss.com/docs/backdrop-filter-sepia) | 0 / 4 | |

## Tables (0 in scope of 4)

| | page | supported rows | PR |
|---|---|---|---|
| — | [border-collapse](https://tailwindcss.com/docs/border-collapse) | 0 / 8 | |
| — | [border-spacing](https://tailwindcss.com/docs/border-spacing) | 0 / 15 | |
| — | [table-layout](https://tailwindcss.com/docs/table-layout) | 0 / 8 | |
| — | [caption-side](https://tailwindcss.com/docs/caption-side) | 0 / 8 | |

## Transitions & Animation (4 in scope of 6)

| | page | supported rows | PR |
|---|---|---|---|
| ☐ | [transition-property](https://tailwindcss.com/docs/transition-property) | 5 / 9 |  |
| — | [transition-behavior](https://tailwindcss.com/docs/transition-behavior) | 0 / 2 | |
| ☐ | [transition-duration](https://tailwindcss.com/docs/transition-duration) | 1 / 4 |  |
| ☐ | [transition-timing-function](https://tailwindcss.com/docs/transition-timing-function) | 4 / 7 |  |
| ☐ | [transition-delay](https://tailwindcss.com/docs/transition-delay) | 1 / 3 |  |
| — | [animation](https://tailwindcss.com/docs/animation) | 0 / 7 | |

## Transforms (1 in scope of 11)

| | page | supported rows | PR |
|---|---|---|---|
| — | [backface-visibility](https://tailwindcss.com/docs/backface-visibility) | 0 / 2 | |
| — | [perspective](https://tailwindcss.com/docs/perspective) | 0 / 8 | |
| — | [perspective-origin](https://tailwindcss.com/docs/perspective-origin) | 0 / 11 | |
| — | [rotate](https://tailwindcss.com/docs/rotate) | 0 / 17 | |
| — | [scale](https://tailwindcss.com/docs/scale) | 0 / 18 | |
| — | [skew](https://tailwindcss.com/docs/skew) | 0 / 12 | |
| — | [transform](https://tailwindcss.com/docs/transform) | 0 / 5 | |
| — | [transform-origin](https://tailwindcss.com/docs/transform-origin) | 0 / 11 | |
| — | [transform-style](https://tailwindcss.com/docs/transform-style) | 0 / 2 | |
| ☐ | [translate](https://tailwindcss.com/docs/translate) | 15 / 37 |  |
| — | [zoom](https://tailwindcss.com/docs/zoom) | 0 / 3 | |

## Interactivity (1 in scope of 20)

| | page | supported rows | PR |
|---|---|---|---|
| — | [accent-color](https://tailwindcss.com/docs/accent-color) | 0 / 293 | |
| — | [appearance](https://tailwindcss.com/docs/appearance) | 0 / 2 | |
| — | [caret-color](https://tailwindcss.com/docs/caret-color) | 0 / 293 | |
| — | [color-scheme](https://tailwindcss.com/docs/color-scheme) | 0 / 6 | |
| ☐ | [cursor](https://tailwindcss.com/docs/cursor) | 18 / 38 |  |
| — | [field-sizing](https://tailwindcss.com/docs/field-sizing) | 0 / 2 | |
| — | [pointer-events](https://tailwindcss.com/docs/pointer-events) | 0 / 2 | |
| — | [resize](https://tailwindcss.com/docs/resize) | 0 / 4 | |
| — | [scroll-behavior](https://tailwindcss.com/docs/scroll-behavior) | 0 / 2 | |
| — | [scrollbar-color](https://tailwindcss.com/docs/scrollbar-color) | 0 / 586 | |
| — | [scrollbar-width](https://tailwindcss.com/docs/scrollbar-width) | 0 / 3 | |
| — | [scrollbar-gutter](https://tailwindcss.com/docs/scrollbar-gutter) | 0 / 3 | |
| — | [scroll-margin](https://tailwindcss.com/docs/scroll-margin) | 0 / 66 | |
| — | [scroll-padding](https://tailwindcss.com/docs/scroll-padding) | 0 / 44 | |
| — | [scroll-snap-align](https://tailwindcss.com/docs/scroll-snap-align) | 0 / 4 | |
| — | [scroll-snap-stop](https://tailwindcss.com/docs/scroll-snap-stop) | 0 / 2 | |
| — | [scroll-snap-type](https://tailwindcss.com/docs/scroll-snap-type) | 0 / 6 | |
| — | [touch-action](https://tailwindcss.com/docs/touch-action) | 0 / 10 | |
| — | [user-select](https://tailwindcss.com/docs/user-select) | 0 / 4 | |
| — | [will-change](https://tailwindcss.com/docs/will-change) | 0 / 6 | |

## SVG (1 in scope of 3)

| | page | supported rows | PR |
|---|---|---|---|
| ☐ | [fill](https://tailwindcss.com/docs/fill) | 245 / 294 |  |
| — | [stroke](https://tailwindcss.com/docs/stroke) | 0 / 294 | |
| — | [stroke-width](https://tailwindcss.com/docs/stroke-width) | 0 / 3 | |

## Accessibility (0 in scope of 1)

| | page | supported rows | PR |
|---|---|---|---|
| — | [forced-color-adjust](https://tailwindcss.com/docs/forced-color-adjust) | 0 / 2 | |

## Notes

- `caption-side`, `table-layout`, `border-collapse`, `border-spacing`: the scraped tables are example content, not a quick reference; excluded (0 supported).
- `background-color`, `border-color`, `text-decoration-color`, `fill`, `box-shadow`: the reference table is the whole palette; the storybook shows the first 48 rows.
- Logical properties (`ps`/`ms`/`start-*`) demo the LTR mapping only, matching the parser.
