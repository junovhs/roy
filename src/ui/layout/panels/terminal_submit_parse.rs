use dioxus::prelude::*;

use crate::session::SessionEvent;

use super::super::super::command_line::ParsedCommand;
use super::super::super::terminal_model::ShellLine;
use super::super::SubmitContext;

pub(super) fn parse_submitted_command(
    raw: &str,
    pre_prompt: &str,
    ctx: &mut SubmitContext,
) -> Option<ParsedCommand> {
    let parsed = super::super::super::command_line::parse_command_line(raw);

    if let Err(message) = parsed.as_ref() {
        let error_text = format!("parse error: {}", message);
        let ts = super::super::super::super::now_millis();

        ctx.session.write().push(SessionEvent::CommandOutput {
            text: error_text.clone(),
            is_error: true,
            ts,
        });

        let mut lines = ctx.lines.write();
        lines.push(ShellLine::echo(pre_prompt.to_string(), raw.to_string()));
        lines.push(ShellLine::error(error_text));
        ctx.input_text.set(String::new());
        return None;
    }

    parsed.ok()
}
