use dioxus::prelude::*;

use crate::session::{Session, SessionEvent};
use crate::shell::{DispatchResult, ShellRuntime};

use super::super::now_millis;
use super::command_line::parse_command_line;

pub(super) const TEXT_ERROR: &str = "#f85149";

#[derive(Clone)]
pub(super) struct ShellLine {
    pub(super) prefix: String,
    pub(super) text: String,
    pub(super) is_error: bool,
}

impl ShellLine {
    pub(super) fn output(text: impl Into<String>) -> Self {
        Self {
            prefix: String::new(),
            text: text.into(),
            is_error: false,
        }
    }

    pub(super) fn error(text: impl Into<String>) -> Self {
        Self {
            prefix: String::new(),
            text: text.into(),
            is_error: true,
        }
    }

    pub(super) fn echo(prompt: impl Into<String>, text: impl Into<String>) -> Self {
        Self {
            prefix: prompt.into(),
            text: text.into(),
            is_error: false,
        }
    }
}

pub(super) fn initial_shell_lines() -> Vec<ShellLine> {
    vec![
        ShellLine::output("ROY - shell runtime ready"),
        ShellLine::output("type 'help' for available commands"),
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

pub(super) struct SubmitContext {
    pub(super) runtime: Signal<ShellRuntime>,
    pub(super) session: Signal<Session>,
    pub(super) lines: Signal<Vec<ShellLine>>,
    pub(super) input_text: Signal<String>,
}

pub(super) fn handle_submit(raw: String, pre_prompt: String, mut ctx: SubmitContext) {
    let ts = now_millis();
    ctx.session.write().push(SessionEvent::UserInput {
        text: raw.clone(),
        ts,
    });

    let parsed = match parse_command_line(&raw) { // neti:allow(LAW OF INTEGRITY) valid Rust; neti false positive
        Ok(parsed) => parsed,
        Err(message) => {
            let error_text = format!("parse error: {}", message);
            let ts = now_millis();

            ctx.session.write().push(SessionEvent::CommandOutput {
                text: error_text.clone(),
                is_error: true,
                ts,
            });

            ctx.lines.write().extend([
                ShellLine::echo(pre_prompt, raw),
                ShellLine::error(error_text),
            ]);

            ctx.input_text.set(String::new());
            return;
        }
    };

    let ts = now_millis();
    ctx.session.write().push(SessionEvent::CommandInvoked {
        command: parsed.command.clone(),
        args: parsed.args.clone(),
        ts,
    });

    let arg_refs: Vec<&str> = parsed.args.iter().map(String::as_str).collect();

    let (result, out, err) = {
        let mut runtime = ctx.runtime.write();
        let result = runtime.dispatch(&parsed.command, &arg_refs);
        let out = runtime.drain_output();
        let err = runtime.drain_errors();
        (result, out, err)
    };

    let mut new_lines = vec![ShellLine::echo(pre_prompt, raw)];
    let output_lines = flatten_chunks(out);
    let error_lines = flatten_chunks(err);

    for line in &output_lines {
        new_lines.push(ShellLine::output(line.clone()));
    }
    for line in &error_lines {
        new_lines.push(ShellLine::error(line.clone()));
    }

    record_session_outcome(
        &mut ctx.session.write(),
        &result,
        &output_lines,
        &error_lines,
    );

    if let DispatchResult::Exit { code } = result {
        new_lines.push(ShellLine::output(format!(
            "[session ended - exit code {}]",
            code
        )));
    }

    ctx.lines.write().extend(new_lines);
    ctx.input_text.set(String::new());
}
