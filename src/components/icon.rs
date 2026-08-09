//! Icon — a `currentColor` svg.
//!
//! gpui's raw `svg()` element only paints when its own style carries an
//! explicit `text_color`; it never consults the inherited text style, so an
//! un-colored icon inside a button silently renders nothing. This element is
//! the web's `currentColor`: at paint time it reads the ambient
//! `window.text_style()` — which parent elements like [`crate::components::Button`]
//! refine via `.text_color(..)` — and paints the svg asset in that color.
//! `.text_color(..)` on the icon itself still overrides, like a CSS `text-*`
//! utility on the svg would.
//!
//! Defaults to the shadcn icon box (`size-4` = 16px); `.size(..)` overrides.

use gpui::{
    App, Bounds, Element, ElementId, GlobalElementId, InspectorElementId, IntoElement, LayoutId,
    Pixels, Radians, Refineable as _, SharedString, Style, StyleRefinement, Styled,
    TransformationMatrix, Window, px,
};

pub struct Icon {
    path: SharedString,
    style: StyleRefinement,
    /// Rotation about the icon's center, for spinners.
    rotation: Option<Radians>,
}

impl Icon {
    pub fn new(path: impl Into<SharedString>) -> Self {
        let mut icon = Self {
            path: path.into(),
            style: StyleRefinement::default(),
            rotation: None,
        };
        icon.style.size.width = Some(px(16.).into());
        icon.style.size.height = Some(px(16.).into());
        icon
    }

    /// Rotate the drawing about its center (the svg `transform: rotate(..)`
    /// used by spinner animations).
    pub fn rotate(mut self, angle: impl Into<Radians>) -> Self {
        self.rotation = Some(angle.into());
        self
    }
}

impl Styled for Icon {
    fn style(&mut self) -> &mut StyleRefinement {
        &mut self.style
    }
}

impl IntoElement for Icon {
    type Element = Self;

    fn into_element(self) -> Self::Element {
        self
    }
}

impl Element for Icon {
    type RequestLayoutState = ();
    type PrepaintState = ();

    fn id(&self) -> Option<ElementId> {
        None
    }

    fn source_location(&self) -> Option<&'static core::panic::Location<'static>> {
        None
    }

    fn request_layout(
        &mut self,
        _id: Option<&GlobalElementId>,
        _inspector_id: Option<&InspectorElementId>,
        window: &mut Window,
        cx: &mut App,
    ) -> (LayoutId, Self::RequestLayoutState) {
        let mut style = Style::default();
        style.refine(&self.style);
        (window.request_layout(style, [], cx), ())
    }

    fn prepaint(
        &mut self,
        _id: Option<&GlobalElementId>,
        _inspector_id: Option<&InspectorElementId>,
        _bounds: Bounds<Pixels>,
        _request_layout: &mut Self::RequestLayoutState,
        _window: &mut Window,
        _cx: &mut App,
    ) -> Self::PrepaintState {
    }

    fn paint(
        &mut self,
        _id: Option<&GlobalElementId>,
        _inspector_id: Option<&InspectorElementId>,
        bounds: Bounds<Pixels>,
        _request_layout: &mut Self::RequestLayoutState,
        _prepaint: &mut Self::PrepaintState,
        window: &mut Window,
        cx: &mut App,
    ) {
        // currentColor: an explicit `.text_color(..)` wins, otherwise the
        // inherited text style set by the parent element.
        let color = self
            .style
            .text
            .color
            .unwrap_or_else(|| window.text_style().color);
        // Rotation about the center, matching Svg's private
        // Transformation::into_matrix for a rotate-only transform.
        let transformation = match self.rotation {
            Some(angle) => {
                let scale_factor = window.scale_factor();
                TransformationMatrix::unit()
                    .translate(bounds.center().scale(scale_factor))
                    .rotate(angle)
                    .translate(bounds.center().scale(-scale_factor))
            }
            None => TransformationMatrix::unit(),
        };
        window
            .paint_svg(bounds, self.path.clone(), None, transformation, color, cx)
            .ok();
    }
}
