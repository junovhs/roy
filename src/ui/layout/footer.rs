use dioxus::prelude::*;

use crate::session::{Session, SessionEvent};
use crate::shell::ShellRuntime;

use super::{short_path_label, BG_PANEL, BORDER, TEXT_DIM};
use super::atoms::{DiagLine, StatCard};

// ── artifacts row ─────────────────────────────────────────────────────────────

#[component]
pub(super) fn ArtifactsRow(session: Signal<Session>) -> Element {
    let session = session.read();
    let denied_count = session.events_of_kind("command_denied").len();
    let output_count = session.events_of_kind("command_output").len();
    let cwd_changes = session.events_of_kind("cwd_changed").len();
    let last_denial = session
        .events()
        .iter()
        .rev()
        .find_map(|event| match event {
            SessionEvent::CommandDenied { command, .. } => Some(command.clone()),
            _ => None,
        })
        .unwrap_or_else(|| "none yet".to_string());
    let last_cwd = session
        .events()
        .iter()
        .rev()
        .find_map(|event| match event {
            SessionEvent::CwdChanged { to, .. } => Some(short_path_label(to)),
            _ => None,
        })
        .unwrap_or_else(|| "workspace root".to_string());

    rsx! {
        div {
            style: "
                min-height: 88px;
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
                    align-items: stretch;
                    gap: 10px;
                    padding: 10px 16px;
                    overflow-x: auto;
                ",
                StatCard {
                    label: "DENIED TRACES",
                    value: denied_count.to_string(),
                    detail: format!("latest: {last_denial}"),
                }
                StatCard {
                    label: "SHELL OUTPUT",
                    value: output_count.to_string(),
                    detail: "transcript is live in-session".to_string(),
                }
                StatCard {
                    label: "WORKSPACE MOVES",
                    value: cwd_changes.to_string(),
                    detail: format!("latest: {last_cwd}"),
                }
            }
        }
    }
}

// ── diagnostics pane (collapsible) ───────────────────────────────────────────

#[component]
pub(super) fn DiagnosticsPane(
    mut open: Signal<bool>,
    runtime: Signal<ShellRuntime>,
    session: Signal<Session>,
) -> Element {
    let is_open = open.read();
    let height = if *is_open { "160px" } else { "28px" };
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
    let session_line = format!("session #{} · {} events", session.id, session.len());
    let storage_line = format!("SQLite pending DB-01 · last exit {last_exit}");
    let trace_line = format!("{denied_count} denied traces · {output_count} output lines");

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
                    DiagLine { tag: "agents", text: "embedded adapters pending AGEN-01".to_string() }
                    DiagLine { tag: "session", text: session_line }
                    DiagLine { tag: "storage", text: storage_line }
                    DiagLine { tag: "traces", text: trace_line }
                }
            }
        }
    }
}
