use dioxus::prelude::*;

use crate::diagnostics::build_trace;
use crate::session::Session;
use crate::shell::ShellRuntime;

use super::atoms::DiagLine;
use super::{BORDER, TEXT_DIM};

#[component]
pub(super) fn DiagnosticsPane(
    mut open: Signal<bool>,
    runtime: Signal<ShellRuntime>,
    session: Signal<Session>,
) -> Element {
    let is_open = open.read();
    let height = if *is_open { "240px" } else { "28px" };
    let runtime = runtime.read();
    let session = session.read();
    let denied_count = session.events_of_kind("command_denied").len();
    let output_count = session.events_of_kind("command_output").len();
    let last_exit = runtime
        .last_exit_status()
        .map(|code| code.to_string())
        .unwrap_or_else(|| "none".to_string());
    let shell_line = format!("runtime online · cwd {}", runtime.env().cwd().display());
    let resolve_line = format!(
        "{} public / {} total commands",
        runtime.public_command_count(),
        runtime.command_count()
    );
    let policy_line = format!("profile {} · pending approvals 0", runtime.policy_name());
    let agents_line =
        "Claude Code adapter shipped · cockpit still defaults to local shell · Codex adapter pending"
            .to_string();
    let session_line = format!("session #{} · {} events", session.id, session.len());
    let artifact_count = session.artifacts().len();
    let storage_line = format!(
        "SQLite session store ready · {artifact_count} artifact refs · last exit {last_exit}"
    );
    let trace_line = format!("{denied_count} denied traces · {output_count} output lines");
    let event_trace = build_trace(session.events(), runtime.registry(), 8);

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
                    DiagLine { tag: "shell", text: shell_line }
                    DiagLine { tag: "resolve", text: resolve_line }
                    DiagLine { tag: "policy", text: policy_line }
                    DiagLine { tag: "agents", text: agents_line }
                    DiagLine { tag: "session", text: session_line }
                    DiagLine { tag: "storage", text: storage_line }
                    DiagLine { tag: "traces", text: trace_line }
                    if !event_trace.is_empty() {
                        div {
                            style: "margin-top:6px;border-top:1px solid #1a1f27;padding-top:6px;display:flex;flex-direction:column;gap:3px;",
                            for entry in &event_trace {
                                div {
                                    style: "display:flex;gap:8px;align-items:baseline;",
                                    span { style: "color:{entry.severity.color()};font-size:10px;min-width:60px;", "[{entry.tag}]" }
                                    span { style: "color:{entry.severity.color()};font-size:10px;", "{entry.text}" }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}
