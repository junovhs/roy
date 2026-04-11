use crate::session::{Session, SessionEvent};
use crate::shell::DispatchResult;

use super::super::now_millis;

pub(super) const TEXT_ERROR: &str = "#f85149";

#[derive(Clone)]
pub(super) struct ShellLine {
    pub(super) prefix: String,
    pub(super) text: String,
    pub(super) is_error: bool,
}

impl ShellLine {
    pub(super) fn output(text: impl Into<String>) -> Self {
        Self { prefix: String::new(), text: text.into(), is_error: false }
    }

    pub(super) fn error(text: impl Into<String>) -> Self {
        Self { prefix: String::new(), text: text.into(), is_error: true }
    }

    pub(super) fn echo(prompt: impl Into<String>, text: impl Into<String>) -> Self {
        Self { prefix: prompt.into(), text: text.into(), is_error: false }
    }
}

pub(super) fn initial_shell_lines() -> Vec<ShellLine> {
    vec![
        ShellLine::output("ROY — shell runtime ready"),
        ShellLine::output("type ‘help’ for available commands"),
    ]
}

pub(super) fn flatten_chunks(chunks: Vec<String>) -> Vec<String> {
    chunks
        .into_iter()
        .flat_map(|chunk| chunk.split('\n').map(str::to_string).collect::<Vec<_>>())
        .collect()
}

pub(super) fn record_session_outcome(
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
        DispatchResult::Denied { command, suggestion } => {
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
        DispatchResult::Executed { .. } => {}
    }
}
