use super::terminal_emulator::{TerminalCell, TerminalColor, TerminalSnapshot, TerminalStyle};
use dioxus::prelude::*;

/// Render a slice of terminal rows with their global row offset (for cursor matching).
pub(super) fn render_row_range(
    rows: &[Vec<TerminalCell>],
    row_offset: usize,
    cursor: Option<(usize, usize)>,
) -> Element {
    rsx! {
        div {
            style: "font-family:'JetBrains Mono',monospace; font-size:13px; line-height:1.0; min-width:max-content;",
            for (local_idx, row) in rows.iter().enumerate() {
                div {
                    style: "white-space:pre; height:1em;",
                    for segment in group_row(row, row_offset + local_idx, cursor) {
                        span { style: "{segment.style}", "{segment.text}" }
                    }
                }
            }
        }
    }
}

pub(super) fn visible_row_count(snapshot: &TerminalSnapshot) -> usize {
    if snapshot.using_alternate_screen {
        return snapshot.rows.len();
    }
    let cursor_row = snapshot.cursor.map(|(r, _)| r).unwrap_or(0);
    let last_text_row = snapshot
        .rows
        .iter()
        .rposition(|r| r.iter().any(|c| c.ch != ' '))
        .unwrap_or(0);
    last_text_row.max(cursor_row) + 1
}

struct RowSegment {
    text: String,
    style: String,
}

fn group_row(
    row: &[TerminalCell],
    row_idx: usize,
    cursor: Option<(usize, usize)>,
) -> Vec<RowSegment> {
    let mut segments = Vec::new();
    let mut current_text = String::new();
    let mut current_style = String::new();

    for (col_idx, cell) in row.iter().enumerate() {
        let style = style_css(
            cell.style,
            cursor == Some((row_idx, col_idx)),
            cell.ch == ' ',
        );
        if current_text.is_empty() || current_style == style {
            current_text.push(cell.ch);
            current_style = style;
            continue;
        }

        segments.push(RowSegment {
            text: std::mem::take(&mut current_text),
            style: std::mem::take(&mut current_style),
        });
        current_text.push(cell.ch);
        current_style = style;
    }

    if !current_text.is_empty() {
        segments.push(RowSegment {
            text: current_text,
            style: current_style,
        });
    }

    segments
}

fn style_css(style: TerminalStyle, is_cursor: bool, cursor_on_space: bool) -> String {
    let mut css = String::from("color:#e6e4df;");

    if let Some(color) = style.fg {
        css.push_str("color:");
        css.push_str(&color_css(color));
        css.push(';');
    }
    if let Some(color) = style.bg {
        css.push_str("background-color:");
        css.push_str(&color_css(color));
        css.push(';');
    }
    if style.bold {
        css.push_str("font-weight:700;");
    }
    if style.faint {
        css.push_str("opacity:.72;");
    }
    if style.italic {
        css.push_str("font-style:italic;");
    }
    if style.underline {
        css.push_str("text-decoration:underline;");
    }
    if is_cursor {
        css.push_str("background:rgba(232,120,88,.35);");
        if cursor_on_space {
            css.push_str("box-shadow:inset 0 0 0 1px rgba(232,120,88,.55);");
        }
    }

    css
}

fn color_css(color: TerminalColor) -> String {
    match color {
        TerminalColor::Rgb(r, g, b) => format!("#{r:02x}{g:02x}{b:02x}"),
        TerminalColor::Indexed(n) => indexed_color_hex(n),
    }
}

fn indexed_color_hex(n: u8) -> String {
    // Standard 16 + bright 16
    let base: Option<&'static str> = match n {
        0 => Some("#3f4248"),
        1 => Some("#f26d5b"),
        2 => Some("#b9d66d"),
        3 => Some("#e7c56a"),
        4 => Some("#78a8ff"),
        5 => Some("#d28cff"),
        6 => Some("#72d6c9"),
        7 => Some("#e6e4df"),
        8 => Some("#6d7078"),
        9 => Some("#ff9078"),
        10 => Some("#d4ec8a"),
        11 => Some("#f3d689"),
        12 => Some("#9cc2ff"),
        13 => Some("#e4b2ff"),
        14 => Some("#9ce9dd"),
        15 => Some("#ffffff"),
        _ => None,
    };
    if let Some(hex) = base {
        return hex.to_string();
    }
    // 16-231: 6×6×6 color cube
    if n <= 231 {
        let idx = n - 16;
        let b = cube_component(idx % 6);
        let g = cube_component((idx / 6) % 6);
        let r = cube_component(idx / 36);
        return format!("#{r:02x}{g:02x}{b:02x}");
    }
    // 232-255: grayscale ramp (8, 18, 28, … 238)
    let v = 8u8.saturating_add((n - 232).saturating_mul(10));
    format!("#{v:02x}{v:02x}{v:02x}")
}

fn cube_component(v: u8) -> u8 {
    if v == 0 { 0 } else { 55 + v * 40 }
}
