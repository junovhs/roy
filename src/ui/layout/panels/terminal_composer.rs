use dioxus::prelude::*;

/// Map a keyboard event to the raw bytes that should be sent to the PTY.
/// Returns `None` for modifier-only keys that carry no PTY payload.
pub(super) fn key_to_pty_bytes(evt: &KeyboardData) -> Option<Vec<u8>> {
    let ctrl = evt.modifiers().ctrl();
    match &evt.key() {
        Key::Enter => Some(b"\r".to_vec()),
        Key::Backspace => Some(vec![0x7f]),
        Key::Tab => Some(b"\t".to_vec()),
        Key::Escape => Some(vec![0x1b]),
        Key::ArrowUp => Some(b"\x1b[A".to_vec()),
        Key::ArrowDown => Some(b"\x1b[B".to_vec()),
        Key::ArrowRight => Some(b"\x1b[C".to_vec()),
        Key::ArrowLeft => Some(b"\x1b[D".to_vec()),
        Key::Home => Some(b"\x1b[H".to_vec()),
        Key::End => Some(b"\x1b[F".to_vec()),
        Key::Delete => Some(b"\x1b[3~".to_vec()),
        Key::PageUp => Some(b"\x1b[5~".to_vec()),
        Key::PageDown => Some(b"\x1b[6~".to_vec()),
        Key::F1 => Some(b"\x1bOP".to_vec()),
        Key::F2 => Some(b"\x1bOQ".to_vec()),
        Key::F3 => Some(b"\x1bOR".to_vec()),
        Key::F4 => Some(b"\x1bOS".to_vec()),
        Key::Character(ch) => {
            if ctrl {
                if let Some(c) = ch.chars().next() {
                    let u = c.to_ascii_uppercase();
                    if u.is_ascii_uppercase() {
                        return Some(vec![(u as u8) - b'@']);
                    }
                    match c {
                        '[' => return Some(vec![0x1b]),
                        '\\' => return Some(vec![0x1c]),
                        ']' => return Some(vec![0x1d]),
                        _ => {}
                    }
                }
            }
            Some(ch.as_bytes().to_vec())
        }
        _ => None,
    }
}
