use alacritty_terminal::term::cell::Flags;
use alacritty_terminal::term::color::Colors;

use super::terminal_colors::color_rgba_with_palette;
use super::terminal_grid::{CursorShapeKind, SnapCell};

const DEFAULT_FG: &str = super::INK;
const DEFAULT_BG: &str = "#16171a";

pub(super) fn row_spans_with_cursor(
    row: &[SnapCell],
    palette: &Colors,
    cursor: Option<(usize, CursorShapeKind)>,
) -> Vec<(String, String)> {
    let mut spans = Vec::new();

    for (idx, cell) in row.iter().enumerate() {
        // Wide-char spacers carry no printable content of their own; skip them.
        // The wide char itself (WIDE_CHAR flag) renders normally in the preceding column.
        if cell.flags.contains(Flags::WIDE_CHAR_SPACER) {
            continue;
        }

        let mut style = cell_css(cell, palette);
        if let Some((_, cursor_shape)) = cursor.filter(|(col, _)| *col == idx) {
            style.push_str(cursor_css(cursor_shape, cell.c));
        }
        spans.push((cell_text(cell), style));
    }

    spans.retain(|(text, _)| !text.is_empty());
    spans
}

fn cursor_css(shape: CursorShapeKind, _ch: char) -> &'static str {
    match shape {
        CursorShapeKind::Block => "background:#e87858;color:#16171a;",
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

fn cell_css(cell: &SnapCell, palette: &Colors) -> String {
    let fg = color_rgba_with_palette(cell.fg, palette);
    let bg = color_rgba_with_palette(cell.bg, palette);
    let mut css = String::from(
        "display:inline-block;box-sizing:border-box;vertical-align:top;overflow:hidden;white-space:pre;height:1em;line-height:1em;letter-spacing:0;",
    );
    if cell.flags.contains(Flags::WIDE_CHAR) {
        css.push_str("width:2ch;");
    } else {
        css.push_str("width:1ch;");
    }
    push_color_css(&mut css, fg, bg, cell.flags);
    push_font_css(&mut css, cell.flags);
    push_decoration_css(&mut css, cell.flags, cell.underline_color, palette);
    css
}

fn cell_text(cell: &SnapCell) -> String {
    let mut text = String::new();
    text.push(if cell.c == ' ' { '\u{00a0}' } else { cell.c });
    for &zw in &cell.zerowidth {
        text.push(zw);
    }
    text
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
    if flags.contains(Flags::ITALIC) {
        css.push_str("font-style:italic;");
    }
}

fn push_decoration_css(
    css: &mut String,
    flags: Flags,
    underline_color: Option<alacritty_terminal::vte::ansi::Color>,
    palette: &Colors,
) {
    let underline = flags.intersects(Flags::ALL_UNDERLINES);
    let strikeout = flags.contains(Flags::STRIKEOUT);
    if !(underline || strikeout) {
        return;
    }

    css.push_str("text-decoration:");
    if underline {
        if flags.contains(Flags::DOUBLE_UNDERLINE) {
            css.push_str("underline double");
        } else if flags.contains(Flags::UNDERCURL) {
            css.push_str("underline wavy");
        } else if flags.contains(Flags::DOTTED_UNDERLINE) {
            css.push_str("underline dotted");
        } else if flags.contains(Flags::DASHED_UNDERLINE) {
            css.push_str("underline dashed");
        } else {
            css.push_str("underline");
        }
        if strikeout {
            css.push(' ');
        }
    }
    if strikeout {
        css.push_str("line-through");
    }
    css.push_str(";text-decoration-skip-ink:none;");

    if let Some(uc) = underline_color {
        let rgba = color_rgba_with_palette(uc, palette);
        if let Some(color) = rgba_css(rgba) {
            css.push_str(&format!("text-decoration-color:{color};"));
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn empty_palette() -> Colors {
        Colors::default()
    }

    fn make_cell(c: char, fg: u32, bg: u32, flags: Flags) -> SnapCell {
        use alacritty_terminal::vte::ansi::{Color, NamedColor};
        SnapCell {
            c,
            // Encode as Spec so color_rgba_with_palette returns the exact RGBA.
            fg: if fg == 0 {
                Color::Named(NamedColor::Foreground)
            } else {
                Color::Spec(alacritty_terminal::vte::ansi::Rgb {
                    r: ((fg >> 24) & 0xff) as u8,
                    g: ((fg >> 16) & 0xff) as u8,
                    b: ((fg >> 8) & 0xff) as u8,
                })
            },
            bg: if bg == 0 {
                Color::Named(NamedColor::Background)
            } else {
                Color::Spec(alacritty_terminal::vte::ansi::Rgb {
                    r: ((bg >> 24) & 0xff) as u8,
                    g: ((bg >> 16) & 0xff) as u8,
                    b: ((bg >> 8) & 0xff) as u8,
                })
            },
            flags,
            zerowidth: Vec::new(),
            underline_color: None,
            hyperlink: None,
        }
    }

    #[test]
    fn keeps_trailing_spaces_for_terminal_rows() {
        let palette = empty_palette();
        let row = vec![
            make_cell('x', 0, 0, Flags::empty()),
            make_cell(' ', 0, 0, Flags::empty()),
            make_cell(' ', 0, 0, Flags::empty()),
        ];
        let spans = row_spans_with_cursor(&row, &palette, None);
        let combined: String = spans.iter().map(|(t, _)| t.as_str()).collect();
        assert_eq!(combined, "x\u{00a0}\u{00a0}");
    }

    #[test]
    fn inverse_swaps_default_terminal_colors() {
        let palette = empty_palette();
        let cell = make_cell(' ', 0, 0, Flags::INVERSE);
        let css = cell_css(&cell, &palette);
        assert!(css.contains("color:#16171a;"));
        assert!(css.contains(&format!("background:{DEFAULT_FG};")));
    }

    #[test]
    fn wide_char_spacer_cells_are_skipped() {
        let palette = empty_palette();
        let row = vec![
            make_cell('W', 0, 0, Flags::WIDE_CHAR),
            make_cell(' ', 0, 0, Flags::WIDE_CHAR_SPACER),
            make_cell('x', 0, 0, Flags::empty()),
        ];
        let spans = row_spans_with_cursor(&row, &palette, None);
        let combined: String = spans.iter().map(|(t, _)| t.as_str()).collect();
        assert!(
            !combined.contains('\u{0}'),
            "spacer produced unexpected output"
        );
        assert!(combined.contains('W'), "wide char missing");
        assert!(combined.contains('x'), "normal char missing");
        // Spacer column should not contribute a separate character.
        assert_eq!(combined.chars().count(), 2, "expected W and x only");
    }

    #[test]
    fn single_width_cells_render_as_fixed_boxes() {
        let palette = empty_palette();
        let row = vec![make_cell('x', 0, 0, Flags::empty())];
        let spans = row_spans_with_cursor(&row, &palette, None);
        assert_eq!(spans.len(), 1);
        assert_eq!(spans[0].0, "x");
        assert!(
            spans[0].1.contains("display:inline-block;"),
            "cells must render as standalone boxes"
        );
        assert!(
            spans[0].1.contains("width:1ch;"),
            "normal cells must reserve one column"
        );
    }

    #[test]
    fn wide_cells_reserve_two_columns() {
        let palette = empty_palette();
        let row = vec![make_cell('W', 0, 0, Flags::WIDE_CHAR)];
        let spans = row_spans_with_cursor(&row, &palette, None);
        assert_eq!(spans.len(), 1);
        assert_eq!(spans[0].0, "W");
        assert!(
            spans[0].1.contains("width:2ch;"),
            "wide glyphs must reserve both terminal columns"
        );
    }

    #[test]
    fn block_cursor_uses_solid_terminal_cell_style() {
        let palette = empty_palette();
        let row = vec![make_cell(' ', 0, 0, Flags::empty())];
        let spans = row_spans_with_cursor(&row, &palette, Some((0, CursorShapeKind::Block)));
        assert_eq!(spans.len(), 1);
        assert!(
            spans[0].1.contains("background:#e87858;"),
            "block cursor should render as a solid filled cell"
        );
        assert!(
            !spans[0].1.contains("box-shadow"),
            "block cursor should not use an outline box"
        );
    }
}
