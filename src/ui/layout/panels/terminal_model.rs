use dioxus::prelude::*;

use crate::session::{Session, SessionEvent};
use crate::shell::{DispatchResult, ShellRuntime};

use super::super::now_millis;
use super::command_line::parse_command_line;

pub(super) const TEXT_ERROR: &str = "#f85149";

// ── line kind ─────────────────────────────────────────────────────────────────

/// Visual classification of a terminal line, driving rendering in terminal.rs.
#[derive(Clone, Debug, PartialEq)]
pub(super) enum LineKind {
    /// Normal command output.
    Output,
    /// Raw stderr / generic error.
    Error,
    /// Input echo (prompt + command).
    Echo,
    /// First line of a denial block: "⊘ command — blocked".
    DenialHeader,
    /// Continuation of a denial: the ROY-world suggestion.
    DenialHint,
    /// Command not in the ROY registry.
    NotFound,
}

// ── shell line ────────────────────────────────────────────────────────────────

#[derive(Clone)]
pub(super) struct ShellLine {
    pub(super) prefix: String,
    pub(super) text: String,
    pub(super) kind: LineKind,
}

impl ShellLine {
    pub(super) fn output(text: impl Into<String>) -> Self {
        Self { prefix: String::new(), text: text.into(), kind: LineKind::Output }
    }

    pub(super) fn error(text: impl Into<String>) -> Self {
        Self { prefix: String::new(), text: text.into(), kind: LineKind::Error }
    }

    pub(super) fn echo(prompt: impl Into<String>, text: impl Into<String>) -> Self {
        Self { prefix: prompt.into(), text: text.into(), kind: LineKind::Echo }
    }

    /// Denial block header — "⊘ {command} — blocked by policy".
    pub(super) fn denial_header(command: impl Into<String>) -> Self {
        Self {
            prefix: "⊘".into(),
            text: format!("{} — blocked by policy", command.into()),
            kind: LineKind::DenialHeader,
        }
    }

    /// Denial suggestion — the ROY-world alternative.
    pub(super) fn denial_hint(hint: impl Into<String>) -> Self {
        Self { prefix: "→".into(), text: hint.into(), kind: LineKind::DenialHint }
    }

    /// Command not found in the ROY registry.
    pub(super) fn not_found(command: impl Into<String>) -> Self {
        Self {
            prefix: "?".into(),
            text: format!(
                "{} — not in the ROY world  ·  run `help` to see available commands",
                command.into()
            ),
            kind: LineKind::NotFound,
        }
    }
}

// ── helpers ───────────────────────────────────────────────────────────────────

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

// ── submit context ────────────────────────────────────────────────────────────

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

    let output_lines = flatten_chunks(out);
    let error_lines = flatten_chunks(err);

    // Build display lines — denial and not-found get structured output;
    // everything else gets the raw output/error streams.
    let mut new_lines = vec![ShellLine::echo(pre_prompt, raw)];

    match &result {
        DispatchResult::Denied { command, suggestion, .. } => {
            // Skip raw error_lines — they duplicate the denial message.
            // Emit a structured two-line denial block instead.
            new_lines.push(ShellLine::denial_header(command));
            if let Some(hint) = suggestion {
                if !hint.trim().is_empty() {
                    new_lines.push(ShellLine::denial_hint(hint));
                }
            }
        }
        DispatchResult::NotFound { command } => {
            new_lines.push(ShellLine::not_found(command));
        }
        DispatchResult::Exit { code } => {
            for line in &output_lines {
                new_lines.push(ShellLine::output(line.clone()));
            }
            for line in &error_lines {
                new_lines.push(ShellLine::error(line.clone()));
            }
            new_lines.push(ShellLine::output(format!(
                "[session ended · exit {}]",
                code
            )));
        }
        _ => {
            for line in &output_lines {
                new_lines.push(ShellLine::output(line.clone()));
            }
            for line in &error_lines {
                new_lines.push(ShellLine::error(line.clone()));
            }
        }
    }

    record_session_outcome(
        &mut ctx.session.write(),
        &result,
        &output_lines,
        &error_lines,
    );

    ctx.lines.write().extend(new_lines);
    ctx.input_text.set(String::new());
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::session::SessionArtifact;
    use dioxus::{
        core::RuntimeGuard,
        prelude::{rsx, Element, ScopeId, VirtualDom},
    };

    fn make_session() -> Session {
        Session::new(101, std::env::temp_dir(), 10)
    }

    fn with_runtime<T>(f: impl FnOnce() -> T) -> T {
        fn app() -> Element {
            rsx! { div {} }
        }

        let mut dom = VirtualDom::new(app);
        dom.rebuild_to_vec();
        let _guard = RuntimeGuard::new(dom.runtime());
        f()
    }

    type TestCtx = (
        Signal<ShellRuntime>,
        Signal<Session>,
        Signal<Vec<ShellLine>>,
        Signal<String>,
        SubmitContext,
    );

    fn make_ctx() -> TestCtx {
        let runtime = Signal::new_in_scope(ShellRuntime::new(std::env::temp_dir()), ScopeId::APP);
        let session = Signal::new_in_scope(make_session(), ScopeId::APP);
        let lines = Signal::new_in_scope(Vec::new(), ScopeId::APP);
        let input_text = Signal::new_in_scope("pending".to_string(), ScopeId::APP);
        let ctx = SubmitContext {
            runtime,
            session,
            lines,
            input_text,
        };
        (runtime, session, lines, input_text, ctx)
    }

    #[test]
    fn initial_shell_lines_show_banner_and_help_hint() {
        let lines = initial_shell_lines();

        assert_eq!(lines.len(), 2);
        assert_eq!(lines[0].text, "ROY - shell runtime ready");
        assert_eq!(lines[0].kind, LineKind::Output);
        assert_eq!(lines[1].text, "type 'help' for available commands");
        assert_eq!(lines[1].kind, LineKind::Output);
    }

    #[test]
    fn flatten_chunks_splits_multiline_output_and_preserves_blanks() {
        let flattened = flatten_chunks(vec!["alpha\nbeta".to_string(), String::new()]);

        assert_eq!(
            flattened,
            vec!["alpha".to_string(), "beta".to_string(), String::new()]
        );
    }

    #[test]
    fn record_session_outcome_orders_denials_after_output_and_artifacts() {
        let mut session = make_session();
        let artifact = SessionArtifact::denied_command("bash", &["-lc", "pwd"], "blocked".into());
        let result = DispatchResult::Denied {
            command: "bash".to_string(),
            suggestion: Some("Use `help`.".to_string()),
            artifacts: vec![artifact.clone()],
        };
        let output_lines = vec!["stdout line".to_string()];
        let error_lines = vec!["stderr line".to_string()];

        record_session_outcome(&mut session, &result, &output_lines, &error_lines);

        let events = session.events();
        let tail = &events[events.len() - 4..];
        assert!(matches!(
            &tail[0],
            SessionEvent::CommandOutput {
                text,
                is_error: false,
                ..
            } if text == "stdout line"
        ));
        assert!(matches!(
            &tail[1],
            SessionEvent::CommandOutput {
                text,
                is_error: true,
                ..
            } if text == "stderr line"
        ));
        assert!(matches!(
            &tail[2],
            SessionEvent::ArtifactCreated { artifact: a, .. } if a == &artifact
        ));
        assert!(matches!(
            &tail[3],
            SessionEvent::CommandDenied {
                command,
                suggestion,
                ..
            } if command == "bash" && suggestion.as_deref() == Some("Use `help`.")
        ));

        let timestamps: Vec<_> = tail.iter().map(SessionEvent::timestamp).collect();
        assert_eq!(
            timestamps,
            vec![
                timestamps[0],
                timestamps[0] + 1,
                timestamps[0] + 2,
                timestamps[0] + 3
            ]
        );
    }

    #[test]
    fn record_session_outcome_closes_session_on_exit() {
        let mut session = make_session();

        record_session_outcome(
            &mut session,
            &DispatchResult::Exit { code: 7 },
            &[],
            &[],
        );

        assert!(matches!(
            session.events().last(),
            Some(SessionEvent::SessionEnded { exit_code: 7, .. })
        ));
    }

    #[test]
    fn record_session_outcome_increments_timestamps_for_multiple_executed_artifacts() {
        let mut session = make_session();
        let first = SessionArtifact::validation_run(
            "cargo check".to_string(),
            std::env::temp_dir(),
            0,
            "ok".to_string(),
            String::new(),
        );
        let second = SessionArtifact::validation_run(
            "cargo test".to_string(),
            std::env::temp_dir(),
            0,
            "ok".to_string(),
            String::new(),
        );

        record_session_outcome(
            &mut session,
            &DispatchResult::Executed {
                output: "ok".to_string(),
                exit_code: 0,
                artifacts: vec![first.clone(), second.clone()],
            },
            &[],
            &[],
        );

        let events = session.events();
        let tail = &events[events.len() - 2..];
        assert!(matches!(
            &tail[0],
            SessionEvent::ArtifactCreated { artifact, .. } if artifact == &first
        ));
        assert!(matches!(
            &tail[1],
            SessionEvent::ArtifactCreated { artifact, .. } if artifact == &second
        ));

        let timestamps: Vec<_> = tail.iter().map(SessionEvent::timestamp).collect();
        assert_eq!(timestamps[1], timestamps[0] + 1);
    }

    #[test]
    fn handle_submit_parse_error_records_echo_and_error() {
        with_runtime(|| {
            let (_runtime, session, lines, input_text, ctx) = make_ctx();

            handle_submit("\"unterminated".to_string(), "roy> ".to_string(), ctx);

            assert_eq!(&*input_text.read(), "");
            let rendered = lines.read();
            assert_eq!(rendered.len(), 2);
            assert_eq!(rendered[0].prefix, "roy> ");
            assert_eq!(rendered[0].text, "\"unterminated");
            assert_eq!(rendered[0].kind, LineKind::Echo);
            assert_eq!(rendered[1].kind, LineKind::Error);
            assert!(rendered[1].text.contains("parse error"));

            let events = session.read();
            assert_eq!(events.events_of_kind("command_invoked").len(), 0);
            assert!(matches!(
                events.events().last(),
                Some(SessionEvent::CommandOutput { is_error: true, .. })
            ));
        });
    }

    #[test]
    fn handle_submit_denied_command_renders_structured_denial() {
        with_runtime(|| {
            let (_runtime, session, lines, input_text, ctx) = make_ctx();

            handle_submit("bash".to_string(), "roy> ".to_string(), ctx);

            assert_eq!(&*input_text.read(), "");
            let rendered = lines.read();
            assert_eq!(rendered.len(), 3);
            assert_eq!(rendered[0].kind, LineKind::Echo);
            assert_eq!(rendered[1].kind, LineKind::DenialHeader);
            assert!(rendered[1].text.contains("bash"));
            assert_eq!(rendered[2].kind, LineKind::DenialHint);
            assert!(!rendered[2].text.trim().is_empty());

            let events = session.read();
            assert_eq!(events.events_of_kind("artifact_created").len(), 1);
            assert_eq!(events.events_of_kind("command_denied").len(), 1);
            assert_eq!(events.events_of_kind("command_output").len(), 1);
        });
    }

    #[test]
    fn handle_submit_not_found_renders_registry_hint() {
        with_runtime(|| {
            let (_runtime, session, lines, _input_text, ctx) = make_ctx();

            handle_submit("not_a_real_roy_command".to_string(), "roy> ".to_string(), ctx);

            let rendered = lines.read();
            assert_eq!(rendered.len(), 2);
            assert_eq!(rendered[1].kind, LineKind::NotFound);
            assert!(rendered[1].text.contains("help"));

            let events = session.read();
            assert_eq!(events.events_of_kind("command_not_found").len(), 1);
            assert_eq!(events.events_of_kind("command_output").len(), 1);
        });
    }

    #[test]
    fn handle_submit_exit_renders_session_end_notice() {
        with_runtime(|| {
            let (_runtime, session, lines, _input_text, ctx) = make_ctx();

            handle_submit("exit 7".to_string(), "roy> ".to_string(), ctx);

            let rendered = lines.read();
            assert_eq!(rendered.len(), 2);
            assert_eq!(rendered[1].kind, LineKind::Output);
            assert_eq!(rendered[1].text, "[session ended · exit 7]");

            let events = session.read();
            assert!(matches!(
                events.events().last(),
                Some(SessionEvent::SessionEnded { exit_code: 7, .. })
            ));
        });
    }

    #[test]
    fn handle_submit_success_renders_runtime_output() {
        with_runtime(|| {
            let (_runtime, session, lines, _input_text, ctx) = make_ctx();

            handle_submit("pwd".to_string(), "roy> ".to_string(), ctx);

            let rendered = lines.read();
            assert_eq!(rendered.len(), 2);
            assert_eq!(rendered[1].kind, LineKind::Output);
            assert!(!rendered[1].text.trim().is_empty());

            let events = session.read();
            assert_eq!(events.events_of_kind("command_invoked").len(), 1);
            assert_eq!(events.events_of_kind("command_output").len(), 1);
        });
    }
}
