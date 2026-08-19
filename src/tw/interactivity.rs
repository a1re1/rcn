//! Interactivity family: mouse cursors. Most of this docs chapter
//! (scroll/snap/touch/select behavior) is behavioral in gpui, not style —
//! see the coverage ledger.
//! Docs chapter: <https://tailwindcss.com/docs/cursor>

use gpui::{CursorStyle, StyleRefinement};

use super::Ctx;

pub(super) fn apply(mut s: StyleRefinement, t: &str, _cx: &mut Ctx) -> (StyleRefinement, bool) {
    if let Some(v) = t.strip_prefix("cursor-") {
        let cursor = match v {
            "default" => Some(CursorStyle::Arrow),
            "pointer" => Some(CursorStyle::PointingHand),
            "text" => Some(CursorStyle::IBeam),
            "crosshair" => Some(CursorStyle::Crosshair),
            "move" | "grabbing" => Some(CursorStyle::ClosedHand),
            "grab" => Some(CursorStyle::OpenHand),
            "not-allowed" | "no-drop" => Some(CursorStyle::OperationNotAllowed),
            "col-resize" => Some(CursorStyle::ResizeColumn),
            "row-resize" => Some(CursorStyle::ResizeRow),
            "ew-resize" => Some(CursorStyle::ResizeLeftRight),
            "ns-resize" => Some(CursorStyle::ResizeUpDown),
            "w-resize" => Some(CursorStyle::ResizeLeft),
            "e-resize" => Some(CursorStyle::ResizeRight),
            "context-menu" => Some(CursorStyle::ContextualMenu),
            "copy" => Some(CursorStyle::DragCopy),
            "alias" => Some(CursorStyle::DragLink),
            _ => None,
        };
        if let Some(cursor) = cursor {
            s.mouse_cursor = Some(cursor);
            return (s, true);
        }
    }
    (s, false)
}

#[cfg(test)]
mod tests {
    use super::super::parse;
    use crate::theme::Theme;
    use gpui::CursorStyle;

    #[test]
    fn common_cursors() {
        let theme = Theme::light();
        assert_eq!(
            parse(&theme, "cursor-pointer").base.mouse_cursor,
            Some(CursorStyle::PointingHand)
        );
        assert_eq!(
            parse(&theme, "cursor-not-allowed").base.mouse_cursor,
            Some(CursorStyle::OperationNotAllowed)
        );
    }

    #[test]
    fn unmapped_cursor_values_are_unknown() {
        let theme = Theme::light();
        assert_eq!(
            parse(&theme, "cursor-zoom-in").unknown,
            vec!["cursor-zoom-in"]
        );
    }
}
