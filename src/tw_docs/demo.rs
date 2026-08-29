//! The demo DSL: a static tree that transcribes a docs snippet element for
//! element, rendered through `tw_div` so every class goes through the parser.
//!
//! Tailwind's docs draw demos with palette colors (a tinted area for the
//! utility's effect and a saturated labeled box for the element) — the same
//! palette classes work here, so demos read like the snippet plus colors.

use gpui::{AnyElement, IntoElement, ParentElement as _, Styled, div, px};

use crate::theme::Theme;
use crate::tw::element::tw_div;

/// One node of a demo tree.
pub enum Node {
    /// An element with Tailwind classes and children (`<div class="…">…</div>`).
    El {
        classes: &'static str,
        children: &'static [Node],
    },
    /// `<div class="…">label</div>` — an element whose only child is a label.
    Labeled {
        classes: &'static str,
        label: &'static str,
    },
    /// A text run. Rendered in the demo's monospace label style — the docs
    /// label boxes with their own class names.
    Text(&'static str),
    /// A text run in the theme's sans font — for prose demos (typography).
    Prose(&'static str),
    /// Escape hatch for demos the DSL cannot express (images, inputs).
    Custom(fn(&Theme) -> AnyElement),
}

/// `<div class="…">` with children.
pub const fn el(classes: &'static str, children: &'static [Node]) -> Node {
    Node::El { classes, children }
}

/// `<div class="…">label</div>` — the most common docs shape.
pub const fn labeled(classes: &'static str, label: &'static str) -> Node {
    Node::Labeled { classes, label }
}

impl Node {
    pub fn render(&self, theme: &Theme) -> AnyElement {
        match self {
            Node::El { classes, children } => tw_div(*classes)
                .children(children.iter().map(|child| child.render(theme)))
                .into_any_element(),
            Node::Labeled { classes, label } => tw_div(*classes)
                .child(Node::Text(label).render(theme))
                .into_any_element(),
            Node::Text(text) => div()
                .font_family("Menlo")
                .text_size(px(13.))
                .line_height(px(20.))
                .child(*text)
                .into_any_element(),
            Node::Prose(text) => div().when_some_font(theme).child(*text).into_any_element(),
            Node::Custom(render) => render(theme),
        }
    }

    /// Visit every class string in the tree (for the parse tests).
    pub fn walk_classes(&self, visit: &mut dyn FnMut(&'static str)) {
        match self {
            Node::El { classes, children } => {
                visit(classes);
                for child in *children {
                    child.walk_classes(visit);
                }
            }
            Node::Labeled { classes, .. } => visit(classes),
            Node::Text(_) | Node::Prose(_) | Node::Custom(_) => {}
        }
    }
}

trait FontExt: Styled + Sized {
    fn when_some_font(self, theme: &Theme) -> Self {
        match theme.font_sans.clone() {
            Some(font) => self.font_family(font),
            None => self,
        }
    }
}

impl<T: Styled> FontExt for T {}

/// The frame every demo renders inside: the docs' checkerboard-free neutral
/// stage, monospace labels, centered content.
pub fn stage(theme: &Theme, demo: &Node) -> AnyElement {
    div()
        .w_full()
        .flex()
        .justify_center()
        .text_color(theme.foreground)
        .child(demo.render(theme))
        .into_any_element()
}
