//! AspectRatio — port of shadcn base-vega `ui/aspect-ratio.tsx`.
//!
//! Constrains its content to a width/height ratio (`aspect-(--ratio)`),
//! via gpui's native aspect-ratio style.
//!
//! Sizing and shape overrides come from the caller via [`Styled`].

use gpui::{
    AnyElement, App, IntoElement, ParentElement, Refineable as _, RenderOnce, StyleRefinement,
    Styled, Window, div,
};

/// Constrains children to a fixed width/height ratio.
///
/// Sizing and shape overrides come from the caller via [`Styled`].
#[derive(IntoElement)]
pub struct AspectRatio {
    ratio: f32,
    children: Vec<AnyElement>,
    style: StyleRefinement,
}

impl AspectRatio {
    /// `ratio` is width / height (e.g. `16. / 9.`).
    pub fn new(ratio: f32) -> Self {
        Self {
            ratio,
            children: Vec::new(),
            style: StyleRefinement::default(),
        }
    }
}

impl Styled for AspectRatio {
    fn style(&mut self) -> &mut StyleRefinement {
        &mut self.style
    }
}

impl ParentElement for AspectRatio {
    fn extend(&mut self, elements: impl IntoIterator<Item = AnyElement>) {
        self.children.extend(elements);
    }
}

impl RenderOnce for AspectRatio {
    fn render(self, _window: &mut Window, _cx: &mut App) -> impl IntoElement {
        // relative aspect-(--ratio)
        let mut root = div()
            .relative()
            .w_full()
            .aspect_ratio(self.ratio)
            .children(self.children);
        root.style().refine(&self.style);
        root
    }
}
