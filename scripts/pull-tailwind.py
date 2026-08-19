#!/usr/bin/env python3
"""Pull the canonical Tailwind v4 utility list and default color palette.

Fetches the pinned tailwindcss sources from GitHub and regenerates:
  - src/tw/manifest.json — every utility root Tailwind registers, split into
    static (exact-name) and functional (root that takes a value) utilities.
    This is the ground truth the tw coverage ledger is checked against.
  - src/tw/palette.rs — the default color palette (--color-* from theme.css)
    as a name → Hsla lookup via crate::theme::oklch.

Usage: scripts/pull-tailwind.py [version]   (default: v4.1.12)

Rerun when bumping the pinned Tailwind version; commit the regenerated files.
"""

import json
import re
import sys
import urllib.request
from pathlib import Path

VERSION = sys.argv[1] if len(sys.argv) > 1 else "v4.1.12"
RAW = f"https://raw.githubusercontent.com/tailwindlabs/tailwindcss/{VERSION}/packages/tailwindcss"
REPO_ROOT = Path(__file__).resolve().parent.parent
TW_DIR = REPO_ROOT / "src" / "tw"


def fetch(path: str) -> str:
    url = f"{RAW}/{path}"
    print(f"fetching {url}")
    with urllib.request.urlopen(url, timeout=30) as resp:
        return resp.read().decode("utf-8")


# Utility roots registered through loop variables (template literals the
# regexes below can't see). Stable across v4.1.x; verify when bumping VERSION
# by grepping utilities.ts for `spacingUtility(name` / `borderSideUtility(`
# loop headers and the `${root}-none` / `${name}-auto` static templates.
INSET_ROOTS = ["inset", "inset-x", "inset-y", "start", "end", "top", "right", "bottom", "left"]
MARGIN_ROOTS = ["m", "mx", "my", "ms", "me", "mt", "mr", "mb", "ml"]
PADDING_ROOTS = ["p", "px", "py", "ps", "pe", "pt", "pr", "pb", "pl"]
SIZING_ROOTS = ["w", "min-w", "max-w", "h", "min-h", "max-h"]
ROUNDED_ROOTS = [
    "rounded", "rounded-s", "rounded-e", "rounded-t", "rounded-r", "rounded-b",
    "rounded-l", "rounded-ss", "rounded-se", "rounded-ee", "rounded-es",
    "rounded-tl", "rounded-tr", "rounded-br", "rounded-bl",
]
TRANSLATE_ROOTS = ["translate", "translate-x", "translate-y", "translate-z"]
SCROLL_ROOTS = [f"scroll-{r}" for r in MARGIN_ROOTS + PADDING_ROOTS]

SUPPLEMENT_FUNCTIONAL = (
    INSET_ROOTS + MARGIN_ROOTS + PADDING_ROOTS + SIZING_ROOTS + ROUNDED_ROOTS
    + TRANSLATE_ROOTS + SCROLL_ROOTS
)
BLEND_MODES = [
    "normal", "multiply", "screen", "overlay", "darken", "lighten",
    "color-dodge", "color-burn", "hard-light", "soft-light", "difference",
    "exclusion", "hue", "saturation", "color", "luminosity",
]
SUPPLEMENT_STATIC = (
    [f"{r}-auto" for r in INSET_ROOTS]
    + [f"{r}-full" for r in INSET_ROOTS]
    + [f"{r}-auto" for r in MARGIN_ROOTS]
    + [f"{r}-none" for r in ROUNDED_ROOTS]
    + [f"{r}-full" for r in ROUNDED_ROOTS]
    + [f"overflow{a}-{v}" for a in ["", "-x", "-y"] for v in ["auto", "hidden", "clip", "visible", "scroll"]]
    + [f"overscroll{a}-{v}" for a in ["", "-x", "-y"] for v in ["auto", "contain", "none"]]
    + [f"touch-{v}" for v in ["auto", "none", "manipulation"]]
    + [f"touch-pan-{v}" for v in ["x", "left", "right", "y", "up", "down"]]
    + [f"select-{v}" for v in ["none", "text", "all", "auto"]]
    + ["snap-x", "snap-y", "snap-both"]
    + [f"break-before-{v}" for v in ["auto", "avoid", "all", "avoid-page", "page", "left", "right", "column"]]
    + [f"break-after-{v}" for v in ["auto", "avoid", "all", "avoid-page", "page", "left", "right", "column"]]
    + [f"break-inside-{v}" for v in ["auto", "avoid", "avoid-page", "avoid-column"]]
    + [f"bg-blend-{v}" for v in BLEND_MODES]
    + [f"mix-blend-{v}" for v in BLEND_MODES]
    + [f"{r}-{k}" for r in SIZING_ROOTS + ["size"]
       for k in ["full", "svw", "lvw", "dvw", "svh", "lvh", "dvh", "min", "max", "fit"]]
)


def extract_utilities(utilities_ts: str) -> tuple[list[str], list[str]]:
    # Both quote styles appear; skip template names with interpolations —
    # those are covered by the supplements above.
    static = [
        n
        for n in re.findall(
            r"(?:staticUtility|utilities\.static)\(\s*(['`])([^'`]+)\1", utilities_ts
        )
        for n in [n[1]]
        if "${" not in n
    ] + SUPPLEMENT_STATIC
    functional = [
        n
        for n in re.findall(
            r"(?:functionalUtility|utilities\.functional|colorUtility"
            r"|spacingUtility|borderSideUtility|gradientStopUtility)\(\s*(['`])([^'`]+)\1",
            utilities_ts,
        )
        for n in [n[1]]
        if "${" not in n
    ] + SUPPLEMENT_FUNCTIONAL

    def dedup(names: list[str]) -> list[str]:
        seen: set[str] = set()
        out = []
        for n in names:
            # Tailwind registers negative forms as separate roots ('-scale');
            # the tw parser handles negation uniformly, so fold them.
            n = n.lstrip("-")
            if n not in seen:
                seen.add(n)
                out.append(n)
        return out

    return dedup(static), dedup(functional)


def extract_palette(theme_css: str) -> list[tuple[str, str]]:
    """Return (name, rust_expr) for every --color-* variable."""
    out = []
    for name, value in re.findall(r"--color-([a-z0-9-]+):\s*([^;]+);", theme_css):
        value = value.strip()
        m = re.fullmatch(r"oklch\(([\d.]+)%\s+([\d.]+)\s+([\d.]+)\)", value)
        if m:
            l, c, h = (float(g) for g in m.groups())
            out.append((name, f"oklch({l / 100.:.4}, {c}, {h})"))
            continue
        m = re.fullmatch(r"#([0-9a-fA-F]{3})", value)
        if m:
            hex3 = m.group(1)
            hex6 = "".join(ch * 2 for ch in hex3)
            out.append((name, f"Hsla::from(gpui::rgb(0x{hex6}))"))
            continue
        m = re.fullmatch(r"#([0-9a-fA-F]{6})", value)
        if m:
            out.append((name, f"Hsla::from(gpui::rgb(0x{m.group(1)}))"))
            continue
        raise SystemExit(f"unhandled color format for --color-{name}: {value}")
    return out


def write_manifest(static: list[str], functional: list[str]) -> None:
    manifest = {
        "tailwind_version": VERSION,
        "generated_by": "scripts/pull-tailwind.py",
        "static": static,
        "functional": functional,
    }
    path = TW_DIR / "manifest.json"
    path.write_text(json.dumps(manifest, indent=2) + "\n")
    print(f"wrote {path} ({len(static)} static, {len(functional)} functional)")


def write_palette(colors: list[tuple[str, str]]) -> None:
    lines = [
        "//! GENERATED by scripts/pull-tailwind.py — do not edit by hand.",
        f"//! Default Tailwind {VERSION} color palette (theme.css --color-*).",
        "",
        "use gpui::Hsla;",
        "",
        "use crate::theme::oklch;",
        "",
        "/// Resolve a default-palette color name (`red-500`, `white`, …).",
        "pub(super) fn palette(name: &str) -> Option<Hsla> {",
        "    Some(match name {",
    ]
    for name, expr in colors:
        lines.append(f'        "{name}" => {expr},')
    lines += [
        "        _ => return None,",
        "    })",
        "}",
        "",
    ]
    path = TW_DIR / "palette.rs"
    path.write_text("\n".join(lines))
    print(f"wrote {path} ({len(colors)} colors)")


def main() -> None:
    TW_DIR.mkdir(parents=True, exist_ok=True)
    static, functional = extract_utilities(fetch("src/utilities.ts"))
    write_manifest(static, functional)
    write_palette(extract_palette(fetch("theme.css")))


if __name__ == "__main__":
    main()
