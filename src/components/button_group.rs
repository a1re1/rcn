//! ButtonGroup — port of shadcn base-vega `ui/button-group.tsx`.
//!
//! Joins a row of [`Button`]s: outer corners keep their rounding, inner
//! edges go square and share borders. Items are typed (`.item(..)` /
//! `.input(..)`) so the group can position-style each segment. Also exports
//! ButtonGroupText and ButtonGroupSeparator for mixed rows.

use gpui::{
    AnyElement, App, Entity, Focusable as _, IntoElement, ParentElement as _, RenderOnce, Styled,
    Window, div, prelude::FluentBuilder as _, px,
};

use crate::components::button::{Button, GroupPosition};
use crate::components::input::Input;
use crate::components::separator::Separator;
use crate::theme::{Theme, alpha};

enum ButtonGroupItem {
    Button(Button),
    Input(Entity<Input>),
}

#[derive(IntoElement)]
pub struct ButtonGroup {
    items: Vec<ButtonGroupItem>,
}

impl ButtonGroup {
    pub fn new() -> Self {
        Self { items: Vec::new() }
    }

    pub fn item(mut self, button: Button) -> Self {
        self.items.push(ButtonGroupItem::Button(button));
        self
    }

    /// A joined input segment (`[&>input]:flex-1`): the group draws the
    /// shared border and squared inner corners around a bare [`Input`]
    /// (`input.set_bare(true)`), like [`InputGroup`](super::InputGroup).
    pub fn input(mut self, input: Entity<Input>) -> Self {
        self.items.push(ButtonGroupItem::Input(input));
        self
    }
}

impl Default for ButtonGroup {
    fn default() -> Self {
        Self::new()
    }
}

impl RenderOnce for ButtonGroup {
    fn render(self, window: &mut Window, cx: &mut App) -> impl IntoElement {
        let theme = Theme::of(cx).clone();
        let count = self.items.len();
        let has_input = self
            .items
            .iter()
            .any(|item| matches!(item, ButtonGroupItem::Input(_)));
        div()
            .flex()
            .flex_row()
            .items_center()
            // w-fit, but an input segment stretches the group (input flex-1).
            .when(has_input, |el| el.w_full())
            .children(self.items.into_iter().enumerate().map(|(index, item)| {
                let position = match (index == 0, index + 1 == count) {
                    (true, true) => GroupPosition::Only,
                    (true, false) => GroupPosition::First,
                    (false, true) => GroupPosition::Last,
                    (false, false) => GroupPosition::Middle,
                };
                match item {
                    ButtonGroupItem::Button(button) => {
                        button.group_position(position).into_any_element()
                    }
                    ButtonGroupItem::Input(input) => {
                        let focused = input.read(cx).focus_handle(cx).is_focused(window);
                        // Input-chrome shell (h-8 border px-2.5), corners
                        // squared on joined edges like Button::group_position.
                        // The focus ring is a border overlay following the
                        // joined corners — gpui box shadows would show through
                        // the transparent bg (see motion::focus_ring_overlay).
                        let ring_radius = theme.radius_lg() + px(3.);
                        let ring_overlay = div()
                            .absolute()
                            .top(px(-4.))
                            .left(px(-4.))
                            .right(px(-4.))
                            .bottom(px(-4.))
                            .border_3()
                            .border_color(alpha(theme.ring, 0.5))
                            .map(|el| match position {
                                GroupPosition::Only => el.rounded(ring_radius),
                                GroupPosition::First => {
                                    el.rounded_l(ring_radius).rounded_r(px(3.))
                                }
                                GroupPosition::Middle => el.rounded(px(3.)),
                                GroupPosition::Last => {
                                    el.rounded_r(ring_radius).rounded_l(px(3.))
                                }
                            });
                        div()
                            .flex()
                            .flex_row()
                            .flex_1()
                            .items_center()
                            .h(px(32.))
                            .border_1()
                            .border_color(if focused { theme.ring } else { theme.input })
                            .map(|el| match position {
                                GroupPosition::Only => el.rounded(theme.radius_lg()),
                                GroupPosition::First => el.rounded_l(theme.radius_lg()),
                                GroupPosition::Middle => el.ml(px(-1.)),
                                GroupPosition::Last => el.rounded_r(theme.radius_lg()).ml(px(-1.)),
                            })
                            .when(theme.dark, |el| el.bg(alpha(theme.input, 0.3)))
                            .px(px(10.))
                            .child(div().flex_1().child(input))
                            .when(focused, |el| el.child(ring_overlay))
                            .into_any_element()
                    }
                }
            }))
    }
}

/// A muted text segment inside a mixed button row (`ButtonGroupText`).
#[derive(IntoElement)]
pub struct ButtonGroupText {
    children: Vec<AnyElement>,
}

impl ButtonGroupText {
    pub fn new() -> Self {
        Self {
            children: Vec::new(),
        }
    }

    pub fn child(mut self, child: impl IntoElement) -> Self {
        self.children.push(child.into_any_element());
        self
    }
}

impl Default for ButtonGroupText {
    fn default() -> Self {
        Self::new()
    }
}

impl RenderOnce for ButtonGroupText {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let theme = Theme::of(cx);
        div()
            .flex()
            .flex_row()
            .items_center()
            .gap(px(6.))
            .px(px(12.))
            .text_size(px(14.))
            .line_height(px(20.))
            .text_color(theme.muted_foreground)
            .children(self.children)
    }
}

/// A vertical rule between grouped rows.
#[derive(IntoElement)]
pub struct ButtonGroupSeparator;

impl ButtonGroupSeparator {
    pub fn new() -> Self {
        Self
    }
}

impl Default for ButtonGroupSeparator {
    fn default() -> Self {
        Self::new()
    }
}

impl RenderOnce for ButtonGroupSeparator {
    fn render(self, _window: &mut Window, _cx: &mut App) -> impl IntoElement {
        div().h(px(20.)).child(Separator::vertical())
    }
}
