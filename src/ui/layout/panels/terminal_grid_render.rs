use dioxus::html::geometry::WheelDelta;
use dioxus::prelude::*;

use super::terminal_grid::TerminalCursor;
use super::terminal_spans::row_spans_with_cursor;

pub(super) fn wheel_lines(delta: WheelDelta) -> i32 {
    let y = match delta {
        WheelDelta::Pixels(vector) => vector.y / 40.0,
        WheelDelta::Lines(vector) => vector.y,
        WheelDelta::Pages(vector) => vector.y * 8.0,
    };

    if y == 0.0 {
        0
    } else {
        (-y.signum() * y.abs().ceil()) as i32
    }
}

pub(super) fn render_grid_row(
    row: Vec<(char, u32, u32, u16)>,
    row_index: usize,
    cursor: Option<TerminalCursor>,
) -> Element {
    let cursor = cursor
        .filter(|cursor| cursor.row == row_index)
        .map(|cursor| (cursor.column, cursor.shape));
    let spans = row_spans_with_cursor(&row, cursor);

    rsx! {
        div {
            key: "{row_index}",
            style: "display:block;white-space:pre;height:1em;line-height:1em;",
            if spans.is_empty() { "\u{00a0}" }
            for (span_index, (text, css)) in spans.into_iter().enumerate() {
                span { key: "{span_index}", style: "{css}", "{text}" }
            }
        }
    }
}
