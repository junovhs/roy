//! alacritty_terminal state helpers for the agent terminal view.

use std::sync::{Arc, Mutex};

use alacritty_terminal::event::{Event, EventListener, WindowSize};
use alacritty_terminal::grid::{Dimensions, Scroll};
use alacritty_terminal::term::{Config as TermConfig, RenderableCursor};
use alacritty_terminal::vte::ansi::{CursorShape, Processor};
use alacritty_terminal::Term;

use super::terminal_colors::color_rgba;

pub(super) const TERM_COLS: usize = 220;
const TERM_ROWS: usize = 50;

type StyledCell = (char, u32, u32, u16);

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

#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub(super) struct TerminalSnapshot {
    pub(super) rows: Vec<Vec<StyledCell>>,
    pub(super) cursor: Option<TerminalCursor>,
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
        let mut grid = vec![vec![(' ', 0, 0, 0); cols]; rows];

        for indexed in content.display_iter {
            let row = indexed.point.line.0 + content.display_offset as i32;
            let col = indexed.point.column.0;
            if row >= 0 && (row as usize) < rows && col < cols {
                let cell = indexed.cell;
                grid[row as usize][col] = (
                    cell.c,
                    color_rgba(cell.fg),
                    color_rgba(cell.bg),
                    cell.flags.bits(),
                );
            }
        }

        let cursor = renderable_cursor(content.cursor, content.display_offset, rows, cols);
        let last = grid
            .iter()
            .rposition(|row| {
                row.iter()
                    .any(|(ch, fg, bg, _)| *ch != ' ' || *fg != 0 || *bg != 0)
            })
            .unwrap_or(0);
        let keep = cursor.map(|cursor| cursor.row.max(last)).unwrap_or(last);
        grid.truncate(keep + 1);

        TerminalSnapshot { rows: grid, cursor }
    }

    pub(super) fn text_rows(&self) -> Vec<String> {
        self.snapshot()
            .rows
            .into_iter()
            .map(|row| row.into_iter().map(|(ch, _, _, _)| ch).collect::<String>())
            .map(|row| row.trim_end().to_string())
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
mod tests {
    use super::*;

    #[test]
    fn cursor_position_queries_emit_reply_bytes() {
        let mut term = TermState {
            term: Term::new(TermConfig::default(), &TermDims, TermListener::default()),
            parser: Processor::default(),
            listener: TermListener::default(),
        };
        // Rebuild with the same listener instance the term stores.
        term.listener = TermListener::default();
        term.term = Term::new(TermConfig::default(), &TermDims, term.listener.clone());

        let replies = term.feed(b"\x1b[6n");
        assert_eq!(replies, vec![b"\x1b[1;1R".to_vec()]);
    }
}
