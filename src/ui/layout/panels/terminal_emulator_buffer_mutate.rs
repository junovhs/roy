use super::*;

impl ScreenBuffer {
    pub(super) fn insert_blank_chars(&mut self, count: usize) {
        let count = count.min(self.width.saturating_sub(self.cursor_col));
        if count == 0 {
            return;
        }

        let row = &mut self.rows[self.cursor_row];
        for col in (self.cursor_col..self.width - count).rev() {
            row[col + count] = row[col];
        }
        for cell in row.iter_mut().skip(self.cursor_col).take(count) {
            *cell = TerminalCell::default();
        }
    }

    pub(super) fn delete_chars(&mut self, count: usize) {
        let count = count.min(self.width.saturating_sub(self.cursor_col));
        if count == 0 {
            return;
        }

        let row = &mut self.rows[self.cursor_row];
        for col in self.cursor_col..self.width - count {
            row[col] = row[col + count];
        }
        for cell in row.iter_mut().skip(self.width - count) {
            *cell = TerminalCell::default();
        }
    }

    pub(super) fn insert_lines(&mut self, count: usize) {
        let count = count.min(self.height.saturating_sub(self.cursor_row));
        if count == 0 {
            return;
        }
        for _ in 0..count {
            self.rows.remove(self.height - 1);
            self.rows
                .insert(self.cursor_row, vec![TerminalCell::default(); self.width]);
        }
    }

    pub(super) fn delete_lines(&mut self, count: usize) {
        let count = count.min(self.height.saturating_sub(self.cursor_row));
        if count == 0 {
            return;
        }
        for _ in 0..count {
            self.rows.remove(self.cursor_row);
            self.rows.push(vec![TerminalCell::default(); self.width]);
        }
    }

    pub(super) fn scroll_up(&mut self, count: usize) {
        for _ in 0..count.max(1) {
            self.rows.remove(0);
            self.rows.push(vec![TerminalCell::default(); self.width]);
        }
    }

    pub(super) fn scroll_down(&mut self, count: usize) {
        for _ in 0..count.max(1) {
            self.rows.pop();
            self.rows
                .insert(0, vec![TerminalCell::default(); self.width]);
        }
    }
}
