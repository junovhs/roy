use dioxus::prelude::*;

use crate::shell::ShellRuntime;

use super::{BG_PANEL, BG_SHELL, BORDER, TEXT_ACCENT, TEXT_DIM};
use super::atoms::{Field, PanelHeader, PlaceholderLine, SectionLabel};

// ── workspace panel (left column) ────────────────────────────────────────────

#[component]
pub(super) fn WorkspacePanel() -> Element {
    rsx! {
        div {
            style: "
                width: 220px;
                flex-shrink: 0;
                background: {BG_PANEL};
                border-right: 1px solid {BORDER};
                display: flex;
                flex-direction: column;
                overflow: hidden;
            ",

            PanelHeader { title: "WORKSPACE" }

            div {
                style: "padding: 12px; display: flex; flex-direction: column; gap: 10px; overflow-y: auto; flex: 1;",

                Field { label: "root",    value: "\u{2014}" }
                Field { label: "cwd",     value: "\u{2014}" }
                Field { label: "scope",   value: "\u{2014}" }
                Field { label: "session", value: "\u{2014}" }

                div {
                    style: "margin-top: 8px; padding-top: 8px; border-top: 1px solid {BORDER};",
                    SectionLabel { text: "POLICY PROFILE" }
                    div {
                        style: "color: {TEXT_DIM}; font-size: 11px; margin-top: 4px;",
                        "none loaded"
                    }
                }

                div {
                    style: "margin-top: 8px; padding-top: 8px; border-top: 1px solid {BORDER};",
                    SectionLabel { text: "INSTALLED AGENTS" }
                    div {
                        style: "color: {TEXT_DIM}; font-size: 11px; margin-top: 4px;",
                        "none configured"
                    }
                }
            }
        }
    }
}

// ── shell pane (center) ───────────────────────────────────────────────────────

/// Central shell interaction pane.
///
/// Owns the visual surface for the shell session hosted by [`ShellRuntime`].
/// `runtime` is the live session signal; the pane reads prompt state from it
/// and will dispatch commands into it once the input field is interactive
/// (pending full PTY wiring in SHEL-01 successors).
#[component]
pub(super) fn ShellPane(runtime: Signal<ShellRuntime>) -> Element {
    let prompt = runtime.read().prompt();

    rsx! {
        div {
            style: "
                flex: 1;
                display: flex;
                flex-direction: column;
                overflow: hidden;
                background: {BG_SHELL};
                border-right: 1px solid {BORDER};
            ",

            PanelHeader { title: "SHELL" }

            // output area — populated by dispatch results once interactive
            div {
                style: "
                    flex: 1;
                    padding: 16px;
                    overflow-y: auto;
                    display: flex;
                    flex-direction: column;
                    gap: 4px;
                ",

                PlaceholderLine {
                    prefix: "roy",
                    prefix_color: TEXT_ACCENT,
                    text: "shell runtime initialized",
                }
                PlaceholderLine {
                    prefix: "roy",
                    prefix_color: TEXT_ACCENT,
                    text: "command resolution: offline (TOOL-01)",
                }
                PlaceholderLine {
                    prefix: "roy",
                    prefix_color: TEXT_ACCENT,
                    text: "policy engine: offline (POL-01)",
                }
                PlaceholderLine {
                    prefix: "roy",
                    prefix_color: TEXT_ACCENT,
                    text: "embedded agents: offline (AGEN-01)",
                }
                div { style: "height: 12px;" }
                div {
                    style: "color: {TEXT_DIM}; font-size: 11px;",
                    "ShellRuntime v0.1 \u{2014} built-ins + compatibility traps active"
                }
            }

            // input bar — shows live prompt from runtime
            div {
                style: "
                    display: flex;
                    align-items: center;
                    gap: 8px;
                    padding: 8px 16px;
                    border-top: 1px solid {BORDER};
                    background: {BG_PANEL};
                    flex-shrink: 0;
                ",
                span {
                    style: "color: {TEXT_ACCENT}; font-weight: bold; user-select: none; font-size: 12px;",
                    "{prompt}"
                }
                div {
                    style: "flex: 1; color: {TEXT_DIM}; font-size: 12px; font-style: italic;",
                    "interactive input pending"
                }
            }
        }
    }
}

// ── activity + approvals panel (right column) ─────────────────────────────────

#[component]
pub(super) fn ActivityPanel() -> Element {
    rsx! {
        div {
            style: "
                width: 240px;
                flex-shrink: 0;
                background: {BG_PANEL};
                display: flex;
                flex-direction: column;
                overflow: hidden;
            ",

            PanelHeader { title: "ACTIVITY" }

            div {
                style: "
                    flex: 1;
                    overflow-y: auto;
                    padding: 8px 12px;
                    display: flex;
                    flex-direction: column;
                    gap: 4px;
                ",
                div {
                    style: "color: {TEXT_DIM}; font-size: 11px;",
                    "no events yet"
                }
            }

            div {
                style: "border-top: 1px solid {BORDER}; flex-shrink: 0;",
                PanelHeader { title: "APPROVALS" }
                div {
                    style: "padding: 8px 12px;",
                    div {
                        style: "color: {TEXT_DIM}; font-size: 11px;",
                        "no pending approvals"
                    }
                }
            }
        }
    }
}
