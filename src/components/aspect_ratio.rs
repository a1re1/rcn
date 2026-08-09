//! AspectRatio — port of shadcn base-vega `ui/aspect-ratio.tsx`.
//!
//! Constrains its content to a width/height ratio (`aspect-(--ratio)`),
//! via gpui's native aspect-ratio style.

use gpui::{AnyElement, App, IntoElement, ParentElement, RenderOnce, Styled, Window, div};

#[derive(IntoElement)]
pub struct AspectRatio {
    ratio: f32,
    children: Vec<AnyElement>,
}

impl AspectRatio {
    /// `ratio` is width / height (e.g. `16. / 9.`).
    pub fn new(ratio: f32) -> Self {
        Self {
            ratio,
            children: Vec::new(),
        }
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
        div()
            .relative()
            .w_full()
            .aspect_ratio(self.ratio)
            .children(self.children)
    }
}
