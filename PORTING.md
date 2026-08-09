# shadcn catalog porting checklist

Tracking the port of the full [shadcn component catalog](https://ui.shadcn.com/docs/components)
to gpui. Reference for every component is the `base-vega` registry source
(`ui.shadcn.com/r/styles/base-vega/<name>.json`). One PR per component; each
lands with a storybook story exercising its variants.

## Wave 0 — foundation (done in #3)

- [x] accordion
- [x] avatar
- [x] badge
- [x] button
- [x] popover
- [x] switch

## Wave 1 — static & simple

- [x] separator
- [x] skeleton
- [x] label
- [x] kbd
- [x] spinner
- [x] aspect-ratio
- [x] card
- [x] alert
- [x] progress
- [x] empty
- [x] item
- [x] table
- [x] breadcrumb

## Wave 2 — interactive controls

- [x] checkbox
- [x] radio-group
- [x] toggle
- [x] toggle-group
- [x] button-group
- [x] collapsible
- [x] tabs
- [x] slider
- [x] pagination
- [x] scroll-area

## Wave 3 — overlays & menus

- [x] tooltip
- [x] hover-card
- [x] dialog
- [x] alert-dialog
- [x] sheet
- [x] drawer
- [x] dropdown-menu
- [x] context-menu
- [x] menubar
- [x] select
- [x] native-select
- [x] navigation-menu
- [x] toast

## Wave 4 — text input

- [x] input
- [x] textarea
- [x] field
- [x] input-group
- [x] input-otp
- [x] command
- [x] combobox

## Wave 5 — composite & complex

- [x] calendar
- [x] carousel
- [x] resizable
- [x] sidebar
- [x] date-picker — no base-vega registry item; composition of calendar + popover, ported as a story once both land
- [x] data-table — no base-vega registry item; docs-only composition over table, ported as a story
- [x] chart — recharts-based upstream; will need a gpui-native adaptation

## Wave 6 — chat / AI components

- [x] attachment
- [x] bubble
- [ ] direction
- [x] marker
- [x] message
- [x] message-scroller
- [ ] questionnaire

## Not ported

- typography — docs page of styled HTML examples, not a registry component;
  covered incidentally by the storybook's own text styles
