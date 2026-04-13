use alacritty_terminal::term::cell::Flags;

use super::terminal_grid::CursorShapeKind;

type StyledCell = (char, u32, u32, u16);

pub(super) fn row_spans_with_cursor(
    row: &[StyledCell],
    cursor: Option<(usize, CursorShapeKind)>,
) -> Vec<(String, String)> {
    let mut spans = Vec::new();
    let mut text = String::new();
    let mut style: Option<String> = None;

    for (idx, &(ch, fg, bg, flags)) in row.iter().enumerate() {
        let mut next_style = span_css(fg, bg, flags);
        if let Some((cursor_col, cursor_shape)) = cursor.filter(|(col, _)| *col == idx) {
            let _ = cursor_col;
            next_style.push_str(cursor_css(cursor_shape, ch));
        }

        if style.as_ref() == Some(&next_style) {
            text.push(ch);
            continue;
        }

        if let Some(active_style) = style.take() {
            spans.push((std::mem::take(&mut text), active_style));
        }
        text.push(ch);
        style = Some(next_style);
    }

    if let Some(active_style) = style {
        spans.push((text, active_style));
    }
    if cursor.is_none() {
        if let Some(last) = spans.last_mut() {
            last.0 = last.0.trim_end().to_string();
        }
    }
    spans.retain(|(text, _)| !text.is_empty());
    spans
}

fn cursor_css(shape: CursorShapeKind, ch: char) -> &'static str {
    match shape {
        CursorShapeKind::Block if ch == ' ' => {
            "background:rgba(232,120,88,.28);box-shadow:inset 0 0 0 1px rgba(232,120,88,.72);"
        }
        CursorShapeKind::Block => {
            "background:rgba(232,120,88,.28);box-shadow:inset 0 0 0 1px rgba(232,120,88,.72);"
        }
        CursorShapeKind::Underline => "box-shadow:inset 0 -2px 0 #e87858;",
        CursorShapeKind::Beam => "box-shadow:inset 2px 0 0 #e87858;",
    }
}

fn rgba_css(rgba: u32) -> Option<String> {
    if rgba == 0 {
        return None;
    }
    let red = (rgba >> 24) & 0xff;
    let green = (rgba >> 16) & 0xff;
    let blue = (rgba >> 8) & 0xff;
    Some(format!("#{red:02x}{green:02x}{blue:02x}"))
}

fn span_css(fg: u32, bg: u32, flags: u16) -> String {
    let mut css = String::new();
    if let Some(color) = rgba_css(fg) {
        css.push_str(&format!("color:{color};"));
    }
    if let Some(color) = rgba_css(bg) {
        css.push_str(&format!("background:{color};"));
    }
    let flags = Flags::from_bits_truncate(flags);
    if flags.contains(Flags::BOLD) {
        css.push_str("font-weight:bold;");
    }
    if flags.contains(Flags::ITALIC) {
        css.push_str("font-style:italic;");
    }
    if flags.intersects(Flags::ALL_UNDERLINES) {
        css.push_str("text-decoration:underline;");
    }
    css
}
