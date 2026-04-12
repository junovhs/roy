use crate::session::{Session, SessionEvent};
use crate::shell::DispatchResult;

use super::super::super::super::now_millis;

pub(crate) fn record_session_outcome(
    session: &mut Session,
    result: &DispatchResult,
    output_lines: &[String],
    error_lines: &[String],
) {
    let mut ts = now_millis();

    for line in output_lines {
        session.push(SessionEvent::CommandOutput {
            text: line.clone(),
            is_error: false,
            ts,
        });
        ts += 1;
    }

    for line in error_lines {
        session.push(SessionEvent::CommandOutput {
            text: line.clone(),
            is_error: true,
            ts,
        });
        ts += 1;
    }

    match result {
        DispatchResult::Denied {
            command,
            suggestion,
            artifacts,
        } => {
            for artifact in artifacts {
                session.push(SessionEvent::ArtifactCreated {
                    artifact: artifact.clone(),
                    ts,
                });
                ts += 1;
            }
            session.push(SessionEvent::CommandDenied {
                command: command.clone(),
                suggestion: suggestion.clone(),
                ts,
            });
        }
        DispatchResult::NotFound { command } => {
            session.push(SessionEvent::CommandNotFound {
                command: command.clone(),
                ts,
            });
        }
        DispatchResult::CwdChanged { to } => {
            session.push(SessionEvent::CwdChanged { to: to.clone(), ts });
        }
        DispatchResult::Exit { code } => session.end(*code, ts),
        DispatchResult::AgentStarted { .. } => {
            // launch message already recorded in output_lines above
        }
        DispatchResult::Executed { artifacts, .. } => {
            for artifact in artifacts {
                session.push(SessionEvent::ArtifactCreated {
                    artifact: artifact.clone(),
                    ts,
                });
                ts += 1;
            }
        }
    }
}
