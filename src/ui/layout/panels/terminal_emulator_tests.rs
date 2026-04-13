use super::*;

fn line_text(snapshot: &TerminalSnapshot, row: usize) -> String {
    snapshot.rows[row]
        .iter()
        .map(|cell| cell.ch)
        .collect::<String>()
        .trim_end()
        .to_string()
}

#[test]
fn plain_text_and_newline_render_into_rows() {
    let mut term = AgentTerminalEmulator::new(12, 4);
    term.apply_bytes(b"hello\nworld");

    let snapshot = term.snapshot().expect("snapshot must exist");
    assert_eq!(line_text(&snapshot, 0), "hello");
    assert_eq!(line_text(&snapshot, 1), "world");
}

#[test]
fn partial_escape_sequence_is_buffered_across_chunks() {
    let mut term = AgentTerminalEmulator::new(12, 4);
    term.apply_bytes(b"\x1b[31");
    term.apply_bytes(b"mR");

    let snapshot = term.snapshot().expect("snapshot must exist");
    assert_eq!(snapshot.rows[0][0].ch, 'R');
    assert_eq!(
        snapshot.rows[0][0].style.fg,
        Some(TerminalColor::Indexed(1))
    );
}

#[test]
fn carriage_return_and_erase_line_replace_existing_text() {
    let mut term = AgentTerminalEmulator::new(12, 4);
    term.apply_bytes(b"hello\r\x1b[Khi");

    let snapshot = term.snapshot().expect("snapshot must exist");
    assert_eq!(line_text(&snapshot, 0), "hi");
}

#[test]
fn alternate_screen_switches_and_restores_main_buffer() {
    let mut term = AgentTerminalEmulator::new(12, 4);
    term.apply_bytes(b"main");
    term.apply_bytes(b"\x1b[?1049halt");

    let alt = term.snapshot().expect("alt snapshot must exist");
    assert!(alt.using_alternate_screen);
    assert_eq!(line_text(&alt, 0), "alt");

    term.apply_bytes(b"\x1b[?1049l");
    let main = term.snapshot().expect("main snapshot must exist");
    assert!(!main.using_alternate_screen);
    assert_eq!(line_text(&main, 0), "main");
}

#[test]
fn clear_screen_and_cursor_addressing_redraw_in_place() {
    let mut term = AgentTerminalEmulator::new(12, 4);
    term.apply_bytes(b"junk");
    term.apply_bytes(b"\x1b[2J\x1b[2;3HXY");

    let snapshot = term.snapshot().expect("snapshot must exist");
    assert_eq!(line_text(&snapshot, 0), "");
    assert_eq!(snapshot.rows[1][2].ch, 'X');
    assert_eq!(snapshot.rows[1][3].ch, 'Y');
}

#[test]
fn sgr_basic_background_colors() {
    let mut term = AgentTerminalEmulator::new(12, 4);
    // ESC[41m = red background, then 'X'
    term.apply_bytes(b"\x1b[41mX");
    let snapshot = term.snapshot().expect("snapshot");
    assert_eq!(snapshot.rows[0][0].style.bg, Some(TerminalColor::Indexed(1)));
    // ESC[49m resets background
    term.apply_bytes(b"\x1b[49mY");
    let snapshot2 = term.snapshot().expect("snapshot2");
    assert_eq!(snapshot2.rows[0][1].style.bg, None);
}

#[test]
fn sgr_256color_fg_and_bg() {
    let mut term = AgentTerminalEmulator::new(20, 4);
    // ESC[38;5;214m = 256-color fg index 214 (orange)
    term.apply_bytes(b"\x1b[38;5;214mA");
    // ESC[48;5;234m = 256-color bg index 234 (very dark gray)
    term.apply_bytes(b"\x1b[48;5;234mB");
    let snapshot = term.snapshot().expect("snapshot");
    assert_eq!(snapshot.rows[0][0].style.fg, Some(TerminalColor::Indexed(214)));
    assert_eq!(snapshot.rows[0][1].style.bg, Some(TerminalColor::Indexed(234)));
}

#[test]
fn sgr_truecolor_fg_and_bg() {
    let mut term = AgentTerminalEmulator::new(20, 4);
    // ESC[38;2;255;165;0m = truecolor fg orange
    term.apply_bytes(b"\x1b[38;2;255;165;0mA");
    // ESC[48;2;30;30;46m = truecolor bg dark navy
    term.apply_bytes(b"\x1b[48;2;30;30;46mB");
    let snapshot = term.snapshot().expect("snapshot");
    assert_eq!(snapshot.rows[0][0].style.fg, Some(TerminalColor::Rgb(255, 165, 0)));
    assert_eq!(snapshot.rows[0][1].style.bg, Some(TerminalColor::Rgb(30, 30, 46)));
}

#[test]
fn sgr_bright_background_colors() {
    let mut term = AgentTerminalEmulator::new(12, 4);
    // ESC[101m = bright red background (index 9)
    term.apply_bytes(b"\x1b[101mX");
    let snapshot = term.snapshot().expect("snapshot");
    assert_eq!(snapshot.rows[0][0].style.bg, Some(TerminalColor::Indexed(9)));
}

#[test]
fn sgr_underline_set_and_reset() {
    let mut term = AgentTerminalEmulator::new(12, 4);
    term.apply_bytes(b"\x1b[4mU\x1b[24mN");
    let snapshot = term.snapshot().expect("snapshot");
    assert!(snapshot.rows[0][0].style.underline, "underline should be set");
    assert!(!snapshot.rows[0][1].style.underline, "underline should be cleared");
}

#[test]
fn finish_for_transcript_drops_live_surface_and_preserves_main_lines() {
    let mut term = AgentTerminalEmulator::new(12, 4);
    term.apply_bytes(b"hello\nworld");

    let lines = term.finish_for_transcript();
    assert_eq!(lines.len(), 2);
    assert_eq!(lines[0].text, "hello");
    assert_eq!(lines[1].text, "world");
    assert!(
        term.snapshot().is_none(),
        "terminal must reset after finish"
    );
}
