# shadcn parity audit checklist

Second pass over every component: audit against the real shadcn source
(pulled via `scripts/pull-shadcn.sh <name>` into the reference project) plus
the libraries underneath it (`@base-ui-components/react` behavior,
`tw-animate-css` motion), then close the gaps. One PR per component.

Each audit covers:
- **Motion** — animations/transitions with tw-animate-css timing and easing
  (enter: 150ms ease; accordion/collapsible: 200ms ease-out; popovers:
  100ms; hover transitions: 150ms cubic-bezier(0.4,0,0.2,1)). gpui animates
  on mount, so enter animations are ported and exit animations are noted
  TODOs.
- **Accessibility** — keyboard focus (`tab_index`), the shadcn
  focus-visible ring (`motion::focus_ring`), Enter/Space activation,
  Escape-to-close on overlays, disabled semantics.
- **Contract** — the props/variants the shadcn component supports, exposed
  as builders and mirrored in the storybook controls panel.

## Infrastructure

- [x] `src/motion.rs` — cubic-bezier easing (browser-verified), tw-animate
  durations, focus-ring shadows
- [x] `scripts/pull-shadcn.sh` — pull any component's real source via the
  shadcn CLI into `scratchpad`'s reference project
- [x] Reference project with `@base-ui-components/react` + `tw-animate-css`
  sources on disk

## Components — audited bottom-up: atoms first, so their contracts are
solid before the molecules that compose them.

### Tier 1 — atoms

- [x] button — audited in the infrastructure PR (keyboard focus + ring)
- [x] badge — (static presentation — matches source; focus-visible/aria-invalid omitted; as-child link not ported)
- [x] label — (matches base-nova source; peer/group disabled via explicit `.disabled` (opacity only — pointer-events/cursor omitted); docs examples: checkbox demo + Label-in-Field in storybook; RTL omitted)
- [x] badge — (variants, link/interactive badge with focus ring + [a]:hover styles, icon/spinner padding trim, custom color overrides, all docs examples; omitted: RTL, hover transition animation, aria-invalid, underline-offset)
- [x] label — (static presentation — matches source; peer/group disabled via explicit `.disabled`)
- [x] kbd — (tooltip-context styles via in_tooltip(); all six shadcn doc examples in the storybook, RTL mirrored manually)
- [x] separator — (static presentation — matches source, no interactive surface)
- [x] skeleton — Styled sizing passthrough (className equivalent), animate-pulse wired (2s opacity breathing, cosine approx of Tailwind's cubic-bezier(0.4,0,0.6,1)); docs examples: demo, avatar, card, text, form, table (RTL omitted — no direction infra)
- [x] spinner — (1s rotate animation; matches source spin)
- [x] aspect-ratio — (static presentation — matches source, no interactive surface)
- [x] avatar — (static presentation — matches source; image+fallback+group)
- [x] checkbox — (keyboard focus + ring)
- [x] radio-group — (keyboard focus + ring)
- [x] switch — (keyboard focus + ring)
- [x] toggle — base-nova parity (h-8/h-7/h-9 metrics, rounded-lg + sm radius/text/icon overrides, no outline shadow, full-muted hover, uncontrolled default_pressed, on_pressed_change, icon_inline_start/end padding; all five docs examples incl. pressed bookmark fill; RTL, aria-invalid, group `value`, and hover transition-all omitted)
- [x] slider — keyboard focus ring + arrow-key steps
- [x] progress — (static bar; width transition on value change TODO; indeterminate TODO)
- [x] input — full focus ring shadow (was border-only)
- [x] textarea — base-nova parity (rounded-lg px-2.5 no-shadow shell; disabled + invalid states with destructive ring rendered as a border overlay, not a see-through box shadow; corner drag-resize grip like the native `resize`; min-h-16 default; all docs examples; RTL, field-sizing-content auto-grow, and multi-line editing omitted/TODO)

### Tier 2 — molecules on the atoms

- [x] accordion — Base UI root state (multiple / controlled / uncontrolled),
  root + item disabled, bordered variant, animated expand/collapse height
  (200ms ease-out); RTL out of scope
- [x] alert — (static presentation — matches source; parent-context destructive tint TODO)
- [x] card — base-nova parity (footer border/bg + pb-0 via mb(-spacing); header `.action()` slot; `.spacing()` override; `.flush_top()`/`.flush_bottom()`; no shadow; RTL and `[.border-b]`/`[.border-t]` utilities omitted)
- [x] empty — (static presentation — matches source; dashed border approximated solid)
- [x] item — (static presentation — matches source layout variants)
- [x] field — base-nova parity (all ten parts incl. FieldLabel/FieldContent/FieldTitle/FieldSeparator; vertical/horizontal orientations + responsive via measured container width (`container_query`, @md 448px); `.invalid()` destructive cascade; choice cards via `FieldLabel::choice_card(checked)` with dark-mode primary tints; `FieldError::errors()` dedup + bullet list; browser-measured legend/description spacing; RTL and aria semantics omitted)
- [x] input-group — focus ring shadow on the shell
- [x] input-otp — (slot presentation + click-to-focus; per-slot caret/paste keyboard TODO)
- [x] button-group — (static presentation — layout wrapper; children own interaction)
- [x] toggle-group — (keyboard focus + ring)
- [x] tabs — (keyboard focus + ring)
- [x] collapsible — (200ms animated reveal)
- [x] breadcrumb — (link focus rings)
- [x] pagination — base-vega parity (Content/Item composition; Link.size + Previous/Next.text; icon-side padding via icon_inline_start/end; currentColor Icon chevrons; sm-breakpoint label hide via viewport width; foreground ellipsis; Simple + Icons Only docs examples; aria/sr-only omitted — no a11y tree; RTL omitted — unsupported, TODO)
- [x] table — (static presentation — matches source table parts)
- [x] scroll-area — (vertical overflow scroll; custom scrollbar + horizontal TODO)
- [x] kbd — (tooltip-context styles via in_tooltip(); all six shadcn doc examples in the storybook, RTL mirrored manually)
- [x] marker — (static presentation — matches source divider variants)

### Tier 3 — overlays (popover foundation → menus → modals)

- [x] tooltip — enter animation (100ms ease fade+slide)
- [x] hover-card — enter animation (100ms ease fade+slide)
- [x] popover — enter animation (100ms ease fade+slide)
- [x] dropdown-menu — enter animation (100ms ease fade+slide)
- [x] context-menu — enter animation (100ms ease fade+slide)
- [x] menubar — enter animation (100ms ease fade+slide)
- [x] select — enter animation (100ms ease fade+slide)
- [x] native-select — enter animation (100ms ease fade+slide)
- [x] dialog — 200ms enter animation, backdrop fade, Escape closes (storybook root)
- [x] alert-dialog — 200ms enter animation
- [x] sheet — 500ms ease-in-out slide from its edge, Escape closes
- [x] drawer — 500ms slide-up, Escape closes
- [x] toast — enter animation
- [x] navigation-menu — enter animation (100ms ease fade+slide)

### Tier 4 — composites

- [x] command — (items hover+click; keyboard selection TODO)
- [x] combobox — enter animation on the panel
- [x] calendar — (nav + day-cell focus rings; keyboard day grid TODO)
- [x] date-picker — enter animation on the panel
- [x] carousel — (prev/next click; swipe gestures + multi-slide TODO)
- [x] resizable — (drag handle; keyboard resize TODO)
- [x] sidebar — (menu focus rings)
- [x] data-table — (storybook composition on table primitives; no dedicated component module)
- [x] chart — (BarChart only; line/area/pie + tooltips TODO)
- [x] message — (static presentation — matches source row layout)
- [x] bubble — (static presentation — matches source variants)
- [x] message-scroller — (scroll container; stick-to-bottom + scroll-to-bottom button TODO)
- [x] attachment — (remove button focus ring; vertical orientation + progress overlay TODO)
- [x] questionnaire — (choice focus rings)
