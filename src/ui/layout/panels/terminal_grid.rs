//! alacritty_terminal state helpers for the agent terminal view.
//!
//! `TerminalSnapshot` preserves every `RenderableContent` field exhaustively:
//! Cell::{c,fg,bg,flags,zerowidth,underline_color,hyperlink} → `SnapCell`;
//! cursor/selection/colors/mode/display_offset → matching `TerminalSnapshot` fields.
//! Colors are stored as raw `Color` enums and resolved via the terminal palette at render time.

use std::sync::{Arc, Mutex};

use alacritty_terminal::event::{Event, EventListener, WindowSize};
use alacritty_terminal::grid::{Dimensions, Scroll};
use alacritty_terminal::selection::SelectionRange;
use alacritty_terminal::term::cell::Flags;
use alacritty_terminal::term::color::Colors;
use alacritty_terminal::term::{Config as TermConfig, RenderableCursor, TermMode};
use alacritty_terminal::vte::ansi::{Color, CursorShape, NamedColor, Processor};
use alacritty_terminal::Term;

/// Must stay aligned with the PTY width until live resize is plumbed through.
pub(super) const TERM_COLS: usize = 96;
const TERM_ROWS: usize = 50;

struct TermDims;

impl Dimensions for TermDims {
    fn total_lines(&self) -> usize {
        TERM_ROWS
    }
    fn screen_lines(&self) -> usize {
        TERM_ROWS
    }
    fn columns(&self) -> usize {
        TERM_COLS
    }
}

#[derive(Clone, Default)]
struct TermListener {
    replies: Arc<Mutex<Vec<Vec<u8>>>>,
}

impl TermListener {
    fn drain_replies(&self) -> Vec<Vec<u8>> {
        let mut replies = self.replies.lock().expect("term reply lock poisoned");
        replies.drain(..).collect()
    }
}

impl EventListener for TermListener {
    fn send_event(&self, event: Event) {
        let mut replies = self.replies.lock().expect("term reply lock poisoned");
        match event {
            Event::PtyWrite(text) => replies.push(text.into_bytes()),
            Event::TextAreaSizeRequest(formatter) => replies.push(
                formatter(WindowSize {
                    num_lines: TERM_ROWS as u16,
                    num_cols: TERM_COLS as u16,
                    cell_width: 0,
                    cell_height: 0,
                })
                .into_bytes(),
            ),
            _ => {}
        }
    }
}

pub(super) struct TermState {
    term: Term<TermListener>,
    parser: Processor,
    listener: TermListener,
}

pub(super) type TermHandle = Arc<Mutex<TermState>>;

/// A single terminal cell captured from alacritty_terminal for palette-aware rendering.
#[derive(Clone, Debug, PartialEq, Eq)]
pub(super) struct SnapCell {
    pub(super) c: char,
    pub(super) fg: Color,
    pub(super) bg: Color,
    pub(super) flags: Flags,
    /// Zero-width combining characters appended to this cell.
    pub(super) zerowidth: Vec<char>,
    /// Per-cell underline color override (from `CellExtra`).
    pub(super) underline_color: Option<Color>,
    /// Hyperlink URI, if set on this cell (from `CellExtra`).
    pub(super) hyperlink: Option<String>,
}

impl Default for SnapCell {
    fn default() -> Self {
        SnapCell {
            c: ' ',
            fg: Color::Named(NamedColor::Foreground),
            bg: Color::Named(NamedColor::Background),
            flags: Flags::empty(),
            zerowidth: Vec::new(),
            underline_color: None,
            hyperlink: None,
        }
    }
}

impl SnapCell {
    fn from_cell(cell: &alacritty_terminal::term::cell::Cell) -> Self {
        SnapCell {
            c: cell.c,
            fg: cell.fg,
            bg: cell.bg,
            flags: cell.flags,
            zerowidth: cell.zerowidth().unwrap_or_default().to_vec(),
            underline_color: cell.underline_color(),
            hyperlink: cell.hyperlink().map(|h| h.uri().to_owned()),
        }
    }

    fn is_blank(&self) -> bool {
        (self.c == ' ' || self.c == '\t')
            && self.fg == Color::Named(NamedColor::Foreground)
            && self.bg == Color::Named(NamedColor::Background)
            && self.flags.is_empty()
            && self.zerowidth.is_empty()
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(super) enum CursorShapeKind {
    Block,
    Underline,
    Beam,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(super) struct TerminalCursor {
    pub(super) row: usize,
    pub(super) column: usize,
    pub(super) shape: CursorShapeKind,
}

/// A lossless snapshot of all alacritty_terminal render state for one frame.
///
/// All fields from `RenderableContent` are preserved; see the module-level mapping inventory.
// Colors does not implement Debug/PartialEq, so those derives are omitted.
#[derive(Clone, Default)]
pub(super) struct TerminalSnapshot {
    pub(super) rows: Vec<Vec<SnapCell>>,
    pub(super) cursor: Option<TerminalCursor>,
    /// Active text selection range, if any. Consumed by TRM-08 cell renderer.
    #[allow(dead_code)]
    pub(super) selection: Option<SelectionRange>,
    /// Terminal palette (OSC 4/10/11 overrides); resolved at render time.
    pub(super) colors: Colors,
    /// Terminal mode flags (SHOW_CURSOR, keyboard protocols, etc.). Consumed by TRM-08.
    #[allow(dead_code)]
    pub(super) mode: TermMode,
    /// Scroll display offset at the time of snapshot. Used by TRM-08/TRM-09.
    #[allow(dead_code)]
    pub(super) display_offset: usize,
}

pub(super) fn new_term_handle() -> TermHandle {
    let listener = TermListener::default();
    Arc::new(Mutex::new(TermState {
        term: Term::new(TermConfig::default(), &TermDims, listener.clone()),
        parser: Processor::default(),
        listener,
    }))
}

impl TermState {
    pub(super) fn feed(&mut self, bytes: &[u8]) -> Vec<Vec<u8>> {
        self.parser.advance(&mut self.term, bytes);
        self.listener.drain_replies()
    }

    pub(super) fn scroll_lines(&mut self, delta: i32) {
        self.term.scroll_display(Scroll::Delta(delta));
    }

    pub(super) fn scroll_to_bottom(&mut self) {
        self.term.scroll_display(Scroll::Bottom);
    }

    pub(super) fn snapshot(&self) -> TerminalSnapshot {
        let content = self.term.renderable_content();
        let rows = self.term.screen_lines();
        let cols = self.term.columns();
        let mut grid: Vec<Vec<SnapCell>> = (0..rows)
            .map(|_| (0..cols).map(|_| SnapCell::default()).collect())
            .collect();

        for indexed in content.display_iter {
            let row = indexed.point.line.0 + content.display_offset as i32;
            let col = indexed.point.column.0;
            if row >= 0 && (row as usize) < rows && col < cols {
                grid[row as usize][col] = SnapCell::from_cell(indexed.cell);
            }
        }

        let cursor = renderable_cursor(content.cursor, content.display_offset, rows, cols);
        let last = grid
            .iter()
            .rposition(|row| row.iter().any(|cell| !cell.is_blank()))
            .unwrap_or(0);
        let keep = cursor.map(|cursor| cursor.row.max(last)).unwrap_or(last);
        grid.truncate(keep + 1);

        TerminalSnapshot {
            rows: grid,
            cursor,
            selection: content.selection,
            colors: *content.colors,
            mode: content.mode,
            display_offset: content.display_offset,
        }
    }

    pub(super) fn text_rows(&self) -> Vec<String> {
        self.snapshot()
            .rows
            .into_iter()
            .map(|row| row.into_iter().map(|cell| cell.c).collect::<String>())
            .map(|row: String| row.trim_end().to_string())
            .collect()
    }
}

fn renderable_cursor(
    cursor: RenderableCursor,
    display_offset: usize,
    rows: usize,
    cols: usize,
) -> Option<TerminalCursor> {
    let shape = match cursor.shape {
        CursorShape::Hidden => return None,
        CursorShape::Block | CursorShape::HollowBlock => CursorShapeKind::Block,
        CursorShape::Underline => CursorShapeKind::Underline,
        CursorShape::Beam => CursorShapeKind::Beam,
    };

    let row = cursor.point.line.0 + display_offset as i32;
    let col = cursor.point.column.0;
    if row < 0 || (row as usize) >= rows || col >= cols {
        return None;
    }

    Some(TerminalCursor {
        row: row as usize,
        column: col,
        shape,
    })
}

#[cfg(test)]
#[path = "terminal_grid_tests.rs"]
mod tests;
