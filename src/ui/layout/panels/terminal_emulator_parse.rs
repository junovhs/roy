use super::csi::{decode_char, parse_params, CsiSequence, DecodeResult};
use super::*;

impl AgentTerminalEmulator {
    pub(crate) fn apply_bytes(&mut self, bytes: &[u8]) {
        self.parser.pending.extend_from_slice(bytes);
        let mut idx = 0;

        while idx < self.parser.pending.len() {
            let byte = self.parser.pending[idx];
            match byte {
                b'\n' => {
                    self.active_buffer_mut().newline();
                    idx += 1;
                }
                b'\r' => {
                    self.active_buffer_mut().carriage_return();
                    idx += 1;
                }
                0x08 => {
                    self.active_buffer_mut().backspace();
                    idx += 1;
                }
                b'\t' => {
                    self.active_buffer_mut().tab();
                    idx += 1;
                }
                0x1b => match self.try_escape(idx) {
                    Some(consumed) => idx += consumed,
                    None => break,
                },
                0x00..=0x1f => idx += 1,
                _ => match decode_char(&self.parser.pending[idx..]) {
                    DecodeResult::Complete(ch, len) => {
                        self.saw_output = true;
                        let style = self.style;
                        self.active_buffer_mut().put_char(ch, style);
                        idx += len;
                    }
                    DecodeResult::NeedMore => break,
                    DecodeResult::Invalid => {
                        self.saw_output = true;
                        let style = self.style;
                        self.active_buffer_mut().put_char('�', style);
                        idx += 1;
                    }
                },
            }
        }

        if idx > 0 {
            self.parser.pending.drain(0..idx);
        }
    }

    pub(crate) fn snapshot(&self) -> Option<TerminalSnapshot> {
        if !self.saw_output {
            return None;
        }

        let buffer = self.active_buffer();
        Some(TerminalSnapshot {
            rows: buffer.rows.clone(),
            cursor: self
                .cursor_visible
                .then_some((buffer.cursor_row, buffer.cursor_col)),
            using_alternate_screen: self.using_alternate_screen,
        })
    }

    pub(crate) fn finish_for_transcript(&mut self) -> Vec<ShellLine> {
        let lines = if self.using_alternate_screen {
            Vec::new()
        } else {
            self.main
                .visible_text_lines()
                .into_iter()
                .map(ShellLine::output)
                .collect()
        };
        self.reset();
        lines
    }

    pub(crate) fn reset(&mut self) {
        self.parser.pending.clear();
        self.main.reset();
        self.alternate.reset();
        self.style = TerminalStyle::default();
        self.using_alternate_screen = false;
        self.cursor_visible = true;
        self.saw_output = false;
    }

    pub(super) fn active_buffer(&self) -> &ScreenBuffer {
        if self.using_alternate_screen {
            &self.alternate
        } else {
            &self.main
        }
    }

    pub(super) fn active_buffer_mut(&mut self) -> &mut ScreenBuffer {
        if self.using_alternate_screen {
            &mut self.alternate
        } else {
            &mut self.main
        }
    }

    fn try_escape(&mut self, start: usize) -> Option<usize> {
        let bytes = &self.parser.pending[start..];
        if bytes.len() < 2 {
            return None;
        }

        match bytes[1] {
            b'[' => {
                let csi = self.parse_csi(bytes)?;
                self.execute_csi(&csi);
                Some(csi.total_len)
            }
            b']' => self.parse_osc(bytes),
            b'7' => {
                self.active_buffer_mut().save_cursor();
                Some(2)
            }
            b'8' => {
                self.active_buffer_mut().restore_cursor();
                Some(2)
            }
            b'c' => {
                self.reset();
                Some(2)
            }
            b'(' | b')' => (bytes.len() >= 3).then_some(3),
            _ => Some(2),
        }
    }

    fn parse_csi(&self, bytes: &[u8]) -> Option<CsiSequence> {
        let mut end = 2;
        while end < bytes.len() {
            if (0x40..=0x7e).contains(&bytes[end]) {
                let body = &bytes[2..end];
                let private = body.first().copied().filter(|b| *b == b'?');
                let body = if private.is_some() { &body[1..] } else { body };
                return Some(CsiSequence {
                    private,
                    params: parse_params(body),
                    final_byte: bytes[end],
                    total_len: end + 1,
                });
            }
            end += 1;
        }
        None
    }

    fn parse_osc(&self, bytes: &[u8]) -> Option<usize> {
        let mut idx = 2;
        while idx < bytes.len() {
            match bytes[idx] {
                0x07 => return Some(idx + 1),
                0x1b if bytes.get(idx + 1) == Some(&b'\\') => return Some(idx + 2),
                _ => idx += 1,
            }
        }
        None
    }
}
