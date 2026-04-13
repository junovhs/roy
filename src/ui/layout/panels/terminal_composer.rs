use dioxus::prelude::*;

/// Map a keyboard event to the raw bytes that should be sent to the PTY.
/// Returns `None` for modifier-only keys that carry no PTY payload.
pub(super) fn key_to_pty_bytes(evt: &KeyboardData) -> Option<Vec<u8>> {
    key_to_pty_bytes_parts(
        &evt.key(),
        evt.modifiers().ctrl(),
        evt.modifiers().alt(),
        evt.modifiers().shift(),
    )
}

pub(super) fn key_to_pty_bytes_parts(
    key: &Key,
    ctrl: bool,
    alt: bool,
    shift: bool,
) -> Option<Vec<u8>> {
    if alt {
        if let Key::Character(_) = key {
            let mut bytes = vec![0x1b];
            bytes.extend(key_to_pty_bytes_parts(key, ctrl, false, shift)?);
            return Some(bytes);
        }
    }

    if let Some(bytes) = navigation_key_bytes(key, shift) {
        return Some(bytes);
    }
    if let Some(bytes) = function_key_bytes(key) {
        return Some(bytes);
    }

    match key {
        Key::Enter => Some(b"\r".to_vec()),
        Key::Backspace => Some(vec![0x7f]),
        Key::Tab => Some(b"\t".to_vec()),
        Key::Escape => Some(vec![0x1b]),
        Key::Character(ch) => {
            if ctrl {
                if let Some(control) = control_character(ch) {
                    return Some(vec![control]);
                }
            }

            Some(ch.as_bytes().to_vec())
        }
        _ => None,
    }
}

fn navigation_key_bytes(key: &Key, shift: bool) -> Option<Vec<u8>> {
    match key {
        Key::Tab if shift => Some(b"\x1b[Z".to_vec()),
        Key::ArrowUp => Some(b"\x1b[A".to_vec()),
        Key::ArrowDown => Some(b"\x1b[B".to_vec()),
        Key::ArrowRight => Some(b"\x1b[C".to_vec()),
        Key::ArrowLeft => Some(b"\x1b[D".to_vec()),
        Key::Home => Some(b"\x1b[H".to_vec()),
        Key::End => Some(b"\x1b[F".to_vec()),
        Key::Delete => Some(b"\x1b[3~".to_vec()),
        Key::PageUp => Some(b"\x1b[5~".to_vec()),
        Key::PageDown => Some(b"\x1b[6~".to_vec()),
        _ => None,
    }
}

fn function_key_bytes(key: &Key) -> Option<Vec<u8>> {
    match key {
        Key::F1 => Some(b"\x1bOP".to_vec()),
        Key::F2 => Some(b"\x1bOQ".to_vec()),
        Key::F3 => Some(b"\x1bOR".to_vec()),
        Key::F4 => Some(b"\x1bOS".to_vec()),
        Key::F5 => Some(b"\x1b[15~".to_vec()),
        Key::F6 => Some(b"\x1b[17~".to_vec()),
        Key::F7 => Some(b"\x1b[18~".to_vec()),
        Key::F8 => Some(b"\x1b[19~".to_vec()),
        Key::F9 => Some(b"\x1b[20~".to_vec()),
        Key::F10 => Some(b"\x1b[21~".to_vec()),
        Key::F11 => Some(b"\x1b[23~".to_vec()),
        Key::F12 => Some(b"\x1b[24~".to_vec()),
        _ => None,
    }
}

fn control_character(ch: &str) -> Option<u8> {
    let c = ch.chars().next()?;
    let upper = c.to_ascii_uppercase();
    if upper.is_ascii_uppercase() {
        return Some((upper as u8) - b'@');
    }

    match c {
        '@' | ' ' => Some(0x00),
        '[' => Some(0x1b),
        '\\' => Some(0x1c),
        ']' => Some(0x1d),
        '^' => Some(0x1e),
        '_' => Some(0x1f),
        '?' => Some(0x7f),
        _ => None,
    }
}

#[cfg(test)]
mod tests {
    use super::key_to_pty_bytes_parts;
    use dioxus::prelude::Key;

    #[test]
    fn ctrl_letter_maps_to_control_byte() {
        assert_eq!(
            key_to_pty_bytes_parts(&Key::Character("c".into()), true, false, false),
            Some(vec![0x03]),
        );
    }

    #[test]
    fn alt_character_is_prefixed_with_escape() {
        assert_eq!(
            key_to_pty_bytes_parts(&Key::Character("b".into()), false, true, false),
            Some(vec![0x1b, b'b']),
        );
    }

    #[test]
    fn shift_tab_uses_backtab_sequence() {
        assert_eq!(
            key_to_pty_bytes_parts(&Key::Tab, false, false, true),
            Some(b"\x1b[Z".to_vec()),
        );
    }

    #[test]
    fn function_keys_extend_beyond_f4() {
        assert_eq!(
            key_to_pty_bytes_parts(&Key::F8, false, false, false),
            Some(b"\x1b[19~".to_vec()),
        );
    }
}
