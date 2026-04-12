use super::*;

impl ScreenBuffer {
    pub(super) fn erase_in_line(&mut self, mode: usize) {
        match mode {
            1 => self.clear_row_range(
                self.cursor_row,
                0,
                self.cursor_col.min(self.width.saturating_sub(1)) + 1,
            ),
            2 => {
                self.rows[self.cursor_row] = vec![TerminalCell::default(); self.width];
            }
            _ => self.clear_row_range(self.cursor_row, self.cursor_col, self.width),
        }
    }

    pub(super) fn erase_chars(&mut self, count: usize) {
        let end = (self.cursor_col + count).min(self.width);
        self.clear_row_range(self.cursor_row, self.cursor_col, end);
    }

    pub(super) fn clear_row_range(&mut self, row: usize, start: usize, end: usize) {
        for col in start..end {
            self.rows[row][col] = TerminalCell::default();
        }
    }
}
