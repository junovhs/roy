use dioxus::prelude::*;

use super::{BG_PANEL, BORDER, TEXT_DIM};
use super::atoms::DiagLine;

// ── artifacts row ─────────────────────────────────────────────────────────────

#[component]
pub(super) fn ArtifactsRow() -> Element {
    rsx! {
        div {
            style: "
                height: 80px;
                flex-shrink: 0;
                background: {BG_PANEL};
                border-top: 1px solid {BORDER};
                display: flex;
                flex-direction: column;
                overflow: hidden;
            ",

            div {
                style: "
                    display: flex;
                    align-items: center;
                    justify-content: space-between;
                    padding: 4px 12px;
                    border-bottom: 1px solid {BORDER};
                ",
                span {
                    style: "color: {TEXT_DIM}; font-size: 10px; letter-spacing: 0.1em;",
                    "ARTIFACTS"
                }
                span {
                    style: "color: {TEXT_DIM}; font-size: 10px;",
                    "diffs \u{00b7} validation runs \u{00b7} denied traces"
                }
            }

            div {
                style: "
                    flex: 1;
                    display: flex;
                    align-items: center;
                    padding: 0 16px;
                    color: {TEXT_DIM};
                    font-size: 11px;
                ",
                "no artifacts \u{2014} pending ART-01"
            }
        }
    }
}

// ── diagnostics pane (collapsible) ───────────────────────────────────────────

#[component]
pub(super) fn DiagnosticsPane(mut open: Signal<bool>) -> Element {
    let is_open = open.read();
    let height = if *is_open { "160px" } else { "28px" };

    rsx! {
        div {
            style: "
                height: {height};
                flex-shrink: 0;
                background: #080b0f;
                border-top: 1px solid {BORDER};
                display: flex;
                flex-direction: column;
                overflow: hidden;
                transition: height 0.15s ease;
            ",

            div {
                style: "
                    display: flex;
                    align-items: center;
                    justify-content: space-between;
                    padding: 4px 12px;
                    cursor: pointer;
                    user-select: none;
                    flex-shrink: 0;
                ",
                onclick: move |_| {
                    let cur = *open.read();
                    open.set(!cur);
                },

                span {
                    style: "color: {TEXT_DIM}; font-size: 10px; letter-spacing: 0.1em;",
                    "DIAGNOSTICS"
                }
                span {
                    style: "color: {TEXT_DIM}; font-size: 10px;",
                    if *is_open { "\u{25bc} collapse" } else { "\u{25b6} expand" }
                }
            }

            if *is_open {
                div {
                    style: "
                        flex: 1;
                        overflow-y: auto;
                        padding: 8px 16px;
                        display: flex;
                        flex-direction: column;
                        gap: 4px;
                    ",
                    DiagLine { tag: "shell",   text: "runtime offline \u{2014} SHEL-01"          }
                    DiagLine { tag: "resolve", text: "command registry offline \u{2014} TOOL-01"  }
                    DiagLine { tag: "policy",  text: "engine offline \u{2014} POL-01"             }
                    DiagLine { tag: "agents",  text: "adapter offline \u{2014} AGEN-01"           }
                    DiagLine { tag: "session", text: "ledger offline \u{2014} SES-01"             }
                    DiagLine { tag: "storage", text: "SQLite offline \u{2014} DB-01"              }
                }
            }
        }
    }
}
