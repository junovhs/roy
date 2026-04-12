use super::*;

impl ScreenBuffer {
    pub(super) fn erase_in_display(&mut self, mode: usize) {
        match mode {
            1 => self.erase_display_before_cursor(),
            2 => self.reset(),
            _ => self.erase_display_after_cursor(),
        }
    }

    fn erase_display_before_cursor(&mut self) {
        let last_col = self.width.saturating_sub(1);
        for row in 0..=self.cursor_row {
            let end = if row == self.cursor_row {
                self.cursor_col.min(last_col)
            } else {
                last_col
            };
            self.clear_row_range(row, 0, end + 1);
        }
    }

    fn erase_display_after_cursor(&mut self) {
        for row in self.cursor_row..self.height {
            let start = if row == self.cursor_row {
                self.cursor_col
            } else {
                0
            };
            self.clear_row_range(row, start, self.width);
        }
    }
}
