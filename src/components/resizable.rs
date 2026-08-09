//! Resizable — port of shadcn base-vega `ui/resizable.tsx`
//! (react-resizable-panels upstream).
//!
//! A two-panel group split by a draggable handle. Controlled: the caller
//! owns the first panel's fraction and receives updates while dragging.
//! N-panel groups, keyboard resize, and collapse are omitted.

use std::rc::Rc;

use gpui::{
    AnyElement, App, AppContext as _, Context, DragMoveEvent, ElementId, InteractiveElement as _,
    IntoElement, ParentElement as _, Render, RenderOnce, StatefulInteractiveElement as _, Styled,
    Window, div, prelude::FluentBuilder as _, px, relative,
};

use crate::theme::Theme;

#[derive(Clone, Copy, PartialEq, Eq, Default, Debug)]
pub enum ResizableDirection {
    #[default]
    Horizontal,
    Vertical,
}

type FractionChangeHandler = Rc<dyn Fn(&f32, &mut Window, &mut App) + 'static>;

struct HandleDrag(ElementId);

struct HandleDragPreview;

impl Render for HandleDragPreview {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        gpui::Empty
    }
}

#[derive(IntoElement)]
pub struct ResizablePanelGroup {
    id: ElementId,
    direction: ResizableDirection,
    /// First panel's share, 0.1..=0.9.
    fraction: f32,
    first: Option<AnyElement>,
    second: Option<AnyElement>,
    on_fraction_change: Option<FractionChangeHandler>,
}

impl ResizablePanelGroup {
    pub fn new(id: impl Into<ElementId>) -> Self {
        Self {
            id: id.into(),
            direction: ResizableDirection::default(),
            fraction: 0.5,
            first: None,
            second: None,
            on_fraction_change: None,
        }
    }

    pub fn direction(mut self, direction: ResizableDirection) -> Self {
        self.direction = direction;
        self
    }

    pub fn fraction(mut self, fraction: f32) -> Self {
        self.fraction = fraction.clamp(0.1, 0.9);
        self
    }

    pub fn first(mut self, panel: impl IntoElement) -> Self {
        self.first = Some(panel.into_any_element());
        self
    }

    pub fn second(mut self, panel: impl IntoElement) -> Self {
        self.second = Some(panel.into_any_element());
        self
    }

    pub fn on_fraction_change(
        mut self,
        handler: impl Fn(&f32, &mut Window, &mut App) + 'static,
    ) -> Self {
        self.on_fraction_change = Some(Rc::new(handler));
        self
    }
}

impl RenderOnce for ResizablePanelGroup {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let theme = Theme::of(cx).clone();
        let horizontal = self.direction == ResizableDirection::Horizontal;
        let fraction = self.fraction.clamp(0.1, 0.9);
        let id = self.id.clone();

        // Handle: a 1px border line with a grip pill, draggable across the
        // whole group (the drag listener lives on the group so the fraction
        // maps to the full span).
        let handle = div()
            .id("resizable-handle")
            .flex()
            .flex_shrink_0()
            .items_center()
            .justify_center()
            .map(|el| {
                if horizontal {
                    el.w(px(1.)).h_full()
                } else {
                    el.h(px(1.)).w_full()
                }
            })
            .bg(theme.border)
            .on_drag(HandleDrag(id.clone()), |_, _, _, cx| {
                cx.new(|_| HandleDragPreview)
            })
            .child(
                div()
                    .flex_shrink_0()
                    .rounded(px(2.))
                    .border_1()
                    .border_color(theme.border)
                    .bg(theme.muted)
                    .map(|el| {
                        if horizontal {
                            el.w(px(6.)).h(px(16.))
                        } else {
                            el.h(px(6.)).w(px(16.))
                        }
                    }),
            );

        div()
            .id(self.id)
            .flex()
            .map(|el| {
                if horizontal {
                    el.flex_row()
                } else {
                    el.flex_col()
                }
            })
            .w_full()
            .h_full()
            .rounded(theme.radius_lg())
            .border_1()
            .border_color(theme.border)
            .overflow_hidden()
            .when_some(self.on_fraction_change, |el, handler| {
                el.on_drag_move(move |event: &DragMoveEvent<HandleDrag>, window, cx| {
                    if event.drag(cx).0 == id {
                        let f = if horizontal {
                            (event.event.position.x - event.bounds.origin.x)
                                / event.bounds.size.width
                        } else {
                            (event.event.position.y - event.bounds.origin.y)
                                / event.bounds.size.height
                        };
                        handler(&f.clamp(0.1, 0.9), window, cx);
                    }
                })
            })
            .child(
                div()
                    .map(|el| {
                        if horizontal {
                            el.w(relative(fraction)).h_full()
                        } else {
                            el.h(relative(fraction)).w_full()
                        }
                    })
                    .overflow_hidden()
                    .children(self.first),
            )
            .child(handle)
            .child(div().flex_1().overflow_hidden().children(self.second))
    }
}
