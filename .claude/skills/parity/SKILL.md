---
name: parity
description: Establish full fine-grained parity between an rcn gpui component and the real shadcn component — contract, motion timings/easings, accessibility, token handling, and matching storybook doc examples — then ship it as one PR via /spellcraft:navis. Use when the user runs /parity <component> [docs-url], or asks to bring a component up to shadcn parity / audit a component against shadcn.
---

# Parity — the shadcn component audit loop

`/parity <component> [shadcn-docs-url]` runs the second-pass audit described in
`PARITY.md`: compare our gpui component against the **real** shadcn source and
the libraries underneath it, close every gap, mirror the docs examples in the
storybook, and ship one PR per component.

The bar: the contract of how the component is used should feel almost identical
to shadcn, so porting a shadcn app to this framework is mechanical. That means
full parity on options/props, animation timings and easing functions, keyboard
accessibility, token handling, and docs examples — not a visual approximation.

If no component name was given, ask which component to audit before starting.
If no docs URL was given, default to
`https://ui.shadcn.com/docs/components/base/<component>`.

## Phase 1 — Gather the references

Do this research yourself, inline. Its output — a concrete gap inventory — is
the payload everything downstream depends on.

1. **Pull the real source.** Run `scripts/pull-shadcn.sh <component>`. This
   installs the real shadcn component (base-vega style) and its dependency
   components into the reference project at `/tmp/rcn-shadcn-ref` (override
   with `SHADCN_REF`), alongside `@base-ui-components/react` and
   `tw-animate-css` sources in `node_modules`. Read:
   - `/tmp/rcn-shadcn-ref/src/components/ui/<component>.tsx` — the component
     itself: every prop, variant, data-slot, and className.
   - The underlying Base UI primitive in
     `node_modules/@base-ui-components/react/` — state model, keyboard
     handling, ARIA behavior. This is where the *behavioral* contract lives.
   - `node_modules/tw-animate-css/` for any animation classes the component
     uses — exact durations and easings.
2. **Read the docs.** If the user attached the md version of the docs, use it;
   otherwise fetch the docs URL (via the standard browsing skill). Enumerate
   **every example on the docs page** — each one must exist in our storybook.
3. **Read ours.** `src/components/<component>.rs`, its storybook preview fns in
   `src/storybook.rs`, and its entry in `PARITY.md`. Also read `src/motion.rs`
   (easings, durations, focus ring) and `src/theme.rs` (tokens) so gaps are
   expressed in terms of existing infrastructure, not new inventions.
4. **Study one finished neighbor.** Pick an already-audited component of
   similar complexity (e.g. `accordion.rs` for stateful molecules, `kbd.rs`
   for static atoms) as the pattern file for conventions: builder API shape,
   controlled/uncontrolled state, storybook controls panel, doc comments.

## Phase 2 — Produce the gap inventory

Diff ours against the reference across these dimensions (from `PARITY.md`) and
write the result as a numbered list — each gap concrete enough to implement
without re-reading the reference:

- **Contract** — every prop/variant shadcn supports, exposed as a builder
  method with the same name and semantics (`defaultValue` → `default_value`,
  `onValueChange` → `on_value_change`, …), mirrored in the storybook controls
  panel. Note anything we support that shadcn doesn't (candidate for removal
  or explicit divergence note).
- **Motion** — animations/transitions with exact tw-animate-css timing and
  easing (enter: 150ms ease; accordion/collapsible: 200ms ease-out; popovers:
  100ms; hover transitions: 150ms cubic-bezier(0.4,0,0.2,1)) — verify against
  the pulled source rather than trusting this list. gpui animates on mount:
  enter animations are ported; exit animations are noted as TODOs, not faked.
- **Accessibility** — keyboard focus (`tab_index`), the shadcn focus-visible
  ring (`motion::focus_ring`), Enter/Space activation, Escape-to-close on
  overlays, arrow-key navigation where Base UI has it, disabled semantics
  (opacity, no pointer events, unfocusable).
- **Tokens** — every color/spacing/radius maps to the `theme.rs` token the
  shadcn className resolves to (e.g. `bg-muted` → `theme.muted`), never a
  hardcoded value. Check dark-mode variants in the source (`dark:` classes).
- **Storybook docs examples** — one preview fn per docs-page example, same
  order and content as the shadcn docs, so the docs pages match side by side.

Anything genuinely unportable (CSS context selectors, RTL, exit animations)
gets an explicit "omitted/TODO" note, not silence — these become the
parenthetical in the `PARITY.md` entry.

## Phase 3 — Delegate the build via /navis

Implementation goes through the ship loop: invoke the `spellcraft:navis` skill,
whose Phase 3 delegates the first pass to lci. **Neither navis's lci run nor a
fresh reviewer has this conversation's context — the args you pass must carry
everything.** Compose the navis feature request as a numbered **Definition of
done (all N required)** contract containing:

1. The goal line: `Parity: <component> — <one-line summary of the gaps>`.
2. The full gap inventory from Phase 2, each gap with exact target details
   (builder name, timing value, token name).
3. File paths: modify `src/components/<component>.rs` and the component's
   preview fns in `src/storybook.rs`; pattern file to imitate (the finished
   neighbor from Phase 1); reference source paths under `/tmp/rcn-shadcn-ref`
   to consult, with the key excerpts inlined in case the path is unreadable.
4. Verification commands, CI-verbatim:
   - `cargo build` (compiles through sccache)
   - `python3 scripts/gen-docs.py` then `cargo build` again — regenerates
     `src/storybook_docs.rs`; required whenever component `pub fn`s or
     storybook previews change, and the diff must include the regenerated file.
5. The invariants: no hardcoded colors (theme tokens only), exact durations
   and easings from `src/motion.rs`, every docs example present in the
   storybook, exit-animation TODOs noted in doc comments.
6. Scope constraint: touch only the component file, `src/storybook.rs`, the
   regenerated `src/storybook_docs.rs`, and `PARITY.md` — plus `src/assets.rs`
   if a docs example needs an icon not yet embedded.
7. `PARITY.md` bookkeeping: update the component's checklist line to
   `- [x] <component> — (<what was matched; what was omitted and why>)`.
8. The wrong-premise escape hatch: if a listed gap mismatches the actual code,
   adapt and note it rather than forcing the change.

Then follow the navis loop to completion — its gates (confirm before build,
green tests before ship) apply as written.

## Phase 4 — Verify like this repo verifies

Supplement navis's test gate with the repo's realities:

- `cargo build` must be clean (no new warnings in the touched files).
- Launch verification: `screencapture` is TCC-blocked in this environment —
  verify by launching the app (`cargo run`) and confirming the window via
  CGWindowList instead of screenshots. Exercise the component's storybook
  page: keyboard focus order, Enter/Space/Escape/arrow keys, controls panel.
- Do **not** mark the component in `verification.json` or
  `docs/manual-audit-log.md` — those record the user's own manual
  look-and-feel verification, after they eyeball the running app.

## Phase 5 — Ship

navis Phase 5 opens the draft PR. Repo conventions for it:

- One component per PR. Branch `navis/<component>-parity`.
- Title: `Parity: <component> — <gaps closed>` (matching #79–#82).
- Body lists what was ported, what was omitted/TODO and why, and how it was
  verified.

Report back: the PR URL, the `PARITY.md` summary line, and any TODOs left for
follow-up (exit animations, RTL, context-selector styles).
