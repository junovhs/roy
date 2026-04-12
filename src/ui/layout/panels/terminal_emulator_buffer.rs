use super::*;

impl ScreenBuffer {
    pub(super) fn new(width: usize, height: usize) -> Self {
        Self {
            width,
            height,
            rows: vec![vec![TerminalCell::default(); width]; height],
            cursor_row: 0,
            cursor_col: 0,
            saved_cursor: None,
        }
    }

    pub(super) fn reset(&mut self) {
        self.rows = vec![vec![TerminalCell::default(); self.width]; self.height];
        self.cursor_row = 0;
        self.cursor_col = 0;
        self.saved_cursor = None;
    }

    pub(super) fn newline(&mut self) {
        if self.cursor_row + 1 >= self.height {
            self.rows.remove(0);
            self.rows.push(vec![TerminalCell::default(); self.width]);
            self.cursor_row = self.height.saturating_sub(1);
        } else {
            self.cursor_row += 1;
        }
        self.cursor_col = 0;
    }

    pub(super) fn carriage_return(&mut self) {
        self.cursor_col = 0;
    }

    pub(super) fn backspace(&mut self) {
        self.cursor_col = self.cursor_col.saturating_sub(1);
    }

    pub(super) fn tab(&mut self) {
        let next = ((self.cursor_col / 8) + 1) * 8;
        self.cursor_col = next.min(self.width.saturating_sub(1));
    }

    pub(super) fn put_char(&mut self, ch: char, style: TerminalStyle) {
        if self.cursor_row >= self.height {
            self.cursor_row = self.height.saturating_sub(1);
        }
        if self.cursor_col >= self.width {
            self.cursor_col = 0;
            self.newline();
        }

        self.rows[self.cursor_row][self.cursor_col] = TerminalCell { ch, style };
        self.cursor_col += 1;
        if self.cursor_col >= self.width {
            self.cursor_col = 0;
            self.newline();
        }
    }

    pub(super) fn save_cursor(&mut self) {
        self.saved_cursor = Some((self.cursor_row, self.cursor_col));
    }

    pub(super) fn restore_cursor(&mut self) {
        if let Some((row, col)) = self.saved_cursor {
            self.set_cursor(row, col);
        }
    }

    pub(super) fn visible_text_lines(&self) -> Vec<String> {
        self.rows
            .iter()
            .map(|row| {
                let mut text = row.iter().map(|cell| cell.ch).collect::<String>();
                while text.ends_with(' ') {
                    text.pop();
                }
                text
            })
            .filter(|line| !line.is_empty())
            .collect()
    }
}
