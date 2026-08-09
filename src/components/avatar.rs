//! Avatar — port of shadcn base-vega `ui/avatar.tsx`.
//!
//! `Avatar` renders an image when a source is given, else the text fallback
//! (initials). `AvatarGroup` overlaps its avatars with a background ring;
//! `AvatarGroupCount` is the trailing "+N" circle.

use gpui::{
    AnyElement, App, ImageSource, IntoElement, ObjectFit, ParentElement, RenderOnce, SharedString,
    Styled, StyledImage as _, Window, div, img, prelude::FluentBuilder as _, px,
};

use crate::theme::Theme;

#[derive(Clone, Copy, PartialEq, Eq, Default, Debug)]
pub enum AvatarSize {
    Sm,
    #[default]
    Default,
    Lg,
}

impl AvatarSize {
    /// size-6 / size-8 / size-10
    fn pixels(self) -> f32 {
        match self {
            Self::Sm => 24.,
            Self::Default => 32.,
            Self::Lg => 40.,
        }
    }

    /// text-sm, sm → text-xs
    fn text_size(self) -> f32 {
        match self {
            Self::Sm => 12.,
            _ => 14.,
        }
    }
}

#[derive(IntoElement)]
pub struct Avatar {
    size: AvatarSize,
    image: Option<ImageSource>,
    fallback: SharedString,
}

impl Avatar {
    pub fn new(fallback: impl Into<SharedString>) -> Self {
        Self {
            size: AvatarSize::default(),
            image: None,
            fallback: fallback.into(),
        }
    }

    pub fn size(mut self, size: AvatarSize) -> Self {
        self.size = size;
        self
    }

    // TODO(rcn): exercise in the storybook once an http client is wired up
    // for remote image sources (gpui's plain Application has none).
    #[allow(dead_code)]
    pub fn image(mut self, source: impl Into<ImageSource>) -> Self {
        self.image = Some(source.into());
        self
    }
}

impl RenderOnce for Avatar {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let theme = Theme::of(cx).clone();
        let size = px(self.size.pixels());

        // relative flex size-8 shrink-0 rounded-full + a border-border ring
        // (the source draws it with an `after:` overlay; a plain border is the
        // closest gpui equivalent).
        div()
            .relative()
            .flex()
            .flex_shrink_0()
            .size(size)
            .rounded_full()
            .overflow_hidden()
            .border_1()
            .border_color(theme.border)
            .map(|el| match self.image {
                Some(source) => el.child(
                    img(source)
                        .size_full()
                        .rounded_full()
                        .object_fit(ObjectFit::Cover),
                ),
                // flex size-full items-center justify-center bg-muted
                // text-muted-foreground
                None => el.child(
                    div()
                        .flex()
                        .size_full()
                        .items_center()
                        .justify_center()
                        .rounded_full()
                        .bg(theme.muted)
                        .text_color(theme.muted_foreground)
                        .text_size(px(self.size.text_size()))
                        .child(self.fallback),
                ),
            })
    }
}

/// flex -space-x-2, each avatar ringed with the background color.
#[derive(IntoElement)]
pub struct AvatarGroup {
    children: Vec<AnyElement>,
}

impl AvatarGroup {
    pub fn new() -> Self {
        Self {
            children: Vec::new(),
        }
    }
}

impl Default for AvatarGroup {
    fn default() -> Self {
        Self::new()
    }
}

impl ParentElement for AvatarGroup {
    fn extend(&mut self, elements: impl IntoIterator<Item = AnyElement>) {
        self.children.extend(elements);
    }
}

impl RenderOnce for AvatarGroup {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let theme = Theme::of(cx).clone();
        div()
            .flex()
            .flex_row()
            .children(self.children.into_iter().enumerate().map(|(index, child)| {
                // ring-2 ring-background, -space-x-2
                div()
                    .rounded_full()
                    .border_2()
                    .border_color(theme.background)
                    .when(index > 0, |el| el.ml(px(-8.)))
                    .child(child)
            }))
    }
}

/// The trailing "+N" counter circle of an avatar group.
#[derive(IntoElement)]
pub struct AvatarGroupCount {
    size: AvatarSize,
    count: usize,
}

impl AvatarGroupCount {
    pub fn new(count: usize) -> Self {
        Self {
            size: AvatarSize::default(),
            count,
        }
    }

    pub fn size(mut self, size: AvatarSize) -> Self {
        self.size = size;
        self
    }
}

impl RenderOnce for AvatarGroupCount {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let theme = Theme::of(cx).clone();
        div()
            .rounded_full()
            .border_2()
            .border_color(theme.background)
            .ml(px(-8.))
            .child(
                div()
                    .flex()
                    .flex_shrink_0()
                    .size(px(self.size.pixels()))
                    .items_center()
                    .justify_center()
                    .rounded_full()
                    .bg(theme.muted)
                    .text_color(theme.muted_foreground)
                    .text_size(px(self.size.text_size()))
                    .child(format!("+{}", self.count)),
            )
    }
}
