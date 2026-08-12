//! Sheet — port of shadcn base-vega `ui/sheet.tsx`.
//!
//! A panel that extends from a screen edge over a dimmed backdrop.
//! Controlled like Dialog (backdrop click and the close button dismiss).
//! Header/Title/Description/Footer share the Dialog shapes; the slide
//! animation is omitted.
//!
//! Sizing and shape overrides come from the caller via [`Styled`] and apply
//! to the sliding sheet panel (the element carrying background, border, and
//! shadow), not the dimmed backdrop.

use gpui::{
    AnyElement, App, ElementId, InteractiveElement as _, IntoElement, ParentElement,
    Refineable as _, RenderOnce, StatefulInteractiveElement as _, StyleRefinement, Styled, Window,
    anchored, deferred, div, point, prelude::FluentBuilder as _, px, svg,
};

use crate::components::dialog::OpenChangeHandler;
pub use crate::components::dialog::{
    DialogDescription as SheetDescription, DialogTitle as SheetTitle,
};
use crate::motion;
use crate::theme::{Theme, alpha};

#[derive(Clone, Copy, PartialEq, Eq, Default, Debug)]
pub enum SheetSide {
    Top,
    #[default]
    Right,
    Bottom,
    Left,
}

/// Sliding edge panel. Sizing and shape overrides via [`Styled`] target the
/// sheet panel root (bg/border/shadow), not the backdrop.
#[derive(IntoElement)]
pub struct Sheet {
    id: ElementId,
    open: bool,
    side: SheetSide,
    on_open_change: Option<OpenChangeHandler>,
    children: Vec<AnyElement>,
    style: StyleRefinement,
}

impl Sheet {
    pub fn new(id: impl Into<ElementId>) -> Self {
        Self {
            id: id.into(),
            open: false,
            side: SheetSide::default(),
            on_open_change: None,
            children: Vec::new(),
            style: StyleRefinement::default(),
        }
    }

    pub fn open(mut self, open: bool) -> Self {
        self.open = open;
        self
    }

    pub fn side(mut self, side: SheetSide) -> Self {
        self.side = side;
        self
    }

    pub fn on_open_change(
        mut self,
        handler: impl Fn(&bool, &mut Window, &mut App) + 'static,
    ) -> Self {
        self.on_open_change = Some(std::rc::Rc::new(handler));
        self
    }
}

impl ParentElement for Sheet {
    fn extend(&mut self, elements: impl IntoIterator<Item = AnyElement>) {
        self.children.extend(elements);
    }
}

impl Styled for Sheet {
    fn style(&mut self) -> &mut StyleRefinement {
        &mut self.style
    }
}

impl RenderOnce for Sheet {
    fn render(self, window: &mut Window, cx: &mut App) -> impl IntoElement {
        if !self.open {
            return div().into_any_element();
        }
        let theme = Theme::of(cx).clone();
        let viewport = window.viewport_size();
        let close = self.on_open_change.clone();
        let close_button = self.on_open_change;

        // Panel: bg-background flex-col gap-4; side rails are w-3/4 max-w-sm
        // full height with the shared edge bordered.
        let mut panel = div()
            .occlude()
            .relative()
            .flex()
            .flex_col()
            .gap(px(16.))
            .bg(theme.background)
            .text_size(px(14.))
            .line_height(px(20.))
            .text_color(theme.foreground)
            .shadow_lg()
            .p(px(24.))
            .map(|el| match self.side {
                SheetSide::Right => el
                    .h_full()
                    .w(px(384.).min(viewport.width * 0.75))
                    .border_l_1()
                    .border_color(theme.border),
                SheetSide::Left => el
                    .h_full()
                    .w(px(384.).min(viewport.width * 0.75))
                    .border_r_1()
                    .border_color(theme.border),
                SheetSide::Top => el.w_full().border_b_1().border_color(theme.border),
                SheetSide::Bottom => el.w_full().border_t_1().border_color(theme.border),
            })
            .children(self.children)
            .when_some(close_button, |el, close| {
                el.child({
                    let ring = motion::focus_ring(&theme);
                    div()
                        .id("sheet-close")
                        .absolute()
                        .top(px(16.))
                        .right(px(16.))
                        .rounded(theme.radius_sm())
                        .p(px(2.))
                        .tab_index(0)
                        .focus_visible(move |s| s.border_color(theme.ring).shadow(ring.clone()))
                        .hover(|s| s.bg(alpha(theme.muted, 0.8)))
                        .on_click(move |_, window, cx| close(&false, window, cx))
                        .child(
                            svg()
                                .path(theme.icons.x())
                                .size(px(16.))
                                .text_color(theme.muted_foreground),
                        )
                })
            });
        panel.style().refine(&self.style);

        deferred(
            anchored().position(point(px(0.), px(0.))).child(
                div()
                    .id(self.id)
                    .occlude()
                    .w(viewport.width)
                    .h(viewport.height)
                    .flex()
                    .map(|el| match self.side {
                        SheetSide::Right => el.flex_row().justify_end(),
                        SheetSide::Left => el.flex_row().justify_start(),
                        SheetSide::Top => el.flex_col().justify_start(),
                        SheetSide::Bottom => el.flex_col().justify_end(),
                    })
                    .bg(gpui::hsla(0., 0., 0., 0.5))
                    .when_some(close, |el, close| {
                        el.on_click(move |_, window, cx| close(&false, window, cx))
                    })
                    .child(match self.side {
                        // 500ms ease-in-out slide from the sheet's edge.
                        SheetSide::Right => crate::motion::slide_in("sheet-in", true, 384., panel),
                        SheetSide::Left => crate::motion::slide_in("sheet-in", true, -384., panel),
                        SheetSide::Top => crate::motion::slide_in("sheet-in", false, -240., panel),
                        SheetSide::Bottom => {
                            crate::motion::slide_in("sheet-in", false, 240., panel)
                        }
                    }),
            ),
        )
        .into_any_element()
    }
}

/// flex flex-col gap-1.5 p-4 — sheet headers carry their own padding in
/// the source; here the panel pads, so this is just the stack.
///
/// Sizing and shape overrides come from the caller via [`Styled`].
#[derive(IntoElement)]
pub struct SheetHeader {
    children: Vec<AnyElement>,
    style: StyleRefinement,
}

impl SheetHeader {
    pub fn new() -> Self {
        Self {
            children: Vec::new(),
            style: StyleRefinement::default(),
        }
    }
}

impl Default for SheetHeader {
    fn default() -> Self {
        Self::new()
    }
}

impl ParentElement for SheetHeader {
    fn extend(&mut self, elements: impl IntoIterator<Item = AnyElement>) {
        self.children.extend(elements);
    }
}

impl Styled for SheetHeader {
    fn style(&mut self) -> &mut StyleRefinement {
        &mut self.style
    }
}

impl RenderOnce for SheetHeader {
    fn render(self, _window: &mut Window, _cx: &mut App) -> impl IntoElement {
        let mut root = div().flex().flex_col().gap(px(6.)).children(self.children);
        root.style().refine(&self.style);
        root
    }
}

/// mt-auto flex flex-col gap-2 — pinned to the bottom of the sheet.
///
/// Sizing and shape overrides come from the caller via [`Styled`].
#[derive(IntoElement)]
pub struct SheetFooter {
    children: Vec<AnyElement>,
    style: StyleRefinement,
}

impl SheetFooter {
    pub fn new() -> Self {
        Self {
            children: Vec::new(),
            style: StyleRefinement::default(),
        }
    }
}

impl Default for SheetFooter {
    fn default() -> Self {
        Self::new()
    }
}

impl ParentElement for SheetFooter {
    fn extend(&mut self, elements: impl IntoIterator<Item = AnyElement>) {
        self.children.extend(elements);
    }
}

impl Styled for SheetFooter {
    fn style(&mut self) -> &mut StyleRefinement {
        &mut self.style
    }
}

impl RenderOnce for SheetFooter {
    fn render(self, _window: &mut Window, _cx: &mut App) -> impl IntoElement {
        let mut root = div()
            .mt_auto()
            .flex()
            .flex_col()
            .gap(px(8.))
            .children(self.children);
        root.style().refine(&self.style);
        root
    }
}
