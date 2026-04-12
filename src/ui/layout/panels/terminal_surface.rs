use dioxus::prelude::*;

use super::terminal_emulator::{TerminalColor, TerminalSnapshot, TerminalStyle};

pub(super) fn render_terminal_surface(snapshot: &TerminalSnapshot) -> Element {
    let visible_rows = visible_row_count(snapshot);
    rsx! {
        div {
            style: "
                margin-top: 14px;
                padding: 16px 18px;
                border: 1px solid {super::LINE};
                border-radius: 10px;
                background: #121316;
                overflow: auto;
            ",
            div {
                style: "
                    font-family: 'JetBrains Mono', monospace;
                    font-size: 13px;
                    line-height: 1.35;
                    min-width: max-content;
                ",
                for (row_idx, row) in snapshot.rows.iter().take(visible_rows).enumerate() {
                    div {
                        style: "white-space: pre; min-height: 1.35em;",
                        for segment in group_row(row, row_idx, snapshot.cursor) {
                            span {
                                style: "{segment.style}",
                                "{segment.text}"
                            }
                        }
                    }
                }
            }
        }
    }
}

fn visible_row_count(snapshot: &TerminalSnapshot) -> usize {
    if snapshot.using_alternate_screen {
        return snapshot.rows.len();
    }

    let cursor_row = snapshot.cursor.map(|(row, _)| row).unwrap_or(0);
    let last_text_row = snapshot
        .rows
        .iter()
        .rposition(|row| row.iter().any(|cell| cell.ch != ' '))
        .unwrap_or(0);

    last_text_row.max(cursor_row) + 1
}

struct RowSegment {
    text: String,
    style: String,
}

fn group_row(
    row: &[super::terminal_emulator::TerminalCell],
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
    let mut css = String::from("color: #e6e4df;");

    if let Some(color) = style.fg {
        css.push_str("color:");
        css.push_str(color_css(color));
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
    if is_cursor {
        css.push_str("background: rgba(232,120,88,.35);");
        if cursor_on_space {
            css.push_str("box-shadow: inset 0 0 0 1px rgba(232,120,88,.55);");
        }
    }

    css
}

fn color_css(color: TerminalColor) -> &'static str {
    match color {
        TerminalColor::Indexed(0) => "#3f4248",
        TerminalColor::Indexed(1) => "#f26d5b",
        TerminalColor::Indexed(2) => "#b9d66d",
        TerminalColor::Indexed(3) => "#e7c56a",
        TerminalColor::Indexed(4) => "#78a8ff",
        TerminalColor::Indexed(5) => "#d28cff",
        TerminalColor::Indexed(6) => "#72d6c9",
        TerminalColor::Indexed(7) => "#e6e4df",
        TerminalColor::Indexed(8) => "#6d7078",
        TerminalColor::Indexed(9) => "#ff9078",
        TerminalColor::Indexed(10) => "#d4ec8a",
        TerminalColor::Indexed(11) => "#f3d689",
        TerminalColor::Indexed(12) => "#9cc2ff",
        TerminalColor::Indexed(13) => "#e4b2ff",
        TerminalColor::Indexed(14) => "#9ce9dd",
        TerminalColor::Indexed(_) => "#ffffff",
    }
}
