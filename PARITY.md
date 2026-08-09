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
- [ ] badge
- [ ] label
- [ ] kbd
- [ ] separator
- [x] skeleton — animate-pulse (2s opacity breathing)
- [ ] spinner
- [ ] aspect-ratio
- [ ] avatar
- [x] checkbox — (keyboard focus + ring)
- [x] radio-group — (keyboard focus + ring)
- [x] switch — (keyboard focus + ring)
- [x] toggle — (keyboard focus + ring)
- [x] slider — keyboard focus ring + arrow-key steps
- [ ] progress
- [x] input — full focus ring shadow (was border-only)
- [x] textarea — focus ring shadow

### Tier 2 — molecules on the atoms

- [x] accordion — audited in the infrastructure PR (focus + keyboard on
  triggers, disabled items, 200ms ease-out reveal)
- [ ] alert
- [ ] card
- [ ] empty
- [ ] item
- [ ] field
- [x] input-group — focus ring shadow on the shell
- [ ] input-otp
- [ ] button-group
- [x] toggle-group — (keyboard focus + ring)
- [x] tabs — (keyboard focus + ring)
- [x] collapsible — (200ms animated reveal)
- [x] breadcrumb — (link focus rings)
- [ ] pagination
- [ ] table
- [ ] scroll-area
- [ ] kbd
- [ ] marker

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

- [ ] command
- [x] combobox — enter animation on the panel
- [ ] calendar
- [x] date-picker — enter animation on the panel
- [ ] carousel
- [ ] resizable
- [x] sidebar — (menu focus rings)
- [ ] data-table
- [ ] chart
- [ ] message
- [ ] bubble
- [ ] message-scroller
- [ ] attachment
- [x] questionnaire — (choice focus rings)
