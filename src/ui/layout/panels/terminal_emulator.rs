use super::super::terminal_model::ShellLine;

const DEFAULT_COLS: usize = 80;
const DEFAULT_ROWS: usize = 24;

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub(super) struct TerminalStyle {
    pub(super) bold: bool,
    pub(super) faint: bool,
    pub(super) italic: bool,
    pub(super) fg: Option<TerminalColor>,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(super) enum TerminalColor {
    Indexed(u8),
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(super) struct TerminalCell {
    pub(super) ch: char,
    pub(super) style: TerminalStyle,
}

impl Default for TerminalCell {
    fn default() -> Self {
        Self {
            ch: ' ',
            style: TerminalStyle::default(),
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub(super) struct TerminalSnapshot {
    pub(super) rows: Vec<Vec<TerminalCell>>,
    pub(super) cursor: Option<(usize, usize)>,
    pub(super) using_alternate_screen: bool,
}

#[derive(Default)]
struct ParseState {
    pending: Vec<u8>,
}

#[derive(Clone)]
struct ScreenBuffer {
    width: usize,
    height: usize,
    rows: Vec<Vec<TerminalCell>>,
    cursor_row: usize,
    cursor_col: usize,
    saved_cursor: Option<(usize, usize)>,
}

pub(super) struct AgentTerminalEmulator {
    parser: ParseState,
    main: ScreenBuffer,
    alternate: ScreenBuffer,
    style: TerminalStyle,
    using_alternate_screen: bool,
    cursor_visible: bool,
    saw_output: bool,
}

impl Default for AgentTerminalEmulator {
    fn default() -> Self {
        Self::new(DEFAULT_COLS, DEFAULT_ROWS)
    }
}

impl AgentTerminalEmulator {
    pub(super) fn new(width: usize, height: usize) -> Self {
        Self {
            parser: ParseState::default(),
            main: ScreenBuffer::new(width, height),
            alternate: ScreenBuffer::new(width, height),
            style: TerminalStyle::default(),
            using_alternate_screen: false,
            cursor_visible: true,
            saw_output: false,
        }
    }
}

#[path = "terminal_emulator_buffer.rs"]
mod buffer;
#[path = "terminal_emulator_buffer_erase.rs"]
mod buffer_erase;
#[path = "terminal_emulator_buffer_erase_display.rs"]
mod buffer_erase_display;
#[path = "terminal_emulator_buffer_mutate.rs"]
mod buffer_mutate;
#[path = "terminal_emulator_buffer_ops.rs"]
mod buffer_ops;
#[path = "terminal_emulator_csi.rs"]
mod csi;
#[path = "terminal_emulator_parse.rs"]
mod parse;

#[cfg(test)]
#[path = "terminal_emulator_tests.rs"]
mod tests;
