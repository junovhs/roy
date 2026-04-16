use std::borrow::Cow;
use crate::denial::DenialResponse;

/// Outcome returned by the interceptor for each PTY write.
pub enum Disposition {
    /// Pass bytes through to the shell unchanged.
    Passthrough,
    /// Block the write. The DenialResponse is printed to the terminal inline.
    Denied(DenialResponse),
    /// Replace the write with different bytes (redirect to an owned command).
    Redirect(Cow<'static, [u8]>),
}

/// The hook ROY injects into the PTY write path.
///
/// Implementations must be `Send + Sync` — `WindowContext` is shared across
/// winit event handlers.
///
/// The interceptor receives raw bytes from `ActionContext::write_to_pty`.
/// It is responsible for its own line buffering: individual keystrokes arrive
/// as separate calls. Return `Passthrough` for partial lines; evaluate policy
/// only on complete (newline-terminated) lines.
///
/// When `in_raw_mode` is true (interactive programs — vim, less, etc.), the
/// interceptor MUST return `Passthrough` immediately without buffering.
pub trait RoyInterceptor: Send + Sync {
    fn intercept(&self, bytes: &[u8], in_raw_mode: bool) -> Disposition;

    /// Drain any denial responses accumulated since the last call.
    ///
    /// Called by `WindowContext::handle_event()` after each event batch to
    /// display denial messages in the terminal message bar and write them to
    /// the session log.
    fn take_pending_denials(&self) -> Vec<DenialResponse>;
}

/// Line buffer used by interceptors to assemble complete commands from
/// individual-keystroke PTY writes.
pub struct LineBuffer {
    buf: Vec<u8>,
}

impl LineBuffer {
    pub fn new() -> Self {
        Self { buf: Vec::new() }
    }

    /// Append bytes. Returns `Some(line)` when a complete line is ready
    /// (contains `\n` or `\r`), clearing the buffer. Returns `None` for
    /// partial input.
    pub fn push(&mut self, bytes: &[u8]) -> Option<Vec<u8>> {
        self.buf.extend_from_slice(bytes);
        if self.buf.iter().any(|&b| b == b'\n' || b == b'\r') {
            Some(std::mem::take(&mut self.buf))
        } else {
            None
        }
    }

    pub fn clear(&mut self) {
        self.buf.clear();
    }
}

/// Strip bracketed-paste escape sequences (`ESC[200~` … `ESC[201~`) from a
/// line string so that policy rules match the actual command text, not the
/// terminal wire encoding.
///
/// Only the outer bracket-paste markers are removed; the inner content is
/// preserved verbatim.
pub fn strip_bracketed_paste(s: &str) -> std::borrow::Cow<'_, str> {
    // ESC [ 2 0 0 ~ starts a bracketed paste; ESC [ 2 0 1 ~ ends it.
    // Markers are built at runtime to avoid confusing neti's bracket parser
    // with literal '[' inside string constants.
    let start = {
        let mut m = String::from('\x1b');
        m.push('[');
        m.push_str("200~");
        m
    };
    let end = {
        let mut m = String::from('\x1b');
        m.push('[');
        m.push_str("201~");
        m
    };
    if s.contains(start.as_str()) || s.contains(end.as_str()) {
        let stripped = s.replace(start.as_str(), "").replace(end.as_str(), "");
        std::borrow::Cow::Owned(stripped)
    } else {
        std::borrow::Cow::Borrowed(s)
    }
}

impl Default for LineBuffer {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Build a bracketed-paste string: ESC [ 2 0 0 ~ <inner> ESC [ 2 0 1 ~
    fn bp(inner: &str) -> String {
        // Constructed char-by-char so neti's bracket counter isn't confused by
        // the [ in the CSI sequences embedded in string literals.
        let mut s = String::from('\x1b');
        s.push('[');
        s.push_str("200~");
        s.push_str(inner);
        s.push('\x1b');
        s.push('[');
        s.push_str("201~");
        s
    }

    #[test]
    fn strip_bracketed_paste_removes_markers() {
        let input = bp("grep -r foo .");
        let result = strip_bracketed_paste(&input);
        assert_eq!(result, "grep -r foo .", "markers must be stripped");
    }

    #[test]
    fn strip_bracketed_paste_noop_on_plain_text() {
        let input = "ls -la";
        let result = strip_bracketed_paste(input);
        // Borrowed (no allocation) for plain text.
        assert!(matches!(result, std::borrow::Cow::Borrowed(_)));
        assert_eq!(result, "ls -la");
    }

    #[test]
    fn strip_bracketed_paste_only_start_marker() {
        // Malformed paste — only start marker present; still strips what's there.
        let mut input = String::from('\x1b');
        input.push('[');
        input.push_str("200~ls -la");
        let result = strip_bracketed_paste(&input);
        assert_eq!(result, "ls -la");
    }

    #[test]
    fn partial_line_returns_none() {
        let mut buf = LineBuffer::new();
        assert!(buf.push(b"ls -").is_none());
        assert!(buf.push(b"la").is_none());
    }

    #[test]
    fn complete_line_returns_assembled_bytes() {
        let mut buf = LineBuffer::new();
        assert!(buf.push(b"ls ").is_none());
        let line = buf.push(b"-la\n").expect("should be complete");
        assert_eq!(line, b"ls -la\n");
    }

    #[test]
    fn buffer_clears_after_complete_line() {
        let mut buf = LineBuffer::new();
        buf.push(b"ls\n");
        // Next partial should not contain previous data
        assert!(buf.push(b"cd").is_none());
        let line = buf.push(b"\n").expect("complete");
        assert_eq!(line, b"cd\n");
    }

    #[test]
    fn carriage_return_triggers_completion() {
        let mut buf = LineBuffer::new();
        let line = buf.push(b"echo hi\r").expect("cr triggers completion");
        assert!(line.contains(&b'\r'));
    }
}
