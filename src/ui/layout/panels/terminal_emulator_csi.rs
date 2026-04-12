use super::*;

impl AgentTerminalEmulator {
    pub(super) fn execute_csi(&mut self, csi: &CsiSequence) {
        let p = |idx: usize, default: usize| {
            csi.params
                .get(idx)
                .and_then(|value| *value)
                .filter(|value| *value > 0)
                .unwrap_or(default)
        };

        match csi.final_byte {
            b'A' => self
                .active_buffer_mut()
                .move_cursor_rel(-(p(0, 1) as isize), 0),
            b'B' => self
                .active_buffer_mut()
                .move_cursor_rel(p(0, 1) as isize, 0),
            b'C' => self
                .active_buffer_mut()
                .move_cursor_rel(0, p(0, 1) as isize),
            b'D' => self
                .active_buffer_mut()
                .move_cursor_rel(0, -(p(0, 1) as isize)),
            b'E' => {
                let buf = self.active_buffer_mut();
                buf.move_cursor_rel(p(0, 1) as isize, 0);
                buf.carriage_return();
            }
            b'F' => {
                let buf = self.active_buffer_mut();
                buf.move_cursor_rel(-(p(0, 1) as isize), 0);
                buf.carriage_return();
            }
            b'G' => {
                let row = self.active_buffer().cursor_row;
                self.active_buffer_mut()
                    .set_cursor(row, p(0, 1).saturating_sub(1));
            }
            b'H' | b'f' => self
                .active_buffer_mut()
                .set_cursor(p(0, 1).saturating_sub(1), p(1, 1).saturating_sub(1)),
            b'J' => self.active_buffer_mut().erase_in_display(p(0, 0)),
            b'K' => self.active_buffer_mut().erase_in_line(p(0, 0)),
            b'X' => self.active_buffer_mut().erase_chars(p(0, 1)),
            b'P' => self.active_buffer_mut().delete_chars(p(0, 1)),
            b'@' => self.active_buffer_mut().insert_blank_chars(p(0, 1)),
            b'L' => self.active_buffer_mut().insert_lines(p(0, 1)),
            b'M' => self.active_buffer_mut().delete_lines(p(0, 1)),
            b'S' => self.active_buffer_mut().scroll_up(p(0, 1)),
            b'T' => self.active_buffer_mut().scroll_down(p(0, 1)),
            b'd' => {
                let col = self.active_buffer().cursor_col;
                self.active_buffer_mut()
                    .set_cursor(p(0, 1).saturating_sub(1), col);
            }
            b'm' => self.apply_sgr(&csi.params),
            b's' => self.active_buffer_mut().save_cursor(),
            b'u' => self.active_buffer_mut().restore_cursor(),
            b'h' if csi.private == Some(b'?') => {
                for value in csi.params.iter().flatten() {
                    match *value {
                        25 => self.cursor_visible = true,
                        1049 => {
                            self.using_alternate_screen = true;
                            self.alternate.reset();
                            self.saw_output = true;
                        }
                        _ => {}
                    }
                }
            }
            b'l' if csi.private == Some(b'?') => {
                for value in csi.params.iter().flatten() {
                    match *value {
                        25 => self.cursor_visible = false,
                        1049 => self.using_alternate_screen = false,
                        _ => {}
                    }
                }
            }
            _ => {}
        }
    }

    pub(super) fn apply_sgr(&mut self, params: &[Option<usize>]) {
        if params.is_empty() {
            self.style = TerminalStyle::default();
            return;
        }

        for value in params.iter().copied().flatten() {
            match value {
                0 => self.style = TerminalStyle::default(),
                1 => self.style.bold = true,
                2 => self.style.faint = true,
                3 => self.style.italic = true,
                22 => {
                    self.style.bold = false;
                    self.style.faint = false;
                }
                23 => self.style.italic = false,
                30..=37 => self.style.fg = Some(TerminalColor::Indexed((value - 30) as u8)),
                90..=97 => self.style.fg = Some(TerminalColor::Indexed((value - 82) as u8)),
                39 => self.style.fg = None,
                _ => {}
            }
        }
    }
}

#[derive(Clone)]
pub(super) struct CsiSequence {
    pub(super) private: Option<u8>,
    pub(super) final_byte: u8,
    pub(super) total_len: usize,
    pub(super) params: Vec<Option<usize>>,
}

pub(super) fn parse_params(raw: &[u8]) -> Vec<Option<usize>> {
    if raw.is_empty() {
        return Vec::new();
    }

    raw.split(|b| *b == b';')
        .map(|part| {
            if part.is_empty() {
                None
            } else {
                std::str::from_utf8(part).ok()?.parse::<usize>().ok()
            }
        })
        .collect()
}

pub(super) enum DecodeResult {
    Complete(char, usize),
    NeedMore,
    Invalid,
}

pub(super) fn decode_char(bytes: &[u8]) -> DecodeResult {
    let first = bytes[0];
    if first.is_ascii() {
        return DecodeResult::Complete(first as char, 1);
    }

    let len = match first {
        0xC2..=0xDF => 2,
        0xE0..=0xEF => 3,
        0xF0..=0xF4 => 4,
        _ => return DecodeResult::Invalid,
    };

    if bytes.len() < len {
        return DecodeResult::NeedMore;
    }

    match std::str::from_utf8(&bytes[..len]) {
        Ok(text) => DecodeResult::Complete(text.chars().next().unwrap_or('�'), len),
        Err(_) => DecodeResult::Invalid,
    }
}
