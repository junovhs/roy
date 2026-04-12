use dioxus::prelude::*;

use crate::session::{Session, SessionEvent};

#[cfg(test)]
use crate::session::{ArtifactBody, ArtifactKind, SessionArtifact};

use super::drawer_shell::DrawerShell;
use super::{CORAL_SOFT, INK, INK_DIM, INK_FAINT, LINE, MINT};

#[component]
pub(super) fn ActivityDrawer(
    open_drawer: Signal<Option<&'static str>>,
    session: Signal<Session>,
) -> Element {
    let session = session.read();
    let events: Vec<(String, String, &'static str)> = session
        .events()
        .iter()
        .rev()
        .filter_map(event_row)
        .take(20)
        .collect();

    rsx! {
        DrawerShell { name: "activity", title: "Activity", subtitle: "Session", open_drawer,
            if events.is_empty() {
                div { style: "color: {INK_FAINT}; font-size: 13px;", "No events yet." }
            } else {
                for (tag, msg, color) in events {
                    div {
                        style: "display:flex;gap:14px;padding:10px 0;border-bottom:1px solid {LINE};",
                        span {
                            style: "font-family:'JetBrains Mono',monospace;color:{INK_FAINT};font-size:12px;min-width:48px;padding-top:1px;",
                            "{tag}"
                        }
                        span {
                            style: "font-size:14px;color:{color};flex:1;line-height:1.45;",
                            "{msg}"
                        }
                    }
                }
            }
        }
    }
}

fn event_row(event: &SessionEvent) -> Option<(String, String, &'static str)> {
    match event {
        SessionEvent::SessionStarted { .. } => Some((
            "SESSION".to_string(),
            "shell session opened".to_string(),
            MINT,
        )),
        SessionEvent::SessionEnded { exit_code, .. } => Some((
            "SESSION".to_string(),
            format!("ended · exit {exit_code}"),
            INK_FAINT,
        )),
        SessionEvent::UserInput { text, .. } => {
            Some(("INPUT".to_string(), text.clone(), CORAL_SOFT))
        }
        SessionEvent::CommandInvoked { command, args, .. } => Some((
            "CMD".to_string(),
            if args.is_empty() {
                command.clone()
            } else {
                format!("{command} {}", args.join(" "))
            },
            INK_DIM,
        )),
        SessionEvent::CommandOutput { text, is_error, .. } => {
            if text.trim().is_empty() {
                None
            } else {
                Some((
                    if *is_error { "STDERR" } else { "STDOUT" }.to_string(),
                    text.clone(),
                    if *is_error { "#f85149" } else { INK },
                ))
            }
        }
        SessionEvent::CommandDenied { command, .. } => Some((
            "DENIED".to_string(),
            format!("{command} blocked"),
            "#f85149",
        )),
        SessionEvent::CommandNotFound { command, .. } => Some((
            "MISSING".to_string(),
            format!("{command} not in ROY world"),
            "#f85149",
        )),
        SessionEvent::CwdChanged { to, .. } => {
            Some(("CWD".to_string(), to.display().to_string(), INK_DIM))
        }
        SessionEvent::HostNotice { message, .. } => {
            Some(("HOST".to_string(), message.clone(), INK_DIM))
        }
        SessionEvent::ArtifactCreated { artifact, .. } => Some((
            "ARTIFACT".to_string(),
            format!("{} · {}", artifact.name, artifact.summary),
            MINT,
        )),
        SessionEvent::AgentOutput { text, .. } => Some(("AGENT".to_string(), text.clone(), INK)),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn event_row_formats_invoked_command_with_args() {
        let row = event_row(&SessionEvent::CommandInvoked {
            command: "read".to_string(),
            args: vec!["Cargo.toml".to_string(), "--json".to_string()],
            ts: 12,
        });

        assert_eq!(
            row,
            Some((
                "CMD".to_string(),
                "read Cargo.toml --json".to_string(),
                INK_DIM,
            ))
        );
    }

    #[test]
    fn event_row_discards_blank_output_lines() {
        let row = event_row(&SessionEvent::CommandOutput {
            text: "   ".to_string(),
            is_error: false,
            ts: 12,
        });

        assert_eq!(row, None);
    }

    #[test]
    fn event_row_formats_artifacts_with_name_and_summary() {
        let row = event_row(&SessionEvent::ArtifactCreated {
            artifact: SessionArtifact {
                name: "neti-report.txt".to_string(),
                kind: ArtifactKind::Note,
                summary: "validation output".to_string(),
                body: ArtifactBody::Note {
                    text: "details".to_string(),
                },
            },
            ts: 13,
        });

        assert_eq!(
            row,
            Some((
                "ARTIFACT".to_string(),
                "neti-report.txt · validation output".to_string(),
                MINT,
            ))
        );
    }

    #[test]
    fn event_row_marks_missing_commands_as_errors() {
        let row = event_row(&SessionEvent::CommandNotFound {
            command: "unknown".to_string(),
            ts: 21,
        });

        assert_eq!(
            row,
            Some((
                "MISSING".to_string(),
                "unknown not in ROY world".to_string(),
                "#f85149",
            ))
        );
    }
}
