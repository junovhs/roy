use dioxus::prelude::*;

use crate::session::{Session, SessionEvent};

use crate::ui::artifacts::{artifact_kind_color, artifact_kind_label};

use super::super::atoms::PanelHeader;
use super::super::{BG_PANEL, BORDER, TEXT_ACCENT, TEXT_DIM, TEXT_PRIMARY};

#[component]
pub(crate) fn ActivityPanel(session: Signal<Session>) -> Element {
    let session = session.read();
    let events: Vec<(String, String, &'static str)> = session
        .events()
        .iter()
        .rev()
        .filter_map(render_event)
        .take(8)
        .collect();

    rsx! {
        div {
            style: "
                width: 280px;
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
                    gap: 8px;
                ",
                if events.is_empty() {
                    div {
                        style: "color: {TEXT_DIM}; font-size: 11px;",
                        "no events yet"
                    }
                } else {
                    for (tag, message, color) in events {
                        div {
                            style: "display: flex; flex-direction: column; gap: 2px;",
                            span {
                                style: "color: {color}; font-size: 10px; letter-spacing: 0.08em;",
                                "{tag}"
                            }
                            span {
                                style: "color: {TEXT_PRIMARY}; font-size: 11px; line-height: 1.45;",
                                "{message}"
                            }
                        }
                    }
                }
            }

            div {
                style: "border-top: 1px solid {BORDER}; flex-shrink: 0;",
                PanelHeader { title: "APPROVALS" }
                div {
                    style: "padding: 8px 12px;",
                    div {
                        style: "color: {TEXT_DIM}; font-size: 11px;",
                        "no pending approvals in the active policy path"
                    }
                }
            }
        }
    }
}

fn render_event(event: &SessionEvent) -> Option<(String, String, &'static str)> {
    match event {
        SessionEvent::SessionStarted { .. } => Some((
            "SESSION".to_string(),
            "shell session opened".to_string(),
            TEXT_ACCENT,
        )),
        SessionEvent::SessionEnded { exit_code, .. } => Some((
            "SESSION".to_string(),
            format!("session ended with exit code {exit_code}"),
            TEXT_DIM,
        )),
        SessionEvent::UserInput { text, .. } => {
            Some(("INPUT".to_string(), text.clone(), TEXT_ACCENT))
        }
        SessionEvent::CommandInvoked { command, args, .. } => Some((
            "COMMAND".to_string(),
            if args.is_empty() {
                command.clone()
            } else {
                format!("{command} {}", args.join(" "))
            },
            TEXT_DIM,
        )),
        SessionEvent::CommandOutput { text, is_error, .. } => {
            if text.trim().is_empty() {
                None
            } else {
                Some((
                    if *is_error { "STDERR" } else { "STDOUT" }.to_string(),
                    text.clone(),
                    if *is_error { "#f85149" } else { TEXT_PRIMARY },
                ))
            }
        }
        SessionEvent::CommandDenied { command, .. } => Some((
            "DENIED".to_string(),
            format!("{command} was blocked by ROY"),
            "#f85149",
        )),
        SessionEvent::CommandNotFound { command, .. } => Some((
            "MISSING".to_string(),
            format!("{command} is not in the ROY command world"),
            "#f85149",
        )),
        SessionEvent::CwdChanged { to, .. } => {
            Some(("CWD".to_string(), to.display().to_string(), TEXT_DIM))
        }
        SessionEvent::HostNotice { message, .. } => {
            Some(("HOST".to_string(), message.clone(), TEXT_DIM))
        }
        SessionEvent::ArtifactCreated { artifact, .. } => Some((
            artifact_kind_label(&artifact.kind).to_string(),
            format!("{} · {}", artifact.name, artifact.summary),
            artifact_kind_color(&artifact.kind),
        )),
        SessionEvent::AgentOutput { text, .. } => {
            Some(("AGENT".to_string(), text.clone(), TEXT_PRIMARY))
        }
    }
}
