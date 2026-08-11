//! Input — port of shadcn base-nova `ui/input.tsx`.
//!
//! A real single-line text field: selection, IME composition, clipboard,
//! and keyboard navigation, adapted from gpui's reference text-input
//! example and styled with the shadcn base-nova input tokens (h-8
//! rounded-lg border-input bg-transparent px-2.5 py-1 text-base/md:text-sm;
//! dark:bg-input/30; focus-visible border-ring + ring-3 ring-ring/50).
//!
//! TODO: border/bg color transition on focus change (base-nova
//! transition-colors 150ms cubic-bezier(0.4,0,0.2,1); gpui animates on
//! mount only).
//!
//! Unlike the RenderOnce components, `Input` is an entity — create it with
//! `cx.new(|cx| Input::new(cx))`, render the `Entity<Input>` directly, and
//! call [`Input::register_key_bindings`] once at app startup.

use std::ops::Range;
use std::rc::Rc;

use gpui::{
    App, Bounds, ClipboardItem, Context, CursorStyle, Element, ElementId, ElementInputHandler,
    Entity, EntityInputHandler, FocusHandle, Focusable, GlobalElementId, KeyBinding, LayoutId,
    MouseButton, MouseDownEvent, MouseMoveEvent, MouseUpEvent, PaintQuad, PathPromptOptions,
    Pixels, Point, ShapedLine, SharedString, Style, TextRun, UTF16Selection, UnderlineStyle,
    Window, actions, div, fill, point, prelude::*, px, relative, size,
};
use unicode_segmentation::UnicodeSegmentation as _;

use crate::theme::{Theme, alpha};

/// Invoked on user edits (typing, paste, cut, IME) — not on `set_text`.
type ChangeHandler = Rc<dyn Fn(&SharedString, &mut Window, &mut App) + 'static>;

actions!(
    rcn_input,
    [
        Backspace,
        Delete,
        Left,
        Right,
        SelectLeft,
        SelectRight,
        SelectAll,
        Home,
        End,
        MoveToPreviousWord,
        MoveToNextWord,
        SelectToPreviousWord,
        SelectToNextWord,
        SelectToBeginning,
        SelectToEnd,
        DeleteToPreviousWord,
        DeleteToNextWord,
        DeleteToBeginning,
        ShowCharacterPalette,
        Paste,
        Cut,
        Copy,
        OpenFile,
    ]
);

pub struct Input {
    focus_handle: FocusHandle,
    content: SharedString,
    placeholder: SharedString,
    disabled: bool,
    /// `aria-invalid` — destructive border + always-on destructive ring.
    invalid: bool,
    /// Native readOnly: focus/selection/copy work; mutations are no-ops.
    read_only: bool,
    /// `type="password"` — display bullets; copy/cut no-ops.
    masked: bool,
    /// `type="file"` — non-editable chrome with Choose File + filename.
    file: bool,
    /// Display name of the picked file (file mode only).
    file_name: Option<SharedString>,
    /// Render without the input chrome (border/height/padding) so wrappers
    /// like Textarea and InputGroup can supply their own shell.
    bare: bool,
    selected_range: Range<usize>,
    selection_reversed: bool,
    marked_range: Option<Range<usize>>,
    last_layout: Option<ShapedLine>,
    last_bounds: Option<Bounds<Pixels>>,
    is_selecting: bool,
    on_change: Option<ChangeHandler>,
}

impl Input {
    pub fn new(cx: &mut Context<Self>) -> Self {
        Self {
            focus_handle: cx.focus_handle(),
            content: "".into(),
            placeholder: "".into(),
            disabled: false,
            invalid: false,
            read_only: false,
            masked: false,
            file: false,
            file_name: None,
            bare: false,
            selected_range: 0..0,
            selection_reversed: false,
            marked_range: None,
            last_layout: None,
            last_bounds: None,
            is_selecting: false,
            on_change: None,
        }
    }

    /// Register the editing key bindings; call once from `main`.
    pub fn register_key_bindings(cx: &mut App) {
        cx.bind_keys([
            KeyBinding::new("backspace", Backspace, Some("RcnInput")),
            KeyBinding::new("delete", Delete, Some("RcnInput")),
            KeyBinding::new("left", Left, Some("RcnInput")),
            KeyBinding::new("right", Right, Some("RcnInput")),
            KeyBinding::new("shift-left", SelectLeft, Some("RcnInput")),
            KeyBinding::new("shift-right", SelectRight, Some("RcnInput")),
            KeyBinding::new("alt-left", MoveToPreviousWord, Some("RcnInput")),
            KeyBinding::new("alt-right", MoveToNextWord, Some("RcnInput")),
            KeyBinding::new("alt-shift-left", SelectToPreviousWord, Some("RcnInput")),
            KeyBinding::new("alt-shift-right", SelectToNextWord, Some("RcnInput")),
            KeyBinding::new("cmd-left", Home, Some("RcnInput")),
            KeyBinding::new("cmd-right", End, Some("RcnInput")),
            KeyBinding::new("cmd-shift-left", SelectToBeginning, Some("RcnInput")),
            KeyBinding::new("cmd-shift-right", SelectToEnd, Some("RcnInput")),
            KeyBinding::new("alt-backspace", DeleteToPreviousWord, Some("RcnInput")),
            KeyBinding::new("alt-delete", DeleteToNextWord, Some("RcnInput")),
            KeyBinding::new("cmd-backspace", DeleteToBeginning, Some("RcnInput")),
            KeyBinding::new("cmd-a", SelectAll, Some("RcnInput")),
            KeyBinding::new("cmd-v", Paste, Some("RcnInput")),
            KeyBinding::new("cmd-c", Copy, Some("RcnInput")),
            KeyBinding::new("cmd-x", Cut, Some("RcnInput")),
            KeyBinding::new("home", Home, Some("RcnInput")),
            KeyBinding::new("end", End, Some("RcnInput")),
            KeyBinding::new("ctrl-cmd-space", ShowCharacterPalette, Some("RcnInput")),
            // File mode uses its own key context so enter/space activate the
            // picker without consuming those keys in editable inputs.
            KeyBinding::new("enter", OpenFile, Some("RcnFileInput")),
            KeyBinding::new("space", OpenFile, Some("RcnFileInput")),
        ]);
    }

    pub fn placeholder(&mut self, placeholder: impl Into<SharedString>) -> &mut Self {
        self.placeholder = placeholder.into();
        self
    }

    pub fn set_disabled(&mut self, disabled: bool) {
        self.disabled = disabled;
    }

    /// Mirrors `aria-invalid`: destructive border + always-on destructive ring.
    pub fn set_invalid(&mut self, invalid: bool) {
        self.invalid = invalid;
    }

    /// Native readOnly: focus, selection, and copy work; mutations are no-ops.
    #[allow(dead_code)] // part of the shadcn contract; no storybook example uses it
    pub fn set_read_only(&mut self, read_only: bool) {
        self.read_only = read_only;
    }

    /// `type="password"`: one bullet per grapheme; copy/cut are no-ops.
    pub fn set_masked(&mut self, masked: bool) {
        self.masked = masked;
    }

    /// `type="file"`: non-editable chrome with Choose File + selected name.
    pub fn set_file(&mut self, file: bool) {
        self.file = file;
    }

    pub fn set_bare(&mut self, bare: bool) {
        self.bare = bare;
    }

    /// Callback invoked on user edits (typing, paste, cut, IME). Programmatic
    /// [`Input::set_text`] does not fire it, matching React `onChange`.
    #[allow(dead_code)] // part of the shadcn contract; no storybook example uses it
    pub fn on_change(
        &mut self,
        handler: impl Fn(&SharedString, &mut Window, &mut App) + 'static,
    ) -> &mut Self {
        self.on_change = Some(Rc::new(handler));
        self
    }

    pub fn text(&self) -> &str {
        &self.content
    }

    #[allow(dead_code)] // part of the file-mode contract; no storybook example reads it
    pub fn file_name(&self) -> Option<&str> {
        self.file_name.as_ref().map(|s| s.as_ref())
    }

    pub fn set_text(&mut self, text: impl Into<SharedString>, cx: &mut Context<Self>) {
        self.content = text.into();
        let end = self.content.len();
        self.selected_range = end..end;
        self.marked_range = None;
        cx.notify();
    }

    fn emit_change(&self, window: &mut Window, cx: &mut Context<Self>) {
        if let Some(handler) = self.on_change.clone() {
            let value = self.content.clone();
            handler(&value, window, cx);
        }
    }

    fn can_mutate(&self) -> bool {
        !self.disabled && !self.read_only && !self.file
    }

    /// Display text: masked content is one U+2022 bullet per grapheme.
    fn display_text(&self) -> SharedString {
        if self.masked && !self.content.is_empty() {
            let bullets: String = self.content.graphemes(true).map(|_| '•').collect();
            SharedString::from(bullets)
        } else {
            self.content.clone()
        }
    }

    /// Map a content (UTF-8) byte offset to the corresponding display offset.
    /// When masked, each grapheme becomes one 3-byte U+2022 bullet.
    fn content_to_display_offset(&self, content_offset: usize) -> usize {
        if !self.masked {
            return content_offset.min(self.content.len());
        }
        masked_display_offset(&self.content, content_offset)
    }

    /// Map a display (shaped-line) byte offset back to a content offset.
    fn display_to_content_offset(&self, display_offset: usize) -> usize {
        if !self.masked {
            return display_offset.min(self.content.len());
        }
        masked_content_offset(&self.content, display_offset)
    }

    fn left(&mut self, _: &Left, _: &mut Window, cx: &mut Context<Self>) {
        if self.selected_range.is_empty() {
            self.move_to(self.previous_boundary(self.cursor_offset()), cx);
        } else {
            self.move_to(self.selected_range.start, cx)
        }
    }

    fn right(&mut self, _: &Right, _: &mut Window, cx: &mut Context<Self>) {
        if self.selected_range.is_empty() {
            self.move_to(self.next_boundary(self.selected_range.end), cx);
        } else {
            self.move_to(self.selected_range.end, cx)
        }
    }

    fn select_left(&mut self, _: &SelectLeft, _: &mut Window, cx: &mut Context<Self>) {
        self.select_to(self.previous_boundary(self.cursor_offset()), cx);
    }

    fn select_right(&mut self, _: &SelectRight, _: &mut Window, cx: &mut Context<Self>) {
        self.select_to(self.next_boundary(self.cursor_offset()), cx);
    }

    fn select_all(&mut self, _: &SelectAll, _: &mut Window, cx: &mut Context<Self>) {
        self.move_to(0, cx);
        self.select_to(self.content.len(), cx)
    }

    fn home(&mut self, _: &Home, _: &mut Window, cx: &mut Context<Self>) {
        self.move_to(0, cx);
    }

    fn end(&mut self, _: &End, _: &mut Window, cx: &mut Context<Self>) {
        self.move_to(self.content.len(), cx);
    }

    fn move_to_previous_word(
        &mut self,
        _: &MoveToPreviousWord,
        _: &mut Window,
        cx: &mut Context<Self>,
    ) {
        self.move_to(self.previous_word_boundary(self.cursor_offset()), cx);
    }

    fn move_to_next_word(&mut self, _: &MoveToNextWord, _: &mut Window, cx: &mut Context<Self>) {
        self.move_to(self.next_word_boundary(self.cursor_offset()), cx);
    }

    fn select_to_previous_word(
        &mut self,
        _: &SelectToPreviousWord,
        _: &mut Window,
        cx: &mut Context<Self>,
    ) {
        self.select_to(self.previous_word_boundary(self.cursor_offset()), cx);
    }

    fn select_to_next_word(
        &mut self,
        _: &SelectToNextWord,
        _: &mut Window,
        cx: &mut Context<Self>,
    ) {
        self.select_to(self.next_word_boundary(self.cursor_offset()), cx);
    }

    fn select_to_beginning(
        &mut self,
        _: &SelectToBeginning,
        _: &mut Window,
        cx: &mut Context<Self>,
    ) {
        self.select_to(0, cx);
    }

    fn select_to_end(&mut self, _: &SelectToEnd, _: &mut Window, cx: &mut Context<Self>) {
        self.select_to(self.content.len(), cx);
    }

    fn delete_to_previous_word(
        &mut self,
        _: &DeleteToPreviousWord,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        if !self.can_mutate() {
            return;
        }
        if self.selected_range.is_empty() {
            let prev = self.previous_word_boundary(self.cursor_offset());
            if self.cursor_offset() == prev {
                window.play_system_bell();
                return;
            }
            self.select_to(prev, cx);
        }
        self.replace_text_in_range(None, "", window, cx);
    }

    fn delete_to_next_word(
        &mut self,
        _: &DeleteToNextWord,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        if !self.can_mutate() {
            return;
        }
        if self.selected_range.is_empty() {
            let next = self.next_word_boundary(self.cursor_offset());
            if self.cursor_offset() == next {
                window.play_system_bell();
                return;
            }
            self.select_to(next, cx);
        }
        self.replace_text_in_range(None, "", window, cx);
    }

    fn delete_to_beginning(
        &mut self,
        _: &DeleteToBeginning,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        if !self.can_mutate() {
            return;
        }
        if self.selected_range.is_empty() {
            if self.cursor_offset() == 0 {
                window.play_system_bell();
                return;
            }
            self.select_to(0, cx);
        }
        self.replace_text_in_range(None, "", window, cx);
    }

    fn backspace(&mut self, _: &Backspace, window: &mut Window, cx: &mut Context<Self>) {
        if !self.can_mutate() {
            return;
        }
        if self.selected_range.is_empty() {
            let prev = self.previous_boundary(self.cursor_offset());
            if self.cursor_offset() == prev {
                window.play_system_bell();
                return;
            }
            self.select_to(prev, cx)
        }
        self.replace_text_in_range(None, "", window, cx)
    }

    fn delete(&mut self, _: &Delete, window: &mut Window, cx: &mut Context<Self>) {
        if !self.can_mutate() {
            return;
        }
        if self.selected_range.is_empty() {
            let next = self.next_boundary(self.cursor_offset());
            if self.cursor_offset() == next {
                window.play_system_bell();
                return;
            }
            self.select_to(next, cx)
        }
        self.replace_text_in_range(None, "", window, cx)
    }

    fn on_mouse_down(
        &mut self,
        event: &MouseDownEvent,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        if self.disabled {
            return;
        }
        // File mode: focus + open native picker.
        if self.file {
            window.focus(&self.focus_handle, cx);
            self.open_file(&OpenFile, window, cx);
            return;
        }
        window.focus(&self.focus_handle, cx);
        // Double-click selects word; triple-click (or more) selects all.
        if event.click_count >= 3 {
            self.move_to(0, cx);
            self.select_to(self.content.len(), cx);
            self.is_selecting = false;
            return;
        }
        if event.click_count == 2 {
            let offset = self.index_for_mouse_position(event.position);
            let (start, end) = word_range_at(&self.content, offset);
            self.move_to(start, cx);
            self.select_to(end, cx);
            self.is_selecting = false;
            return;
        }
        self.is_selecting = true;
        if event.modifiers.shift {
            self.select_to(self.index_for_mouse_position(event.position), cx);
        } else {
            self.move_to(self.index_for_mouse_position(event.position), cx)
        }
    }

    fn on_mouse_up(&mut self, _: &MouseUpEvent, _window: &mut Window, _: &mut Context<Self>) {
        self.is_selecting = false;
    }

    fn on_mouse_move(&mut self, event: &MouseMoveEvent, _: &mut Window, cx: &mut Context<Self>) {
        if self.disabled || self.file {
            return;
        }
        if self.is_selecting {
            self.select_to(self.index_for_mouse_position(event.position), cx);
        }
    }

    fn open_file(&mut self, _: &OpenFile, _window: &mut Window, cx: &mut Context<Self>) {
        if !self.file || self.disabled {
            return;
        }
        let receiver = cx.prompt_for_paths(PathPromptOptions {
            files: true,
            directories: false,
            multiple: false,
            prompt: None,
        });
        cx.spawn(async move |this, cx| {
            let Ok(Ok(Some(paths))) = receiver.await else {
                return;
            };
            let Some(path) = paths.into_iter().next() else {
                return;
            };
            let name = path
                .file_name()
                .map(|n| n.to_string_lossy().to_string())
                .unwrap_or_else(|| path.to_string_lossy().to_string());
            this.update(cx, |this, cx| {
                this.file_name = Some(SharedString::from(name));
                cx.notify();
            })
            .ok();
        })
        .detach();
    }

    fn show_character_palette(
        &mut self,
        _: &ShowCharacterPalette,
        window: &mut Window,
        _: &mut Context<Self>,
    ) {
        if !self.can_mutate() {
            return;
        }
        window.show_character_palette();
    }

    fn paste(&mut self, _: &Paste, window: &mut Window, cx: &mut Context<Self>) {
        if !self.can_mutate() {
            return;
        }
        if let Some(text) = cx.read_from_clipboard().and_then(|item| item.text()) {
            self.replace_text_in_range(None, &text.replace("\n", " "), window, cx);
        }
    }

    fn copy(&mut self, _: &Copy, _: &mut Window, cx: &mut Context<Self>) {
        // Native password fields do not copy plaintext.
        if self.masked || self.file {
            return;
        }
        if !self.selected_range.is_empty() {
            cx.write_to_clipboard(ClipboardItem::new_string(
                self.content[self.selected_range.clone()].to_string(),
            ));
        }
    }

    fn cut(&mut self, _: &Cut, window: &mut Window, cx: &mut Context<Self>) {
        // Native password fields do not cut; read-only/file block mutations.
        if self.masked || !self.can_mutate() {
            return;
        }
        if !self.selected_range.is_empty() {
            cx.write_to_clipboard(ClipboardItem::new_string(
                self.content[self.selected_range.clone()].to_string(),
            ));
            self.replace_text_in_range(None, "", window, cx)
        }
    }

    fn move_to(&mut self, offset: usize, cx: &mut Context<Self>) {
        self.selected_range = offset..offset;
        cx.notify()
    }

    fn cursor_offset(&self) -> usize {
        if self.selection_reversed {
            self.selected_range.start
        } else {
            self.selected_range.end
        }
    }

    fn index_for_mouse_position(&self, position: Point<Pixels>) -> usize {
        if self.content.is_empty() {
            return 0;
        }
        let (Some(bounds), Some(line)) = (self.last_bounds.as_ref(), self.last_layout.as_ref())
        else {
            return 0;
        };
        if position.y < bounds.top() {
            return 0;
        }
        if position.y > bounds.bottom() {
            return self.content.len();
        }
        let display_idx = line.closest_index_for_x(position.x - bounds.left());
        self.display_to_content_offset(display_idx)
    }

    fn select_to(&mut self, offset: usize, cx: &mut Context<Self>) {
        if self.selection_reversed {
            self.selected_range.start = offset
        } else {
            self.selected_range.end = offset
        };
        if self.selected_range.end < self.selected_range.start {
            self.selection_reversed = !self.selection_reversed;
            self.selected_range = self.selected_range.end..self.selected_range.start;
        }
        cx.notify()
    }

    fn offset_from_utf16(&self, offset: usize) -> usize {
        let mut utf8_offset = 0;
        let mut utf16_count = 0;
        for ch in self.content.chars() {
            if utf16_count >= offset {
                break;
            }
            utf16_count += ch.len_utf16();
            utf8_offset += ch.len_utf8();
        }
        utf8_offset
    }

    fn offset_to_utf16(&self, offset: usize) -> usize {
        let mut utf16_offset = 0;
        let mut utf8_count = 0;
        for ch in self.content.chars() {
            if utf8_count >= offset {
                break;
            }
            utf8_count += ch.len_utf8();
            utf16_offset += ch.len_utf16();
        }
        utf16_offset
    }

    fn range_to_utf16(&self, range: &Range<usize>) -> Range<usize> {
        self.offset_to_utf16(range.start)..self.offset_to_utf16(range.end)
    }

    fn range_from_utf16(&self, range_utf16: &Range<usize>) -> Range<usize> {
        self.offset_from_utf16(range_utf16.start)..self.offset_from_utf16(range_utf16.end)
    }

    fn previous_boundary(&self, offset: usize) -> usize {
        self.content
            .grapheme_indices(true)
            .rev()
            .find_map(|(idx, _)| (idx < offset).then_some(idx))
            .unwrap_or(0)
    }

    fn next_boundary(&self, offset: usize) -> usize {
        self.content
            .grapheme_indices(true)
            .find_map(|(idx, _)| (idx > offset).then_some(idx))
            .unwrap_or(self.content.len())
    }

    fn previous_word_boundary(&self, offset: usize) -> usize {
        word_bound_offsets(&self.content)
            .into_iter()
            .rev()
            .find(|&idx| idx < offset)
            .unwrap_or(0)
    }

    fn next_word_boundary(&self, offset: usize) -> usize {
        word_bound_offsets(&self.content)
            .into_iter()
            .find(|&idx| idx > offset)
            .unwrap_or(self.content.len())
    }
}

/// Word-boundary offsets via unicode word breaks, skipping whitespace-only
/// segments (matches native NSTextField alt-arrow motion).
fn word_bound_offsets(text: &str) -> Vec<usize> {
    let mut offsets = vec![0usize];
    for (idx, word) in text.split_word_bound_indices() {
        if word.chars().all(|c| c.is_whitespace()) {
            continue;
        }
        if idx != 0 {
            offsets.push(idx);
        }
        let end = idx + word.len();
        if end != 0 {
            offsets.push(end);
        }
    }
    let len = text.len();
    if offsets.last().copied() != Some(len) {
        offsets.push(len);
    }
    offsets.sort_unstable();
    offsets.dedup();
    offsets
}

/// Inclusive word span under `offset` for double-click selection.
fn word_range_at(text: &str, offset: usize) -> (usize, usize) {
    if text.is_empty() {
        return (0, 0);
    }
    let offset = offset.min(text.len());
    for (idx, word) in text.split_word_bound_indices() {
        let end = idx + word.len();
        if offset < idx || offset > end {
            continue;
        }
        // Prefer the word that contains the caret; at a boundary prefer the
        // following non-whitespace word (macOS double-click behavior).
        if offset == end {
            continue;
        }
        if word.chars().all(|c| c.is_whitespace()) {
            continue;
        }
        return (idx, end);
    }
    // Caret at end of a word or on whitespace — fall back to nearest prior word.
    let mut last = None;
    for (idx, word) in text.split_word_bound_indices() {
        if word.chars().all(|c| c.is_whitespace()) {
            continue;
        }
        let end = idx + word.len();
        if end <= offset {
            last = Some((idx, end));
        } else if idx >= offset {
            return last.unwrap_or((idx, end));
        }
    }
    last.unwrap_or((offset, offset))
}

/// Masked display offset for a content offset: each grapheme renders as one
/// 3-byte U+2022 bullet.
fn masked_display_offset(text: &str, content_offset: usize) -> usize {
    let mut display = 0usize;
    for (idx, _g) in text.grapheme_indices(true) {
        if idx >= content_offset {
            return display;
        }
        display += '•'.len_utf8(); // 3
    }
    display
}

/// Inverse of [`masked_display_offset`]: display (shaped-line) offset back to
/// a content offset.
fn masked_content_offset(text: &str, display_offset: usize) -> usize {
    let bullet_len = '•'.len_utf8(); // 3
    let mut display = 0usize;
    for (idx, _g) in text.grapheme_indices(true) {
        if display + bullet_len > display_offset {
            return idx;
        }
        display += bullet_len;
    }
    text.len()
}

impl EntityInputHandler for Input {
    fn text_for_range(
        &mut self,
        range_utf16: Range<usize>,
        actual_range: &mut Option<Range<usize>>,
        _window: &mut Window,
        _cx: &mut Context<Self>,
    ) -> Option<String> {
        let range = self.range_from_utf16(&range_utf16);
        actual_range.replace(self.range_to_utf16(&range));
        Some(self.content[range].to_string())
    }

    fn selected_text_range(
        &mut self,
        _ignore_disabled_input: bool,
        _window: &mut Window,
        _cx: &mut Context<Self>,
    ) -> Option<UTF16Selection> {
        Some(UTF16Selection {
            range: self.range_to_utf16(&self.selected_range),
            reversed: self.selection_reversed,
        })
    }

    fn marked_text_range(
        &self,
        _window: &mut Window,
        _cx: &mut Context<Self>,
    ) -> Option<Range<usize>> {
        self.marked_range
            .as_ref()
            .map(|range| self.range_to_utf16(range))
    }

    fn unmark_text(&mut self, _window: &mut Window, _cx: &mut Context<Self>) {
        self.marked_range = None;
    }

    fn replace_text_in_range(
        &mut self,
        range_utf16: Option<Range<usize>>,
        new_text: &str,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        if !self.can_mutate() {
            return;
        }
        let range = range_utf16
            .as_ref()
            .map(|range_utf16| self.range_from_utf16(range_utf16))
            .or(self.marked_range.clone())
            .unwrap_or(self.selected_range.clone());

        self.content =
            (self.content[0..range.start].to_owned() + new_text + &self.content[range.end..])
                .into();
        self.selected_range = range.start + new_text.len()..range.start + new_text.len();
        self.marked_range.take();
        self.emit_change(window, cx);
        cx.notify();
    }

    fn replace_and_mark_text_in_range(
        &mut self,
        range_utf16: Option<Range<usize>>,
        new_text: &str,
        new_selected_range_utf16: Option<Range<usize>>,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        if !self.can_mutate() {
            return;
        }
        let range = range_utf16
            .as_ref()
            .map(|range_utf16| self.range_from_utf16(range_utf16))
            .or(self.marked_range.clone())
            .unwrap_or(self.selected_range.clone());

        self.content =
            (self.content[0..range.start].to_owned() + new_text + &self.content[range.end..])
                .into();
        if !new_text.is_empty() {
            self.marked_range = Some(range.start..range.start + new_text.len());
        } else {
            self.marked_range = None;
        }
        self.selected_range = new_selected_range_utf16
            .as_ref()
            .map(|range_utf16| self.range_from_utf16(range_utf16))
            .map(|new_range| new_range.start + range.start..new_range.end + range.end)
            .unwrap_or_else(|| range.start + new_text.len()..range.start + new_text.len());

        self.emit_change(window, cx);
        cx.notify();
    }

    fn bounds_for_range(
        &mut self,
        range_utf16: Range<usize>,
        bounds: Bounds<Pixels>,
        _window: &mut Window,
        _cx: &mut Context<Self>,
    ) -> Option<Bounds<Pixels>> {
        let last_layout = self.last_layout.as_ref()?;
        let range = self.range_from_utf16(&range_utf16);
        let start = self.content_to_display_offset(range.start);
        let end = self.content_to_display_offset(range.end);
        Some(Bounds::from_corners(
            point(bounds.left() + last_layout.x_for_index(start), bounds.top()),
            point(
                bounds.left() + last_layout.x_for_index(end),
                bounds.bottom(),
            ),
        ))
    }

    fn character_index_for_point(
        &mut self,
        point: gpui::Point<Pixels>,
        _window: &mut Window,
        _cx: &mut Context<Self>,
    ) -> Option<usize> {
        let line_point = self.last_bounds?.localize(&point)?;
        let last_layout = self.last_layout.as_ref()?;
        let display_index = last_layout.index_for_x(point.x - line_point.x)?;
        let content_index = self.display_to_content_offset(display_index);
        Some(self.offset_to_utf16(content_index))
    }
}

/// Custom element that shapes, paints, and hit-tests the text line.
struct TextElement {
    input: Entity<Input>,
}

struct PrepaintState {
    line: Option<ShapedLine>,
    cursor: Option<PaintQuad>,
    selection: Option<PaintQuad>,
}

impl IntoElement for TextElement {
    type Element = Self;

    fn into_element(self) -> Self::Element {
        self
    }
}

impl Element for TextElement {
    type RequestLayoutState = ();
    type PrepaintState = PrepaintState;

    fn id(&self) -> Option<ElementId> {
        None
    }

    fn source_location(&self) -> Option<&'static core::panic::Location<'static>> {
        None
    }

    fn request_layout(
        &mut self,
        _id: Option<&GlobalElementId>,
        _inspector_id: Option<&gpui::InspectorElementId>,
        window: &mut Window,
        cx: &mut App,
    ) -> (LayoutId, Self::RequestLayoutState) {
        let mut style = Style::default();
        style.size.width = relative(1.).into();
        style.size.height = window.line_height().into();
        (window.request_layout(style, [], cx), ())
    }

    fn prepaint(
        &mut self,
        _id: Option<&GlobalElementId>,
        _inspector_id: Option<&gpui::InspectorElementId>,
        bounds: Bounds<Pixels>,
        _request_layout: &mut Self::RequestLayoutState,
        window: &mut Window,
        cx: &mut App,
    ) -> Self::PrepaintState {
        let theme = Theme::of(cx).clone();
        let input = self.input.read(cx);
        let content = input.content.clone();
        let selected_range = input.selected_range.clone();
        let cursor = input.cursor_offset();
        let style = window.text_style();

        // Content offsets → display offsets (identity unless masked).
        let to_display = |off: usize| -> usize { input.content_to_display_offset(off) };

        let (display_text, text_color) = if content.is_empty() {
            (input.placeholder.clone(), theme.muted_foreground)
        } else {
            (input.display_text(), style.color)
        };

        let run = TextRun {
            len: display_text.len(),
            font: style.font(),
            color: text_color,
            background_color: None,
            underline: None,
            strikethrough: None,
        };
        let runs = if let Some(marked_range) = input.marked_range.as_ref() {
            let ms = to_display(marked_range.start);
            let me = to_display(marked_range.end);
            vec![
                TextRun {
                    len: ms,
                    ..run.clone()
                },
                TextRun {
                    len: me.saturating_sub(ms),
                    underline: Some(UnderlineStyle {
                        color: Some(run.color),
                        thickness: px(1.0),
                        wavy: false,
                    }),
                    ..run.clone()
                },
                TextRun {
                    len: display_text.len().saturating_sub(me),
                    ..run
                },
            ]
            .into_iter()
            .filter(|run| run.len > 0)
            .collect()
        } else {
            vec![run]
        };

        let font_size = style.font_size.to_pixels(window.rem_size());
        let line = window
            .text_system()
            .shape_line(display_text, font_size, &runs, None);

        let cursor_display = to_display(cursor);
        let sel_start = to_display(selected_range.start);
        let sel_end = to_display(selected_range.end);
        let cursor_pos = line.x_for_index(cursor_display);
        let (selection, cursor) = if selected_range.is_empty() {
            (
                None,
                Some(fill(
                    Bounds::new(
                        point(bounds.left() + cursor_pos, bounds.top()),
                        size(px(1.), bounds.bottom() - bounds.top()),
                    ),
                    theme.foreground,
                )),
            )
        } else {
            (
                Some(fill(
                    Bounds::from_corners(
                        point(bounds.left() + line.x_for_index(sel_start), bounds.top()),
                        point(bounds.left() + line.x_for_index(sel_end), bounds.bottom()),
                    ),
                    alpha(theme.primary, 0.2),
                )),
                None,
            )
        };
        PrepaintState {
            line: Some(line),
            cursor,
            selection,
        }
    }

    fn paint(
        &mut self,
        _id: Option<&GlobalElementId>,
        _inspector_id: Option<&gpui::InspectorElementId>,
        bounds: Bounds<Pixels>,
        _request_layout: &mut Self::RequestLayoutState,
        prepaint: &mut Self::PrepaintState,
        window: &mut Window,
        cx: &mut App,
    ) {
        let focus_handle = self.input.read(cx).focus_handle.clone();
        window.handle_input(
            &focus_handle,
            ElementInputHandler::new(bounds, self.input.clone()),
            cx,
        );
        if let Some(selection) = prepaint.selection.take() {
            window.paint_quad(selection)
        }
        let line = prepaint.line.take().unwrap();
        line.paint(
            bounds.origin,
            window.line_height(),
            gpui::TextAlign::Left,
            None,
            window,
            cx,
        )
        .unwrap();

        if focus_handle.is_focused(window)
            && let Some(cursor) = prepaint.cursor.take()
        {
            window.paint_quad(cursor);
        }

        self.input.update(cx, |input, _cx| {
            input.last_layout = Some(line);
            input.last_bounds = Some(bounds);
        });
    }
}

impl Render for Input {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let theme = Theme::of(cx).clone();
        let focused = !self.disabled && self.focus_handle.is_focused(window);
        let invalid = self.invalid;
        let disabled = self.disabled;
        let file_mode = self.file;
        let file_label = self
            .file_name
            .clone()
            .unwrap_or_else(|| SharedString::from("No file chosen"));

        // Disabled: not focusable (no track_focus / no mouse handlers / no IBeam).
        let mut el = div()
            .key_context(if file_mode {
                "RcnFileInput"
            } else {
                "RcnInput"
            })
            .flex()
            .flex_row()
            .items_center()
            .w_full()
            .min_w_0()
            .text_size(px(14.))
            .line_height(px(20.))
            .text_color(theme.foreground);

        if !disabled {
            el = el
                .track_focus(&self.focus_handle(cx))
                .cursor(if file_mode {
                    CursorStyle::PointingHand
                } else {
                    CursorStyle::IBeam
                })
                .on_action(cx.listener(Self::backspace))
                .on_action(cx.listener(Self::delete))
                .on_action(cx.listener(Self::left))
                .on_action(cx.listener(Self::right))
                .on_action(cx.listener(Self::select_left))
                .on_action(cx.listener(Self::select_right))
                .on_action(cx.listener(Self::select_all))
                .on_action(cx.listener(Self::home))
                .on_action(cx.listener(Self::end))
                .on_action(cx.listener(Self::move_to_previous_word))
                .on_action(cx.listener(Self::move_to_next_word))
                .on_action(cx.listener(Self::select_to_previous_word))
                .on_action(cx.listener(Self::select_to_next_word))
                .on_action(cx.listener(Self::select_to_beginning))
                .on_action(cx.listener(Self::select_to_end))
                .on_action(cx.listener(Self::delete_to_previous_word))
                .on_action(cx.listener(Self::delete_to_next_word))
                .on_action(cx.listener(Self::delete_to_beginning))
                .on_action(cx.listener(Self::show_character_palette))
                .on_action(cx.listener(Self::paste))
                .on_action(cx.listener(Self::cut))
                .on_action(cx.listener(Self::copy))
                .on_action(cx.listener(Self::open_file))
                .on_mouse_down(MouseButton::Left, cx.listener(Self::on_mouse_down))
                .on_mouse_up(MouseButton::Left, cx.listener(Self::on_mouse_up))
                .on_mouse_up_out(MouseButton::Left, cx.listener(Self::on_mouse_up))
                .on_mouse_move(cx.listener(Self::on_mouse_move));
        }

        el
            // h-8 w-full min-w-0 rounded-lg border border-input bg-transparent
            // px-2.5 py-1 text-base md:text-sm; dark:bg-input/30;
            // focus-visible:border-ring focus-visible:ring-3 focus-visible:ring-ring/50
            // aria-invalid: border-destructive + ring-destructive (always on)
            // disabled: bg-input/50 (dark: bg-input/80) opacity-50, no pointer
            // (no shadow-xs — base-nova drops it)
            .when(!self.bare, |el| {
                let border = if invalid {
                    if theme.dark {
                        alpha(theme.destructive, 0.5)
                    } else {
                        theme.destructive
                    }
                } else if focused {
                    theme.ring
                } else {
                    theme.input
                };
                el.h(px(32.))
                    .rounded(theme.radius_lg())
                    .border_1()
                    .border_color(border)
                    .when(invalid, |el| {
                        // Always-on destructive ring (rest + focused); overrides blue ring.
                        el.shadow(crate::motion::focus_ring_destructive(&theme))
                    })
                    .when(!invalid && focused, |el| {
                        el.shadow(crate::motion::focus_ring(&theme))
                    })
                    .when(disabled, |el| {
                        // disabled:bg-input/50 dark:disabled:bg-input/80 (replaces normal tint)
                        el.bg(alpha(theme.input, if theme.dark { 0.8 } else { 0.5 }))
                    })
                    .when(!disabled && theme.dark, |el| el.bg(alpha(theme.input, 0.3)))
                    .px(px(10.))
            })
            .when(disabled, |el| el.opacity(0.5))
            .when(file_mode, {
                let theme = theme.clone();
                move |el| {
                    el.gap(px(8.))
                        .child(
                            div()
                                .flex()
                                .items_center()
                                .h(px(24.))
                                .text_size(px(14.))
                                .font_weight(gpui::FontWeight::MEDIUM)
                                .text_color(theme.foreground)
                                .child("Choose File"),
                        )
                        .child(
                            div()
                                .text_size(px(14.))
                                .text_color(theme.muted_foreground)
                                .child(file_label),
                        )
                }
            })
            .when(!file_mode, |el| {
                el.child(TextElement { input: cx.entity() })
            })
    }
}

impl Focusable for Input {
    fn focus_handle(&self, _: &App) -> FocusHandle {
        self.focus_handle.clone()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn word_boundaries_skip_whitespace_runs() {
        let text = "hello  brave world";
        assert_eq!(word_bound_offsets(text), vec![0, 5, 7, 12, 13, 18]);
        // alt-right from inside "hello" lands at its end; alt-left from
        // inside "world" lands at its start.
        assert!(word_bound_offsets(text).contains(&5));
        assert!(word_bound_offsets(text).contains(&13));
    }

    #[test]
    fn word_boundaries_empty_and_unicode() {
        assert_eq!(word_bound_offsets(""), vec![0]);
        // "héllo wörld" — boundaries at byte offsets around multibyte chars.
        let text = "héllo wörld";
        let offsets = word_bound_offsets(text);
        assert_eq!(offsets.first(), Some(&0));
        assert_eq!(offsets.last(), Some(&text.len()));
        for &o in &offsets {
            assert!(text.is_char_boundary(o));
        }
    }

    #[test]
    fn double_click_word_span() {
        let text = "hello world";
        // Caret inside "hello".
        assert_eq!(word_range_at(text, 2), (0, 5));
        // Caret inside "world".
        assert_eq!(word_range_at(text, 8), (6, 11));
        // Caret at very end selects the trailing word.
        assert_eq!(word_range_at(text, 11), (6, 11));
        // Empty text.
        assert_eq!(word_range_at("", 0), (0, 0));
    }

    #[test]
    fn masked_offsets_round_trip() {
        // "aé👍" → graphemes of 1, 2, and 4 bytes; each displays as one
        // 3-byte bullet.
        let text = "aé👍";
        assert_eq!(masked_display_offset(text, 0), 0);
        assert_eq!(masked_display_offset(text, 1), 3);
        assert_eq!(masked_display_offset(text, 3), 6);
        assert_eq!(masked_display_offset(text, text.len()), 9);
        assert_eq!(masked_content_offset(text, 0), 0);
        assert_eq!(masked_content_offset(text, 3), 1);
        assert_eq!(masked_content_offset(text, 6), 3);
        assert_eq!(masked_content_offset(text, 9), text.len());
        // Mid-bullet display offsets snap to the grapheme start.
        assert_eq!(masked_content_offset(text, 1), 0);
        assert_eq!(masked_content_offset(text, 4), 1);
    }
}
