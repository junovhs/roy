use dioxus::prelude::*;

use super::super::terminal_model::{LineKind, ShellLine, TEXT_ERROR};

pub(super) fn render_line(line: &ShellLine) -> Element {
    match line.kind {
        LineKind::DenialHeader => rsx! {
            div {
                style: "
                    display:flex;gap:8px;align-items:baseline;
                    margin-top:6px;
                    white-space:pre-wrap;word-break:break-all;
                ",
                span {
                    style: "color:{TEXT_ERROR};flex-shrink:0;font-weight:600;",
                    "{line.prefix}"
                }
                span { style: "color:{TEXT_ERROR};font-weight:500;", "{line.text}" }
            }
        },
        LineKind::DenialHint => rsx! {
            div {
                style: "
                    display:flex;gap:8px;align-items:baseline;
                    margin-bottom:6px;padding-left:4px;
                    white-space:pre-wrap;word-break:break-all;
                ",
                span {
                    style: "color:{super::CORAL_SOFT};flex-shrink:0;",
                    "{line.prefix}"
                }
                span { style: "color:{super::INK_DIM};", "{line.text}" }
            }
        },
        LineKind::NotFound => rsx! {
            div {
                style: "
                    display:flex;gap:8px;align-items:baseline;
                    white-space:pre-wrap;word-break:break-all;
                ",
                span { style: "color:{super::INK_FAINT};flex-shrink:0;", "{line.prefix}" }
                span { style: "color:{super::INK_FAINT};font-style:italic;", "{line.text}" }
            }
        },
        LineKind::Echo => rsx! {
            div {
                style: "display:flex;gap:8px;white-space:pre-wrap;word-break:break-all;color:{super::INK};",
                if !line.prefix.is_empty() {
                    span {
                        style: "color:{super::CORAL_SOFT};flex-shrink:0;font-weight:bold;",
                        "{line.prefix}"
                    }
                }
                span { "{line.text}" }
            }
        },
        LineKind::Error => rsx! {
            div {
                style: "display:flex;gap:8px;white-space:pre-wrap;word-break:break-all;color:{TEXT_ERROR};",
                span { "{line.text}" }
            }
        },
        LineKind::Output => rsx! {
            div {
                style: "display:flex;gap:8px;white-space:pre-wrap;word-break:break-all;color:{super::INK};",
                if !line.prefix.is_empty() {
                    span {
                        style: "color:{super::CORAL_SOFT};flex-shrink:0;font-weight:bold;",
                        "{line.prefix}"
                    }
                }
                span { "{line.text}" }
            }
        },
    }
}
