use alacritty_terminal::term::cell::Flags;

use super::terminal_grid::CursorShapeKind;

type StyledCell = (char, u32, u32, u16);
const DEFAULT_FG: &str = super::INK;
const DEFAULT_BG: &str = "#16171a";

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
    let flags = Flags::from_bits_truncate(flags);
    let mut css = String::from("white-space:pre;line-height:1em;letter-spacing:0;");
    push_color_css(&mut css, fg, bg, flags);
    push_font_css(&mut css, flags);
    push_decoration_css(&mut css, flags);
    css
}

fn push_color_css(css: &mut String, fg: u32, bg: u32, flags: Flags) {
    let inverse = flags.contains(Flags::INVERSE);
    let (fg, bg) = if inverse { (bg, fg) } else { (fg, bg) };

    if let Some(color) = rgba_css(fg) {
        css.push_str(&format!("color:{color};"));
    } else if inverse {
        css.push_str(&format!("color:{DEFAULT_BG};"));
    }

    if let Some(color) = rgba_css(bg) {
        css.push_str(&format!("background:{color};"));
    } else if inverse {
        css.push_str(&format!("background:{DEFAULT_FG};"));
    }

    if flags.contains(Flags::HIDDEN) {
        css.push_str("color:transparent;");
    }
}

fn push_font_css(css: &mut String, flags: Flags) {
    if flags.contains(Flags::BOLD) {
        css.push_str("font-weight:bold;");
    }
    if flags.contains(Flags::DIM) {
        css.push_str("opacity:.75;");
    }
    if flags.contains(Flags::ITALIC) {
        css.push_str("font-style:italic;");
    }
}

fn push_decoration_css(css: &mut String, flags: Flags) {
    let underline = flags.intersects(Flags::ALL_UNDERLINES);
    let strikeout = flags.contains(Flags::STRIKEOUT);
    if !(underline || strikeout) {
        return;
    }

    css.push_str("text-decoration:");
    if underline {
        css.push_str("underline");
        if strikeout {
            css.push(' ');
        }
    }
    if strikeout {
        css.push_str("line-through");
    }
    css.push_str(";text-decoration-skip-ink:none;");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn keeps_trailing_spaces_for_terminal_rows() {
        let spans = row_spans_with_cursor(&[('x', 0, 0, 0), (' ', 0, 0, 0), (' ', 0, 0, 0)], None);
        assert_eq!(spans, vec![("x  ".to_string(), span_css(0, 0, 0))]);
    }

    #[test]
    fn inverse_swaps_default_terminal_colors() {
        let css = span_css(0, 0, Flags::INVERSE.bits());
        assert!(css.contains("color:#16171a;"));
        assert!(css.contains(&format!("background:{DEFAULT_FG};")));
    }
}
