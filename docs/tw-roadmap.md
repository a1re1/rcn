# tw gap-support roadmap

Sequenced plan for closing the remaining Tailwind→gpui gaps, beyond the
mapping registry in `src/tw/`. Each phase lands as its own PR with ledger
flips and enforcement-tested coverage. Status legend: ☐ planned, ◐ partial,
☑ landed.

## ☑ Phase 1 — viewport context

`parse_at(theme, viewport, classes)` + `tw_*_at(theme, window, classes)`:

- Responsive variants `sm:` `md:` `lg:` `xl:` `2xl:` gate on window width
  (min-width semantics, resolved at parse time; gpui re-renders on resize).
- Viewport units `w-screen`, `h-dvh`, `size-svw`, … resolve to pixels
  (+48 roots supported).

## ☑ Phase 2 — TwExt channel (element-level utilities)

Widen the parser output beyond `StyleRefinement` with an extension channel
consumed at apply time:

- `space-x/y-*`, `divide-x/y-*` + `divide-<color>` — child-combinator
  spacing/separators via a children-aware container (`tw_div`).
- `object-*`, image `grayscale` — routed to gpui's `img` APIs through
  components that wrap images (Avatar).
- Ledger gains a `Status::Extended` state; enforcement asserts these tokens
  parse into the ext channel (not unknown/skipped).

## ☐ Phase 3 — gpui fork patches (expose what taffy 0.12 already supports)

rcn pins gpui by git rev; point at a patched fork branch (upstreamable):

- `justify_items` / `justify_self` Style fields → `place-*`,
  `justify-items-*`, `justify-self-*` (~38 roots). Plumbing only.
- `grid_auto_flow` (+ auto-track presets) → `grid-flow-*`, `auto-cols/rows-*`
  (~15 roots).
- fit/min/max-content lengths → `w-fit`, `max-h-min`, … (~21 roots; taffy
  `Dimension` supports these natively).

## ☐ Phase 4 — transition engine

Property interpolation for `transition-*` / `duration-*` / `ease-*` (~17
roots) on top of `crate::motion`: track the previous resolved style per
element (`window.use_keyed_state`), interpolate colors/opacity/shadow on
state change. Start with colors + opacity (covers most shadcn usage).

## Deliberately out of scope (renderer work, stays ledgered)

Filters/backdrops, masks, blend modes, conic/radial gradients, text shadows,
general element transforms (scale/rotate) — gpui shader/compositor territory;
revisit only when a component needs one. Text-system items (`tracking-*`,
`text-transform`, `underline-offset`) are medium gpui patches, queued behind
the phases above.
