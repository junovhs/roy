use super::*;

impl ScreenBuffer {
    pub(super) fn move_cursor_rel(&mut self, row_delta: isize, col_delta: isize) {
        self.cursor_row = self
            .cursor_row
            .saturating_add_signed(row_delta)
            .min(self.height.saturating_sub(1));
        self.cursor_col = self
            .cursor_col
            .saturating_add_signed(col_delta)
            .min(self.width.saturating_sub(1));
    }

    pub(super) fn set_cursor(&mut self, row: usize, col: usize) {
        self.cursor_row = row.min(self.height.saturating_sub(1));
        self.cursor_col = col.min(self.width.saturating_sub(1));
    }
}
