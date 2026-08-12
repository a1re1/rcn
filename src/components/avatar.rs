//! Avatar — port of shadcn base-nova `ui/avatar.tsx`.
//!
//! `Avatar` renders an image when a source is given, else the text fallback
//! (initials). `AvatarBadge` is an absolute bottom-right status/icon disc
//! attached via [`Avatar::badge`]. `AvatarGroup` overlaps its avatars with a
//! background ring; `AvatarGroupCount` is the trailing "+N" circle.
//!
//! Sizing and shape overrides come from the caller via [`Styled`].
//!
//! Divergences from the source:
//! - The shadcn `after:` border overlay with `mix-blend-darken` /
//!   `mix-blend-lighten` is approximated by a plain 1px `theme.border` border.
//! - `select-none` is inherent to gpui text (no text selection).
//! - Badge `ring-2 ring-background` and group avatar rings are faked as
//!   opaque background discs / 2px `theme.background` borders (gpui has no
//!   CSS `ring-*` utility).

use gpui::{
    AnyElement, App, Hsla, ImageSource, IntoElement, ObjectFit, ParentElement, Refineable as _,
    RenderOnce, SharedString, StyleRefinement, Styled, StyledImage as _, Window, div, img,
    prelude::FluentBuilder as _, px,
};

use crate::components::Icon;
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

    /// Badge circle: sm size-2 (8px) / default size-2.5 (10px) / lg size-3 (12px).
    fn badge_pixels(self) -> f32 {
        match self {
            Self::Sm => 8.,
            Self::Default => 10.,
            Self::Lg => 12.,
        }
    }

    /// Badge icon size. Sm hides icons; default/lg use size-2 (8px).
    fn badge_icon_pixels(self) -> Option<f32> {
        match self {
            Self::Sm => None,
            Self::Default | Self::Lg => Some(8.),
        }
    }
}

/// Avatar image or initials fallback. Sizing/shape overrides via [`Styled`].
#[derive(IntoElement)]
pub struct Avatar {
    size: AvatarSize,
    image: Option<ImageSource>,
    grayscale: bool,
    fallback: SharedString,
    badge: Option<AvatarBadge>,
    style: StyleRefinement,
}

impl Avatar {
    pub fn new(fallback: impl Into<SharedString>) -> Self {
        Self {
            size: AvatarSize::default(),
            image: None,
            grayscale: false,
            fallback: fallback.into(),
            badge: None,
            style: StyleRefinement::default(),
        }
    }

    /// Render the image desaturated — shadcn `<AvatarImage className="grayscale">`.
    pub fn grayscale(mut self, grayscale: bool) -> Self {
        self.grayscale = grayscale;
        self
    }

    pub fn size(mut self, size: AvatarSize) -> Self {
        self.size = size;
        self
    }

    /// Image source. Embedded asset paths (e.g. `images/avatar.png`) work;
    /// remote URLs would need an http client, which gpui's plain
    /// Application does not have.
    pub fn image(mut self, source: impl Into<ImageSource>) -> Self {
        self.image = Some(source.into());
        self
    }

    /// Attach an [`AvatarBadge`] (shadcn renders it as a child of `<Avatar>`;
    /// the builder API hangs it off the root).
    pub fn badge(mut self, badge: AvatarBadge) -> Self {
        self.badge = Some(badge);
        self
    }
}

impl Styled for Avatar {
    fn style(&mut self) -> &mut StyleRefinement {
        &mut self.style
    }
}

impl RenderOnce for Avatar {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let theme = Theme::of(cx).clone();
        let size = px(self.size.pixels());

        // Root stays relative and unclipped so the badge can overlay at
        // bottom-right. Overflow clipping (and the border ring) live on the
        // inner image/fallback circle.
        //
        // relative flex size-8 shrink-0 rounded-full + a border-border ring
        // (the source draws it with an `after:` overlay; a plain border is the
        // closest gpui equivalent).
        let mut root = div()
            .relative()
            .flex()
            .flex_shrink_0()
            .size(size)
            .child(
                div()
                    .size_full()
                    .rounded_full()
                    .overflow_hidden()
                    .border_1()
                    .border_color(theme.border)
                    .map(|el| match self.image {
                        Some(source) => el.child(
                            img(source)
                                .size_full()
                                .rounded_full()
                                .object_fit(ObjectFit::Cover)
                                .grayscale(self.grayscale),
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
                    }),
            )
            .when_some(self.badge, |el, mut badge| {
                badge.size = self.size;
                el.child(badge)
            });
        root.style().refine(&self.style);
        root
    }
}

/// Absolute bottom-right status/icon disc on an [`Avatar`].
///
/// Source className: `absolute right-0 bottom-0 z-10 inline-flex items-center
/// justify-center rounded-full bg-primary text-primary-foreground ring-2
/// ring-background select-none`, sized by the parent avatar (`size-2` /
/// `size-2.5` / `size-3`). Tailwind's ring paints *outside* the element, so
/// the ring is an enclosing `theme.background` disc 2px larger on every side,
/// overhanging the avatar corner by 2px like the CSS box-shadow does.
///
/// Sizing/shape overrides via [`Styled`] apply to the outer ring disc root.
#[derive(IntoElement)]
pub struct AvatarBadge {
    size: AvatarSize,
    color: Option<Hsla>,
    children: Vec<AnyElement>,
    style: StyleRefinement,
}

impl AvatarBadge {
    pub fn new() -> Self {
        Self {
            size: AvatarSize::default(),
            color: None,
            children: Vec::new(),
            style: StyleRefinement::default(),
        }
    }

    /// Override the default `theme.primary` fill (e.g. docs green status).
    pub fn color(mut self, color: Hsla) -> Self {
        self.color = Some(color);
        self
    }

    /// Convenience for an icon child (rendered at 8px when the parent avatar
    /// is Default/Lg; hidden for Sm). Use [`ParentElement::child`] for arbitrary
    /// content.
    pub fn icon(mut self, path: impl Into<SharedString>) -> Self {
        self.children
            .push(Icon::new(path).size(px(8.)).into_any_element());
        self
    }
}

impl Default for AvatarBadge {
    fn default() -> Self {
        Self::new()
    }
}

impl ParentElement for AvatarBadge {
    fn extend(&mut self, elements: impl IntoIterator<Item = AnyElement>) {
        self.children.extend(elements);
    }
}

impl Styled for AvatarBadge {
    fn style(&mut self) -> &mut StyleRefinement {
        &mut self.style
    }
}

impl RenderOnce for AvatarBadge {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let theme = Theme::of(cx).clone();
        let badge_px = self.size.badge_pixels();
        let show_icon = self.size.badge_icon_pixels().is_some();
        let fill = self.color.unwrap_or(theme.primary);

        // absolute right-0 bottom-0 z-10 inline-flex items-center justify-center
        // rounded-full bg-primary text-primary-foreground ring-2 ring-background.
        // The ring disc is 2px larger than the badge on every side and offset
        // -2px so the badge itself sits at right-0 bottom-0 while the ring
        // overhangs, matching the outside-painted CSS box-shadow.
        let mut root = div()
            .absolute()
            .right(px(-2.))
            .bottom(px(-2.))
            .flex()
            .items_center()
            .justify_center()
            .size(px(badge_px + 4.))
            .rounded_full()
            .bg(theme.background)
            .child(
                div()
                    .flex()
                    .items_center()
                    .justify_center()
                    .size(px(badge_px))
                    .rounded_full()
                    .bg(fill)
                    .text_color(theme.primary_foreground)
                    .when(show_icon, |el| el.children(self.children)),
            );
        root.style().refine(&self.style);
        root
    }
}

/// flex -space-x-2, each avatar ringed with the background color.
///
/// The group sizes its rings itself (gpui can't restyle children the way the
/// source's `*:data-[slot=avatar]:ring-2` selector does), so give it the same
/// size as the avatars inside.
///
/// Sizing/shape overrides via [`Styled`].
#[derive(IntoElement)]
pub struct AvatarGroup {
    size: AvatarSize,
    children: Vec<AnyElement>,
    style: StyleRefinement,
}

impl AvatarGroup {
    pub fn new() -> Self {
        Self {
            size: AvatarSize::default(),
            children: Vec::new(),
            style: StyleRefinement::default(),
        }
    }

    pub fn size(mut self, size: AvatarSize) -> Self {
        self.size = size;
        self
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

impl Styled for AvatarGroup {
    fn style(&mut self) -> &mut StyleRefinement {
        &mut self.style
    }
}

impl RenderOnce for AvatarGroup {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let theme = Theme::of(cx).clone();
        // ring-2 ring-background: an opaque background disc 2px larger than
        // the avatar. A fixed-size disc (rather than a border on an
        // auto-sized wrapper) keeps the circles round under the negative
        // overlap margins.
        let ring = px(self.size.pixels() + 4.);
        // Explicit width: ring + (n-1) * (ring - 8px overlap). The negative
        // child margins otherwise confuse nested min-content measurement
        // (the group collapses when placed inside another flex row, e.g.
        // ItemMedia), letting siblings draw over the avatars.
        let count = self.children.len().max(1) as f32;
        let width = px((self.size.pixels() + 4.) + (count - 1.) * (self.size.pixels() - 4.));
        let mut root = div()
            .flex()
            .flex_row()
            .flex_shrink_0()
            .items_center()
            .w(width)
            .children(self.children.into_iter().enumerate().map(|(index, child)| {
                div()
                    .flex()
                    .flex_shrink_0()
                    .items_center()
                    .justify_center()
                    .size(ring)
                    .rounded_full()
                    .bg(theme.background)
                    .when(index > 0, |el| el.ml(px(-8.)))
                    .child(child)
            }));
        root.style().refine(&self.style);
        root
    }
}

/// The trailing counter circle of an avatar group.
///
/// Children-based like shadcn (`<AvatarGroupCount>+3</AvatarGroupCount>` or an
/// icon child). Text sizing: text-sm (14px), sm → text-xs (12px). Icon children
/// should be sized by the caller: Sm 12px / Default 16px / Lg 20px.
///
/// The enclosing [`AvatarGroup`] supplies the ring and overlap.
///
/// Sizing/shape overrides via [`Styled`].
#[derive(IntoElement)]
pub struct AvatarGroupCount {
    size: AvatarSize,
    children: Vec<AnyElement>,
    style: StyleRefinement,
}

impl AvatarGroupCount {
    pub fn new() -> Self {
        Self {
            size: AvatarSize::default(),
            children: Vec::new(),
            style: StyleRefinement::default(),
        }
    }

    pub fn size(mut self, size: AvatarSize) -> Self {
        self.size = size;
        self
    }

    pub fn child(mut self, child: impl IntoElement) -> Self {
        self.children.push(child.into_any_element());
        self
    }
}

impl Default for AvatarGroupCount {
    fn default() -> Self {
        Self::new()
    }
}

impl ParentElement for AvatarGroupCount {
    fn extend(&mut self, elements: impl IntoIterator<Item = AnyElement>) {
        self.children.extend(elements);
    }
}

impl Styled for AvatarGroupCount {
    fn style(&mut self) -> &mut StyleRefinement {
        &mut self.style
    }
}

impl RenderOnce for AvatarGroupCount {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let theme = Theme::of(cx).clone();
        // relative flex size-8 shrink-0 items-center justify-center rounded-full
        // bg-muted text-sm text-muted-foreground (ring supplied by AvatarGroup)
        let mut root = div()
            .relative()
            .flex()
            .flex_shrink_0()
            .size(px(self.size.pixels()))
            .items_center()
            .justify_center()
            .rounded_full()
            .bg(theme.muted)
            .text_color(theme.muted_foreground)
            .text_size(px(self.size.text_size()))
            .children(self.children);
        root.style().refine(&self.style);
        root
    }
}
