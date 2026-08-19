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

## ◐ Phase 3 — gpui fork patches (expose what taffy 0.12 already supports)

gpui is now pinned to the fork branch `a1re1/zed#rcn-gpui-patches`
(upstream rev 027cf0d + upstreamable patches):

- ☑ `justify_items` / `justify_self` Style fields → `place-*`,
  `justify-items-*`, `justify-self-*` (+24 roots).
- ☑ `grid_auto_flow` → `grid-flow-*` (+5 roots).
- ☑ auto-track presets (`auto-cols/rows-*`) via `GridAutoTrackSize`
  (+10 roots).
- ✗ fit/min/max-content sizing (`w-fit`, `max-h-min`, …): **not portable** —
  investigated and taffy 0.12's `Dimension` only supports length/percent/
  auto/calc; content-based sizes exist only for grid tracks. Stays ledgered
  no-equivalent until taffy grows it upstream.

## ◐ Phase 4 — transition engine

v1 landed: `transition`/`transition-all`/`transition-colors`/
`transition-opacity`/`transition-none`, `duration-*`, `delay-*`, `ease-*`
parse into the ext channel, and `tw_div(...).id(...)` interpolates hover
changes to background/text/border colors and opacity over the requested
curve (`crate::motion::cubic_bezier`, per-flip animation generations via
`use_keyed_state`). Limits, by design: hover bucket only, colors+opacity
only (a property on one side fades through transparent), id required.

Full engine — every element, all buckets, more properties — belongs in the
gpui fork (interpolating inside `Interactivity`'s style resolution) and is
the next candidate patch there.

## Deliberately out of scope (renderer work, stays ledgered)

Filters/backdrops, masks, blend modes, conic/radial gradients, text shadows,
general element transforms (scale/rotate) — gpui shader/compositor territory;
revisit only when a component needs one. Text-system items (`tracking-*`,
`text-transform`, `underline-offset`) are medium gpui patches, queued behind
the phases above.
