//! `tw_div` — a children-aware container for the ext channel's
//! child-combinator utilities.
//!
//! `space-x/y-*` interleave margins onto every child after the first (like
//! Tailwind's `> :not(:first-child)` margins), and `divide-x/y-*` insert
//! separator elements between children (visually equivalent to Tailwind's
//! child borders; a Tailwind `divide` colors via `divide-<color>`, defaulting
//! to the theme border color). Style-channel classes (flex, gap, padding,
//! colors, `hover:` states, …) apply to the container exactly like
//! `div().tw(...)`.
//!
//! Approximation note: spaced children are wrapped in a margin div, so a
//! child's own `flex_grow`/`flex_shrink` should be set via the wrapper's
//! classes instead (or use `gap-*`, which needs no wrapper).

use gpui::{
    AnyElement, App, IntoElement, ParentElement, RenderOnce, SharedString, Styled, Window, div,
    prelude::FluentBuilder as _,
};

use crate::theme::Theme;

/// A `div` whose Tailwind classes may include child-combinator utilities.
pub fn tw_div(classes: impl Into<SharedString>) -> TwDiv {
    TwDiv {
        classes: classes.into(),
        children: Vec::new(),
    }
}

#[derive(IntoElement)]
pub struct TwDiv {
    classes: SharedString,
    children: Vec<AnyElement>,
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

        super::apply_interactive(div(), styles).children(children_out)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// tw_div is exercised through the parser tests (ext channel) and the
    /// demo window; this smoke test just checks the builder shape.
    #[test]
    fn builder_collects_children() {
        let mut d = tw_div("flex divide-y");
        d.extend([div().into_any_element(), div().into_any_element()]);
        assert_eq!(d.children.len(), 2);
        assert_eq!(d.classes.as_ref(), "flex divide-y");
    }
}
