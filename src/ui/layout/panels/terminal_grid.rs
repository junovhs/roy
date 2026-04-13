//! alacritty_terminal state helpers for the agent terminal view.

use std::sync::{Arc, Mutex};

use alacritty_terminal::event::VoidListener;
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

pub(super) struct TermState {
    term: Term<VoidListener>,
    parser: Processor,
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
    Arc::new(Mutex::new(TermState {
        term: Term::new(TermConfig::default(), &TermDims, VoidListener),
        parser: Processor::default(),
    }))
}

impl TermState {
    pub(super) fn feed(&mut self, bytes: &[u8]) {
        self.parser.advance(&mut self.term, bytes);
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
