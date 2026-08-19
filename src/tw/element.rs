//! `tw_div` — a children-aware container for the ext channel's
//! child-combinator utilities, and home of the v1 transition engine.
//!
//! `space-x/y-*` interleave margins onto every child after the first (like
//! Tailwind's `> :not(:first-child)` margins), and `divide-x/y-*` insert
//! separator elements between children (visually equivalent to Tailwind's
//! child borders; colored via `divide-<color>`, defaulting to the theme
//! border color). Style-channel classes (flex, gap, padding, colors,
//! `hover:` states, …) apply to the container exactly like `div().tw(...)`.
//!
//! ## Transitions (v1)
//!
//! With an [`TwDiv::id`] and `transition`/`transition-colors`/
//! `transition-opacity` in the class string, hover changes to background,
//! text color, border color, and opacity interpolate over the
//! `duration-*`/`ease-*`/`delay-*` curve instead of snapping. Other hover
//! properties still apply instantly, and elements without an id fall back to
//! instant styling. A property present on only one side fades through its
//! transparent counterpart, like CSS.
//!
//! Approximation note: spaced children are wrapped in a margin div, so a
//! child's own `flex_grow`/`flex_shrink` should be set via the wrapper's
//! classes instead (or use `gap-*`, which needs no wrapper).

use std::time::Duration;

use gpui::{
    AnyElement, App, ElementId, Fill, Hsla, InteractiveElement as _, IntoElement, ParentElement,
    Refineable as _, RenderOnce, SharedString, StatefulInteractiveElement as _, StyleRefinement,
    Styled, Window, div, prelude::FluentBuilder as _,
};

use super::{TwEasing, TwTransition};
use crate::motion;
use crate::theme::Theme;

/// A `div` whose Tailwind classes may include child-combinator and
/// transition utilities.
pub fn tw_div(classes: impl Into<SharedString>) -> TwDiv {
    TwDiv {
        classes: classes.into(),
        id: None,
        children: Vec::new(),
    }
}

#[derive(IntoElement)]
pub struct TwDiv {
    classes: SharedString,
    id: Option<ElementId>,
    children: Vec<AnyElement>,
}

impl TwDiv {
    /// Stable identity — required for hover transitions (per-element state).
    pub fn id(mut self, id: impl Into<ElementId>) -> Self {
        self.id = Some(id.into());
        self
    }
}

impl ParentElement for TwDiv {
    fn extend(&mut self, elements: impl IntoIterator<Item = AnyElement>) {
        self.children.extend(elements);
    }
}

impl RenderOnce for TwDiv {
    fn render(self, window: &mut Window, cx: &mut App) -> impl IntoElement {
        let theme = Theme::of(cx).clone();
        let mut styles = super::parse_at(&theme, window.viewport_size(), &self.classes);
        let ext = std::mem::take(&mut styles.ext);
        let divide_color = ext.divide_color.unwrap_or(theme.border);

        let mut children_out: Vec<AnyElement> = Vec::with_capacity(self.children.len() * 2);
        for (i, child) in self.children.into_iter().enumerate() {
            if i > 0 {
                if let Some(w) = ext.divide_x {
                    children_out.push(div().w(w).flex_none().bg(divide_color).into_any_element());
                }
                if let Some(w) = ext.divide_y {
                    children_out.push(div().h(w).flex_none().bg(divide_color).into_any_element());
                }
            }
            let needs_margin = i > 0 && (ext.space_x.is_some() || ext.space_y.is_some());
            if needs_margin {
                children_out.push(
                    div()
                        .when_some(ext.space_x, |el, x| el.ml(x))
                        .when_some(ext.space_y, |el, y| el.mt(y))
                        .child(child)
                        .into_any_element(),
                );
            } else {
                children_out.push(child);
            }
        }

        // Animated hover path: needs an id, an active transition spec, and a
        // hover bucket to interpolate toward.
        let transition = ext.transition.filter(|t| t.colors || t.opacity);
        if let (Some(id), Some(tr), Some(hover)) =
            (self.id.clone(), transition, styles.hover.take())
        {
            return animated_hover(id, tr, styles, hover, children_out, window, cx)
                .into_any_element();
        }

        let el = super::apply_interactive(div(), styles).children(children_out);
        match self.id {
            Some(id) => el.id(id).into_any_element(),
            None => el.into_any_element(),
        }
    }
}

/// The animated properties of one side of a transition.
#[derive(Clone, Copy, PartialEq, Default)]
struct AnimProps {
    bg: Option<Hsla>,
    text: Option<Hsla>,
    border: Option<Hsla>,
    opacity: Option<f32>,
}

fn extract(s: &StyleRefinement, tr: &TwTransition) -> AnimProps {
    AnimProps {
        bg: tr.colors.then(|| fill_color(&s.background)).flatten(),
        text: tr.colors.then(|| s.text.color).flatten(),
        border: tr.colors.then(|| s.border_color).flatten(),
        opacity: tr.opacity.then(|| s.opacity).flatten(),
    }
}

fn fill_color(fill: &Option<Fill>) -> Option<Hsla> {
    match fill {
        Some(Fill::Color(background)) => background.as_solid(),
        _ => None,
    }
}

/// Clear the animated properties from a refinement (they are driven by the
/// animation instead).
fn strip_animated(s: &mut StyleRefinement, tr: &TwTransition) {
    if tr.colors {
        // Only solid backgrounds animate; gradients keep their instant swap.
        if fill_color(&s.background).is_some() {
            s.background = None;
        }
        s.text.color = None;
        s.border_color = None;
    }
    if tr.opacity {
        s.opacity = None;
    }
}

fn lerp(a: f32, b: f32, t: f32) -> f32 {
    a + (b - a) * t
}

fn lerp_hsla(a: Hsla, b: Hsla, t: f32) -> Hsla {
    Hsla {
        h: lerp(a.h, b.h, t),
        s: lerp(a.s, b.s, t),
        l: lerp(a.l, b.l, t),
        a: lerp(a.a, b.a, t),
    }
}

/// Pair one property across the two sides: a side missing the property fades
/// through the other side's color at alpha 0 (CSS-transparent semantics).
fn pair(from: Option<Hsla>, to: Option<Hsla>) -> Option<(Hsla, Hsla)> {
    match (from, to) {
        (Some(a), Some(b)) => Some((a, b)),
        (None, Some(b)) => Some((Hsla { a: 0., ..b }, b)),
        (Some(a), None) => Some((a, Hsla { a: 0., ..a })),
        (None, None) => None,
    }
}

fn easing_fn(tr: &TwTransition) -> Box<dyn Fn(f32) -> f32> {
    let inner: Box<dyn Fn(f32) -> f32> = match tr.easing {
        TwEasing::Default => Box::new(motion::ease_transition()),
        TwEasing::Linear => Box::new(|t| t),
        TwEasing::In => Box::new(motion::cubic_bezier(0.4, 0., 1., 1.)),
        TwEasing::Out => Box::new(motion::cubic_bezier(0., 0., 0.2, 1.)),
        TwEasing::InOut => Box::new(motion::cubic_bezier(0.4, 0., 0.2, 1.)),
    };
    let total = tr.delay_ms + tr.duration_ms;
    if tr.delay_ms > 0. && total > 0. {
        let start = tr.delay_ms / total;
        Box::new(move |t| {
            if t <= start {
                0.
            } else {
                inner((t - start) / (1. - start))
            }
        })
    } else {
        inner
    }
}

fn animated_hover(
    id: ElementId,
    tr: TwTransition,
    mut styles: super::TwStyles,
    mut hover: StyleRefinement,
    children: Vec<AnyElement>,
    window: &mut Window,
    cx: &mut App,
) -> impl IntoElement + use<> {
    use gpui::AnimationExt as _;

    // Per-element hover state: (hovered, flip generation). The generation
    // keys the animation id so each flip replays the interpolation.
    let state = window.use_keyed_state((id.clone(), "tw-transition"), cx, |_, _| (false, 0u64));
    let (hovered, generation) = *state.read(cx);

    let base_props = extract(&styles.base, &tr);
    let mut hovered_style = styles.base.clone();
    hovered_style.refine(&hover);
    let hover_props = extract(&hovered_style, &tr);
    let (from, to) = if hovered {
        (base_props, hover_props)
    } else {
        (hover_props, base_props)
    };

    // Animated properties leave the hover bucket (they are driven below);
    // everything else in it still applies instantly.
    strip_animated(&mut hover, &tr);
    styles.hover = Some(hover);

    let el = super::apply_interactive(div(), styles)
        .children(children)
        .id(id)
        .on_hover(move |hovered, window, cx| {
            let is_hovered = *hovered;
            state.update(cx, |(h, generation), _| {
                if *h != is_hovered {
                    *h = is_hovered;
                    *generation += 1;
                }
            });
            window.refresh();
        });

    // First mount (generation 0) renders statically — nothing to animate yet.
    if generation == 0 {
        return el.into_any_element();
    }

    let duration = Duration::from_millis((tr.delay_ms + tr.duration_ms).max(1.) as u64);
    let easing = easing_fn(&tr);
    el.with_animation(
        ("tw-transition", generation),
        gpui::Animation::new(duration).with_easing(move |t| easing(t)),
        move |mut el, t| {
            let s = el.style();
            if let Some((a, b)) = pair(from.bg, to.bg) {
                s.background = Some(lerp_hsla(a, b, t).into());
            }
            if let Some((a, b)) = pair(from.text, to.text) {
                s.text.color = Some(lerp_hsla(a, b, t));
            }
            if let Some((a, b)) = pair(from.border, to.border) {
                s.border_color = Some(lerp_hsla(a, b, t));
            }
            if from.opacity.is_some() || to.opacity.is_some() {
                let a = from.opacity.unwrap_or(1.);
                let b = to.opacity.unwrap_or(1.);
                s.opacity = Some(lerp(a, b, t));
            }
            el
        },
    )
    .into_any_element()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::theme::Theme;
    use crate::tw::parse;
    use gpui::px;

    #[test]
    fn builder_collects_children() {
        let mut d = tw_div("flex divide-y");
        d.extend([div().into_any_element(), div().into_any_element()]);
        assert_eq!(d.children.len(), 2);
        assert_eq!(d.classes.as_ref(), "flex divide-y");
    }

    #[test]
    fn transition_props_extract_and_pair() {
        let theme = Theme::light();
        let styles = parse(&theme, "bg-primary transition-colors hover:bg-primary/80");
        let tr = styles.ext.transition.unwrap();
        assert!(tr.colors && !tr.opacity);
        let props = extract(&styles.base, &tr);
        assert_eq!(props.bg, Some(theme.primary));

        // A side without the property fades through transparent.
        let (a, b) = pair(None, Some(theme.primary)).unwrap();
        assert_eq!(a.a, 0.);
        assert_eq!(b, theme.primary);
    }

    #[test]
    fn easing_delay_holds_then_runs() {
        let tr = TwTransition {
            colors: true,
            duration_ms: 100.,
            delay_ms: 100.,
            ..Default::default()
        };
        let f = easing_fn(&tr);
        assert_eq!(f(0.25), 0.);
        assert!(f(1.) > 0.99);
        let _ = px(0.);
    }
}
