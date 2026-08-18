//! Carousel — port of shadcn base-vega `ui/carousel.tsx` (embla upstream).
//!
//! One visible slide with round previous/next buttons at the sides.
//! Controlled: the caller owns the index. Swipe gestures, momentum, and
//! multi-slide viewports are omitted.
//!
//! Sizing and shape overrides come from the caller via [`Styled`].

use std::rc::Rc;

use gpui::{
    AnyElement, App, ElementId, InteractiveElement as _, IntoElement, ParentElement,
    Refineable as _, RenderOnce, StatefulInteractiveElement as _, StyleRefinement, Styled, Window,
    div, prelude::FluentBuilder as _, px, svg,
};

use crate::theme::Theme;

type IndexChangeHandler = Rc<dyn Fn(&usize, &mut Window, &mut App) + 'static>;

/// One-at-a-time slide viewer with prev/next controls.
///
/// Sizing and shape overrides come from the caller via [`Styled`].
#[derive(IntoElement)]
pub struct Carousel {
    id: ElementId,
    index: usize,
    slides: Vec<AnyElement>,
    on_index_change: Option<IndexChangeHandler>,
    style: StyleRefinement,
}

impl Carousel {
    pub fn new(id: impl Into<ElementId>) -> Self {
        Self {
            id: id.into(),
            index: 0,
            slides: Vec::new(),
            on_index_change: None,
            style: StyleRefinement::default(),
        }
    }

    pub fn index(mut self, index: usize) -> Self {
        self.index = index;
        self
    }

    pub fn on_index_change(
        mut self,
        handler: impl Fn(&usize, &mut Window, &mut App) + 'static,
    ) -> Self {
        self.on_index_change = Some(Rc::new(handler));
        self
    }
}

impl ParentElement for Carousel {
    fn extend(&mut self, elements: impl IntoIterator<Item = AnyElement>) {
        self.slides.extend(elements);
    }
}

impl Styled for Carousel {
    fn style(&mut self) -> &mut StyleRefinement {
        &mut self.style
    }
}

impl RenderOnce for Carousel {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let theme = Theme::of(cx).clone();
        let count = self.slides.len().max(1);
        let index = self.index.min(count - 1);

        let arrow = |id: &'static str, icon: String, target: Option<usize>| {
            let handler = self.on_index_change.clone();
            let enabled = target.is_some();
            div()
                .id(id)
                .flex()
                .flex_shrink_0()
                .size(px(32.))
                .items_center()
                .justify_center()
                .rounded_full()
                .border_1()
                .border_color(theme.border)
                .bg(theme.background)
                .shadow_xs()
                .when(!enabled, |el| el.opacity(0.5))
                .when(enabled, |el| {
                    el.hover(|s| s.bg(theme.muted))
                        .when_some(handler, |el, handler| {
                            el.on_click(move |_, window, cx| {
                                if let Some(target) = target {
                                    handler(&target, window, cx)
                                }
                            })
                        })
                })
                .child(svg().path(icon).size(px(16.)).text_color(theme.foreground))
        };

        let visible = self
            .slides
            .into_iter()
            .nth(index)
            .unwrap_or_else(|| div().into_any_element());

        let mut root = div()
            .id(self.id)
            .flex()
            .flex_row()
            .items_center()
            .gap(px(12.))
            .child(arrow(
                "carousel-prev",
                theme.icons.chevron_left(),
                index.checked_sub(1),
            ))
            .child(
                div()
                    .flex()
                    .flex_col()
                    .items_center()
                    .gap(px(8.))
                    .child(visible)
                    .child(
                        div()
                            .text_size(px(12.))
                            .text_color(theme.muted_foreground)
                            .child(format!("{} / {count}", index + 1)),
                    ),
            )
            .child(arrow(
                "carousel-next",
                theme.icons.chevron_right(),
                (index + 1 < count).then_some(index + 1),
            ));
        root.style().refine(&self.style);
        root
    }
}
