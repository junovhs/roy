use dioxus::prelude::*;

use crate::session::{Session, SessionEvent};
use crate::shell::ShellRuntime;

#[path = "terminal_composer.rs"]
mod terminal_composer;
#[path = "terminal_line.rs"]
mod terminal_line;
#[path = "terminal_submit.rs"]
pub(super) mod terminal_submit;
#[path = "terminal_view.rs"]
mod terminal_view;

pub(super) const SURFACE: &str = "#16171a";
pub(super) const SURFACE_2: &str = "#1c1d21";
pub(super) const LINE: &str = "rgba(255,255,255,.06)";
pub(super) const INK: &str = "#e6e4df";
pub(super) const INK_DIM: &str = "#9b9892";
pub(super) const INK_FAINT: &str = "#5f5d58";
pub(super) const CORAL: &str = "#e87858";
pub(super) const CORAL_SOFT: &str = "#f09a7e";

pub(crate) struct SubmitContext {
    pub(crate) runtime: Signal<ShellRuntime>,
    pub(crate) session: Signal<Session>,
    pub(crate) lines: Signal<Vec<super::terminal_model::ShellLine>>,
    pub(crate) input_text: Signal<String>,
}

fn parse_submitted_command(
    raw: &str,
    pre_prompt: &str,
    ctx: &mut SubmitContext,
) -> Option<super::command_line::ParsedCommand> {
    let parsed = super::command_line::parse_command_line(raw);

    if let Err(message) = parsed.as_ref() {
        let error_text = format!("parse error: {}", message);
        let ts = super::super::now_millis();

        ctx.session.write().push(SessionEvent::CommandOutput {
            text: error_text.clone(),
            is_error: true,
            ts,
        });

        let mut lines = ctx.lines.write();
        lines.push(super::terminal_model::ShellLine::echo(
            pre_prompt.to_string(),
            raw.to_string(),
        ));
        lines.push(super::terminal_model::ShellLine::error(error_text));
        ctx.input_text.set(String::new());
        return None;
    }

    parsed.ok()
}

pub(crate) fn handle_submit(raw: String, pre_prompt: String, mut ctx: SubmitContext) {
    let ts = super::super::now_millis();
    ctx.session.write().push(SessionEvent::UserInput {
        text: raw.clone(),
        ts,
    });

    let parsed = if let Some(parsed) = parse_submitted_command(&raw, &pre_prompt, &mut ctx) {
        parsed
    } else {
        return;
    };

    let ts = super::super::now_millis();
    ctx.session.write().push(SessionEvent::CommandInvoked {
        command: parsed.command.clone(),
        args: parsed.args.clone(),
        ts,
    });

    let arg_refs: Vec<&str> = parsed.args.iter().map(String::as_str).collect();
    let mut runtime = ctx.runtime.write();
    let result = runtime.dispatch(&parsed.command, &arg_refs);
    let out = runtime.drain_output();
    let err = runtime.drain_errors();
    drop(runtime);

    let output_lines = super::terminal_model::flatten_chunks(out);
    let error_lines = super::terminal_model::flatten_chunks(err);
    let mut new_lines = vec![super::terminal_model::ShellLine::echo(pre_prompt, raw)];

    match &result {
        crate::shell::DispatchResult::Denied {
            command,
            suggestion,
            ..
        } => {
            new_lines.push(super::terminal_model::ShellLine::denial_header(command));
            if let Some(hint) = suggestion {
                if !hint.trim().is_empty() {
                    new_lines.push(super::terminal_model::ShellLine::denial_hint(hint));
                }
            }
        }
        crate::shell::DispatchResult::NotFound { command } => {
            new_lines.push(super::terminal_model::ShellLine::not_found(command));
        }
        crate::shell::DispatchResult::Exit { code } => {
            for line in &output_lines {
                new_lines.push(super::terminal_model::ShellLine::output(line.clone()));
            }
            for line in &error_lines {
                new_lines.push(super::terminal_model::ShellLine::error(line.clone()));
            }
            new_lines.push(super::terminal_model::ShellLine::output(format!(
                "[session ended · exit {}]",
                code
            )));
        }
        _ => {
            for line in &output_lines {
                new_lines.push(super::terminal_model::ShellLine::output(line.clone()));
            }
            for line in &error_lines {
                new_lines.push(super::terminal_model::ShellLine::error(line.clone()));
            }
        }
    }

    terminal_submit::record_session_outcome(
        &mut ctx.session.write(),
        &result,
        &output_lines,
        &error_lines,
    );

    ctx.lines.write().extend(new_lines);
    ctx.input_text.set(String::new());
}

pub(crate) use terminal_view::ShellPane;
