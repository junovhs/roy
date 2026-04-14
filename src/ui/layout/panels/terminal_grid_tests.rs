use super::*;

fn fresh_term() -> TermState {
    let listener = TermListener::default();
    let term = Term::new(TermConfig::default(), &TermDims, listener.clone());
    TermState { term, parser: Processor::default(), listener }
}

#[test]
fn cursor_position_queries_emit_reply_bytes() {
    let mut term = fresh_term();
    let replies = term.feed(b"\x1b[6n");
    assert_eq!(replies, vec![b"\x1b[1;1R".to_vec()]);
}

#[test]
fn snapshot_captures_mode_and_no_selection_on_fresh_terminal() {
    let term = fresh_term();
    let snap = term.snapshot();
    assert!(snap.selection.is_none(), "expected no selection on fresh term");
    assert!(
        snap.mode.contains(TermMode::SHOW_CURSOR),
        "expected SHOW_CURSOR in default mode"
    );
    assert_eq!(snap.display_offset, 0);
}

#[test]
fn snapshot_palette_starts_empty() {
    let term = fresh_term();
    let snap = term.snapshot();
    for i in 0..alacritty_terminal::term::color::COUNT {
        assert!(
            snap.colors[i].is_none(),
            "palette slot {i} should be None on fresh terminal"
        );
    }
}

#[test]
fn snapshot_captures_cell_zerowidth_chars() {
    let mut term = fresh_term();
    // 'a' + U+0301 COMBINING ACUTE ACCENT; VTE stores the accent as a zerowidth char.
    term.feed("a\u{0301}".as_bytes());
    let snap = term.snapshot();
    let first_row = snap.rows.first().expect("snapshot must have at least one row");
    let cell = &first_row[0];
    assert_eq!(cell.c, 'a');
    assert!(!cell.zerowidth.is_empty(), "combining accent must be in zerowidth");
    assert_eq!(cell.zerowidth[0], '\u{0301}');
}
