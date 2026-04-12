use dioxus::prelude::*;

use crate::session::SessionEvent;

use super::super::super::terminal_model::{self, ShellLine};
use super::super::SubmitContext;

pub(crate) fn handle_submit(raw: String, pre_prompt: String, mut ctx: SubmitContext) {
    let ts = super::super::super::super::now_millis();
    ctx.session.write().push(SessionEvent::UserInput {
        text: raw.clone(),
        ts,
    });

    if ctx.runtime.read().agent_active() {
        let result = ctx.runtime.write().send_agent_input(&raw);
        let mut new_lines = vec![ShellLine::echo(pre_prompt, raw)];

        if let Err(message) = result {
            let ts = super::super::super::super::now_millis();
            ctx.session.write().push(SessionEvent::CommandOutput {
                text: message.clone(),
                is_error: true,
                ts,
            });
            new_lines.push(ShellLine::error(message));
        }

        ctx.lines.write().extend(new_lines);
        ctx.input_text.set(String::new());
        return;
    }

    let parsed =
        if let Some(parsed) = super::parse::parse_submitted_command(&raw, &pre_prompt, &mut ctx) {
            parsed
        } else {
            return;
        };

    let ts = super::super::super::super::now_millis();
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

    let output_lines = terminal_model::flatten_chunks(out);
    let error_lines = terminal_model::flatten_chunks(err);
    let mut new_lines = vec![ShellLine::echo(pre_prompt, raw)];
    super::lines::append_dispatch_lines(&mut new_lines, &result, &output_lines, &error_lines);

    super::record::record_session_outcome(
        &mut ctx.session.write(),
        &result,
        &output_lines,
        &error_lines,
    );

    ctx.lines.write().extend(new_lines);
    ctx.input_text.set(String::new());
}
