//! Borders family: widths (per side/axis/logical), color, style, radius
//! (per corner/side/logical), and `ring-*` (synthesized into a box shadow by
//! the core, matching `motion::focus_ring`).
//! Docs chapter: <https://tailwindcss.com/docs/border-width>
//!
//! Per-side border *colors* are unsupported — gpui has a single
//! `border_color` — so `border-t-red-500` falls through as unknown.

use gpui::{BorderStyle, Pixels, StyleRefinement, Styled, px};

use super::{Ctx, color, radius};

type Corners = &'static [Corner];

#[derive(Clone, Copy)]
enum Corner {
    Tl,
    Tr,
    Br,
    Bl,
}

/// Radius roots, longest prefix first. Logical corners map physically (LTR).
const ROUNDED: &[(&str, Corners)] = &[
    ("rounded-ss", &[Corner::Tl]),
    ("rounded-se", &[Corner::Tr]),
    ("rounded-ee", &[Corner::Br]),
    ("rounded-es", &[Corner::Bl]),
    ("rounded-tl", &[Corner::Tl]),
    ("rounded-tr", &[Corner::Tr]),
    ("rounded-br", &[Corner::Br]),
    ("rounded-bl", &[Corner::Bl]),
    ("rounded-t", &[Corner::Tl, Corner::Tr]),
    ("rounded-r", &[Corner::Tr, Corner::Br]),
    ("rounded-b", &[Corner::Bl, Corner::Br]),
    ("rounded-l", &[Corner::Tl, Corner::Bl]),
    ("rounded-s", &[Corner::Tl, Corner::Bl]),
    ("rounded-e", &[Corner::Tr, Corner::Br]),
    ("rounded", &[Corner::Tl, Corner::Tr, Corner::Br, Corner::Bl]),
];

#[derive(Clone, Copy)]
enum Edge {
    Top,
    Right,
    Bottom,
    Left,
}

/// Border-width roots, longest prefix first. Logical sides map physically.
const BORDER_SIDES: &[(&str, &[Edge])] = &[
    ("border-x", &[Edge::Left, Edge::Right]),
    ("border-y", &[Edge::Top, Edge::Bottom]),
    ("border-s", &[Edge::Left]),
    ("border-e", &[Edge::Right]),
    ("border-t", &[Edge::Top]),
    ("border-r", &[Edge::Right]),
    ("border-b", &[Edge::Bottom]),
    ("border-l", &[Edge::Left]),
    (
        "border",
        &[Edge::Top, Edge::Right, Edge::Bottom, Edge::Left],
    ),
];

pub(super) fn apply(mut s: StyleRefinement, t: &str, cx: &mut Ctx) -> (StyleRefinement, bool) {
    match t {
        "border-solid" => {
            s.border_style = Some(BorderStyle::Solid);
            return (s, true);
        }
        "border-dashed" => {
            s.border_style = Some(BorderStyle::Dashed);
            return (s, true);
        }
        "border-none" => return (s.border_0(), true),
        _ => {}
    }

    // `ring-inset` marks the regular ring shadow as inset.
    if t == "ring-inset" {
        *cx.ring_inset = true;
        return (s, true);
    }

    // Inset ring: separate channel synthesized with `inset: true`.
    if t == "inset-ring" {
        cx.inset_ring.width = Some(px(1.));
        return (s, true);
    }
    if let Some(v) = t.strip_prefix("inset-ring-") {
        if let Ok(n) = v.parse::<f32>() {
            cx.inset_ring.width = Some(px(n));
            return (s, true);
        }
        if let Some(inner) = v.strip_prefix('[').and_then(|x| x.strip_suffix("px]"))
            && let Ok(n) = inner.parse::<f32>()
        {
            cx.inset_ring.width = Some(px(n));
            return (s, true);
        }
        if let Some(c) = color(cx.theme, v) {
            cx.inset_ring.color = Some(c);
            return (s, true);
        }
        return (s, false);
    }

    // Ring: width and color accumulate; the core synthesizes the shadow.
    if t == "ring" {
        cx.ring.width = Some(px(1.));
        return (s, true);
    }
    if let Some(v) = t.strip_prefix("ring-") {
        // `ring-2` and `ring-[3px]` are widths in raw px (not the 4px scale).
        if let Ok(n) = v.parse::<f32>() {
            cx.ring.width = Some(px(n));
            return (s, true);
        }
        if let Some(inner) = v.strip_prefix('[').and_then(|x| x.strip_suffix("px]"))
            && let Ok(n) = inner.parse::<f32>()
        {
            cx.ring.width = Some(px(n));
            return (s, true);
        }
        if let Some(c) = color(cx.theme, v) {
            cx.ring.color = Some(c);
            return (s, true);
        }
        return (s, false);
    }

    // Radius (rounded, rounded-t-lg, rounded-tl-[4px], rounded-full, …).
    for (root, corners) in ROUNDED {
        if t == *root {
            // Bare roots use the 4px default (`rounded` = 0.25rem in v4).
            set_corners(&mut s, corners, px(4.));
            return (s, true);
        }
        if let Some(v) = t.strip_prefix(root).and_then(|r| r.strip_prefix('-'))
            && let Some(r) = radius(cx.theme, v)
        {
            set_corners(&mut s, corners, r);
            return (s, true);
        }
    }

    // Border widths (bare = 1px, numeric = raw px), then border color.
    for (root, edges) in BORDER_SIDES {
        if t == *root {
            set_edges(&mut s, edges, px(1.));
            return (s, true);
        }
        if let Some(v) = t.strip_prefix(root).and_then(|r| r.strip_prefix('-')) {
            let width = if let Ok(n) = v.parse::<f32>() {
                Some(px(n))
            } else {
                v.strip_prefix('[')
                    .and_then(|x| x.strip_suffix("px]"))
                    .and_then(|n| n.parse::<f32>().ok().map(px))
            };
            if let Some(w) = width {
                set_edges(&mut s, edges, w);
                return (s, true);
            }
            // Colors only on the all-sides root (single border_color in gpui).
            if *root == "border"
                && let Some(c) = color(cx.theme, v)
            {
                return (s.border_color(c), true);
            }
        }
    }

    (s, false)
}

fn set_corners(s: &mut StyleRefinement, corners: Corners, r: Pixels) {
    for corner in corners {
        let slot = match corner {
            Corner::Tl => &mut s.corner_radii.top_left,
            Corner::Tr => &mut s.corner_radii.top_right,
            Corner::Br => &mut s.corner_radii.bottom_right,
            Corner::Bl => &mut s.corner_radii.bottom_left,
        };
        *slot = Some(r.into());
    }
}

fn set_edges(s: &mut StyleRefinement, edges: &[Edge], w: Pixels) {
    for edge in edges {
        let slot = match edge {
            Edge::Top => &mut s.border_widths.top,
            Edge::Right => &mut s.border_widths.right,
            Edge::Bottom => &mut s.border_widths.bottom,
            Edge::Left => &mut s.border_widths.left,
        };
        *slot = Some(w.into());
    }
}

#[cfg(test)]
mod tests {
    use super::super::{parse, tests::assert_style_eq};
    use crate::theme::Theme;
    use gpui::{BorderStyle, StyleRefinement, Styled, px};

    #[test]
    fn widths_sides_and_bare() {
        let theme = Theme::light();
        let styles = parse(&theme, "border border-t-2 border-x-4");
        let mut expected = StyleRefinement::default().border_1();
        expected.border_widths.top = Some(px(2.).into());
        expected.border_widths.left = Some(px(4.).into());
        expected.border_widths.right = Some(px(4.).into());
        assert_style_eq(&styles.base, &expected);
    }

    #[test]
    fn radius_corners_and_scale() {
        let theme = Theme::light();
        let styles = parse(&theme, "rounded-md rounded-tl-none");
        let mut expected = StyleRefinement::default().rounded(theme.radius_md());
        expected.corner_radii.top_left = Some(px(0.).into());
        assert_style_eq(&styles.base, &expected);

        let styles = parse(&theme, "rounded-t-full");
        let mut expected = StyleRefinement::default();
        expected.corner_radii.top_left = Some(px(9999.).into());
        expected.corner_radii.top_right = Some(px(9999.).into());
        assert_style_eq(&styles.base, &expected);
    }

    #[test]
    fn style_and_none() {
        let theme = Theme::light();
        let styles = parse(&theme, "border-dashed");
        let mut expected = StyleRefinement::default();
        expected.border_style = Some(BorderStyle::Dashed);
        assert_style_eq(&styles.base, &expected);

        let styles = parse(&theme, "border-none");
        assert_style_eq(&styles.base, &StyleRefinement::default().border_0());
    }

    #[test]
    fn per_side_colors_fall_through_as_unknown() {
        let theme = Theme::light();
        let styles = parse(&theme, "border-t-red-500");
        assert_eq!(styles.unknown, vec!["border-t-red-500"]);
    }

    #[test]
    fn bare_ring_is_one_pixel() {
        let theme = Theme::light();
        let styles = parse(&theme, "ring");
        let shadows = styles.base.box_shadow.as_ref().unwrap();
        assert_eq!(shadows.len(), 1);
        assert_eq!(shadows[0].spread_radius, px(1.));
        assert!(!shadows[0].inset);
    }

    #[test]
    fn inset_ring_synthesizes_inset_shadow() {
        let theme = Theme::light();
        let styles = parse(&theme, "inset-ring-2");
        let shadows = styles.base.box_shadow.as_ref().unwrap();
        assert_eq!(shadows.len(), 1);
        assert_eq!(shadows[0].spread_radius, px(2.));
        assert!(shadows[0].inset);
    }

    #[test]
    fn ring_inset_marks_regular_ring_inset() {
        let theme = Theme::light();
        let styles = parse(&theme, "ring-2 ring-inset");
        let shadows = styles.base.box_shadow.as_ref().unwrap();
        assert_eq!(shadows.len(), 1);
        assert_eq!(shadows[0].spread_radius, px(2.));
        assert!(shadows[0].inset);
    }
}
