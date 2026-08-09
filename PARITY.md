# shadcn parity audit checklist

Second pass over every component: audit against the real shadcn source
(pulled via `tools/pull-shadcn.sh <name>` into the reference project) plus
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
- [x] `tools/pull-shadcn.sh` — pull any component's real source via the
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
- [ ] skeleton
- [ ] spinner
- [ ] aspect-ratio
- [ ] avatar
- [ ] checkbox
- [ ] radio-group
- [ ] switch
- [ ] toggle
- [ ] slider
- [ ] progress
- [ ] input
- [ ] textarea

### Tier 2 — molecules on the atoms

- [x] accordion — audited in the infrastructure PR (focus + keyboard on
  triggers, disabled items, 200ms ease-out reveal)
- [ ] alert
- [ ] card
- [ ] empty
- [ ] item
- [ ] field
- [ ] input-group
- [ ] input-otp
- [ ] button-group
- [ ] toggle-group
- [ ] tabs
- [ ] collapsible
- [ ] breadcrumb
- [ ] pagination
- [ ] table
- [ ] scroll-area
- [ ] kbd
- [ ] marker

### Tier 3 — overlays (popover foundation → menus → modals)

- [ ] tooltip
- [ ] hover-card
- [ ] popover
- [ ] dropdown-menu
- [ ] context-menu
- [ ] menubar
- [ ] select
- [ ] native-select
- [ ] dialog
- [ ] alert-dialog
- [ ] sheet
- [ ] drawer
- [ ] toast
- [ ] navigation-menu

### Tier 4 — composites

- [ ] command
- [ ] combobox
- [ ] calendar
- [ ] date-picker
- [ ] carousel
- [ ] resizable
- [ ] sidebar
- [ ] data-table
- [ ] chart
- [ ] message
- [ ] bubble
- [ ] message-scroller
- [ ] attachment
- [ ] questionnaire
