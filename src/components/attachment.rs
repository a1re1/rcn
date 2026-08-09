//! Attachment — port of shadcn base-vega `ui/attachment.tsx`.
//!
//! A file chip: media tile (icon or image slot) + name/description +
//! optional remove button, on a card border (dashed-idle and upload
//! states are approximated with border/opacity; vertical orientation and
//! progress overlays are omitted).

use gpui::{
    AnyElement, App, ClickEvent, ElementId, FontWeight, InteractiveElement as _, IntoElement,
    ParentElement as _, RenderOnce, SharedString, StatefulInteractiveElement as _, Styled, Window,
    div, prelude::FluentBuilder as _, px, svg,
};

use crate::theme::{Theme, alpha};

#[derive(Clone, Copy, PartialEq, Eq, Default, Debug)]
pub enum AttachmentState {
    #[default]
    Done,
    Uploading,
    Error,
}

#[derive(IntoElement)]
pub struct Attachment {
    id: ElementId,
    name: SharedString,
    description: Option<SharedString>,
    state: AttachmentState,
    media: Option<AnyElement>,
    on_remove: Option<Box<dyn Fn(&ClickEvent, &mut Window, &mut App) + 'static>>,
}

impl Attachment {
    pub fn new(id: impl Into<ElementId>, name: impl Into<SharedString>) -> Self {
        Self {
            id: id.into(),
            name: name.into(),
            description: None,
            state: AttachmentState::default(),
            media: None,
            on_remove: None,
        }
    }

    /// Secondary line (size, type, or upload status).
    pub fn description(mut self, description: impl Into<SharedString>) -> Self {
        self.description = Some(description.into());
        self
    }

    pub fn state(mut self, state: AttachmentState) -> Self {
        self.state = state;
        self
    }

    /// Custom media tile content (defaults to a file glyph).
    pub fn media(mut self, media: impl IntoElement) -> Self {
        self.media = Some(media.into_any_element());
        self
    }

    pub fn on_remove(
        mut self,
        handler: impl Fn(&ClickEvent, &mut Window, &mut App) + 'static,
    ) -> Self {
        self.on_remove = Some(Box::new(handler));
        self
    }
}

impl RenderOnce for Attachment {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let theme = Theme::of(cx).clone();
        let error = self.state == AttachmentState::Error;

        // Media tile: aspect-square w-10 rounded-lg bg-muted.
        let media = div()
            .flex()
            .flex_shrink_0()
            .size(px(40.))
            .items_center()
            .justify_center()
            .overflow_hidden()
            .rounded(theme.radius_lg())
            .map(|el| {
                if error {
                    el.bg(alpha(theme.destructive, 0.1))
                        .text_color(theme.destructive)
                } else {
                    el.bg(theme.muted).text_color(theme.foreground)
                }
            })
            .map(|el| match self.media {
                Some(media) => el.child(media),
                None => el.child(
                    svg()
                        .path(theme.icons.chevron_right())
                        .size(px(16.))
                        .text_color(if error {
                            theme.destructive
                        } else {
                            theme.muted_foreground
                        }),
                ),
            });

        div()
            .id(self.id)
            .flex()
            .flex_row()
            .items_center()
            .gap(px(8.))
            .min_w(px(160.))
            .rounded(theme.radius_xl())
            .border_1()
            .border_color(if error {
                alpha(theme.destructive, 0.3)
            } else {
                theme.border
            })
            .bg(theme.card)
            .text_color(theme.card_foreground)
            .p(px(8.))
            .when(self.state == AttachmentState::Uploading, |el| {
                el.opacity(0.7)
            })
            .child(media)
            .child(
                div()
                    .flex()
                    .flex_col()
                    .flex_1()
                    .min_w(px(0.))
                    .child(
                        div()
                            .text_size(px(14.))
                            .line_height(px(20.))
                            .font_weight(FontWeight::MEDIUM)
                            .whitespace_nowrap()
                            .overflow_hidden()
                            .child(self.name),
                    )
                    .when_some(self.description, |el, description| {
                        el.child(
                            div()
                                .text_size(px(12.))
                                .line_height(px(16.))
                                .text_color(if error {
                                    theme.destructive
                                } else {
                                    theme.muted_foreground
                                })
                                .child(description),
                        )
                    }),
            )
            .when_some(self.on_remove, |el, on_remove| {
                el.child(
                    div()
                        .id("attachment-remove")
                        .flex_shrink_0()
                        .rounded(theme.radius_sm())
                        .p(px(2.))
                        .hover(|s| s.bg(alpha(theme.muted, 0.8)))
                        .on_click(on_remove)
                        .child(
                            svg()
                                .path(theme.icons.x())
                                .size(px(14.))
                                .text_color(theme.muted_foreground),
                        ),
                )
            })
    }
}
